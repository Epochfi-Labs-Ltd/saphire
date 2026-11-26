//! One builder per instruction. PDAs are derived here from the semantic
//! inputs, so callers pass wallets, mints and token accounts only.

use anchor_lang::{
    prelude::{Clock, Instructions, Pubkey},
    solana_program::{instruction::Instruction, system_program, sysvar::SysvarId},
    InstructionData, ToAccountMetas,
};
use saphire::{accounts as acc, instruction as data, MarketParams, Side, SignedDeal};

use crate::{pda, MarketRef};

fn build(accounts: impl ToAccountMetas, data: impl InstructionData) -> Instruction {
    Instruction {
        program_id: saphire::ID,
        accounts: accounts.to_account_metas(None),
        data: data.data(),
    }
}

// ── Segment A ──────────────────────────────────────────────────────────────

/// IX-01
pub fn initialize_config(
    authority: &Pubkey,
    settlement_mint: &Pubkey,
    token_program: &Pubkey,
    protocol_fee_bps: u16,
    rate_ceiling: u64,
) -> Instruction {
    build(
        acc::InitializeConfig {
            authority: *authority,
            config: pda::config().0,
            fee_vault: pda::fee_vault().0,
            settlement_mint: *settlement_mint,
            token_program: *token_program,
            system_program: system_program::ID,
        },
        data::InitializeConfig {
            protocol_fee_bps,
            rate_ceiling,
        },
    )
}

/// IX-02
pub fn update_config(
    authority: &Pubkey,
    protocol_fee_bps: Option<u16>,
    rate_ceiling: Option<u64>,
) -> Instruction {
    build(
        acc::UpdateConfig {
            authority: *authority,
            config: pda::config().0,
        },
        data::UpdateConfig {
            protocol_fee_bps,
            rate_ceiling,
        },
    )
}

/// IX-03
pub fn register_operator(authority: &Pubkey, operator: &Pubkey) -> Instruction {
    build(
        acc::RegisterOperator {
            authority: *authority,
            config: pda::config().0,
            operator: *operator,
            operator_registration: pda::operator_registration(operator).0,
            system_program: system_program::ID,
        },
        data::RegisterOperator {},
    )
}

/// IX-04
pub fn deactivate_operator(authority: &Pubkey, operator: &Pubkey) -> Instruction {
    build(
        acc::DeactivateOperator {
            authority: *authority,
            config: pda::config().0,
            operator_registration: pda::operator_registration(operator).0,
        },
        data::DeactivateOperator {},
    )
}

/// IX-05. The market address is derived from `params` and the mint.
pub fn create_market(
    operator: &Pubkey,
    settlement_mint: &Pubkey,
    token_program: &Pubkey,
    params: MarketParams,
    keeper: &Pubkey,
) -> Instruction {
    let market = pda::market(
        params.benchmark_id,
        settlement_mint,
        params.start_offset_months,
        params.tenor_months,
    )
    .0;
    build(
        acc::CreateMarket {
            operator: *operator,
            config: pda::config().0,
            operator_registration: pda::operator_registration(operator).0,
            settlement_mint: *settlement_mint,
            market,
            reserve: pda::reserve(&market).0,
            reserve_vault: pda::reserve_vault(&market).0,
            quote: pda::quote(&market).0,
            token_program: *token_program,
            system_program: system_program::ID,
        },
        data::CreateMarket {
            params,
            keeper: *keeper,
        },
    )
}

/// IX-06
pub fn fund_reserve(
    operator: &Pubkey,
    m: &MarketRef,
    operator_token_account: &Pubkey,
    amount: u64,
) -> Instruction {
    build(
        acc::FundReserve {
            operator: *operator,
            market: m.market,
            reserve: pda::reserve(&m.market).0,
            reserve_vault: pda::reserve_vault(&m.market).0,
            operator_token_account: *operator_token_account,
            settlement_mint: m.settlement_mint,
            token_program: m.token_program,
        },
        data::FundReserve { amount },
    )
}

/// IX-07
pub fn withdraw_reserve(
    operator: &Pubkey,
    m: &MarketRef,
    operator_token_account: &Pubkey,
    amount: u64,
) -> Instruction {
    build(
        acc::WithdrawReserve {
            operator: *operator,
            market: m.market,
            reserve: pda::reserve(&m.market).0,
            reserve_vault: pda::reserve_vault(&m.market).0,
            operator_token_account: *operator_token_account,
            settlement_mint: m.settlement_mint,
            token_program: m.token_program,
        },
        data::WithdrawReserve { amount },
    )
}

/// IX-08
pub fn publish_quote(
    operator: &Pubkey,
    market: &Pubkey,
    fixed_mid: u64,
    half_spread_bps: u16,
    max_notional_at_quote: u64,
) -> Instruction {
    build(
        acc::PublishQuote {
            operator: *operator,
            config: pda::config().0,
            market: *market,
            quote: pda::quote(market).0,
            clock: Clock::id(),
        },
        data::PublishQuote {
            fixed_mid,
            half_spread_bps,
            max_notional_at_quote,
        },
    )
}

