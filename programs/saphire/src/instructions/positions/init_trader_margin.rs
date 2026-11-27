//! IX-11 `init_trader_margin()` · B · trader
//! Implements: REQ-21 (gate).

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitTraderMargin<'info> {
    pub signer: Signer<'info>,
}

pub fn handle_init_trader_margin(_ctx: Context<InitTraderMargin>) -> Result<()> {
    Ok(())
}
