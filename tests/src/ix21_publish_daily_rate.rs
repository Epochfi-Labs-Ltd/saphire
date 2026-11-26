//! IX-21 `publish_daily_rate` · operator · halt-ok · pause-ok · C.
// TODO(C): implement these tests and remove the #[ignore]s once IX-21 is implemented.

/// DailyRate written (index_before = old Market.rate_index, index_after =
/// compound_index(...)); Market.rate_index and last_rate_date advance.
#[test]
#[ignore = "TODO(C): IX-21 not implemented"]
fn happy_path_records_print_and_advances_index() {
    todo!("IX-21: happy_path_records_print_and_advances_index")
}

/// Friday print with days_applied = 3 compounds three days; next print is Monday.
#[test]
#[ignore = "TODO(C): IX-21 not implemented"]
fn friday_print_applies_three_days() {
    todo!("IX-21: friday_print_applies_three_days")
}

/// Several missed days caught up in order in one transaction.
#[test]
#[ignore = "TODO(C): IX-21 not implemented"]
fn catch_up_several_days_in_one_transaction() {
    todo!("IX-21: catch_up_several_days_in_one_transaction")
}

/// NotOperator: signer is not Market.operator.
#[test]
#[ignore = "TODO(C): IX-21 not implemented"]
fn rejects_non_operator() {
    todo!("IX-21: rejects_non_operator")
}

/// RateDateNotNext: skipping a business day (gap).
#[test]
#[ignore = "TODO(C): IX-21 not implemented"]
fn rejects_gap() {
    todo!("IX-21: rejects_gap")
}

/// Duplicate date: fails at init (system AccountAlreadyInUse, see docs/skeleton.md).
#[test]
#[ignore = "TODO(C): IX-21 not implemented"]
fn rejects_duplicate_date() {
    todo!("IX-21: rejects_duplicate_date")
}

/// RateDateInFuture: date + days_applied days > now.
#[test]
#[ignore = "TODO(C): IX-21 not implemented"]
fn rejects_incomplete_accrual_window() {
    todo!("IX-21: rejects_incomplete_accrual_window")
}

/// RateOutOfRange: rate == 0 or > config.rate_ceiling.
#[test]
#[ignore = "TODO(C): IX-21 not implemented"]
fn rejects_rate_out_of_range() {
    todo!("IX-21: rejects_rate_out_of_range")
}

/// InvalidParams: days_applied != days_until(date, next_business_day(date)).
#[test]
#[ignore = "TODO(C): IX-21 not implemented"]
fn rejects_wrong_days_applied() {
    todo!("IX-21: rejects_wrong_days_applied")
}

/// halt-ok.
#[test]
#[ignore = "TODO(C): IX-21 not implemented"]
fn allowed_while_market_halted() {
    todo!("IX-21: allowed_while_market_halted")
}

/// pause-ok.
#[test]
#[ignore = "TODO(C): IX-21 not implemented"]
fn allowed_while_program_paused() {
    todo!("IX-21: allowed_while_program_paused")
}
