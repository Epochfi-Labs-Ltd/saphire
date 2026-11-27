//! IX-20 `close_position` · trader · halt-ok · pause-ok · B.
// TODO(B): implement these tests and remove the #[ignore]s once IX-20 is implemented.

/// Net PnL minus early-close fee transferred; locks released; status Closed.
#[test]
#[ignore = "TODO(B): IX-20 not implemented"]
fn happy_path_closes_at_quote_with_fee() {
    todo!("IX-20: happy_path_closes_at_quote_with_fee")
}

/// NotTrader (REQ-65).
#[test]
#[ignore = "TODO(B): IX-20 not implemented"]
fn rejects_non_owner() {
    todo!("IX-20: rejects_non_owner")
}

/// NotOpen (REQ-66).
#[test]
#[ignore = "TODO(B): IX-20 not implemented"]
fn rejects_not_open() {
    todo!("IX-20: rejects_not_open")
}

/// UseSettle (REQ-67).
#[test]
#[ignore = "TODO(B): IX-20 not implemented"]
fn rejects_after_period_end() {
    todo!("IX-20: rejects_after_period_end")
}

/// QuoteStale (REQ-66).
#[test]
#[ignore = "TODO(B): IX-20 not implemented"]
fn rejects_stale_quote() {
    todo!("IX-20: rejects_stale_quote")
}

/// No eligibility check.
#[test]
#[ignore = "TODO(B): IX-20 not implemented"]
fn suspended_trader_can_close() {
    todo!("IX-20: suspended_trader_can_close")
}

/// halt-ok.
#[test]
#[ignore = "TODO(B): IX-20 not implemented"]
fn allowed_while_market_halted() {
    todo!("IX-20: allowed_while_market_halted")
}

/// pause-ok.
#[test]
#[ignore = "TODO(B): IX-20 not implemented"]
fn allowed_while_program_paused() {
    todo!("IX-20: allowed_while_program_paused")
}
