//! IX-09 `admit_trader` · operator · A.
// TODO(A): implement these tests and remove the #[ignore]s once IX-09 is implemented.
// Build state with the `mock_*` builders (saphire_tests::mock), not the setup_* chains.

/// Eligibility created with market, trader, active = true.
#[test]
#[ignore = "TODO(A): IX-09 not implemented"]
fn happy_path_creates_active_eligibility() {
    todo!("IX-09: happy_path_creates_active_eligibility")
}

/// NotOperator: signer is not Market.operator.
#[test]
#[ignore = "TODO(A): IX-09 not implemented"]
fn rejects_non_operator() {
    todo!("IX-09: rejects_non_operator")
}

/// init-if-needed: re-admitting a suspended trader sets active = true.
#[test]
#[ignore = "TODO(A): IX-09 not implemented"]
fn readmission_reactivates() {
    todo!("IX-09: readmission_reactivates")
}
