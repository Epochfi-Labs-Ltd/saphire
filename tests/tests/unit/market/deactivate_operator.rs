//! IX-04 `deactivate_operator` · admin · A.
// TODO(A): implement these tests and remove the #[ignore]s once IX-04 is implemented.
// Build state with the `mock_*` builders (saphire_tests::mock), not the setup_* chains.

/// active = false; account kept; existing markets untouched.
#[test]
#[ignore = "TODO(A): IX-04 not implemented"]
fn happy_path_deactivates_and_keeps_market() {
    todo!("IX-04: happy_path_deactivates_and_keeps_market")
}

/// NotAuthority: signer is not Config.authority.
#[test]
#[ignore = "TODO(A): IX-04 not implemented"]
fn rejects_non_authority() {
    todo!("IX-04: rejects_non_authority")
}

/// OperatorInactive: registration already inactive.
#[test]
#[ignore = "TODO(A): IX-04 not implemented"]
fn rejects_already_inactive() {
    todo!("IX-04: rejects_already_inactive")
}
