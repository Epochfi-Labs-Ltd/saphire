//! IX-02 `update_config` · admin · optional · A.
// TODO(A): implement these tests and remove the #[ignore]s once IX-02 is implemented.

/// Admin overwrites protocol_fee_bps and rate_ceiling.
#[test]
#[ignore = "TODO(A): IX-02 not implemented"]
fn happy_path_updates_both_fields() {
    todo!("IX-02: happy_path_updates_both_fields")
}

/// Fields passed as None are left unchanged.
#[test]
#[ignore = "TODO(A): IX-02 not implemented"]
fn happy_path_updates_only_provided_field() {
    todo!("IX-02: happy_path_updates_only_provided_field")
}

/// NotAuthority: signer is not Config.authority.
#[test]
#[ignore = "TODO(A): IX-02 not implemented"]
fn rejects_non_authority() {
    todo!("IX-02: rejects_non_authority")
}

/// InvalidParams: protocol_fee_bps > MAX_FEE_BPS.
#[test]
#[ignore = "TODO(A): IX-02 not implemented"]
fn rejects_fee_above_max() {
    todo!("IX-02: rejects_fee_above_max")
}

/// InvalidParams: rate_ceiling == 0.
#[test]
#[ignore = "TODO(A): IX-02 not implemented"]
fn rejects_zero_rate_ceiling() {
    todo!("IX-02: rejects_zero_rate_ceiling")
}
