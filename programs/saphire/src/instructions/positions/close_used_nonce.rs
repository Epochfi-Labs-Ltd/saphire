//! IX-24 `close_used_nonce()` · B/C · trader · optional
//!
//! DECISION PENDING: removed if IX-19 moves to the co-signer design.

use anchor_lang::prelude::*;

use crate::{
    errors::FraError,
    state::{Position, UsedNonce},
};

#[derive(Accounts)]
pub struct CloseUsedNonce<'info> {
    /// Signer and rent receiver.
    #[account(mut)]
    pub trader: Signer<'info>,

    #[account(has_one = trader @ FraError::NotTrader)]
    pub position: Account<'info, Position>,

    #[account(mut, close = trader, has_one = position)]
    pub used_nonce: Account<'info, UsedNonce>,
}

/// Validation: `position.status != Open`.
/// Logic: close, rent to trader (done by the `close` constraint on exit) ·
/// emit `UsedNonceClosed`.
/// Errors: `PositionStillOpen`.
pub fn handle_close_used_nonce(_ctx: Context<CloseUsedNonce>) -> Result<()> {
    todo!("IX-24 B/C")
}
