//! Business days and period dates.
//!
//! Dates are Unix seconds; a "day" is a whole day since 1970-01-01 (UTC).
//!
//! SKELETON: [`derive_period`] is `todo!()`. The calendar and the daily-print
//! sequencing helpers ([`next_business_day`], [`days_until`]) are
//! implemented.

use crate::{MathError, Result, SECONDS_PER_DAY};

/// Longest run of consecutive non-business days a calendar may have before
/// the date helpers give up.
const MAX_ROLL_DAYS: i64 = 31;

/// Business-day calendar. `day` is whole days since 1970-01-01 (UTC).
pub trait Calendar {
    fn is_business_day(&self, day: i64) -> bool;
}

/// v1 calendar: Saturday and Sunday are holidays, nothing else is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WeekendOnly;

impl Calendar for WeekendOnly {
    fn is_business_day(&self, day: i64) -> bool {
        // 1970-01-01 was a Thursday; 0 = Sunday .. 6 = Saturday.
        !matches!((day + 4).rem_euclid(7), 0 | 6)
    }
}

/// 00:00 UTC of the first business day strictly after `date`'s calendar day.
///
/// Sequencing for daily prints: IX-21 accepts `date` only if it equals
/// `next_business_day(market.last_rate_date, cal)`.
pub fn next_business_day<C: Calendar + ?Sized>(date: i64, cal: &C) -> Result<i64> {
    let mut day = date.div_euclid(SECONDS_PER_DAY);
    for _ in 0..MAX_ROLL_DAYS {
        day = day.checked_add(1).ok_or(MathError::Overflow)?;
        if cal.is_business_day(day) {
            return day.checked_mul(SECONDS_PER_DAY).ok_or(MathError::Overflow);
        }
    }
    Err(MathError::InvalidInput)
}

/// Whole calendar days from `date` to `next`: the `days_applied` of the print
/// for `date` when `next = next_business_day(date)` (1 on a normal day, 3 on
/// a Friday under [`WeekendOnly`]).
///
/// `next` must be after `date` by a whole number of days that fits `u8`.
pub fn days_until(date: i64, next: i64) -> Result<u8> {
    let secs = next.checked_sub(date).ok_or(MathError::Overflow)?;
    if secs <= 0 || secs % SECONDS_PER_DAY != 0 {
        return Err(MathError::InvalidInput);
    }
    u8::try_from(secs / SECONDS_PER_DAY).map_err(|_| MathError::Overflow)
}

/// FRA period `(period_start, period_end)` as Unix seconds: add months to the
/// trade date `date(ts)`, then Modified Following each date on `cal`.
pub fn derive_period<C: Calendar + ?Sized>(
    _ts: i64,
    _offset_m: u8,
    _tenor_m: u8,
    _cal: &C,
) -> Result<(i64, i64)> {
    todo!("fra_math::derive_period (B)")
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i64 = SECONDS_PER_DAY;
    /// 2026-01-01 (Thursday) 00:00 UTC.
    const JAN_1_2026: i64 = 20_454 * DAY;
    const MON_JAN_5: i64 = JAN_1_2026 + 4 * DAY;
    const FRI_JAN_9: i64 = JAN_1_2026 + 8 * DAY;
    const SAT_JAN_10: i64 = JAN_1_2026 + 9 * DAY;
    const MON_JAN_12: i64 = JAN_1_2026 + 11 * DAY;

    #[test]
    fn weekend_only_calendar() {
        assert!(WeekendOnly.is_business_day(0)); // Thu 1970-01-01
        assert!(!WeekendOnly.is_business_day(2)); // Sat
        assert!(!WeekendOnly.is_business_day(3)); // Sun
        assert!(WeekendOnly.is_business_day(4)); // Mon
        assert!(WeekendOnly.is_business_day(MON_JAN_5 / DAY));
        assert!(!WeekendOnly.is_business_day(SAT_JAN_10 / DAY));
        assert!(WeekendOnly.is_business_day(-3)); // Mon 1969-12-29
    }

    #[test]
    fn next_business_day_midweek_is_tomorrow() {
        assert_eq!(
            next_business_day(MON_JAN_5, &WeekendOnly),
            Ok(MON_JAN_5 + DAY)
        );
    }

    #[test]
    fn next_business_day_friday_skips_weekend() {
        assert_eq!(next_business_day(FRI_JAN_9, &WeekendOnly), Ok(MON_JAN_12));
    }

    #[test]
    fn next_business_day_from_weekend_is_monday() {
        assert_eq!(next_business_day(SAT_JAN_10, &WeekendOnly), Ok(MON_JAN_12));
    }

    #[test]
    fn next_business_day_ignores_time_of_day() {
        assert_eq!(
            next_business_day(FRI_JAN_9 + 15 * 3_600, &WeekendOnly),
            Ok(MON_JAN_12)
        );
    }

    #[test]
    fn days_until_friday_is_3_otherwise_1() {
        let next = next_business_day(FRI_JAN_9, &WeekendOnly).unwrap();
        assert_eq!(days_until(FRI_JAN_9, next), Ok(3));
        for d in 0..4 {
            let date = MON_JAN_5 + d * DAY; // Mon..Thu
            let next = next_business_day(date, &WeekendOnly).unwrap();
            assert_eq!(days_until(date, next), Ok(1));
        }
    }

    #[test]
    fn days_until_rejects_non_positive_and_partial_days() {
        assert_eq!(
            days_until(MON_JAN_5, MON_JAN_5),
            Err(MathError::InvalidInput)
        );
        assert_eq!(
            days_until(MON_JAN_5, MON_JAN_5 - DAY),
            Err(MathError::InvalidInput)
        );
        assert_eq!(
            days_until(MON_JAN_5, MON_JAN_5 + DAY + 1),
            Err(MathError::InvalidInput)
        );
    }

    #[test]
    fn days_until_rejects_more_than_u8() {
        assert_eq!(days_until(0, 256 * DAY), Err(MathError::Overflow));
        assert_eq!(days_until(0, 255 * DAY), Ok(255));
    }

    struct NoBusinessDays;
    impl Calendar for NoBusinessDays {
        fn is_business_day(&self, _day: i64) -> bool {
            false
        }
    }

    #[test]
    fn next_business_day_gives_up_on_degenerate_calendar() {
        assert_eq!(
            next_business_day(0, &NoBusinessDays),
            Err(MathError::InvalidInput)
        );
    }

    /// Trade date Sat 2026-01-31, 1x4: Feb 28 (Sat) -> Fri 2026-02-27 and
    /// May 31 (Sun) -> Fri 2026-05-29 (Modified Following across a month end).
    #[test]
    #[ignore = "TODO(B): derive_period not implemented"]
    fn derive_period_modified_following_across_month_end() {
        todo!()
    }

    /// Weekend date mid-month rolls forward to Monday.
    #[test]
    #[ignore = "TODO(B): derive_period not implemented"]
    fn derive_period_rolls_forward_within_month() {
        todo!()
    }

    /// tenor 0 -> InvalidInput.
    #[test]
    #[ignore = "TODO(B): derive_period not implemented"]
    fn derive_period_rejects_zero_tenor() {
        todo!()
    }
}
