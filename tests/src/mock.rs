//! Mock account state for the unit tests.
//!
//! A unit test runs exactly one instruction. Every account that instruction
//! reads is written straight into the SVM here (`LiteSVM::set_account`), at
//! its real PDA, so a test never depends on another handler working. The
//! integration tests use the `setup_*` / `ready_*` chains in
//! [`crate::fixture`] instead, which build the same state by sending the real
//! instructions.
//!
//! Each builder has a plain form that writes sensible defaults and a `_with`
//! form that lets the test edit the struct before it is written. Builders
//! keep the mirrors the program relies on: a vault's token balance always
//! equals `Reserve.balance` / `TraderMargin.balance`, and
//! [`TestEnv::mock_position`] adds its locks to `TraderMargin`, `Reserve`
//! and `Market` the way IX-18 would.
//!
//! Figures the program would compute with `fra_math` functions that are
//! still `todo!()` (lock, commit, fixed rate, period) use simple stand-ins
//! here; they are only inputs, and a test can override any of them.

use anchor_lang::{prelude::Pubkey, AccountSerialize, Space};
use saphire::{
    Config, DailyRate, Eligibility, Market, MarketParams, MarketStatus, OperatorRegistration,
    Origin, Position, PositionStatus, Quote, Reserve, TraderMargin, UsedNonce,
};
use saphire_client::types::Side;
use solana_account::Account;
use solana_keypair::Keypair;
use solana_signer::Signer;

use crate::{
    fixture::{
        default_market_params, program_side, TestEnv, TestTrader, PROTOCOL_FEE_BPS,
        QUOTE_HALF_SPREAD_BPS, QUOTE_MAX_NOTIONAL, QUOTE_MID, RATE_CEILING, TEST_MARGIN,
        TEST_RESERVE, TRADER_STARTING_USDC,
    },
    pda,
};

/// Default mocked period start: Monday 2026-04-06 00:00 UTC (3 months after
/// [`crate::fixture::START_TS`]).
pub const MOCK_PERIOD_START: i64 = 1_775_433_600;
/// Default mocked period end: Monday 2026-07-06 00:00 UTC.
pub const MOCK_PERIOD_END: i64 = 1_783_296_000;
/// Days between [`MOCK_PERIOD_START`] and [`MOCK_PERIOD_END`].
pub const MOCK_PERIOD_DAYS: u16 = 91;

/// `ceil(amount × bps / 10_000)`: stand-in for `fra_math::bps_ceil` (still
/// `todo!()`), used for mocked locks and commits.
pub fn mock_bps_ceil(amount: u64, bps: u16) -> u64 {
    let product = u128::from(amount) * u128::from(bps);
    u64::try_from(product.div_ceil(u128::from(fra_math::BPS_DENOMINATOR))).expect("bps_ceil fits")
}

/// Quote-priced fixed rate for `side` at [`QUOTE_MID`] ± [`QUOTE_HALF_SPREAD_BPS`]:
/// stand-in for `fra_math::compute_fixed_rate` (still `todo!()`).
pub fn mock_fixed_rate(side: Side) -> u64 {
    let spread = u64::from(QUOTE_HALF_SPREAD_BPS) * fra_math::RATE_PER_BP;
    match side {
        Side::PayFixed => QUOTE_MID + spread,
        Side::ReceiveFixed => QUOTE_MID - spread,
    }
}

impl TestEnv {
    // ── Raw writes ─────────────────────────────────────────────────────────

    /// Write `state` as a rent-exempt Saphire-owned account at `address`
    /// (discriminator + Borsh, sized `8 + INIT_SPACE` like `init` would).
    pub fn write_account<T: AccountSerialize + Space>(&mut self, address: Pubkey, state: &T) {
        let mut data = Vec::with_capacity(8 + T::INIT_SPACE);
        state.try_serialize(&mut data).expect("serialize account");
        data.resize(8 + T::INIT_SPACE, 0);
        let lamports = self.svm.minimum_balance_for_rent_exemption(data.len());
        self.svm
            .set_account(
                address,
                Account {
                    lamports,
                    data,
                    owner: saphire::ID,
                    executable: false,
                    rent_epoch: 0,
                },
            )
            .expect("set program account");
    }

    // ── Config / operator ──────────────────────────────────────────────────

    /// Config (authority = `admin`, [`PROTOCOL_FEE_BPS`], [`RATE_CEILING`],
    /// not paused) and an empty fee vault owned by the Config PDA.
    pub fn mock_config(&mut self) -> Config {
        self.mock_config_with(|_| {})
    }

