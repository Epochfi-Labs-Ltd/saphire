//! IX-23 `unwind_position()` · C · anyone · halt-ok · pause-ok
//! Implements: REQ-69–71, 74.

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UnwindPosition<'info> {
    pub signer: Signer<'info>,
}

pub fn handle_unwind_position(_ctx: Context<UnwindPosition>) -> Result<()> {
    Ok(())
}
