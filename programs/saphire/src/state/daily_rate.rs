use anchor_lang::prelude::*;

/// Seeds `[b"rate", market, date_le]`, one per business day per market.
/// Segment C. `date` is 00:00 UTC of the print's calendar day.
/// Created by IX-21 · never written again (REQ-83) · read by IX-22 (two
/// checkpoints: period_start and period_end); the *absence* of the
/// period_end checkpoint is checked by IX-23.
///
/// `Market.rate_index` / `Market.last_rate_date` mirror the latest
/// checkpoint.
#[account]
#[derive(InitSpace)]
pub struct DailyRate {
    pub market: Pubkey,
    /// 00:00 UTC of the business day.
    pub date: i64,
    /// SOFR print, scaled 10^4 per percent.
    pub rate: u64,
    /// Calendar days this print accrues for (1; 3 over a weekend; more over
    /// holidays).
    pub days_applied: u8,
    /// `Market.rate_index` when this print was applied.
    pub index_before: u128,
    /// `index_before × (1 + rate × days_applied / 360)`, scale 10^18.
    pub index_after: u128,
    pub published_at: i64,
    pub bump: u8,
    pub _reserved: [u8; 32],
}
