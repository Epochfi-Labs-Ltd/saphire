//! IX-23 `unwind_position()` · C · anyone · halt-ok · pause-ok
//! Implements: REQ-69–71, 74.

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    constants::*,
    errors::FraError,
    state::{DailyRate, Market, Position, Reserve, TraderMargin},
};

#[derive(Accounts)]
pub struct UnwindPosition<'info> {
    /// Anyone (crank). Receives `position.settlement_fee`.
    pub payer: Signer<'info>,

    #[account(mut)]
    pub market: Box<Account<'info, Market>>,

    #[account(mut, has_one = market)]
    pub reserve: Box<Account<'info, Reserve>>,

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

    /// CHECK: must equal the derived DailyRate PDA for `position.period_end`;
    /// the handler checks it holds no lamports and no data.
    #[account(
        constraint = rate_end_address.key()
            == DailyRate::pda(&market.key(), position.period_end).0
            @ FraError::BadRateCheckpointAddress,
    )]
    pub rate_end_address: UncheckedAccount<'info>,

    // ── Settlement-fee payout. Not on the cards yet; see docs/skeleton.md "Fee additions". ──
    #[account(mut, address = trader_margin.vault)]
    pub margin_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Cranker's token account for the settlement fee.
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

/// Validation: `status == Open` · `now >= period_end + rate_grace_days ·
/// 86_400` (REQ-70) · `rate_end_address` has zero lamports and no data
/// (REQ-71).
///
/// Logic: release `locked_margin`, `committed_reserve`, `open_interest`; no
/// settlement transfer (REQ-69) · settlement fee: `margin_vault →
/// payer_token_account, position.settlement_fee` (TraderMargin PDA signs;
/// skip if 0), `trader_margin.balance −= settlement_fee`, `locked −=
/// settlement_fee` (not on the cards yet; see docs/skeleton.md "Fee additions") · `status = Unwound`;
/// `settled_amount = 0`; `settled_at = now` · emit `PositionUnwound`.
///
/// Errors: `NotOpen`, `GraceNotElapsed`, `RateCheckpointExists`,
/// `BadRateCheckpointAddress`.
pub fn handle_unwind_position(_ctx: Context<UnwindPosition>) -> Result<()> {
    todo!("IX-23 C")
}
