//! Saphire — on-chain forward rate agreements on a real-world interest rate.
//!
//! Specification: README.md and docs/implementation.md. One handler per
//! instruction card; the mapping IX-nn -> file is in docs/skeleton.md.

pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("7DCT2pqaqtijF2kgN8jAa9Lbftzi1NiEmGAJ8SbWCMUv");

#[program]
pub mod saphire {
    use super::*;

    // ── Segment A — Market & Reserve ───────────────────────────────────────

    /// IX-01 · admin
    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        protocol_fee_bps: u16,
        rate_ceiling: u64,
    ) -> Result<()> {
        market::initialize_config::handle_initialize_config(ctx, protocol_fee_bps, rate_ceiling)
    }

    /// IX-02 · admin · optional
    pub fn update_config(
        ctx: Context<UpdateConfig>,
        protocol_fee_bps: Option<u16>,
        rate_ceiling: Option<u64>,
    ) -> Result<()> {
        market::update_config::handle_update_config(ctx, protocol_fee_bps, rate_ceiling)
    }

    /// IX-03 · admin
    pub fn register_operator(ctx: Context<RegisterOperator>) -> Result<()> {
        market::register_operator::handle_register_operator(ctx)
    }

    /// IX-04 · admin
    pub fn deactivate_operator(ctx: Context<DeactivateOperator>) -> Result<()> {
        market::deactivate_operator::handle_deactivate_operator(ctx)
    }

    /// IX-05 · operator
    pub fn create_market(
        ctx: Context<CreateMarket>,
        params: MarketParams,
        keeper: Pubkey,
    ) -> Result<()> {
        market::create_market::handle_create_market(ctx, params, keeper)
    }

    /// IX-06 · operator · halt-ok · pause-ok
    pub fn fund_reserve(ctx: Context<FundReserve>, amount: u64) -> Result<()> {
        market::fund_reserve::handle_fund_reserve(ctx, amount)
    }

    /// IX-07 · operator · halt-ok · pause-ok
    pub fn withdraw_reserve(ctx: Context<WithdrawReserve>, amount: u64) -> Result<()> {
        market::withdraw_reserve::handle_withdraw_reserve(ctx, amount)
    }

    /// IX-08 · operator · halt-ok
    pub fn publish_quote(
        ctx: Context<PublishQuote>,
        fixed_mid: u64,
        half_spread_bps: u16,
        max_notional_at_quote: u64,
    ) -> Result<()> {
        market::publish_quote::handle_publish_quote(
            ctx,
            fixed_mid,
            half_spread_bps,
            max_notional_at_quote,
        )
    }

    /// IX-09 · operator
    pub fn admit_trader(ctx: Context<AdmitTrader>) -> Result<()> {
        market::admit_trader::handle_admit_trader(ctx)
    }

    /// IX-10 · operator
    pub fn suspend_trader(ctx: Context<SuspendTrader>) -> Result<()> {
        market::suspend_trader::handle_suspend_trader(ctx)
    }

    /// IX-13 · operator
    pub fn halt_market(ctx: Context<HaltMarket>) -> Result<()> {
        market::halt_market::handle_halt_market(ctx)
    }

    /// IX-14 · operator
    pub fn resume_market(ctx: Context<ResumeMarket>) -> Result<()> {
        market::resume_market::handle_resume_market(ctx)
    }

    /// IX-15 · admin
    pub fn pause_program(ctx: Context<PauseProgram>) -> Result<()> {
        market::pause_program::handle_pause_program(ctx)
    }

    /// IX-16 · admin
    pub fn resume_program(ctx: Context<ResumeProgram>) -> Result<()> {
        market::resume_program::handle_resume_program(ctx)
    }

    // ── Segment B — Positions ──────────────────────────────────────────────

    /// IX-11 · trader
    pub fn init_trader_margin(ctx: Context<InitTraderMargin>) -> Result<()> {
        positions::init_trader_margin::handle_init_trader_margin(ctx)
    }

    /// IX-12 · trader · halt-ok · pause-ok
    pub fn deposit_margin(ctx: Context<DepositMargin>, amount: u64) -> Result<()> {
        positions::deposit_margin::handle_deposit_margin(ctx, amount)
    }

    /// IX-17 · trader · halt-ok · pause-ok
    pub fn withdraw_margin(ctx: Context<WithdrawMargin>, amount: u64) -> Result<()> {
        positions::withdraw_margin::handle_withdraw_margin(ctx, amount)
    }

    /// IX-18 · trader
    pub fn open_position(ctx: Context<OpenPosition>, side: Side, notional: u64) -> Result<()> {
        positions::open_position::handle_open_position(ctx, side, notional)
    }

    /// IX-19 · trader + operator signature (DECISION PENDING)
    pub fn open_position_signed(ctx: Context<OpenPositionSigned>, deal: SignedDeal) -> Result<()> {
        positions::open_position_signed::handle_open_position_signed(ctx, deal)
    }

    /// IX-20 · trader · halt-ok · pause-ok
    pub fn close_position(ctx: Context<ClosePosition>) -> Result<()> {
        positions::close_position::handle_close_position(ctx)
    }

    /// IX-24 · trader · optional (DECISION PENDING)
    pub fn close_used_nonce(ctx: Context<CloseUsedNonce>) -> Result<()> {
        positions::close_used_nonce::handle_close_used_nonce(ctx)
    }

    // ── Segment C — Daily Rates & Settlement ───────────────────────────────

    /// IX-21 · operator · halt-ok · pause-ok
    pub fn publish_daily_rate(
        ctx: Context<PublishDailyRate>,
        date: i64,
        rate: u64,
        days_applied: u8,
    ) -> Result<()> {
        settlement::publish_daily_rate::handle_publish_daily_rate(ctx, date, rate, days_applied)
    }

    /// IX-22 · anyone · halt-ok · pause-ok
    pub fn settle_position(ctx: Context<SettlePosition>) -> Result<()> {
        settlement::settle_position::handle_settle_position(ctx)
    }

    /// IX-23 · anyone · halt-ok · pause-ok
    pub fn unwind_position(ctx: Context<UnwindPosition>) -> Result<()> {
        settlement::unwind_position::handle_unwind_position(ctx)
    }
}
