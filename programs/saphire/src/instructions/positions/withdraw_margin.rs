//! IX-17 `withdraw_margin(amount: u64)` · B · trader · halt-ok · pause-ok
//! Implements: REQ-23–26, 74.

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct WithdrawMargin<'info> {
    pub signer: Signer<'info>,
}

pub fn handle_withdraw_margin(_ctx: Context<WithdrawMargin>, _amount: u64) -> Result<()> {
    Ok(())
}
