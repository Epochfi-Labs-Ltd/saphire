//! One enum, grouped by section so each refusal has a home. Codes are
//! assigned in this order; the comment on each variant names the requirement
//! it enforces. Source: docs/implementation.md, "Errors".
//!
//! | Group | Raised by | Owner |
//! | --- | --- | --- |
//! | Access | every instruction with a signer constraint | A, B, C |
//! | Program and market state | IX-04, 05, 08, 13–16, 18, 19 | A |
//! | Parameters, mints, accounting | IX-01, 05–08, 12, 17–20, 22 | A, B, C |
//! | Reserve and margin | IX-07, 17, 18, 19 | A, B |
//! | Quote and opening | IX-08, 18, 19, 20 | A, B |
//! | Signed deal | IX-19 | B |
//! | Position lifecycle | IX-20, 22, 23, 24 | B, C |
//! | Daily rates and settlement | IX-21, 22, 23 | C |
//! | Fees | IX-25 | A |

use anchor_lang::prelude::*;

#[error_code]
pub enum FraError {
    // ── Access: wrong signer ──────────────────────────────────────── A, B, C
    #[msg("Signer is not Config.authority")]
    NotAuthority, // REQ-02a, 02b, 02d
    #[msg("Signer is not Market.operator")]
    NotOperator, // REQ-08, 11, 13, 19, 52, 73
    #[msg("Signer is not the owner of this margin/position")]
    NotTrader, // REQ-25, 65
    #[msg("Trader is not eligible for this market")]
    NotEligible, // REQ-21, 35

    // ── Program and market state ──────────────────────────────────── A
    #[msg("Operator is not registered or is inactive")]
    OperatorInactive, // REQ-02, 02c
    #[msg("Market with these seeds already exists")]
    MarketExists, // REQ-03
    #[msg("Market is Closed")]
    MarketClosed,
    #[msg("Market is Halted; opening refused")]
    MarketHalted, // REQ-41
    #[msg("Market is not Halted")]
    MarketNotHalted, // REQ-72
    #[msg("Status transition not allowed")]
    InvalidTransition, // REQ-72
    #[msg("Program is paused; opening refused")]
    ProgramPaused, // REQ-41, 74a

    // ── Parameters, mints, accounting ─────────────────────────────── A, B, C
    #[msg("Parameter out of range or zero")]
    InvalidParams, // REQ-04, 05, 22
    #[msg("Token account mint or owner mismatch")]
    WrongMint,
    #[msg("Balance mirror differs from vault amount")]
    MirrorMismatch, // REQ-75
    #[msg("Checked arithmetic overflow")]
    MathOverflow,

    // ── Reserve and margin ────────────────────────────────────────── A, B
    #[msg("Withdrawal exceeds balance minus committed")]
    InsufficientWithdrawable, // REQ-10
    #[msg("Free margin below lock plus fee, or withdrawal")]
    InsufficientFreeMargin, // REQ-24, 37
    #[msg("Uncommitted reserve below required commit")]
    InsufficientReserve, // REQ-38

    // ── Quote and opening ─────────────────────────────────────────── A, B
    #[msg("Quote unpublished or older than validity")]
    QuoteStale, // REQ-36, 66
    #[msg("Half-spread above market maximum")]
    SpreadTooWide, // REQ-15
    #[msg("Rate is zero or above Config.rate_ceiling")]
    RateOutOfRange, // REQ-16, 55 (quote mid and daily print)
    #[msg("Notional outside market or quote bounds")]
    NotionalOutOfBounds, // REQ-39
    #[msg("Open interest cap would be exceeded")]
    OpenInterestCap, // REQ-40

    // ── Signed deal (IX-19) ───────────────────────────────────────── B
    #[msg("No Ed25519 instruction precedes this one")]
    MissingSignature, // REQ-42–50
    #[msg("Signature is not from Market.operator")]
    BadSigner,
    #[msg("Signed message differs from the submitted deal")]
    MessageMismatch,
    #[msg("Deal has expired")]
    OfferExpired,
    #[msg("Deal submitted at or after period_start")]
    PeriodStarted,
    #[msg("Period dates inconsistent with offset/tenor")]
    BadPeriod,
    #[msg("Deal nonce already used")]
    NonceUsed,

    // ── Position lifecycle ────────────────────────────────────────── B, C
    #[msg("Position is not Open")]
    NotOpen, // REQ-62, 66, 84, 85
    #[msg("Period has ended; use settle_position")]
    UseSettle, // REQ-67
    #[msg("Period has not ended")]
    PeriodNotEnded, // REQ-54
    #[msg("UsedNonce cannot be closed while Open")]
    PositionStillOpen,

    // ── Daily rates and settlement ────────────────────────────────── C
    #[msg("Date is not the next business day after the last print")]
    RateDateNotNext, // REQ-53
    #[msg("Print's accrual window has not completed")]
    RateDateInFuture, // REQ-54
    #[msg("Rate checkpoint for period end already exists")]
    RateCheckpointExists, // REQ-71
    #[msg("Rate checkpoint for period start or end is missing")]
    RateCheckpointMissing, // REQ-61
    #[msg("Grace period after period_end has not elapsed")]
    GraceNotElapsed, // REQ-70
    #[msg("Checkpoint address is not the derived PDA")]
    BadRateCheckpointAddress, // REQ-71

    // ── Fees ──────────────────────────────────────────────────────── A
    // Appended last so existing codes do not shift. Not on the cards yet; see docs/skeleton.md "Fee additions".
    #[msg("Withdrawal exceeds fee vault balance")]
    InsufficientFees,
}

impl From<fra_math::MathError> for FraError {
    /// Default mapping for `fra_math` errors. Handlers that need a
    /// card-specific error for `InvalidInput` (e.g. `BadPeriod`) map it
    /// explicitly instead of using `?`.
    fn from(e: fra_math::MathError) -> Self {
        match e {
            fra_math::MathError::Overflow => FraError::MathOverflow,
            fra_math::MathError::InvalidInput => FraError::InvalidParams,
        }
    }
}
