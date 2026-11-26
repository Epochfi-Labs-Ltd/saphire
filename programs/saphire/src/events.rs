//! One event per instruction, named as on the instruction cards. Cards that
//! say only "emit event" or name none get the name noted on the struct (see
//! docs/skeleton.md).

use anchor_lang::prelude::*;

use crate::state::{Origin, Side};

// ── Segment A ──────────────────────────────────────────────────────────────

/// IX-01
#[event]
pub struct ConfigInitialized {
    pub authority: Pubkey,
    pub fee_vault: Pubkey,
    pub settlement_mint: Pubkey,
    pub protocol_fee_bps: u16,
    pub rate_ceiling: u64,
}

/// IX-02
#[event]
pub struct ConfigUpdated {
    pub protocol_fee_bps: u16,
    pub rate_ceiling: u64,
}

/// IX-03
#[event]
pub struct OperatorRegistered {
    pub operator: Pubkey,
}

/// IX-04
#[event]
pub struct OperatorDeactivated {
    pub operator: Pubkey,
}

/// IX-05
#[event]
pub struct MarketCreated {
    pub market: Pubkey,
    pub operator: Pubkey,
    pub keeper: Pubkey,
    pub settlement_mint: Pubkey,
    pub benchmark_id: u8,
    pub start_offset_months: u8,
    pub tenor_months: u8,
}

/// IX-06
#[event]
pub struct ReserveFunded {
    pub market: Pubkey,
    pub amount: u64,
    pub balance: u64,
}

/// IX-07
#[event]
pub struct ReserveWithdrawn {
    pub market: Pubkey,
    pub amount: u64,
    pub balance: u64,
}

/// IX-08
#[event]
pub struct QuotePublished {
    pub market: Pubkey,
    pub fixed_mid: u64,
    pub half_spread_bps: u16,
    pub max_notional_at_quote: u64,
    pub published_at: i64,
    pub epoch: u64,
}

/// IX-09
#[event]
pub struct TraderAdmitted {
    pub market: Pubkey,
    pub trader: Pubkey,
}

/// IX-10
#[event]
pub struct TraderSuspended {
    pub market: Pubkey,
    pub trader: Pubkey,
}

/// IX-13
#[event]
pub struct MarketHalted {
    pub market: Pubkey,
}

/// IX-14
#[event]
pub struct MarketResumed {
    pub market: Pubkey,
}

/// IX-25. Not on the cards yet; see docs/skeleton.md "Fee additions".
#[event]
pub struct FeesWithdrawn {
    pub authority: Pubkey,
    pub destination: Pubkey,
    pub amount: u64,
}

/// IX-15. Card says "emit event" without a name.
#[event]
pub struct ProgramPaused {}

/// IX-16. Card says "emit event" without a name.
#[event]
pub struct ProgramResumed {}

// ── Segment B ──────────────────────────────────────────────────────────────

/// IX-11. Card names no event.
#[event]
pub struct TraderMarginInitialized {
    pub market: Pubkey,
    pub trader: Pubkey,
    pub vault: Pubkey,
}

/// IX-12
#[event]
pub struct MarginDeposited {
    pub market: Pubkey,
    pub trader: Pubkey,
    pub amount: u64,
    pub balance: u64,
}

/// IX-17
#[event]
pub struct MarginWithdrawn {
    pub market: Pubkey,
    pub trader: Pubkey,
    pub amount: u64,
    pub balance: u64,
}

/// IX-18, IX-19
#[event]
pub struct PositionOpened {
    pub market: Pubkey,
    pub trader: Pubkey,
    pub position: Pubkey,
    pub nonce: u64,
    pub side: Side,
    pub notional: u64,
    pub fixed_rate: u64,
    pub period_start: i64,
    pub period_end: i64,
    pub locked_margin: u64,
    pub committed_reserve: u64,
    pub fee_paid: u64,
    pub settlement_fee: u64,
    pub origin: Origin,
}

/// IX-19
#[event]
pub struct DealNonceUsed {
    pub market: Pubkey,
    pub trader: Pubkey,
    pub nonce: u64,
    pub position: Pubkey,
}

/// IX-20
#[event]
pub struct PositionClosed {
    pub market: Pubkey,
    pub position: Pubkey,
    pub close_rate: u64,
    pub settled_amount: i64,
    pub fee: u64,
}

/// IX-24. Card names no event.
#[event]
pub struct UsedNonceClosed {
    pub market: Pubkey,
    pub trader: Pubkey,
    pub nonce: u64,
}

// ── Segment C ──────────────────────────────────────────────────────────────

/// IX-21
#[event]
pub struct DailyRatePublished {
    pub market: Pubkey,
    pub date: i64,
    pub rate: u64,
    pub days_applied: u8,
    pub index_after: u128,
}

/// IX-22
#[event]
pub struct PositionSettled {
    pub market: Pubkey,
    pub position: Pubkey,
    /// Signer who cranked; received `settlement_fee`.
    pub cranker: Pubkey,
    pub settlement_fee: u64,
    /// L derived from the two rate-index checkpoints.
    pub float_rate: u64,
    pub settled_amount: i64,
}

/// IX-23
#[event]
pub struct PositionUnwound {
    pub market: Pubkey,
    pub position: Pubkey,
    /// Signer who cranked; received `settlement_fee`.
    pub cranker: Pubkey,
    pub settlement_fee: u64,
}
