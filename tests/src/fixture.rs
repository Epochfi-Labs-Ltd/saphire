//! Shared LiteSVM fixture for the Saphire integration tests.
//!
//! Loads `target/deploy/saphire.so` (run `anchor build` first), creates the
//! settlement mint and token accounts by writing SPL state directly (so the
//! same fixture works for SPL Token and Token-2022), and wraps the Codama
//! client's instruction builders in the setup steps the instruction tests
//! share.

use anchor_lang::{
    error::ErrorCode as AnchorErrorCode,
    prelude::{Clock, Pubkey},
    solana_program::{instruction::Instruction, program_option::COption, program_pack::Pack},
    AccountDeserialize, AccountSerialize,
};
use anchor_spl::compat::spl_token_interface::state::{
    Account as SplTokenAccount, AccountState, Mint as SplMint,
};
use litesvm::{
    types::{TransactionMetadata, TransactionResult},
    LiteSVM,
};
use saphire::{
    Config, DailyRate, Eligibility, FraError, Market, MarketParams, MarketStatus, Position, Quote,
    Reserve, SignedDeal, TraderMargin, UsedNonce,
};
use saphire_client::{instructions as ix, types::Side};
use solana_account::Account;
use solana_instruction_error::InstructionError;
use solana_keypair::Keypair;
use solana_message::{Message, VersionedMessage};
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;
use solana_transaction_error::TransactionError;

use crate::{deal, pda};

pub type TxResult = TransactionResult;

/// SPL Token program id.
pub const TOKEN_PROGRAM_ID: Pubkey = anchor_spl::token::ID;
/// Token-2022 program id.
pub const TOKEN_2022_PROGRAM_ID: Pubkey = anchor_spl::token_2022::ID;

// ── Units and defaults ─────────────────────────────────────────────────────

pub const DECIMALS: u8 = 6;
/// One whole USDC in base units.
pub const USDC: u64 = 1_000_000;
pub const SOL: u64 = 1_000_000_000;
pub const SECONDS_PER_DAY: i64 = 86_400;

/// 2026-01-05 14:00:00 UTC (a Monday). Every fixture starts its clock here.
pub const START_TS: i64 = 1_767_621_600;

/// Friday 2026-01-02 00:00 UTC: the business day before the first daily
/// print (Monday 2026-01-05). `Market.last_rate_date` starts here.
pub const INDEX_START_DATE: i64 = 1_767_312_000;

/// Flat per-position crank bounty (`Market.settlement_fee`): 1 USDC.
pub const TEST_SETTLEMENT_FEE: u64 = USDC;

pub const PROTOCOL_FEE_BPS: u16 = 2;
/// 20.0000%.
pub const RATE_CEILING: u64 = 200_000;

/// 4.0500% mid, ±2 bp -> PayFixed K = 40_700, ReceiveFixed K = 40_300.
pub const QUOTE_MID: u64 = 40_500;
pub const QUOTE_HALF_SPREAD_BPS: u16 = 2;
pub const QUOTE_MAX_NOTIONAL: u64 = 5_000_000 * USDC;

pub const OPERATOR_STARTING_USDC: u64 = 100_000_000 * USDC;
pub const TRADER_STARTING_USDC: u64 = 10_000_000 * USDC;

/// USD SOFR 3x6, ACT/360, Modified Following.
pub fn default_market_params() -> MarketParams {
    MarketParams {
        benchmark_id: saphire::BENCHMARK_SOFR,
        start_offset_months: 3,
        tenor_months: 3,
        day_count_basis: saphire::DAY_COUNT_ACT_360,
        business_day_convention: saphire::BDC_MODIFIED_FOLLOWING,
        initial_margin_bps: 1_000,
        operator_commit_bps: 1_000,
        min_notional: 1_000 * USDC,
        max_notional: 10_000_000 * USDC,
        open_interest_cap: 50_000_000 * USDC,
        quote_validity_secs: 300,
        rate_grace_days: 5,
        early_close_fee_bps: 5,
        max_half_spread_bps: 20,
        index_start_date: INDEX_START_DATE,
        settlement_fee: TEST_SETTLEMENT_FEE,
    }
}

