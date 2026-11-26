//! IX-01 `initialize_config` · A · admin. The handler writes fields only;
//! the happy-path tests run, the validation tests are stubs.
// TODO(A): fill in the validation stubs once IX-01 validation lands.

use saphire_tests::{fixture::*, pda};
use solana_signer::Signer;

fn init(env: &mut TestEnv, fee_bps: u16, rate_ceiling: u64) -> TxResult {
    let admin = env.admin.insecure_clone();
    let ix = ix_initialize_config(env, &admin.pubkey(), fee_bps, rate_ceiling);
    env.send(&[ix], &[&admin])
}

#[test]
fn happy_path_writes_config_and_fee_vault() {
    let mut env = TestEnv::new();
    assert_ok(&init(&mut env, PROTOCOL_FEE_BPS, RATE_CEILING));

    let (config_addr, bump) = pda::config();
    let config = env.config_state();
    assert_eq!(config.authority, env.admin.pubkey());
    assert_eq!(config.protocol_fee_bps, PROTOCOL_FEE_BPS);
    assert_eq!(config.fee_vault, pda::fee_vault().0);
    assert_eq!(config.rate_ceiling, RATE_CEILING);
    assert!(!config.paused);
    assert_eq!(config.bump, bump);
    assert_eq!(config._reserved, [0u8; 32]);

    // fee_vault: token account, authority = Config PDA, mint = settlement mint.
    let fee_vault = env.fee_vault();
    assert_eq!(env.token_owner(&fee_vault), config_addr);
    assert_eq!(env.token_mint(&fee_vault), env.mint);
    assert_eq!(env.token_balance(&fee_vault), 0);
}

#[test]
fn happy_path_token_2022_mint() {
    let mut env = TestEnv::with_token_program(TOKEN_2022_PROGRAM_ID);
    assert_ok(&init(&mut env, PROTOCOL_FEE_BPS, RATE_CEILING));
    let fee_vault = env.fee_vault();
    assert_eq!(
        env.svm.get_account(&fee_vault).expect("fee vault").owner,
        TOKEN_2022_PROGRAM_ID
    );
    assert_eq!(env.token_owner(&fee_vault), pda::config().0);
}

/// InvalidParams: protocol_fee_bps > MAX_FEE_BPS.
#[test]
#[ignore = "TODO(A): IX-01 validation not implemented"]
fn rejects_fee_above_max() {
    todo!("IX-01: rejects_fee_above_max")
}

/// InvalidParams: rate_ceiling == 0.
#[test]
#[ignore = "TODO(A): IX-01 validation not implemented"]
fn rejects_zero_rate_ceiling() {
    todo!("IX-01: rejects_zero_rate_ceiling")
}

#[test]
fn rejects_second_initialization() {
    let mut env = TestEnv::new();
    assert_ok(&init(&mut env, PROTOCOL_FEE_BPS, RATE_CEILING));
    let res = init(&mut env, PROTOCOL_FEE_BPS, RATE_CEILING);
    assert_custom_err(&res, SYSTEM_ACCOUNT_ALREADY_IN_USE);
}
