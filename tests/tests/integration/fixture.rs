//! Sanity checks on the shared LiteSVM fixture itself.

use saphire_tests::fixture::*;
use solana_signer::Signer;

#[test]
fn warp_to_sets_clock() {
    let mut env = TestEnv::new();
    assert_eq!(env.now(), START_TS);
    warp_to(&mut env, START_TS + 1_000);
    assert_eq!(env.now(), START_TS + 1_000);
}

#[test]
fn token_accounts_are_written_with_balance() {
    let env = TestEnv::new();
    assert_eq!(
        env.token_balance(&env.operator_token_account),
        OPERATOR_STARTING_USDC
    );
    assert_eq!(
        env.token_owner(&env.operator_token_account),
        env.operator.pubkey()
    );
    assert_eq!(env.token_mint(&env.operator_token_account), env.mint);
}