    pub fn mock_config_with(&mut self, edit: impl FnOnce(&mut Config)) -> Config {
        let (config_addr, bump) = pda::config();
        let fee_vault = pda::fee_vault().0;
        let mut config = Config {
            authority: self.admin.pubkey(),
            protocol_fee_bps: PROTOCOL_FEE_BPS,
            fee_vault,
            rate_ceiling: RATE_CEILING,
            paused: false,
            bump,
            _reserved: [0; 32],
        };
        edit(&mut config);
        self.write_account(config_addr, &config);
        if !self.exists(&fee_vault) {
            let mint = self.mint;
            self.write_token_account(fee_vault, &mint, &config_addr, 0);
        }
        config
    }

    /// Active registration for `operator`.
    pub fn mock_operator_registration(&mut self) -> OperatorRegistration {
        self.mock_operator_registration_with(|_| {})
    }

    pub fn mock_operator_registration_with(
        &mut self,
        edit: impl FnOnce(&mut OperatorRegistration),
    ) -> OperatorRegistration {
        let operator = self.operator.pubkey();
        let (address, bump) = pda::operator_registration(&operator);
        let mut reg = OperatorRegistration {
            operator,
            active: true,
            bump,
            _reserved: [0; 32],
        };
        edit(&mut reg);
        self.write_account(address, &reg);
        reg
    }

    // ── Market / reserve / quote ───────────────────────────────────────────

    /// Market from `self.params`, as IX-05 leaves it: Active, no positions,
    /// `rate_index = 10^18`, `last_rate_date = index_start_date`.
    pub fn mock_market(&mut self) -> Market {
        self.mock_market_with(|_| {})
    }

    pub fn mock_market_with(&mut self, edit: impl FnOnce(&mut Market)) -> Market {
        let p = self.params.clone();
        let address = self.market();
        let bump = pda::market(
            p.benchmark_id,
            &self.mint,
            p.start_offset_months,
            p.tenor_months,
        )
        .1;
        let mut market = Market {
            operator: self.operator.pubkey(),
            keeper: self.keeper,
            benchmark_id: p.benchmark_id,
            settlement_mint: self.mint,
            start_offset_months: p.start_offset_months,
            tenor_months: p.tenor_months,
            day_count_basis: p.day_count_basis,
            business_day_convention: p.business_day_convention,
            initial_margin_bps: p.initial_margin_bps,
            operator_commit_bps: p.operator_commit_bps,
            min_notional: p.min_notional,
            max_notional: p.max_notional,
            open_interest_cap: p.open_interest_cap,
            open_interest: 0,
            quote_validity_secs: p.quote_validity_secs,
            rate_grace_days: p.rate_grace_days,
            early_close_fee_bps: p.early_close_fee_bps,
            max_half_spread_bps: p.max_half_spread_bps,
            settlement_fee: p.settlement_fee,
            status: MarketStatus::Active,
            position_count: 0,
            rate_index: saphire::constants::INDEX_SCALE,
            last_rate_date: p.index_start_date,
            bump,
            _reserved: [0; 32],
        };
        edit(&mut market);
        self.write_account(address, &market);
        market
    }

    /// Reserve holding `balance` (nothing committed), with its vault funded
    /// to match.
    pub fn mock_reserve(&mut self, balance: u64) -> Reserve {
        self.mock_reserve_with(|r| r.balance = balance)
    }

    /// The vault is written with the edited `balance`, so the mirror holds.
    pub fn mock_reserve_with(&mut self, edit: impl FnOnce(&mut Reserve)) -> Reserve {
        let market = self.market();
        let (address, bump) = pda::reserve(&market);
        let vault = pda::reserve_vault(&market).0;
        let mut reserve = Reserve {
            market,
            vault,
            balance: 0,
            committed: 0,
            bump,
            _reserved: [0; 32],
        };
        edit(&mut reserve);
        self.write_account(address, &reserve);
        let mint = self.mint;
        self.write_token_account(vault, &mint, &address, reserve.balance);
        reserve
    }

    /// Quote as IX-05 leaves it: epoch 0, never published.
    pub fn mock_unpublished_quote(&mut self) -> Quote {
        self.mock_quote_with(|q| {
            q.fixed_mid = 0;
            q.half_spread_bps = 0;
            q.max_notional_at_quote = 0;
            q.published_at = 0;
            q.epoch = 0;
        })
    }

    /// Fresh quote: [`QUOTE_MID`] ± [`QUOTE_HALF_SPREAD_BPS`], cap
    /// [`QUOTE_MAX_NOTIONAL`], published now, epoch 1.
    pub fn mock_quote(&mut self) -> Quote {
        self.mock_quote_with(|_| {})
    }