// ── Environment ────────────────────────────────────────────────────────────

pub struct TestEnv {
    pub svm: LiteSVM,
    /// `Config.authority`.
    pub admin: Keypair,
    pub operator: Keypair,
    pub keeper: Pubkey,
    pub mint: Pubkey,
    /// SPL Token or Token-2022.
    pub token_program: Pubkey,
    pub operator_token_account: Pubkey,
    pub params: MarketParams,
}

/// Everything a market-scoped instruction needs to know about the market.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MarketRef {
    pub market: Pubkey,
    pub settlement_mint: Pubkey,
    /// SPL Token or Token-2022, whichever owns `settlement_mint`.
    pub token_program: Pubkey,
}

pub struct TestTrader {
    pub kp: Keypair,
    pub token_account: Pubkey,
}

impl TestTrader {
    pub fn pubkey(&self) -> Pubkey {
        self.kp.pubkey()
    }
}

fn program_bytes() -> Vec<u8> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/deploy/saphire.so");
    std::fs::read(path).unwrap_or_else(|e| {
        panic!("cannot read {path}: {e}. Run `anchor build` before `cargo test`.")
    })
}

impl Default for TestEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl TestEnv {
    /// Fresh SVM with the program loaded, clock at [`START_TS`], an SPL
    /// Token settlement mint and a funded operator token account. Nothing of
    /// Saphire is initialised yet.
    pub fn new() -> Self {
        Self::with_token_program(TOKEN_PROGRAM_ID)
    }

    /// As [`TestEnv::new`] with the settlement mint owned by `token_program`
    /// (pass [`TOKEN_2022_PROGRAM_ID`] for Token-2022).
    pub fn with_token_program(token_program: Pubkey) -> Self {
        let mut svm = LiteSVM::new();
        svm.add_program(saphire::ID, &program_bytes())
            .expect("load saphire.so");

        let admin = Keypair::new();
        let operator = Keypair::new();
        svm.airdrop(&admin.pubkey(), 100 * SOL)
            .expect("airdrop admin");
        svm.airdrop(&operator.pubkey(), 100 * SOL)
            .expect("airdrop operator");

        let mut env = Self {
            svm,
            keeper: Keypair::new().pubkey(),
            mint: Pubkey::default(),
            token_program,
            operator_token_account: Pubkey::default(),
            params: default_market_params(),
            admin,
            operator,
        };
        warp_to(&mut env, START_TS);
        let mint_authority = env.admin.pubkey();
        env.mint = env.create_mint(&mint_authority);
        let operator_key = env.operator.pubkey();
        env.operator_token_account =
            env.create_token_account(&operator_key, OPERATOR_STARTING_USDC);
        env
    }

    // ── Transactions ───────────────────────────────────────────────────────

