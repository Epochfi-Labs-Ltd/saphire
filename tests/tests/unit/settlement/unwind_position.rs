//! IX-23 `unwind_position` · anyone · halt-ok · pause-ok · C.
// TODO(C): implement these tests and remove the #[ignore]s once IX-23 is implemented.
// Build state with the `mock_*` builders (saphire_tests::mock), not the setup_* chains.

/// Locks, commit, OI released; no transfer; status Unwound (REQ-69).
#[test]
#[ignore = "TODO(C): IX-23 not implemented"]
fn happy_path_releases_without_transfer() {
    todo!("IX-23: happy_path_releases_without_transfer")
}

/// NotOpen.
#[test]
#[ignore = "TODO(C): IX-23 not implemented"]
fn rejects_not_open() {
    todo!("IX-23: rejects_not_open")
}

/// GraceNotElapsed: now < period_end + rate_grace_days (REQ-70).
#[test]
#[ignore = "TODO(C): IX-23 not implemented"]
fn rejects_before_grace_elapsed() {
    todo!("IX-23: rejects_before_grace_elapsed")
}

/// RateCheckpointExists: the period_end print was published (REQ-71).
#[test]
#[ignore = "TODO(C): IX-23 not implemented"]
fn rejects_when_end_checkpoint_exists() {
    todo!("IX-23: rejects_when_end_checkpoint_exists")
}

/// BadRateCheckpointAddress: rate_end_address is not the DailyRate PDA for
/// position.period_end.
#[test]
#[ignore = "TODO(C): IX-23 not implemented"]
fn rejects_wrong_checkpoint_address() {
    todo!("IX-23: rejects_wrong_checkpoint_address")
}

/// halt-ok.
#[test]
#[ignore = "TODO(C): IX-23 not implemented"]
fn allowed_while_market_halted() {
    todo!("IX-23: allowed_while_market_halted")
}

/// pause-ok.
#[test]
#[ignore = "TODO(C): IX-23 not implemented"]
fn allowed_while_program_paused() {
    todo!("IX-23: allowed_while_program_paused")
}

/// margin_vault -> payer_token_account, position.settlement_fee; no settlement transfer.
#[test]
#[ignore = "TODO(C): IX-23 not implemented"]
fn cranker_receives_settlement_fee() {
    todo!("IX-23: cranker_receives_settlement_fee")
}

/// WrongMint: payer_token_account mint != settlement mint, or owner != payer.
#[test]
#[ignore = "TODO(C): IX-23 not implemented"]
fn rejects_payer_token_account_wrong_mint_or_owner() {
    todo!("IX-23: rejects_payer_token_account_wrong_mint_or_owner")
}
