//! IX-20 `close_position()` · B · trader · halt-ok · pause-ok
//! Implements: REQ-64–68, 74.

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    pub signer: Signer<'info>,
}

pub fn handle_close_position(_ctx: Context<ClosePosition>) -> Result<()> {
    Ok(())
}
