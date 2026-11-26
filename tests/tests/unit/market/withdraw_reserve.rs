//! IX-07 `withdraw_reserve` · operator · halt-ok · pause-ok · A.
// TODO(A): implement these tests and remove the #[ignore]s once IX-07 is implemented.
// Build state with the `mock_*` builders (saphire_tests::mock), not the setup_* chains.

/// Tokens move to operator; reserve.balance decremented; mirror holds.
#[test]
#[ignore = "TODO(A): IX-07 not implemented"]
fn happy_path_moves_tokens_and_mirrors_balance() {
    todo!("IX-07: happy_path_moves_tokens_and_mirrors_balance")
}

/// NotOperator: signer is not Market.operator.
#[test]
#[ignore = "TODO(A): IX-07 not implemented"]
fn rejects_non_operator() {
    todo!("IX-07: rejects_non_operator")
}

/// InsufficientWithdrawable: amount > balance − committed (REQ-10).
#[test]
#[ignore = "TODO(A): IX-07 not implemented"]
fn rejects_more_than_uncommitted() {
    todo!("IX-07: rejects_more_than_uncommitted")
}

/// InvalidParams: amount == 0.
#[test]
#[ignore = "TODO(A): IX-07 not implemented"]
fn rejects_zero_amount() {
    todo!("IX-07: rejects_zero_amount")
}

/// halt-ok.
#[test]
#[ignore = "TODO(A): IX-07 not implemented"]
fn allowed_while_market_halted() {
    todo!("IX-07: allowed_while_market_halted")
}

/// pause-ok.
#[test]
#[ignore = "TODO(A): IX-07 not implemented"]
fn allowed_while_program_paused() {
    todo!("IX-07: allowed_while_program_paused")
}