    pub fn mock_quote_with(&mut self, edit: impl FnOnce(&mut Quote)) -> Quote {
        let market = self.market();
        let (address, bump) = pda::quote(&market);
        let mut quote = Quote {
            market,
            fixed_mid: QUOTE_MID,
            half_spread_bps: QUOTE_HALF_SPREAD_BPS,
            max_notional_at_quote: QUOTE_MAX_NOTIONAL,
            published_at: self.now(),
            epoch: 1,
            bump,
            _reserved: [0; 32],
        };
        edit(&mut quote);
        self.write_account(address, &quote);
        quote
    }

    // ── Traders ────────────────────────────────────────────────────────────

    /// Funded trader wallet with a settlement-mint token account holding
    /// [`TRADER_STARTING_USDC`]. No Saphire accounts.
    pub fn mock_wallet(&mut self) -> TestTrader {
        let kp: Keypair = self.new_funded_keypair();
        let token_account = self.create_token_account(&kp.pubkey(), TRADER_STARTING_USDC);
        TestTrader { kp, token_account }
    }

    /// Active eligibility for `trader` on this market.
    pub fn mock_eligibility(&mut self, trader: &Pubkey) -> Eligibility {
        self.mock_eligibility_with(trader, |_| {})
    }

    pub fn mock_eligibility_with(
        &mut self,
        trader: &Pubkey,
        edit: impl FnOnce(&mut Eligibility),
    ) -> Eligibility {
        let market = self.market();
        let (address, bump) = pda::eligibility(&market, trader);
        let mut elig = Eligibility {
            market,
            trader: *trader,
            active: true,
            bump,
            _reserved: [0; 32],
        };
        edit(&mut elig);
        self.write_account(address, &elig);
        elig
    }

    /// TraderMargin holding `balance` (nothing locked), with its vault funded
    /// to match.
    pub fn mock_trader_margin(&mut self, trader: &Pubkey, balance: u64) -> TraderMargin {
        self.mock_trader_margin_with(trader, |m| m.balance = balance)
    }

    /// The vault is written with the edited `balance`, so the mirror holds.
    pub fn mock_trader_margin_with(
        &mut self,
        trader: &Pubkey,
        edit: impl FnOnce(&mut TraderMargin),
    ) -> TraderMargin {
        let market = self.market();
        let (address, bump) = pda::trader_margin(&market, trader);
        let vault = pda::margin_vault(&market, trader).0;
        let mut margin = TraderMargin {
            market,
            trader: *trader,
            vault,
            balance: 0,
            locked: 0,
            open_positions: 0,
            bump,
            _reserved: [0; 32],
        };
        edit(&mut margin);
        self.write_account(address, &margin);
        let mint = self.mint;
        self.write_token_account(vault, &mint, &address, margin.balance);
        margin
    }

    /// [`TestEnv::mock_wallet`] + eligibility. No margin account.
    pub fn mock_admitted_trader(&mut self) -> TestTrader {
        let trader = self.mock_wallet();
        self.mock_eligibility(&trader.pubkey());
        trader
    }

    /// [`TestEnv::mock_admitted_trader`] + a margin account holding `margin`.
    pub fn mock_trader_with_margin(&mut self, margin: u64) -> TestTrader {
        let trader = self.mock_admitted_trader();
        self.mock_trader_margin(&trader.pubkey(), margin);
        trader
    }

    // ── Positions ──────────────────────────────────────────────────────────

    /// Open quote-origin position in the next slot (`Market.position_count`),
    /// with the aggregates updated as IX-18 would. Returns the nonce.
    ///
    /// Needs the market, reserve and the trader's margin account mocked first.
    pub fn mock_position(&mut self, trader: &Pubkey, side: Side, notional: u64) -> u64 {
        self.mock_position_with(trader, side, notional, |_| {})
            .nonce
    }

    /// As [`TestEnv::mock_position`]; `edit` runs before the aggregates are
    /// updated, so they reflect the edited figures. A position edited to a
    /// non-Open status leaves the aggregates alone (only `position_count`
    /// moves).
    pub fn mock_position_with(
        &mut self,
        trader: &Pubkey,
        side: Side,
        notional: u64,
        edit: impl FnOnce(&mut Position),
    ) -> Position {
        let market_addr = self.market();
        let mut market = self.market_state();
        let nonce = market.position_count;
        let (address, bump) = pda::position(&market_addr, trader, nonce);
        let mut position = Position {
            market: market_addr,
            trader: *trader,
            nonce,
            side: program_side(side),
            notional,
            fixed_rate: mock_fixed_rate(side),
            trade_date: self.now(),
            period_start: MOCK_PERIOD_START,
            period_end: MOCK_PERIOD_END,
            day_count_days: MOCK_PERIOD_DAYS,
            locked_margin: mock_bps_ceil(notional, market.initial_margin_bps),
            committed_reserve: mock_bps_ceil(notional, market.operator_commit_bps),
            fee_paid: 0,
            settlement_fee: market.settlement_fee,
            origin: Origin::Quote,
            status: PositionStatus::Open,
            settled_amount: 0,
            settled_at: 0,
            bump,
            _reserved: [0; 32],
        };
        edit(&mut position);
        self.write_account(address, &position);

        market.position_count += 1;
        if position.status == PositionStatus::Open {
            market.open_interest += position.notional;

            let reserve_addr = pda::reserve(&market_addr).0;
            let mut reserve = self.reserve_state();
            reserve.committed += position.committed_reserve;
            self.write_account(reserve_addr, &reserve);

            let margin_addr = pda::trader_margin(&market_addr, trader).0;
            let mut margin = self.margin_state(trader);
            margin.locked += position.locked_margin + position.settlement_fee;
            margin.open_positions += 1;
            self.write_account(margin_addr, &margin);
        }
        self.write_account(market_addr, &market);
        position
    }

