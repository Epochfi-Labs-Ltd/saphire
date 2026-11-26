//! IX-20 `close_position()` · B · trader · halt-ok · pause-ok
//! Implements: REQ-64–68, 74.

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    constants::*,
    errors::FraError,
    state::{Market, Position, Quote, Reserve, TraderMargin},
};

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    pub trader: Signer<'info>,

    /// Listed before the margin accounts so a foreign position fails with
    /// `NotTrader` rather than a seeds mismatch on `trader_margin`.
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
        has_one = trader @ FraError::NotTrader,
    )]
    pub position: Box<Account<'info, Position>>,

    #[account(mut)]
    pub market: Box<Account<'info, Market>>,

    #[account(has_one = market)]
    pub quote: Box<Account<'info, Quote>>,

    #[account(
        mut,
        seeds = [SEED_MARGIN, market.key().as_ref(), trader.key().as_ref()],
        bump = trader_margin.bump,
    )]
    pub trader_margin: Box<Account<'info, TraderMargin>>,

    #[account(mut, address = trader_margin.vault)]
    pub margin_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut, has_one = market)]
    pub reserve: Box<Account<'info, Reserve>>,

    #[account(mut, address = reserve.vault)]
    pub reserve_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Not on the card: needed by `transfer_checked`.
    #[account(address = market.settlement_mint @ FraError::WrongMint)]
    pub settlement_mint: Box<InterfaceAccount<'info, Mint>>,

    pub clock: Sysvar<'info, Clock>,
    pub token_program: Interface<'info, TokenInterface>,
}

/// Validation: `status == Open` (REQ-66) · `now < period_end` (REQ-67) ·
/// quote fresh (REQ-66 stale rule). No halt, pause or eligibility check.
///
/// Logic:
/// 1. `close_rate = compute_fixed_rate(quote, opposite(side))`
/// 2. `pnl = ±rate_pnl(N, close_rate, K, days)` from the trader's view;
///    `clamp(pnl, −locked_margin, +committed_reserve)`
/// 3. `fee = bps_ceil(N, early_close_fee_bps)`; `net = pnl − fee`
/// 4. `net > 0`: `reserve_vault → margin_vault` (Reserve PDA signs);
///    `net < 0`: `margin_vault → reserve_vault` (TraderMargin PDA signs);
///    update both balances
/// 5. `locked −= locked_margin + settlement_fee` (settlement fee released to
///    the trader, no transfer; not on the cards yet); `open_positions −= 1`; `committed −=
///    committed_reserve`; `open_interest −= N`
/// 6. `status = Closed`; `settled_amount = net`; `settled_at = now`
/// 7. Mirror asserts · emit `PositionClosed`
///
/// Errors: `NotTrader` (REQ-65), `NotOpen`, `UseSettle` (REQ-67),
/// `QuoteStale` (REQ-66).
pub fn handle_close_position(_ctx: Context<ClosePosition>) -> Result<()> {
    todo!("IX-20 B")
}
