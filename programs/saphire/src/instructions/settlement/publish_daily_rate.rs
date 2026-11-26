//! IX-21 `publish_daily_rate(date: i64, rate: u64, days_applied: u8)` · C · operator
//! · halt-ok · pause-ok
//! Implements: REQ-51–55 (restated for daily prints: one immutable print per
//! business day, operator-only, sequential, bounded).
//!
//! Off-chain: operator cron reads the NY Fed SOFR print each business day and
//! submits it; a missed day is caught up by submitting the missing days in
//! order in one transaction. Traders mark to market from `Market.rate_index`
//! without any on-chain work.

use anchor_lang::prelude::*;

use crate::{
    constants::*,
    errors::FraError,
    state::{Config, DailyRate, Market},
};

#[derive(Accounts)]
#[instruction(date: i64)]
pub struct PublishDailyRate<'info> {
    #[account(mut)]
    pub operator: Signer<'info>,

    #[account(seeds = [SEED_CONFIG], bump = config.bump)]
    pub config: Account<'info, Config>,

    /// Written: `rate_index`, `last_rate_date`.
    #[account(mut, has_one = operator @ FraError::NotOperator)]
    pub market: Account<'info, Market>,

    #[account(
        init,
        payer = operator,
        space = 8 + DailyRate::INIT_SPACE,
        seeds = [SEED_RATE, market.key().as_ref(), &date.to_le_bytes()],
        bump,
    )]
    pub daily_rate: Account<'info, DailyRate>,

    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

/// Validation:
/// - `date == next_business_day(market.last_rate_date, cal)` →
///   `RateDateNotNext` (no gaps, no duplicates — duplicate also fails at init)
/// - `days_applied == days_until(date, next_business_day(date, cal))` →
///   `InvalidParams` (1 on a normal day, 3 on a Friday)
/// - `date + days_applied × 86_400 <= now` → `RateDateInFuture` (the print
///   must be for a completed accrual window)
/// - `0 < rate <= config.rate_ceiling` → `RateOutOfRange`
///
/// Logic:
/// 1. `index_after = fra_math::compound_index(market.rate_index, rate,
///    days_applied)`
/// 2. Write DailyRate `{ market, date, rate, days_applied, index_before:
///    market.rate_index, index_after, published_at: now }`
/// 3. `market.rate_index = index_after`; `market.last_rate_date = date`
/// 4. Emit `DailyRatePublished`
///
/// Errors: `NotOperator`, `RateDateNotNext`, `RateDateInFuture`,
/// `RateOutOfRange`, `InvalidParams`.
pub fn handle_publish_daily_rate(
    _ctx: Context<PublishDailyRate>,
    _date: i64,
    _rate: u64,
    _days_applied: u8,
) -> Result<()> {
    todo!("IX-21 C")
}