    /// Sign and send `ixs`; `signers[0]` pays. The blockhash is expired first
    /// so identical instructions can be resent.
    pub fn send(&mut self, ixs: &[Instruction], signers: &[&Keypair]) -> TxResult {
        self.svm.expire_blockhash();
        let payer = signers.first().expect("at least one signer").pubkey();
        let msg = Message::new_with_blockhash(ixs, Some(&payer), &self.svm.latest_blockhash());
        let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), signers)
            .expect("sign transaction");
        self.svm.send_transaction(tx)
    }

    /// [`TestEnv::send`] and panic with the program logs on failure.
    pub fn send_ok(&mut self, ixs: &[Instruction], signers: &[&Keypair]) -> TransactionMetadata {
        match self.send(ixs, signers) {
            Ok(meta) => meta,
            Err(failed) => panic!(
                "transaction failed: {:?}\n{}",
                failed.err,
                failed.meta.pretty_logs()
            ),
        }
    }

    pub fn new_funded_keypair(&mut self) -> Keypair {
        let kp = Keypair::new();
        self.svm.airdrop(&kp.pubkey(), 10 * SOL).expect("airdrop");
        kp
    }

    // ── Clock ──────────────────────────────────────────────────────────────

    pub fn now(&self) -> i64 {
        self.svm.get_sysvar::<Clock>().unix_timestamp
    }

    // ── Tokens ─────────────────────────────────────────────────────────────

    /// New mint owned by `self.token_program`, written directly as account
    /// state (no mint-authority signature needed).
    pub fn create_mint(&mut self, authority: &Pubkey) -> Pubkey {
        let mint = Keypair::new().pubkey();
        let state = SplMint {
            mint_authority: COption::Some(*authority),
            supply: 0,
            decimals: DECIMALS,
            is_initialized: true,
            freeze_authority: COption::None,
        };
        let mut data = vec![0u8; SplMint::LEN];
        SplMint::pack(state, &mut data).expect("pack mint");
        self.set_token_owned_account(mint, data);
        mint
    }

    /// Token account for the settlement mint, owned by `owner`, holding
    /// `amount`.
    pub fn create_token_account(&mut self, owner: &Pubkey, amount: u64) -> Pubkey {
        let mint = self.mint;
        self.create_token_account_for_mint(&mint, owner, amount)
    }

    pub fn create_token_account_for_mint(
        &mut self,
        mint: &Pubkey,
        owner: &Pubkey,
        amount: u64,
    ) -> Pubkey {
        let address = Keypair::new().pubkey();
        let state = SplTokenAccount {
            mint: *mint,
            owner: *owner,
            amount,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        };
        let mut data = vec![0u8; SplTokenAccount::LEN];
        SplTokenAccount::pack(state, &mut data).expect("pack token account");
        self.set_token_owned_account(address, data);
        address
    }

    fn set_token_owned_account(&mut self, address: Pubkey, data: Vec<u8>) {
        let lamports = self.svm.minimum_balance_for_rent_exemption(data.len());
        self.svm
            .set_account(
                address,
                Account {
                    lamports,
                    data,
                    owner: self.token_program,
                    executable: false,
                    rent_epoch: 0,
                },
            )
            .expect("set token account");
    }

    pub fn token_balance(&self, token_account: &Pubkey) -> u64 {
        let account = self
            .svm
            .get_account(token_account)
            .unwrap_or_else(|| panic!("token account {token_account} missing"));
        SplTokenAccount::unpack_from_slice(&account.data[..SplTokenAccount::LEN])
            .expect("unpack token account")
            .amount
    }

    /// Test-only: overwrite a token account's amount, e.g. to simulate a
    /// direct donation into a vault.
    pub fn set_token_balance(&mut self, token_account: &Pubkey, amount: u64) {
        let mut account = self.svm.get_account(token_account).expect("token account");
        let mut state = SplTokenAccount::unpack_from_slice(&account.data[..SplTokenAccount::LEN])
            .expect("unpack token account");
        state.amount = amount;
        SplTokenAccount::pack(state, &mut account.data[..SplTokenAccount::LEN]).expect("pack");
        self.svm
            .set_account(*token_account, account)
            .expect("set token account");
    }

    pub fn token_owner(&self, token_account: &Pubkey) -> Pubkey {
        let account = self.svm.get_account(token_account).expect("token account");
        SplTokenAccount::unpack_from_slice(&account.data[..SplTokenAccount::LEN])
            .expect("unpack token account")
            .owner
    }

    pub fn token_mint(&self, token_account: &Pubkey) -> Pubkey {
        let account = self.svm.get_account(token_account).expect("token account");
        SplTokenAccount::unpack_from_slice(&account.data[..SplTokenAccount::LEN])
            .expect("unpack token account")
            .mint
    }

    // ── Program accounts ───────────────────────────────────────────────────

    pub fn try_account<T: AccountDeserialize>(&self, address: &Pubkey) -> Option<T> {
        let account = self.svm.get_account(address)?;
        T::try_deserialize(&mut account.data.as_slice()).ok()
    }

    pub fn account<T: AccountDeserialize>(&self, address: &Pubkey) -> T {
        self.try_account(address)
            .unwrap_or_else(|| panic!("account {address} missing or not deserializable"))
    }

    pub fn exists(&self, address: &Pubkey) -> bool {
        self.svm
            .get_account(address)
            .is_some_and(|a| a.lamports > 0)
    }

    pub fn market_ref(&self) -> MarketRef {
        MarketRef {
            market: self.market(),
            settlement_mint: self.mint,
            token_program: self.token_program,
        }
    }

    pub fn market(&self) -> Pubkey {
        pda::market(
            self.params.benchmark_id,
            &self.mint,
            self.params.start_offset_months,
            self.params.tenor_months,
        )
        .0
    }

    pub fn config_state(&self) -> Config {
        self.account(&pda::config().0)
    }

    pub fn market_state(&self) -> Market {
        self.account(&self.market())
    }

    pub fn reserve_state(&self) -> Reserve {
        self.account(&pda::reserve(&self.market()).0)
    }

    pub fn quote_state(&self) -> Quote {
        self.account(&pda::quote(&self.market()).0)
    }

    pub fn eligibility_state(&self, trader: &Pubkey) -> Eligibility {
        self.account(&pda::eligibility(&self.market(), trader).0)
    }

    pub fn margin_state(&self, trader: &Pubkey) -> TraderMargin {
        self.account(&pda::trader_margin(&self.market(), trader).0)
    }

    pub fn position_state(&self, trader: &Pubkey, nonce: u64) -> Position {
        self.account(&pda::position(&self.market(), trader, nonce).0)
    }

    pub fn used_nonce_state(&self, trader: &Pubkey, nonce: u64) -> UsedNonce {
        self.account(&pda::used_nonce(&self.market(), trader, nonce).0)
    }

    pub fn daily_rate_state(&self, date: i64) -> DailyRate {
        self.account(&pda::daily_rate(&self.market(), date).0)
    }

    pub fn reserve_vault(&self) -> Pubkey {
        pda::reserve_vault(&self.market()).0
    }

    pub fn margin_vault(&self, trader: &Pubkey) -> Pubkey {
        pda::margin_vault(&self.market(), trader).0
    }

    pub fn fee_vault(&self) -> Pubkey {
        pda::fee_vault().0
    }

    /// Test-only: overwrite `Market.status` directly. No instruction sets
    /// `Closed` in v1, so tests reach it this way.
    pub fn force_market_status(&mut self, status: MarketStatus) {
        let address = self.market();
        let mut market = self.market_state();
        market.status = status;
        let mut account = self.svm.get_account(&address).expect("market account");
        let mut data = Vec::with_capacity(account.data.len());
        market.try_serialize(&mut data).expect("serialize market");
        account.data[..data.len()].copy_from_slice(&data);
        self.svm.set_account(address, account).expect("set market");
    }

    // ── Instructions (Codama builders, PDAs filled in) ─────────────────────

    /// `open_position` for the next position slot (`Market.position_count`).
    pub fn ix_open_position(&self, trader: &Pubkey, side: Side, notional: u64) -> Instruction {
        let m = self.market_ref();
        let count = self.market_state().position_count;
        ix::OpenPositionBuilder::new()
            .trader(*trader)
            .config(pda::config().0)
            .market(m.market)
            .quote(pda::quote(&m.market).0)
            .eligibility(pda::eligibility(&m.market, trader).0)
            .trader_margin(pda::trader_margin(&m.market, trader).0)
            .margin_vault(pda::margin_vault(&m.market, trader).0)
            .reserve(pda::reserve(&m.market).0)
            .fee_vault(pda::fee_vault().0)
            .position(pda::position(&m.market, trader, count).0)
            .settlement_mint(m.settlement_mint)
            .token_program(m.token_program)
            .side(side)
            .notional(notional)
            .instruction()
    }

    /// `open_position_signed` for the next position slot, submitting `deal`.
    pub fn ix_open_position_signed(&self, trader: &Pubkey, deal: &SignedDeal) -> Instruction {
        let m = self.market_ref();
        let count = self.market_state().position_count;
        ix::OpenPositionSignedBuilder::new()
            .trader(*trader)
            .config(pda::config().0)
            .market(m.market)
            .eligibility(pda::eligibility(&m.market, trader).0)
            .trader_margin(pda::trader_margin(&m.market, trader).0)
            .margin_vault(pda::margin_vault(&m.market, trader).0)
            .reserve(pda::reserve(&m.market).0)
            .fee_vault(pda::fee_vault().0)
            .position(pda::position(&m.market, trader, count).0)
            .used_nonce(pda::used_nonce(&m.market, trader, deal.nonce).0)
            .settlement_mint(m.settlement_mint)
            .token_program(m.token_program)
            .side(client_side(deal.side))
            .notional(deal.notional)
            .fixed_rate(deal.fixed_rate)
            .period_start(deal.period_start)
            .period_end(deal.period_end)
            .expires_at(deal.expires_at)
            .nonce(deal.nonce)
            .instruction()
    }

    /// Period a position opened now would get: `derive_period(now, offset,
    /// tenor, WeekendOnly)`.
    pub fn expected_period(&self) -> (i64, i64) {
        fra_math::derive_period(
            self.now(),
            self.params.start_offset_months,
            self.params.tenor_months,
            &fra_math::WeekendOnly,
        )
        .expect("derive_period")
    }
}

