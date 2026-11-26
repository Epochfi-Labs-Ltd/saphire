//! IX-17 `withdraw_margin` · trader · halt-ok · pause-ok · B.
// TODO(B): implement these tests and remove the #[ignore]s once IX-17 is implemented.

/// Tokens move to trader; balance decremented; mirror holds.
#[test]
#[ignore = "TODO(B): IX-17 not implemented"]
fn happy_path_moves_tokens_and_mirrors_balance() {
    todo!("IX-17: happy_path_moves_tokens_and_mirrors_balance")
}

/// NotTrader: trader_margin.trader != signer (REQ-25).
#[test]
#[ignore = "TODO(B): IX-17 not implemented"]
fn rejects_non_owner() {
    todo!("IX-17: rejects_non_owner")
}

/// InsufficientFreeMargin: amount > balance − locked (REQ-24).
#[test]
#[ignore = "TODO(B): IX-17 not implemented"]
fn rejects_more_than_free_margin() {
    todo!("IX-17: rejects_more_than_free_margin")
}

/// InvalidParams: amount == 0.
#[test]
#[ignore = "TODO(B): IX-17 not implemented"]
fn rejects_zero_amount() {
    todo!("IX-17: rejects_zero_amount")
}

/// REQ-26: no eligibility read.
#[test]
#[ignore = "TODO(B): IX-17 not implemented"]
fn suspended_trader_can_withdraw() {
    todo!("IX-17: suspended_trader_can_withdraw")
}

/// halt-ok.
#[test]
#[ignore = "TODO(B): IX-17 not implemented"]
fn allowed_while_market_halted() {
    todo!("IX-17: allowed_while_market_halted")
}

/// pause-ok.
#[test]
#[ignore = "TODO(B): IX-17 not implemented"]
fn allowed_while_program_paused() {
    todo!("IX-17: allowed_while_program_paused")
}