/// IX-09
pub fn admit_trader(operator: &Pubkey, market: &Pubkey, trader: &Pubkey) -> Instruction {
    build(
        acc::AdmitTrader {
            operator: *operator,
            market: *market,
            trader: *trader,
            eligibility: pda::eligibility(market, trader).0,
            system_program: system_program::ID,
        },
        data::AdmitTrader {},
    )
}

/// IX-10
pub fn suspend_trader(operator: &Pubkey, market: &Pubkey, trader: &Pubkey) -> Instruction {
    build(
        acc::SuspendTrader {
            operator: *operator,
            market: *market,
            eligibility: pda::eligibility(market, trader).0,
        },
        data::SuspendTrader {},
    )
}

/// IX-13
pub fn halt_market(operator: &Pubkey, market: &Pubkey) -> Instruction {
    build(
        acc::HaltMarket {
            operator: *operator,
            market: *market,
        },
        data::HaltMarket {},
    )
}

/// IX-14
pub fn resume_market(operator: &Pubkey, market: &Pubkey) -> Instruction {
    build(
        acc::ResumeMarket {
            operator: *operator,
            market: *market,
        },
        data::ResumeMarket {},
    )
}

/// IX-15
pub fn pause_program(authority: &Pubkey) -> Instruction {
    build(
        acc::PauseProgram {
            authority: *authority,
            config: pda::config().0,
        },
        data::PauseProgram {},
    )
}

/// IX-16
pub fn resume_program(authority: &Pubkey) -> Instruction {
    build(
        acc::ResumeProgram {
            authority: *authority,
            config: pda::config().0,
        },
        data::ResumeProgram {},
    )
}

/// IX-25 (not on the cards yet). `destination` is any token account of the
/// fee mint.
pub fn withdraw_fees(
    authority: &Pubkey,
    destination: &Pubkey,
    settlement_mint: &Pubkey,
    token_program: &Pubkey,
    amount: u64,
) -> Instruction {
    build(
        acc::WithdrawFees {
            authority: *authority,
            config: pda::config().0,
            fee_vault: pda::fee_vault().0,
            destination: *destination,
            settlement_mint: *settlement_mint,
            token_program: *token_program,
        },
        data::WithdrawFees { amount },
    )
}

// ── Segment B ──────────────────────────────────────────────────────────────

/// IX-11
pub fn init_trader_margin(trader: &Pubkey, m: &MarketRef) -> Instruction {
    build(
        acc::InitTraderMargin {
            trader: *trader,
            market: m.market,
            eligibility: pda::eligibility(&m.market, trader).0,
            trader_margin: pda::trader_margin(&m.market, trader).0,
            margin_vault: pda::margin_vault(&m.market, trader).0,
            settlement_mint: m.settlement_mint,
            token_program: m.token_program,
            system_program: system_program::ID,
        },
        data::InitTraderMargin {},
    )
}

/// IX-12
pub fn deposit_margin(
    trader: &Pubkey,
    m: &MarketRef,
    trader_token_account: &Pubkey,
    amount: u64,
) -> Instruction {
    build(
        acc::DepositMargin {
            trader: *trader,
            market: m.market,
            eligibility: pda::eligibility(&m.market, trader).0,
            trader_margin: pda::trader_margin(&m.market, trader).0,
            margin_vault: pda::margin_vault(&m.market, trader).0,
            trader_token_account: *trader_token_account,
            settlement_mint: m.settlement_mint,
            token_program: m.token_program,
        },
        data::DepositMargin { amount },
    )
}

/// IX-17
pub fn withdraw_margin(
    trader: &Pubkey,
    m: &MarketRef,
    trader_token_account: &Pubkey,
    amount: u64,
) -> Instruction {
    build(
        acc::WithdrawMargin {
            trader: *trader,
            trader_margin: pda::trader_margin(&m.market, trader).0,
            margin_vault: pda::margin_vault(&m.market, trader).0,
            trader_token_account: *trader_token_account,
            settlement_mint: m.settlement_mint,
            token_program: m.token_program,
        },
        data::WithdrawMargin { amount },
    )
}

/// IX-18. `position_count` is `Market.position_count` before the open.
pub fn open_position(
    trader: &Pubkey,
    m: &MarketRef,
    position_count: u64,
    side: Side,
    notional: u64,
) -> Instruction {
    build(
        acc::OpenPosition {
            trader: *trader,
            config: pda::config().0,
            market: m.market,
            quote: pda::quote(&m.market).0,
            eligibility: pda::eligibility(&m.market, trader).0,
            trader_margin: pda::trader_margin(&m.market, trader).0,
            margin_vault: pda::margin_vault(&m.market, trader).0,
            reserve: pda::reserve(&m.market).0,
            fee_vault: pda::fee_vault().0,
            position: pda::position(&m.market, trader, position_count).0,
            settlement_mint: m.settlement_mint,
            clock: Clock::id(),
            token_program: m.token_program,
            system_program: system_program::ID,
        },
        data::OpenPosition { side, notional },
    )
}

