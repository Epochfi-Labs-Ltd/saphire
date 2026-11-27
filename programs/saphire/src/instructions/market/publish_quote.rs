//! IX-08 `publish_quote(fixed_mid: u64, half_spread_bps: u16, max_notional_at_quote: u64)`
//! · A · operator · halt-ok

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct PublishQuote<'info> {
    pub signer: Signer<'info>,
}

pub fn handle_publish_quote(
    _ctx: Context<PublishQuote>,
    _fixed_mid: u64,
    _half_spread_bps: u16,
    _max_notional_at_quote: u64,
) -> Result<()> {
    Ok(())
}
