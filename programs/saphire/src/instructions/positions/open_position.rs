//! IX-18 `open_position(side: Side, notional: u64)` · B · trader
//! Implements: REQ-27–41, 74a.

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    constants::*,
    errors::FraError,
    state::{Config, Eligibility, Market, Position, Quote, Reserve, Side, TraderMargin},
};

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub trader: Signer<'info>,

    #[account(seeds = [SEED_CONFIG], bump = config.bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(mut)]
    pub market: Box<Account<'info, Market>>,

    #[account(has_one = market)]
    pub quote: Box<Account<'info, Quote>>,

    #[account(
        seeds = [SEED_ELIGIBLE, market.key().as_ref(), trader.key().as_ref()],
        bump = eligibility.bump,
        constraint = eligibility.active @ FraError::NotEligible, // REQ-35
    )]
    pub eligibility: Box<Account<'info, Eligibility>>,

    #[account(
        mut,
        seeds = [SEED_MARGIN, market.key().as_ref(), trader.key().as_ref()],
        bump = trader_margin.bump,
    )]
    pub trader_margin: Box<Account<'info, TraderMargin>>,

    #[account(mut, address = trader_margin.vault)]
    pub margin_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Card: "reserve W". `has_one = market` added so a foreign reserve
    /// cannot be committed.
    #[account(mut, has_one = market)]
    pub reserve: Box<Account<'info, Reserve>>,

    #[account(mut, address = config.fee_vault)]
    pub fee_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = trader,
        space = 8 + Position::INIT_SPACE,
        seeds = [
            SEED_POSITION,
            market.key().as_ref(),
            trader.key().as_ref(),
            &market.position_count.to_le_bytes(),
        ],
        bump,
    )]
    pub position: Box<Account<'info, Position>>,

    /// Not on the card: needed by `transfer_checked` for the fee.
    #[account(address = market.settlement_mint @ FraError::WrongMint)]
    pub settlement_mint: Box<InterfaceAccount<'info, Mint>>,

    pub clock: Sysvar<'info, Clock>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

/// Validation (cheap refusals first):
/// 1. `!config.paused` · `market.status == Active` (REQ-41, 74a)
/// 2. quote fresh: `epoch > 0 && now − published_at <= quote_validity_secs`
///    (REQ-36)
/// 3. `min_notional <= notional <= min(max_notional,
///    quote.max_notional_at_quote)` (REQ-39)
/// 4. `open_interest + notional <= open_interest_cap` (REQ-40)
/// 5. `lock = bps_ceil(N, initial_margin_bps)` · `commit = bps_ceil(N,
///    operator_commit_bps)` · `fee = bps_ceil(N, protocol_fee_bps)`
/// 6. `trader_margin.free() >= lock + fee + market.settlement_fee` (REQ-37;
///    settlement fee not on the cards yet)
/// 7. `reserve.balance − reserve.committed >= commit` (REQ-38)
///
/// Logic:
/// 1. `K = compute_fixed_rate(quote.fixed_mid, quote.half_spread_bps, side)`
///    (REQ-27/28)
/// 2. `(period_start, period_end) = derive_period(now, …)`; `days =
///    year_fraction_days(…)` (REQ-29/30)
/// 3. CPI `transfer_checked` `margin_vault → fee_vault, fee`, TraderMargin PDA
///    signs; `trader_margin.balance −= fee` (REQ-33)
/// 4. `trader_margin.locked += lock + market.settlement_fee`;
///    `open_positions += 1` (REQ-31)
/// 5. `reserve.committed += commit` (REQ-32)
/// 6. `nonce = market.position_count`; `position_count += 1`;
///    `open_interest += N` (REQ-34)
/// 7. Write Position (`origin = Quote`, `status = Open`, `trade_date = now`,
///    locks, fee, `settlement_fee = market.settlement_fee`)
/// 8. Emit `PositionOpened`
///
/// Errors: `ProgramPaused`, `MarketHalted`, `NotEligible`, `QuoteStale`,
/// `NotionalOutOfBounds`, `OpenInterestCap`, `InsufficientFreeMargin`,
/// `InsufficientReserve`.
pub fn handle_open_position(
    _ctx: Context<OpenPosition>,
    _side: Side,
    _notional: u64,
) -> Result<()> {
    todo!("IX-18 B")
}