    /// UsedNonce for an RFQ deal `nonce` that opened `position`.
    pub fn mock_used_nonce(&mut self, trader: &Pubkey, nonce: u64, position: &Pubkey) -> UsedNonce {
        let market = self.market();
        let (address, bump) = pda::used_nonce(&market, trader, nonce);
        let used = UsedNonce {
            market,
            trader: *trader,
            nonce,
            position: *position,
            bump,
            _reserved: [0; 32],
        };
        self.write_account(address, &used);
        used
    }

    // ── Daily rates ────────────────────────────────────────────────────────

    /// DailyRate checkpoint for `date` chained from `Market.rate_index`
    /// (`days_applied` from the calendar, `index_after` via
    /// `fra_math::compound_index`), and the market advanced to it as IX-21
    /// would.
    pub fn mock_daily_rate(&mut self, date: i64, rate: u64) -> DailyRate {
        let next =
            fra_math::next_business_day(date, &fra_math::WeekendOnly).expect("next business day");
        let days_applied = fra_math::days_until(date, next).expect("days_applied");
        let market_addr = self.market();
        let mut market = self.market_state();
        let index_before = market.rate_index;
        let index_after =
            fra_math::compound_index(index_before, rate, days_applied).expect("compound_index");
        let (address, bump) = pda::daily_rate(&market_addr, date);
        let checkpoint = DailyRate {
            market: market_addr,
            date,
            rate,
            days_applied,
            index_before,
            index_after,
            published_at: self.now(),
            bump,
            _reserved: [0; 32],
        };
        self.write_account(address, &checkpoint);
        market.rate_index = index_after;
        market.last_rate_date = date;
        self.write_account(market_addr, &market);
        checkpoint
    }

    /// [`TestEnv::mock_daily_rate`] for every business day after
    /// `Market.last_rate_date` up to and including `through`.
    pub fn mock_daily_rates_through(&mut self, through: i64, rate: u64) {
        loop {
            let last = self.market_state().last_rate_date;
            let date = fra_math::next_business_day(last, &fra_math::WeekendOnly)
                .expect("next business day");
            if date > through {
                break;
            }
            self.mock_daily_rate(date, rate);
        }
    }
}

// ── Scenarios (mocked equivalents of the fixture's setup chains) ───────────

/// Mocked [`crate::fixture::setup_config`]: Config + empty fee vault.
pub fn mock_config_env() -> TestEnv {
    let mut env = TestEnv::new();
    env.mock_config();
    env
}

/// Mocked [`crate::fixture::setup_fra_market`]: config, active operator
/// registration, market, empty reserve, unpublished quote.
pub fn mock_market_env() -> TestEnv {
    mock_market_env_with(default_market_params())
}

pub fn mock_market_env_with(params: MarketParams) -> TestEnv {
    let mut env = mock_config_env();
    env.params = params;
    env.mock_operator_registration();
    env.mock_market();
    env.mock_reserve(0);
    env.mock_unpublished_quote();
    env
}

/// Mocked [`crate::fixture::ready_market`]: reserve holding
/// [`TEST_RESERVE`] and a fresh quote.
pub fn mock_ready_market() -> TestEnv {
    mock_ready_market_with(default_market_params())
}

pub fn mock_ready_market_with(params: MarketParams) -> TestEnv {
    let mut env = mock_market_env_with(params);
    env.mock_reserve(TEST_RESERVE);
    env.mock_quote();
    env
}

/// Mocked [`crate::fixture::ready_trader`]: eligible trader with
/// [`TEST_MARGIN`] in their margin account.
pub fn mock_ready_trader(env: &mut TestEnv) -> TestTrader {
    env.mock_trader_with_margin(TEST_MARGIN)
}