// ── Setup steps (named as in the scaffolding brief) ────────────────────────

/// IX-01 only: Config + fee vault.
pub fn setup_config() -> TestEnv {
    let mut env = TestEnv::new();
    init_config(&mut env);
    env
}

fn init_config(env: &mut TestEnv) {
    let admin = env.admin.insecure_clone();
    let ix = ix_initialize_config(env, &admin.pubkey(), PROTOCOL_FEE_BPS, RATE_CEILING);
    env.send_ok(&[ix], &[&admin]);
}

/// `initialize_config` for the env's settlement mint and token program.
pub fn ix_initialize_config(
    env: &TestEnv,
    authority: &Pubkey,
    protocol_fee_bps: u16,
    rate_ceiling: u64,
) -> Instruction {
    ix::InitializeConfigBuilder::new()
        .authority(*authority)
        .config(pda::config().0)
        .fee_vault(pda::fee_vault().0)
        .settlement_mint(env.mint)
        .token_program(env.token_program)
        .protocol_fee_bps(protocol_fee_bps)
        .rate_ceiling(rate_ceiling)
        .instruction()
}

/// IX-01 + IX-03 + IX-05: config, registered operator, one market with
/// [`default_market_params`]. Reserve empty, quote unpublished.
pub fn setup_fra_market() -> TestEnv {
    setup_fra_market_with(default_market_params())
}

