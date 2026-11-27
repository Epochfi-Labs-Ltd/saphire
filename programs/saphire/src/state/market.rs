use anchor_lang::prelude::*;

use super::MarketStatus;

/// Seeds `[b"market", benchmark_id, settlement_mint, start_offset_months,
/// tenor_months]`. Segment A.
/// Created by IX-05 · written by IX-13/14 (`status`); IX-18/19/20, IX-22/23
/// (`open_interest`); IX-18/19 (`position_count`); IX-21 (`rate_index`,
/// `last_rate_date`) · read by everything else.
#[account]
#[derive(InitSpace)]
pub struct Market {
    pub operator: Pubkey,
    pub keeper: Pubkey,
    pub benchmark_id: u8,
    pub settlement_mint: Pubkey,
    pub start_offset_months: u8,
    pub tenor_months: u8,
    pub day_count_basis: u16,
    pub business_day_convention: u8,
    pub initial_margin_bps: u16,
    pub operator_commit_bps: u16,
    pub min_notional: u64,
    pub max_notional: u64,
    pub open_interest_cap: u64,
    pub open_interest: u64,
    pub quote_validity_secs: u32,
    pub rate_grace_days: u16,
    pub early_close_fee_bps: u16,
    pub max_half_spread_bps: u16,
    pub status: MarketStatus,
    pub position_count: u64,
    /// Cumulative compounding factor, scale 10^18; starts
    /// at 10^18. Mirrors the latest `DailyRate.index_after`.
    pub rate_index: u128,
    /// `date` of the latest `DailyRate`; initialised to
    /// `params.index_start_date` (the business day before the first print).
    pub last_rate_date: i64,
    pub bump: u8,
    pub _reserved: [u8; 32],
}
