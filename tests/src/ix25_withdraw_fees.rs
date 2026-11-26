//! IX-25 `withdraw_fees` · admin · pause-ok · A. Not on the cards yet; see
//! docs/skeleton.md "Fee additions".
// TODO(A): implement these tests and remove the #[ignore]s once IX-25 is implemented.

/// fee_vault -> destination; fee_vault balance decreases by amount.
#[test]
#[ignore = "TODO(A): IX-25 not implemented"]
fn happy_path_moves_fees_to_destination() {
    todo!("IX-25: happy_path_moves_fees_to_destination")
}

/// Withdrawing the full fee_vault balance leaves it at zero.
#[test]
#[ignore = "TODO(A): IX-25 not implemented"]
fn withdraws_full_balance() {
    todo!("IX-25: withdraws_full_balance")
}

/// NotAuthority: signer is not Config.authority.
#[test]
#[ignore = "TODO(A): IX-25 not implemented"]
fn rejects_non_authority() {
    todo!("IX-25: rejects_non_authority")
}

/// InvalidParams: amount == 0.
#[test]
#[ignore = "TODO(A): IX-25 not implemented"]
fn rejects_zero_amount() {
    todo!("IX-25: rejects_zero_amount")
}

/// InsufficientFees: amount > fee_vault.amount.
#[test]
#[ignore = "TODO(A): IX-25 not implemented"]
fn rejects_more_than_vault_balance() {
    todo!("IX-25: rejects_more_than_vault_balance")
}

/// WrongMint: destination mint != fee_vault mint.
#[test]
#[ignore = "TODO(A): IX-25 not implemented"]
fn rejects_destination_wrong_mint() {
    todo!("IX-25: rejects_destination_wrong_mint")
}

/// pause-ok.
#[test]
#[ignore = "TODO(A): IX-25 not implemented"]
fn allowed_while_program_paused() {
    todo!("IX-25: allowed_while_program_paused")
}