/// IX-19. Must be preceded in the same transaction by
/// [`crate::deal::ed25519_verify_instruction`] over
/// [`crate::deal::deal_message_bytes`].
pub fn open_position_signed(
    trader: &Pubkey,
    m: &MarketRef,
    position_count: u64,
    deal: SignedDeal,
) -> Instruction {
    build(
        acc::OpenPositionSigned {
            trader: *trader,
            config: pda::config().0,
            market: m.market,
            eligibility: pda::eligibility(&m.market, trader).0,
            trader_margin: pda::trader_margin(&m.market, trader).0,
            margin_vault: pda::margin_vault(&m.market, trader).0,
            reserve: pda::reserve(&m.market).0,
            fee_vault: pda::fee_vault().0,
            position: pda::position(&m.market, trader, position_count).0,
            instructions_sysvar: Instructions::id(),
            used_nonce: pda::used_nonce(&m.market, trader, deal.nonce).0,
            settlement_mint: m.settlement_mint,
            clock: Clock::id(),
            token_program: m.token_program,
            system_program: system_program::ID,
        },
        data::OpenPositionSigned { deal },
    )
}

/// IX-20
pub fn close_position(trader: &Pubkey, m: &MarketRef, position_nonce: u64) -> Instruction {
    build(
        acc::ClosePosition {
            trader: *trader,
            position: pda::position(&m.market, trader, position_nonce).0,
            market: m.market,
            quote: pda::quote(&m.market).0,
            trader_margin: pda::trader_margin(&m.market, trader).0,
            margin_vault: pda::margin_vault(&m.market, trader).0,
            reserve: pda::reserve(&m.market).0,
            reserve_vault: pda::reserve_vault(&m.market).0,
            settlement_mint: m.settlement_mint,
            clock: Clock::id(),
            token_program: m.token_program,
        },
        data::ClosePosition {},
    )
}

/// IX-24
pub fn close_used_nonce(
    trader: &Pubkey,
    market: &Pubkey,
    position_nonce: u64,
    deal_nonce: u64,
) -> Instruction {
    build(
        acc::CloseUsedNonce {
            trader: *trader,
            position: pda::position(market, trader, position_nonce).0,
            used_nonce: pda::used_nonce(market, trader, deal_nonce).0,
        },
        data::CloseUsedNonce {},
    )
}

// ── Segment C ──────────────────────────────────────────────────────────────

/// IX-21
pub fn publish_daily_rate(
    operator: &Pubkey,
    market: &Pubkey,
    date: i64,
    rate: u64,
    days_applied: u8,
) -> Instruction {
    build(
        acc::PublishDailyRate {
            operator: *operator,
            config: pda::config().0,
            market: *market,
            daily_rate: pda::daily_rate(market, date).0,
            clock: Clock::id(),
            system_program: system_program::ID,
        },
        data::PublishDailyRate {
            date,
            rate,
            days_applied,
        },
    )
}

/// The position fields settle/unwind need to derive their accounts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PositionRef {
    pub trader: Pubkey,
    pub nonce: u64,
    pub period_start: i64,
    pub period_end: i64,
}

/// IX-22. `payer` is any signer (crank); `payer_token_account` (settlement
/// mint, owned by `payer`) receives the position's settlement fee.
pub fn settle_position(
    payer: &Pubkey,
    m: &MarketRef,
    p: &PositionRef,
    payer_token_account: &Pubkey,
) -> Instruction {
    build(
        acc::SettlePosition {
            payer: *payer,
            market: m.market,
            reserve: pda::reserve(&m.market).0,
            reserve_vault: pda::reserve_vault(&m.market).0,
            rate_start: pda::daily_rate(&m.market, p.period_start).0,
            rate_end: pda::daily_rate(&m.market, p.period_end).0,
            position: pda::position(&m.market, &p.trader, p.nonce).0,
            trader_margin: pda::trader_margin(&m.market, &p.trader).0,
            margin_vault: pda::margin_vault(&m.market, &p.trader).0,
            payer_token_account: *payer_token_account,
            settlement_mint: m.settlement_mint,
            clock: Clock::id(),
            token_program: m.token_program,
        },
        data::SettlePosition {},
    )
}

/// IX-23. `payer` is any signer (crank); `payer_token_account` (settlement
/// mint, owned by `payer`) receives the position's settlement fee.
pub fn unwind_position(
    payer: &Pubkey,
    m: &MarketRef,
    p: &PositionRef,
    payer_token_account: &Pubkey,
) -> Instruction {
    build(
        acc::UnwindPosition {
            payer: *payer,
            market: m.market,
            reserve: pda::reserve(&m.market).0,
            position: pda::position(&m.market, &p.trader, p.nonce).0,
            trader_margin: pda::trader_margin(&m.market, &p.trader).0,
            rate_end_address: pda::daily_rate(&m.market, p.period_end).0,
            margin_vault: pda::margin_vault(&m.market, &p.trader).0,
            payer_token_account: *payer_token_account,
            settlement_mint: m.settlement_mint,
            clock: Clock::id(),
            token_program: m.token_program,
        },
        data::UnwindPosition {},
    )
}
