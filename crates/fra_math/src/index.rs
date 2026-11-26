//! Daily SOFR rate index (docs/implementation.md, "DailyRate").
//!
//! The index is a cumulative compounding factor at scale 10^18, starting at
//! 10^18 when a market is created. Each daily print compounds it:
//!
//! `index_after = index_before × (10^18 + rate × days_applied × 10^18 / (10^6 × 360)) / 10^18`
//!
//! The floating rate for a period is derived from two checkpoints:
//!
//! `L = ((index_end × 10^18 / index_start) − 10^18) × 360 × 10^6 / (days × 10^18)`
//!
//! All divisions truncate. Products that can exceed `u128` use a 256-bit
//! intermediate ([`mul_div`]), so the crate stays dependency-free and
//! `no_std`.

use crate::{MathError, Result, DAY_COUNT_BASIS, RATE_DENOMINATOR};

/// Index scale: 1.0 == 10^18.
pub const INDEX_SCALE: u128 = 1_000_000_000_000_000_000;

/// `floor(a × b / d)` with a 256-bit intermediate.
///
/// `InvalidInput` if `d == 0`; `Overflow` if the quotient does not fit
/// `u128`.
pub fn mul_div(a: u128, b: u128, d: u128) -> Result<u128> {
    if d == 0 {
        return Err(MathError::InvalidInput);
    }
    let (hi, lo) = mul_wide(a, b);
    if hi >= d {
        return Err(MathError::Overflow);
    }
    // Restoring long division of (hi:lo) by d, one bit of `lo` at a time.
    // Invariant: remainder < d, so the quotient fits in 128 bits.
    let mut rem = hi;
    let mut quot: u128 = 0;
    for i in (0..128).rev() {
        let carry = rem >> 127;
        rem = (rem << 1) | ((lo >> i) & 1);
        quot <<= 1;
        if carry == 1 || rem >= d {
            // With carry the true remainder is rem + 2^128 >= d; the wrapping
            // subtraction yields the exact (< d) result.
            rem = rem.wrapping_sub(d);
            quot |= 1;
        }
    }
    Ok(quot)
}

/// Full 256-bit product of two `u128`s as `(high, low)`.
fn mul_wide(a: u128, b: u128) -> (u128, u128) {
    const MASK: u128 = u64::MAX as u128;
    let (a0, a1) = (a & MASK, a >> 64);
    let (b0, b1) = (b & MASK, b >> 64);
    let p00 = a0 * b0;
    let p01 = a0 * b1;
    let p10 = a1 * b0;
    let p11 = a1 * b1;
    // Each term < 2^64, so the sum < 3 × 2^64: no overflow.
    let mid = (p00 >> 64) + (p01 & MASK) + (p10 & MASK);
    let lo = (p00 & MASK) | (mid << 64);
    // The full product is < 2^256, so `hi` cannot overflow.
    let hi = p11 + (p01 >> 64) + (p10 >> 64) + (mid >> 64);
    (hi, lo)
}

/// Apply one daily print: `index × (10^18 + rate × days_applied × 10^18 /
/// (10^6 × 360)) / 10^18`, truncating.
///
/// The growth term is computed as `rate × days_applied × 10^12 / 360`, which
/// is the same rational as the spec's form and cannot overflow `u128`.
pub fn compound_index(index_before: u128, rate: u64, days_applied: u8) -> Result<u128> {
    if days_applied == 0 {
        return Err(MathError::InvalidInput);
    }
    let per_unit = INDEX_SCALE / u128::from(RATE_DENOMINATOR); // 10^12
    let growth = u128::from(rate)
        .checked_mul(u128::from(days_applied))
        .and_then(|v| v.checked_mul(per_unit))
        .ok_or(MathError::Overflow)?
        / u128::from(DAY_COUNT_BASIS);
    let factor = INDEX_SCALE.checked_add(growth).ok_or(MathError::Overflow)?;
    mul_div(index_before, factor, INDEX_SCALE)
}

