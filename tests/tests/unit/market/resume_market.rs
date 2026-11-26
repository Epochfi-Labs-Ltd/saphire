//! IX-14 `resume_market` · operator · A.
// TODO(A): implement these tests and remove the #[ignore]s once IX-14 is implemented.
// Build state with the `mock_*` builders (saphire_tests::mock), not the setup_* chains.

/// status Halted -> Active.
#[test]
#[ignore = "TODO(A): IX-14 not implemented"]
fn happy_path_resumes_halted_market() {
    todo!("IX-14: happy_path_resumes_halted_market")
}

/// NotOperator: signer is not Market.operator.
#[test]
#[ignore = "TODO(A): IX-14 not implemented"]
fn rejects_non_operator() {
    todo!("IX-14: rejects_non_operator")
}

/// MarketNotHalted: status != Halted.
#[test]
#[ignore = "TODO(A): IX-14 not implemented"]
fn rejects_when_not_halted() {
    todo!("IX-14: rejects_when_not_halted")
}
