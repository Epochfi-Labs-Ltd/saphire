//! IX-24 `close_used_nonce()` · B/C · trader · optional
//!

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseUsedNonce<'info> {
    pub signer: Signer<'info>,
}

pub fn handle_close_used_nonce(_ctx: Context<CloseUsedNonce>) -> Result<()> {
    Ok(())
}
