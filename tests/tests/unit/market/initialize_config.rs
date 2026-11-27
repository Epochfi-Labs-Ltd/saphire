//! IX-01 `initialize_config` · admin · A.
// TODO(A): implement these tests and remove the #[ignore]s once IX-01 is implemented.

/// Config written (authority, fee, fee_vault, rate_ceiling, paused = false,
/// bump); fee_vault owned by Config PDA with the settlement mint.
#[test]
#[ignore = "TODO(A): IX-01 not implemented"]
fn happy_path_writes_config_and_fee_vault() {
    todo!("IX-01: happy_path_writes_config_and_fee_vault")
}

/// Same with a Token-2022 settlement mint.
#[test]
#[ignore = "TODO(A): IX-01 not implemented"]
fn happy_path_token_2022_mint() {
    todo!("IX-01: happy_path_token_2022_mint")
}

/// InvalidParams: protocol_fee_bps > MAX_FEE_BPS.
#[test]
#[ignore = "TODO(A): IX-01 not implemented"]
fn rejects_fee_above_max() {
    todo!("IX-01: rejects_fee_above_max")
}

/// InvalidParams: rate_ceiling == 0.
#[test]
#[ignore = "TODO(A): IX-01 not implemented"]
fn rejects_zero_rate_ceiling() {
    todo!("IX-01: rejects_zero_rate_ceiling")
}

/// Config already exists.
#[test]
#[ignore = "TODO(A): IX-01 not implemented"]
fn rejects_second_initialization() {
    todo!("IX-01: rejects_second_initialization")
}
