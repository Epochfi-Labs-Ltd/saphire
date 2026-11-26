//! IX-08 `publish_quote(fixed_mid: u64, half_spread_bps: u16, max_notional_at_quote: u64)`
//! · A · operator · halt-ok
//! Implements: REQ-12–16.

use anchor_lang::prelude::*;

use crate::{
    constants::*,
    errors::FraError,
    state::{Config, Market, Quote},
};

#[derive(Accounts)]
pub struct PublishQuote<'info> {
    pub operator: Signer<'info>,

    #[account(seeds = [SEED_CONFIG], bump = config.bump)]
    pub config: Account<'info, Config>,

    #[account(has_one = operator @ FraError::NotOperator)]
    pub market: Account<'info, Market>,

    #[account(mut, has_one = market)]
    pub quote: Account<'info, Quote>,

    pub clock: Sysvar<'info, Clock>,
}

/// Validation:
/// - `market.status != Closed`
/// - `0 < fixed_mid <= config.rate_ceiling` (REQ-16)
/// - `half_spread_bps <= market.max_half_spread_bps` (REQ-15)
/// - `fixed_mid > half_spread_bps · 100` (receive-fixed rate stays positive)
/// - `market.min_notional <= max_notional_at_quote <= market.max_notional`
///
/// Logic: overwrite fields · `published_at = now` · `epoch += 1`
/// (program-advanced, REQ-14) · emit `QuotePublished`.
///
/// Errors: `NotOperator` (REQ-13), `MarketClosed`, `RateOutOfRange`,
/// `SpreadTooWide`, `InvalidParams`.
pub fn handle_publish_quote(
    _ctx: Context<PublishQuote>,
    _fixed_mid: u64,
    _half_spread_bps: u16,
    _max_notional_at_quote: u64,
) -> Result<()> {
    todo!("IX-08 A")
}
