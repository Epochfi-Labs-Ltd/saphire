//! `fra_math`: the only place in Saphire where rates are priced, amounts are
//! rounded, and period dates are derived. Pure Rust, `no_std`, no Solana deps,
//! so the program, the SDK, the keeper and the tests all compute identically.
//! Owned by Segment B; called by all segments.
//!
//! SKELETON: the quote/margin/period functions are signatures with
//! `todo!()` bodies. The daily-rate index functions ([`index`] and the
//! business-day helpers in [`date`]) are implemented.
//!
//! Units (docs/implementation.md, "Conventions"):
//! - amounts: `u64` base units of the settlement mint
//! - rates: `u64` scaled 10^4 per percent (4.0700% = 40_700, 1 bp = 100), so a
//!   rate of `r` is the fraction `r / 10^6`
//! - bps: `u16`
//! - dates: `i64` Unix seconds
//!
//! Rounding: locks, commits and fees round **up** ([`bps_ceil`]); PnL
//! truncates **toward zero** ([`rate_pnl`]).

#![cfg_attr(not(test), no_std)]

pub mod date;
pub mod index;

pub use date::{days_until, derive_period, next_business_day, Calendar, WeekendOnly};
pub use index::{compound_index, period_rate_from_index, INDEX_SCALE};

/// Rate units per basis point (rates are scaled 10^4 per percent).
pub const RATE_PER_BP: u64 = 100;
/// Denominator turning a scaled rate into a fraction: 1.0 == 1_000_000.
pub const RATE_DENOMINATOR: u64 = 1_000_000;
/// Basis-point denominator.
pub const BPS_DENOMINATOR: u64 = 10_000;
/// ACT/360 day-count basis.
pub const DAY_COUNT_BASIS: u64 = 360;
/// Seconds per calendar day.
pub const SECONDS_PER_DAY: i64 = 86_400;

/// Errors from `fra_math`. The program maps `Overflow` to
/// `FraError::MathOverflow` and `InvalidInput` to the card-specific error
/// (`InvalidParams` or `BadPeriod`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MathError {
    /// A checked operation overflowed or underflowed, or a result does not
    /// fit the return type.
    Overflow,
    /// An input is outside the function's domain (e.g. `end <= start`).
    InvalidInput,
}

pub type Result<T> = core::result::Result<T, MathError>;

/// Side of an FRA from the trader's point of view. Mirrors the program's
/// `Side` enum; the program converts at the call site.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    /// Pays K, receives L: gains when L > K.
    PayFixed,
    /// Receives K, pays L: gains when L < K.
    ReceiveFixed,
}

/// Executable fixed rate K from a quote.
/// PayFixed `mid + half_spread_bps · 100`; ReceiveFixed `mid − half_spread_bps · 100`.
pub fn compute_fixed_rate(_mid: u64, _half_spread_bps: u16, _side: Side) -> Result<u64> {
    todo!("fra_math::compute_fixed_rate (B)")
}

/// `ceil(amount · bps / 10_000)` in `u128`. Locks, commits, fees.
pub fn bps_ceil(_amount: u64, _bps: u16) -> Result<u64> {
    todo!("fra_math::bps_ceil (B)")
}

/// `(end − start) / 86_400`, must be > 0 and fit `u16`.
pub fn year_fraction_days(_start: i64, _end: i64) -> Result<u16> {
    todo!("fra_math::year_fraction_days (B)")
}

/// `notional · (rate_a − rate_b) · days / (10^6 · 360)`, multiply first in
/// `i128`, divide last, truncate toward zero.
pub fn rate_pnl(_notional: u64, _rate_a: u64, _rate_b: u64, _days: u16) -> Result<i128> {
    todo!("fra_math::rate_pnl (B)")
}

// TODO(B): implement these and remove the #[ignore]s. Cases are the worked
// examples from docs/implementation.md and the scaffolding brief.
#[cfg(test)]
mod tests {
    /// mid 40_500 + 2 bp, PayFixed -> K = 40_700.
    #[test]
    #[ignore = "TODO(B): fra_math not implemented"]
    fn fixed_rate_pay_adds_spread_doc_example() {
        todo!()
    }

    /// mid 40_500 − 2 bp, ReceiveFixed -> 40_300; underflow -> Overflow.
    #[test]
    #[ignore = "TODO(B): fra_math not implemented"]
    fn fixed_rate_receive_subtracts_spread() {
        todo!()
    }

    /// bps_ceil rounds up: (1, 1) -> 1; (10_001, 1) -> 2; exact multiples unchanged.
    #[test]
    #[ignore = "TODO(B): fra_math not implemented"]
    fn bps_ceil_rounds_up() {
        todo!()
    }

    /// bps_ceil result above u64::MAX -> Overflow.
    #[test]
    #[ignore = "TODO(B): fra_math not implemented"]
    fn bps_ceil_overflow_errors() {
        todo!()
    }

    /// 89-day period -> 89; end <= start or < 1 day -> InvalidInput.
    #[test]
    #[ignore = "TODO(B): fra_math not implemented"]
    fn year_fraction_days_cases() {
        todo!()
    }

    /// 1,000,000 USDC × (43000 − 40700) × 89 / (10^6 × 360) = 568_611_111 base units.
    #[test]
    #[ignore = "TODO(B): fra_math not implemented"]
    fn rate_pnl_doc_example() {
        todo!()
    }

    /// Reversed rates -> −568_611_111 (toward zero, not −568_611_112).
    #[test]
    #[ignore = "TODO(B): fra_math not implemented"]
    fn rate_pnl_negative_truncates_toward_zero() {
        todo!()
    }

    /// Overflowing inputs -> Overflow.
    #[test]
    #[ignore = "TODO(B): fra_math not implemented"]
    fn rate_pnl_overflow_errors() {
        todo!()
    }
}