/// [`setup_fra_market`] with custom market parameters.
pub fn setup_fra_market_with(params: MarketParams) -> TestEnv {
    let mut env = setup_config();
    env.params = params;
    let admin = env.admin.insecure_clone();
    let operator = env.operator.insecure_clone();
    let register = ix::RegisterOperatorBuilder::new()
        .authority(admin.pubkey())
        .config(pda::config().0)
        .operator(operator.pubkey())
        .operator_registration(pda::operator_registration(&operator.pubkey()).0)
        .instruction();
    env.send_ok(&[register], &[&admin]);

    let market = env.market();
    let p = &env.params;
    let create = ix::CreateMarketBuilder::new()
        .operator(operator.pubkey())
        .config(pda::config().0)
        .operator_registration(pda::operator_registration(&operator.pubkey()).0)
        .settlement_mint(env.mint)
        .market(market)
        .reserve(pda::reserve(&market).0)
        .reserve_vault(pda::reserve_vault(&market).0)
        .quote(pda::quote(&market).0)
        .token_program(env.token_program)
        .benchmark_id(p.benchmark_id)
        .start_offset_months(p.start_offset_months)
        .tenor_months(p.tenor_months)
        .day_count_basis(p.day_count_basis)
        .business_day_convention(p.business_day_convention)
        .initial_margin_bps(p.initial_margin_bps)
        .operator_commit_bps(p.operator_commit_bps)
        .min_notional(p.min_notional)
        .max_notional(p.max_notional)
        .open_interest_cap(p.open_interest_cap)
        .quote_validity_secs(p.quote_validity_secs)
        .rate_grace_days(p.rate_grace_days)
        .early_close_fee_bps(p.early_close_fee_bps)
        .max_half_spread_bps(p.max_half_spread_bps)
        .index_start_date(p.index_start_date)
        .settlement_fee(p.settlement_fee)
        .keeper(env.keeper)
        .instruction();
    env.send_ok(&[create], &[&operator]);
    env
}

