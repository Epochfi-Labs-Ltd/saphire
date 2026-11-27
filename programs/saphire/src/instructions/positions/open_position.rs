//! IX-18 `open_position(side: Side, notional: u64)` · B · trader
//! Implements: REQ-27–41, 74a.

use anchor_lang::prelude::*;

use crate::state::Side;

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    pub signer: Signer<'info>,
}

pub fn handle_open_position(
    _ctx: Context<OpenPosition>,
    _side: Side,
    _notional: u64,
) -> Result<()> {
    Ok(())
}