/// Floating rate L over a period from its two index checkpoints, scaled 10^4
/// per percent: `((index_end × 10^18 / index_start) − 10^18) × 360 × 10^6 /
/// (days × 10^18)`, truncating at each division as the spec states.
///
/// `InvalidInput` if `index_start == 0`, `days == 0`, or `index_end <
/// index_start` (the index never decreases).
pub fn period_rate_from_index(index_start: u128, index_end: u128, days: u16) -> Result<u64> {
    if index_start == 0 || days == 0 || index_end < index_start {
        return Err(MathError::InvalidInput);
    }
    let ratio = mul_div(index_end, INDEX_SCALE, index_start)?;
    let growth = ratio - INDEX_SCALE; // ratio >= INDEX_SCALE since end >= start
    let numerator_scale = u128::from(DAY_COUNT_BASIS) * u128::from(RATE_DENOMINATOR); // 3.6e8
    let denominator = u128::from(days)
        .checked_mul(INDEX_SCALE)
        .ok_or(MathError::Overflow)?;
    let l = mul_div(growth, numerator_scale, denominator)?;
    u64::try_from(l).map_err(|_| MathError::Overflow)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{date::next_business_day, days_until, WeekendOnly, SECONDS_PER_DAY};

    // ── mul_div ─────────────────────────────────────────────────────────

    #[test]
    fn mul_div_small_values_match_native() {
        for (a, b, d) in [
            (7u128, 9u128, 4u128),
            (0, 5, 3),
            (1, 1, 1),
            (123_456, 789, 1_000),
        ] {
            assert_eq!(mul_div(a, b, d), Ok(a * b / d));
        }
    }

    #[test]
    fn mul_div_handles_256_bit_intermediate() {
        // u128::MAX × u128::MAX / u128::MAX = u128::MAX
        assert_eq!(mul_div(u128::MAX, u128::MAX, u128::MAX), Ok(u128::MAX));
        // (2^127) × 4 / 8 = 2^126
        assert_eq!(mul_div(1 << 127, 4, 8), Ok(1 << 126));
        // u128::MAX × 10^18 / 10^18 = u128::MAX
        assert_eq!(mul_div(u128::MAX, INDEX_SCALE, INDEX_SCALE), Ok(u128::MAX));
    }

    #[test]
    fn mul_div_truncates() {
        assert_eq!(mul_div(u128::MAX, 2, 3), Ok(u128::MAX / 3 * 2));
        assert_eq!(mul_div(10, 1, 3), Ok(3));
    }

    #[test]
    fn mul_div_errors() {
        assert_eq!(mul_div(1, 1, 0), Err(MathError::InvalidInput));
        assert_eq!(mul_div(u128::MAX, 2, 1), Err(MathError::Overflow));
    }

    // ── compound_index ──────────────────────────────────────────────────

    #[test]
    fn compound_one_day() {
        // 5.3000% for 1 day: growth = 53_000 × 10^12 / 360 = 147_222_222_222_222.2 -> trunc
        assert_eq!(
            compound_index(INDEX_SCALE, 53_000, 1),
            Ok(INDEX_SCALE + 147_222_222_222_222)
        );
    }

    #[test]
    fn compound_friday_applies_three_days() {
        // 3 days: 159_000 × 10^12 / 360 = 441_666_666_666_666.6 -> trunc
        assert_eq!(
            compound_index(INDEX_SCALE, 53_000, 3),
            Ok(INDEX_SCALE + 441_666_666_666_666)
        );
    }

    #[test]
    fn compound_multiplies_existing_index() {
        // index 2.0 × (1 + 3.6% / 360) = 2.0 × 1.0001 = 2.0002
        let two = 2 * INDEX_SCALE;
        assert_eq!(
            compound_index(two, 36_000, 1),
            Ok(2_000_200_000_000_000_000)
        );
    }

    #[test]
    fn compound_zero_rate_leaves_index() {
        assert_eq!(compound_index(INDEX_SCALE, 0, 1), Ok(INDEX_SCALE));
    }

    #[test]
    fn compound_rejects_zero_days() {
        assert_eq!(
            compound_index(INDEX_SCALE, 53_000, 0),
            Err(MathError::InvalidInput)
        );
    }

    #[test]
    fn compound_overflow_errors() {
        assert_eq!(
            compound_index(u128::MAX, 53_000, 1),
            Err(MathError::Overflow)
        );
    }

    #[test]
    fn compound_extreme_inputs_do_not_panic() {
        assert!(compound_index(INDEX_SCALE, u64::MAX, u8::MAX).is_ok());
    }

    // ── period_rate_from_index ──────────────────────────────────────────

    #[test]
    fn single_day_round_trip_truncates_by_at_most_one_unit() {
        // Compounding 4.0700% for one day then deriving L gives 40_699: each
        // of the two truncations can drop a sub-unit remainder.
        let end = compound_index(INDEX_SCALE, 40_700, 1).unwrap();
        assert_eq!(period_rate_from_index(INDEX_SCALE, end, 1), Ok(40_699));
    }

    #[test]
    fn exact_growth_round_trips_exactly() {
        // 3.6% for 1 day grows the index by exactly 10^14, so L is exact.
        let end = compound_index(INDEX_SCALE, 36_000, 1).unwrap();
        assert_eq!(end, INDEX_SCALE + 100_000_000_000_000);
        assert_eq!(period_rate_from_index(INDEX_SCALE, end, 1), Ok(36_000));
    }

    #[test]
    fn ratio_not_absolute_level_matters() {
        let a = compound_index(INDEX_SCALE, 36_000, 1).unwrap();
        let start = 3 * INDEX_SCALE;
        let end = compound_index(start, 36_000, 1).unwrap();
        assert_eq!(
            period_rate_from_index(start, end, 1),
            period_rate_from_index(INDEX_SCALE, a, 1)
        );
    }

    #[test]
    fn quarter_of_daily_prints_compounds_above_the_flat_rate() {
        // 5.0000% printed every business day for 13 weeks (91 calendar days)
        // under WeekendOnly, starting Monday 2026-01-05.
        let day = SECONDS_PER_DAY;
        let start_date = 20_458 * day; // Mon 2026-01-05
        let end_date = start_date + 91 * day;
        let mut date = start_date;
        let mut index = INDEX_SCALE;
        let mut applied_days = 0u32;
        while date < end_date {
            let next = next_business_day(date, &WeekendOnly).unwrap();
            let n = days_until(date, next).unwrap();
            index = compound_index(index, 50_000, n).unwrap();
            applied_days += u32::from(n);
            date = next;
        }
        assert_eq!(applied_days, 91);
        let l = period_rate_from_index(INDEX_SCALE, index, 91).unwrap();
        // Compounding in arrears lifts L to 5.0310%, 3.1 bp above the flat
        // print. Pinned as a regression value.
        assert_eq!(l, 50_310);
    }

    #[test]
    fn equal_checkpoints_give_zero() {
        assert_eq!(period_rate_from_index(INDEX_SCALE, INDEX_SCALE, 91), Ok(0));
    }

    #[test]
    fn rejects_bad_inputs() {
        assert_eq!(
            period_rate_from_index(0, INDEX_SCALE, 91),
            Err(MathError::InvalidInput)
        );
        assert_eq!(
            period_rate_from_index(INDEX_SCALE, INDEX_SCALE, 0),
            Err(MathError::InvalidInput)
        );
        assert_eq!(
            period_rate_from_index(INDEX_SCALE + 1, INDEX_SCALE, 91),
            Err(MathError::InvalidInput)
        );
    }

    #[test]
    fn result_above_u64_errors() {
        // index grew 10^18-fold in one day -> L far beyond u64.
        let end = INDEX_SCALE * INDEX_SCALE;
        assert_eq!(
            period_rate_from_index(INDEX_SCALE, end, 1),
            Err(MathError::Overflow)
        );
    }
}
