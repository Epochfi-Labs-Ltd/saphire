//! IX-18 `open_position` · trader · B.
// TODO(B): implement these tests and remove the #[ignore]s once IX-18 is implemented.

/// Position created (origin Quote); K, period, locks, commit, fee, OI, position_count updated.
#[test]
#[ignore = "TODO(B): IX-18 not implemented"]
fn happy_path_pay_fixed_at_quote() {
    todo!("IX-18: happy_path_pay_fixed_at_quote")
}

/// Receive-fixed side priced at mid − spread.
#[test]
#[ignore = "TODO(B): IX-18 not implemented"]
fn happy_path_receive_fixed_at_quote() {
    todo!("IX-18: happy_path_receive_fixed_at_quote")
}

/// ProgramPaused (REQ-41, 74a).
#[test]
#[ignore = "TODO(B): IX-18 not implemented"]
fn rejects_when_program_paused() {
    todo!("IX-18: rejects_when_program_paused")
}

/// MarketHalted (REQ-41).
#[test]
#[ignore = "TODO(B): IX-18 not implemented"]
fn rejects_when_market_halted() {
    todo!("IX-18: rejects_when_market_halted")
}

/// NotEligible (REQ-35).
#[test]
#[ignore = "TODO(B): IX-18 not implemented"]
fn rejects_suspended_trader() {
    todo!("IX-18: rejects_suspended_trader")
}

/// QuoteStale: epoch == 0 (REQ-36).
#[test]
#[ignore = "TODO(B): IX-18 not implemented"]
fn rejects_unpublished_quote() {
    todo!("IX-18: rejects_unpublished_quote")
}

/// QuoteStale: now − published_at > quote_validity_secs (REQ-36).
#[test]
#[ignore = "TODO(B): IX-18 not implemented"]
fn rejects_expired_quote() {
    todo!("IX-18: rejects_expired_quote")
}

/// NotionalOutOfBounds (REQ-39).
#[test]
#[ignore = "TODO(B): IX-18 not implemented"]
fn rejects_notional_below_min() {
    todo!("IX-18: rejects_notional_below_min")
}

/// NotionalOutOfBounds (REQ-39).
#[test]
#[ignore = "TODO(B): IX-18 not implemented"]
fn rejects_notional_above_market_or_quote_cap() {
    todo!("IX-18: rejects_notional_above_market_or_quote_cap")
}

/// OpenInterestCap (REQ-40).
#[test]
#[ignore = "TODO(B): IX-18 not implemented"]
fn rejects_open_interest_cap() {
    todo!("IX-18: rejects_open_interest_cap")
}

/// InsufficientFreeMargin: free < lock + fee (REQ-37).
#[test]
#[ignore = "TODO(B): IX-18 not implemented"]
fn rejects_insufficient_free_margin() {
    todo!("IX-18: rejects_insufficient_free_margin")
}

/// InsufficientReserve: balance − committed < commit (REQ-38).
#[test]
#[ignore = "TODO(B): IX-18 not implemented"]
fn rejects_insufficient_reserve() {
    todo!("IX-18: rejects_insufficient_reserve")
}

/// TraderMargin.locked += lock + settlement_fee; Position.settlement_fee = Market.settlement_fee.
#[test]
#[ignore = "TODO(B): IX-18 not implemented"]
fn locks_settlement_fee() {
    todo!("IX-18: locks_settlement_fee")
}

/// InsufficientFreeMargin: free >= lock + fee but < lock + fee + settlement_fee.
#[test]
#[ignore = "TODO(B): IX-18 not implemented"]
fn rejects_free_margin_below_lock_fee_and_settlement_fee() {
    todo!("IX-18: rejects_free_margin_below_lock_fee_and_settlement_fee")
}
