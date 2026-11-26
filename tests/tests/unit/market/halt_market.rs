//! IX-13 `halt_market` · operator · A.
// TODO(A): implement these tests and remove the #[ignore]s once IX-13 is implemented.
// Build state with the `mock_*` builders (saphire_tests::mock), not the setup_* chains.

/// status Active -> Halted.
#[test]
#[ignore = "TODO(A): IX-13 not implemented"]
fn happy_path_halts_active_market() {
    todo!("IX-13: happy_path_halts_active_market")
}

/// NotOperator: signer is not Market.operator.
#[test]
#[ignore = "TODO(A): IX-13 not implemented"]
fn rejects_non_operator() {
    todo!("IX-13: rejects_non_operator")
}

/// InvalidTransition: status != Active.
#[test]
#[ignore = "TODO(A): IX-13 not implemented"]
fn rejects_when_not_active() {
    todo!("IX-13: rejects_when_not_active")
}
