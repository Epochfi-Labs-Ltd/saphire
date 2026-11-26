//! IX-06 `fund_reserve` · operator · halt-ok · pause-ok · A.
// TODO(A): implement these tests and remove the #[ignore]s once IX-06 is implemented.
// Build state with the `mock_*` builders (saphire_tests::mock), not the setup_* chains.

/// Tokens move to reserve_vault; reserve.balance == vault amount.
#[test]
#[ignore = "TODO(A): IX-06 not implemented"]
fn happy_path_moves_tokens_and_mirrors_balance() {
    todo!("IX-06: happy_path_moves_tokens_and_mirrors_balance")
}

/// NotOperator: signer is not Market.operator.
#[test]
#[ignore = "TODO(A): IX-06 not implemented"]
fn rejects_non_operator() {
    todo!("IX-06: rejects_non_operator")
}

/// InvalidParams: amount == 0.
#[test]
#[ignore = "TODO(A): IX-06 not implemented"]
fn rejects_zero_amount() {
    todo!("IX-06: rejects_zero_amount")
}

/// WrongMint: source token account has a different mint.
#[test]
#[ignore = "TODO(A): IX-06 not implemented"]
fn rejects_wrong_mint_source() {
    todo!("IX-06: rejects_wrong_mint_source")
}

/// WrongMint: source token account not owned by operator.
#[test]
#[ignore = "TODO(A): IX-06 not implemented"]
fn rejects_source_not_owned_by_operator() {
    todo!("IX-06: rejects_source_not_owned_by_operator")
}

/// MirrorMismatch: balance != reserve_vault.amount after transfer.
#[test]
#[ignore = "TODO(A): IX-06 not implemented"]
fn rejects_when_vault_mirror_broken() {
    todo!("IX-06: rejects_when_vault_mirror_broken")
}

/// halt-ok.
#[test]
#[ignore = "TODO(A): IX-06 not implemented"]
fn allowed_while_market_halted() {
    todo!("IX-06: allowed_while_market_halted")
}

/// pause-ok.
#[test]
#[ignore = "TODO(A): IX-06 not implemented"]
fn allowed_while_program_paused() {
    todo!("IX-06: allowed_while_program_paused")
}