/// Notional used by most position tests: 1,000,000 USDC.
pub const TEST_NOTIONAL: u64 = 1_000_000 * USDC;
/// Reserve funded by [`ready_market`].
pub const TEST_RESERVE: u64 = 10_000_000 * USDC;
/// Margin deposited by [`ready_trader`].
pub const TEST_MARGIN: u64 = 1_000_000 * USDC;

/// Market funded with [`TEST_RESERVE`] and a fresh default quote.
pub fn ready_market() -> TestEnv {
    ready_market_with(default_market_params())
}

pub fn ready_market_with(params: MarketParams) -> TestEnv {
    let mut env = setup_fra_market_with(params);
    fund_test_reserve(&mut env, TEST_RESERVE);
    publish_test_quote(&mut env);
    env
}

/// Eligible trader with [`TEST_MARGIN`] deposited.
pub fn ready_trader(env: &mut TestEnv) -> TestTrader {
    setup_eligible_trader_with_margin(env, TEST_MARGIN)
}

/// IX-06 from the operator's token account.
pub fn fund_test_reserve(env: &mut TestEnv, amount: u64) {
    let operator = env.operator.insecure_clone();
    let m = env.market_ref();
    let ix = ix::FundReserveBuilder::new()
        .operator(operator.pubkey())
        .market(m.market)
        .reserve(pda::reserve(&m.market).0)
        .reserve_vault(pda::reserve_vault(&m.market).0)
        .operator_token_account(env.operator_token_account)
        .settlement_mint(m.settlement_mint)
        .token_program(m.token_program)
        .amount(amount)
        .instruction();
    env.send_ok(&[ix], &[&operator]);
}

/// IX-08 with [`QUOTE_MID`] ± [`QUOTE_HALF_SPREAD_BPS`], cap
/// [`QUOTE_MAX_NOTIONAL`].
pub fn publish_test_quote(env: &mut TestEnv) {
    publish_quote_with(env, QUOTE_MID, QUOTE_HALF_SPREAD_BPS, QUOTE_MAX_NOTIONAL);
}

pub fn publish_quote_with(env: &mut TestEnv, mid: u64, half_spread_bps: u16, max_notional: u64) {
    let operator = env.operator.insecure_clone();
    let market = env.market();
    let ix = ix::PublishQuoteBuilder::new()
        .operator(operator.pubkey())
        .config(pda::config().0)
        .market(market)
        .quote(pda::quote(&market).0)
        .fixed_mid(mid)
        .half_spread_bps(half_spread_bps)
        .max_notional_at_quote(max_notional)
        .instruction();
    env.send_ok(&[ix], &[&operator]);
}

