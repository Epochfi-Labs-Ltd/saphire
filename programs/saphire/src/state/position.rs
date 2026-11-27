use anchor_lang::prelude::*;

use super::{Origin, PositionStatus, Side};

/// Seeds `[b"position", market, trader, position_count_le]`. Segment B.
/// Created by IX-18, IX-19 · written by IX-20 (→ Closed), IX-22 (→ Settled),
/// IX-23 (→ Unwound) · never closed in v1.
///
/// Terms (`side, notional, fixed_rate, period_*`) never change after creation
/// (REQ-82); status only moves Open → terminal (REQ-84).
#[account]
#[derive(InitSpace)]
pub struct Position {
    pub market: Pubkey,
    pub trader: Pubkey,
    /// `Market.position_count` at open; part of the seeds.
    pub nonce: u64,
    pub side: Side,
    pub notional: u64,
    /// K, scaled 10^4 per percent.
    pub fixed_rate: u64,
    pub trade_date: i64,
    pub period_start: i64,
    pub period_end: i64,
    pub day_count_days: u16,
    pub locked_margin: u64,
    pub committed_reserve: u64,
    pub fee_paid: u64,
    pub origin: Origin,
    pub status: PositionStatus,
    /// Trader's view: > 0 paid to trader, < 0 paid to reserve.
    pub settled_amount: i64,
    pub settled_at: i64,
    pub bump: u8,
    pub _reserved: [u8; 32],
}
