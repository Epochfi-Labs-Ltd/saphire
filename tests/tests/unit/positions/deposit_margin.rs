//! IX-12 `deposit_margin` · trader · halt-ok · pause-ok · B.
// TODO(B): implement these tests and remove the #[ignore]s once IX-12 is implemented.
// Build state with the `mock_*` builders (saphire_tests::mock), not the setup_* chains.

/// Tokens move to margin_vault; balance == vault amount.
#[test]
#[ignore = "TODO(B): IX-12 not implemented"]
fn happy_path_moves_tokens_and_mirrors_balance() {
    todo!("IX-12: happy_path_moves_tokens_and_mirrors_balance")
}

/// NotEligible (REQ-21).
#[test]
#[ignore = "TODO(B): IX-12 not implemented"]
fn rejects_suspended_trader() {
    todo!("IX-12: rejects_suspended_trader")
}

/// InvalidParams: amount == 0 (REQ-22).
#[test]
#[ignore = "TODO(B): IX-12 not implemented"]
fn rejects_zero_amount() {
    todo!("IX-12: rejects_zero_amount")
}

/// WrongMint: source token account has a different mint.
#[test]
#[ignore = "TODO(B): IX-12 not implemented"]
fn rejects_wrong_mint_source() {
    todo!("IX-12: rejects_wrong_mint_source")
}

/// WrongMint: source token account not owned by trader.
#[test]
#[ignore = "TODO(B): IX-12 not implemented"]
fn rejects_source_not_owned_by_trader() {
    todo!("IX-12: rejects_source_not_owned_by_trader")
}

/// halt-ok.
#[test]
#[ignore = "TODO(B): IX-12 not implemented"]
fn allowed_while_market_halted() {
    todo!("IX-12: allowed_while_market_halted")
}

/// pause-ok.
#[test]
#[ignore = "TODO(B): IX-12 not implemented"]
fn allowed_while_program_paused() {
    todo!("IX-12: allowed_while_program_paused")
}