/// New funded trader wallet with a settlement-mint token account, admitted
/// (IX-09). No margin account yet.
pub fn admit_test_trader(env: &mut TestEnv) -> TestTrader {
    let kp = env.new_funded_keypair();
    let token_account = env.create_token_account(&kp.pubkey(), TRADER_STARTING_USDC);
    let operator = env.operator.insecure_clone();
    let market = env.market();
    let admit = ix::AdmitTraderBuilder::new()
        .operator(operator.pubkey())
        .market(market)
        .trader(kp.pubkey())
        .eligibility(pda::eligibility(&market, &kp.pubkey()).0)
        .instruction();
    env.send_ok(&[admit], &[&operator]);
    TestTrader { kp, token_account }
}

/// [`admit_test_trader`] + IX-11 + IX-12 (`margin` deposited).
pub fn setup_eligible_trader_with_margin(env: &mut TestEnv, margin: u64) -> TestTrader {
    let trader = admit_test_trader(env);
    let m = env.market_ref();
    let t = trader.pubkey();
    let init = ix::InitTraderMarginBuilder::new()
        .trader(t)
        .market(m.market)
        .eligibility(pda::eligibility(&m.market, &t).0)
        .trader_margin(pda::trader_margin(&m.market, &t).0)
        .margin_vault(pda::margin_vault(&m.market, &t).0)
        .settlement_mint(m.settlement_mint)
        .token_program(m.token_program)
        .instruction();
    env.send_ok(&[init], &[&trader.kp]);
    if margin > 0 {
        let deposit = ix::DepositMarginBuilder::new()
            .trader(t)
            .market(m.market)
            .eligibility(pda::eligibility(&m.market, &t).0)
            .trader_margin(pda::trader_margin(&m.market, &t).0)
            .margin_vault(pda::margin_vault(&m.market, &t).0)
            .trader_token_account(trader.token_account)
            .settlement_mint(m.settlement_mint)
            .token_program(m.token_program)
            .amount(margin)
            .instruction();
        env.send_ok(&[deposit], &[&trader.kp]);
    }
    trader
}

/// IX-18 at the current quote. Returns the position nonce.
pub fn open_test_position(
    env: &mut TestEnv,
    trader: &TestTrader,
    side: Side,
    notional: u64,
) -> u64 {
    let nonce = env.market_state().position_count;
    let ix = env.ix_open_position(&trader.pubkey(), side, notional);
    env.send_ok(&[ix], &[&trader.kp]);
    nonce
}

/// IX-21: publish one print for `date` with the `days_applied` the
/// calendar implies (1, or 3 on a Friday). The clock must already be past
/// `date + days_applied` days.
pub fn publish_test_daily_rate(env: &mut TestEnv, date: i64, rate: u64) {
    let next =
        fra_math::next_business_day(date, &fra_math::WeekendOnly).expect("next business day");
    let days_applied = fra_math::days_until(date, next).expect("days_applied");
    let operator = env.operator.insecure_clone();
    let market = env.market();
    let ix = ix::PublishDailyRateBuilder::new()
        .operator(operator.pubkey())
        .config(pda::config().0)
        .market(market)
        .daily_rate(pda::daily_rate(&market, date).0)
        .date(date)
        .rate(rate)
        .days_applied(days_applied)
        .instruction();
    env.send_ok(&[ix], &[&operator]);
}

/// IX-21 repeatedly: publish `rate` for every business day after
/// `Market.last_rate_date` up to and including `through` (00:00 UTC), in
/// order, as the operator cron would when catching up.
pub fn publish_test_daily_rates_through(env: &mut TestEnv, through: i64, rate: u64) {
    loop {
        let last = env.market_state().last_rate_date;
        let date =
            fra_math::next_business_day(last, &fra_math::WeekendOnly).expect("next business day");
        if date > through {
            break;
        }
        publish_test_daily_rate(env, date, rate);
    }
}

