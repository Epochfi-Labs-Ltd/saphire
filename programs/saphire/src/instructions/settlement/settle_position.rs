//! IX-22 `settle_position()` · C · anyone · halt-ok · pause-ok
//! Implements: REQ-56–63, 74.
//!
//! Off-chain: keeper cron lists Open positions past `period_end` whose
//! `period_end` checkpoint exists and submits this.

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    constants::*,
    errors::FraError,
    state::{DailyRate, Market, Position, Reserve, TraderMargin},
};

#[derive(Accounts)]
pub struct SettlePosition<'info> {
    /// Anyone (crank). Receives `position.settlement_fee`.
    pub payer: Signer<'info>,

    #[account(mut)]
    pub market: Box<Account<'info, Market>>,

    #[account(mut, has_one = market)]
    pub reserve: Box<Account<'info, Reserve>>,

    #[account(mut, address = reserve.vault)]
    pub reserve_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Checkpoint for the period start: `index_after` already includes
    /// period_start's own accrual, so the window is (period_start,
    /// period_end].
    #[account(
        seeds = [
            SEED_RATE,
            market.key().as_ref(),
            &position.period_start.to_le_bytes(),
        ],
        bump = rate_start.bump,
    )]
    pub rate_start: Box<Account<'info, DailyRate>>,

    /// Checkpoint for the period end.
    #[account(
        seeds = [
            SEED_RATE,
            market.key().as_ref(),
            &position.period_end.to_le_bytes(),
        ],
        bump = rate_end.bump,
    )]
    pub rate_end: Box<Account<'info, DailyRate>>,

    #[account(
        mut,
        seeds = [
            SEED_POSITION,
            market.key().as_ref(),
            position.trader.as_ref(),
            &position.nonce.to_le_bytes(),
        ],
        bump = position.bump,
        has_one = market,
    )]
    pub position: Box<Account<'info, Position>>,

    #[account(
        mut,
        seeds = [SEED_MARGIN, market.key().as_ref(), position.trader.as_ref()],
        bump = trader_margin.bump,
    )]
    pub trader_margin: Box<Account<'info, TraderMargin>>,

    #[account(mut, address = trader_margin.vault)]
    pub margin_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Not on the card: needed by `transfer_checked`.
    /// Cranker's token account for the settlement fee. Not on the cards yet; see docs/skeleton.md "Fee additions".
    #[account(
        mut,
        constraint = payer_token_account.mint == market.settlement_mint @ FraError::WrongMint,
        constraint = payer_token_account.owner == payer.key() @ FraError::WrongMint,
    )]
    pub payer_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(address = market.settlement_mint @ FraError::WrongMint)]
    pub settlement_mint: Box<InterfaceAccount<'info, Mint>>,

    pub clock: Sysvar<'info, Clock>,
    pub token_program: Interface<'info, TokenInterface>,
}

/// Validation: `status == Open` (REQ-62) · `now >= period_end`. No halt,
/// pause or eligibility check (REQ-63, 74).
///
/// Logic:
/// 1. `L = fra_math::period_rate_from_index(rate_start.index_after,
///    rate_end.index_after, days)`; `pnl = ±rate_pnl(N, L, K, days)`;
///    `settled = clamp(pnl, −locked_margin, +committed_reserve)` (REQ-58)
/// 2. `settled > 0`: `reserve_vault → margin_vault`; `< 0`: `margin_vault →
///    reserve_vault`; `== 0`: none (REQ-56/57)
/// 3. Settlement fee: `margin_vault → payer_token_account,
///    position.settlement_fee` (TraderMargin PDA signs; skip if 0);
///    `trader_margin.balance −= settlement_fee` (not on the cards yet; see docs/skeleton.md "Fee additions")
/// 4. `locked −= locked_margin + settlement_fee`; `open_positions −= 1`; `committed −=
///    committed_reserve`; `open_interest −= N` (REQ-59/60)
/// 5. Mirror asserts
/// 6. `status = Settled`; `settled_amount`; `settled_at = now` · emit
///    `PositionSettled` (with `cranker`, `settlement_fee`)
///
/// Errors: `NotOpen`, `PeriodNotEnded`, `RateCheckpointMissing` (REQ-61,
/// either checkpoint absent — seeds enforce the match), `MirrorMismatch`.
pub fn handle_settle_position(_ctx: Context<SettlePosition>) -> Result<()> {
    todo!("IX-22 C")
}
