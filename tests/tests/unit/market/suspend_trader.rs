//! IX-10 `suspend_trader` · operator · A.
// TODO(A): implement these tests and remove the #[ignore]s once IX-10 is implemented.
// Build state with the `mock_*` builders (saphire_tests::mock), not the setup_* chains.

/// active = false; margin and positions untouched (REQ-18).
#[test]
#[ignore = "TODO(A): IX-10 not implemented"]
fn happy_path_deactivates_and_leaves_margin_and_positions() {
    todo!("IX-10: happy_path_deactivates_and_leaves_margin_and_positions")
}

/// NotOperator: signer is not Market.operator.
#[test]
#[ignore = "TODO(A): IX-10 not implemented"]
fn rejects_non_operator() {
    todo!("IX-10: rejects_non_operator")
}