/// Set the Clock sysvar's `unix_timestamp` (LiteSVM clock control).
pub fn warp_to(env: &mut TestEnv, unix_ts: i64) {
    let mut clock: Clock = env.svm.get_sysvar();
    clock.unix_timestamp = unix_ts;
    clock.slot = clock.slot.saturating_add(1);
    env.svm.set_sysvar(&clock);
}

/// Advance the clock by `secs`.
pub fn warp_by(env: &mut TestEnv, secs: i64) {
    let now = env.now();
    warp_to(env, now + secs);
}

// ── Signed deals (IX-19) ───────────────────────────────────────────────────

/// A deal on the period a quote-priced open would get right now.
pub fn test_deal(env: &TestEnv, side: Side, notional: u64, nonce: u64) -> SignedDeal {
    let (period_start, period_end) = env.expected_period();
    SignedDeal {
        side: program_side(side),
        notional,
        fixed_rate: 40_650,
        period_start,
        period_end,
        expires_at: env.now() + 600,
        nonce,
    }
}

/// `[ed25519 verify over borsh(DealMessage), open_position_signed]`, signed
/// by `signer` (normally the operator).
pub fn signed_deal_ixs(
    env: &TestEnv,
    trader: &Pubkey,
    signer: &Keypair,
    signed: &SignedDeal,
    submitted: SignedDeal,
) -> [Instruction; 2] {
    let market = env.market();
    let message = deal::deal_message_bytes(&market, trader, signed);
    let signature: [u8; 64] = signer.sign_message(&message).into();
    [
        deal::ed25519_verify_instruction(&signer.pubkey(), &signature, &message),
        env.ix_open_position_signed(trader, &submitted),
    ]
}

/// Program-side [`saphire::Side`] for a client [`Side`].
pub fn program_side(side: Side) -> saphire::Side {
    match side {
        Side::PayFixed => saphire::Side::PayFixed,
        Side::ReceiveFixed => saphire::Side::ReceiveFixed,
    }
}

/// Client [`Side`] for a program-side [`saphire::Side`].
pub fn client_side(side: saphire::Side) -> Side {
    match side {
        saphire::Side::PayFixed => Side::PayFixed,
        saphire::Side::ReceiveFixed => Side::ReceiveFixed,
    }
}

// ── Error assertions ───────────────────────────────────────────────────────

/// Custom program error code of a failed transaction, if any.
pub fn custom_code(res: &TxResult) -> Option<u32> {
    match res {
        Err(failed) => match &failed.err {
            TransactionError::InstructionError(_, InstructionError::Custom(code)) => Some(*code),
            _ => None,
        },
        Ok(_) => None,
    }
}

fn describe(res: &TxResult) -> String {
    match res {
        Ok(meta) => format!("succeeded\n{}", meta.pretty_logs()),
        Err(failed) => format!("{:?}\n{}", failed.err, failed.meta.pretty_logs()),
    }
}

#[track_caller]
pub fn assert_fra_err(res: &TxResult, expected: FraError) {
    let code = u32::from(expected);
    assert_eq!(
        custom_code(res),
        Some(code),
        "expected FraError::{expected:?} ({code}), got {}",
        describe(res)
    );
}

#[track_caller]
pub fn assert_anchor_err(res: &TxResult, expected: AnchorErrorCode) {
    let code = u32::from(expected);
    assert_eq!(
        custom_code(res),
        Some(code),
        "expected anchor ErrorCode::{expected:?} ({code}), got {}",
        describe(res)
    );
}

/// System program `AccountAlreadyInUse`, surfaced when an `init` target
/// already exists.
pub const SYSTEM_ACCOUNT_ALREADY_IN_USE: u32 = 0;

#[track_caller]
pub fn assert_custom_err(res: &TxResult, code: u32) {
    assert_eq!(
        custom_code(res),
        Some(code),
        "expected custom error {code}, got {}",
        describe(res)
    );
}

#[track_caller]
pub fn assert_ok(res: &TxResult) {
    assert!(res.is_ok(), "expected success, got {}", describe(res));
}
