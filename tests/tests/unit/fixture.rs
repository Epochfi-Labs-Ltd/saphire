//! Self-checks on the shared LiteSVM fixture and the mock builders. No
//! Saphire instruction is sent.

use saphire::{Origin, PositionStatus};
use saphire_client::types::Side;
use saphire_tests::{fixture::*, mock::*, pda};
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

#[test]
fn mock_config_writes_config_and_fee_vault() {
    let env = mock_config_env();
    let config = env.config_state();
    assert_eq!(config.authority, env.admin.pubkey());
    assert_eq!(config.fee_vault, env.fee_vault());
    assert_eq!(config.bump, pda::config().1);
    assert_eq!(env.token_owner(&env.fee_vault()), pda::config().0);
    assert_eq!(env.token_balance(&env.fee_vault()), 0);
    let account = env.svm.get_account(&pda::config().0).expect("config");
    assert_eq!(account.owner, saphire::ID);
    assert_eq!(
        account.data.len(),
        8 + <saphire::Config as anchor_lang::Space>::INIT_SPACE
    );
}

#[test]
fn mock_market_env_matches_create_market_output() {
    let env = mock_market_env();
    let market = env.market_state();
    assert_eq!(market.operator, env.operator.pubkey());
    assert_eq!(market.settlement_mint, env.mint);
    assert_eq!(market.rate_index, saphire::constants::INDEX_SCALE);
    assert_eq!(market.last_rate_date, INDEX_START_DATE);
    assert_eq!(market.position_count, 0);
    let reserve = env.reserve_state();
    assert_eq!(reserve.vault, env.reserve_vault());
    assert_eq!(reserve.balance, 0);
    assert_eq!(env.quote_state().epoch, 0);
    let reg: saphire::OperatorRegistration =
        env.account(&pda::operator_registration(&env.operator.pubkey()).0);
    assert!(reg.active);
}

#[test]
fn mock_vaults_mirror_balances() {
    let mut env = mock_ready_market();
    assert_eq!(env.reserve_state().balance, TEST_RESERVE);
    assert_eq!(env.token_balance(&env.reserve_vault()), TEST_RESERVE);
    assert_eq!(
        env.token_owner(&env.reserve_vault()),
        pda::reserve(&env.market()).0
    );

    let trader = mock_ready_trader(&mut env);
    let t = trader.pubkey();
    assert!(env.eligibility_state(&t).active);
    assert_eq!(env.margin_state(&t).balance, TEST_MARGIN);
    assert_eq!(env.token_balance(&env.margin_vault(&t)), TEST_MARGIN);
    assert_eq!(
        env.token_owner(&env.margin_vault(&t)),
        pda::trader_margin(&env.market(), &t).0
    );
}

#[test]
fn mock_quote_is_fresh() {
    let env = mock_ready_market();
    let quote = env.quote_state();
    assert!(quote.is_fresh(env.now(), env.params.quote_validity_secs));
}

#[test]
fn mock_position_updates_aggregates() {
    let mut env = mock_ready_market();
    let trader = mock_ready_trader(&mut env);
    let t = trader.pubkey();
    let first = env.mock_position(&t, Side::PayFixed, TEST_NOTIONAL);
    let second = env.mock_position(&t, Side::ReceiveFixed, TEST_NOTIONAL);
    assert_eq!((first, second), (0, 1));

    let p = env.position_state(&t, first);
    assert_eq!(p.origin, Origin::Quote);
    assert_eq!(p.status, PositionStatus::Open);
    assert_eq!(p.fixed_rate, 40_700);
    assert_eq!(env.position_state(&t, second).fixed_rate, 40_300);

    let market = env.market_state();
    assert_eq!(market.position_count, 2);
    assert_eq!(market.open_interest, 2 * TEST_NOTIONAL);
    assert_eq!(env.reserve_state().committed, 2 * p.committed_reserve);
    let margin = env.margin_state(&t);
    assert_eq!(margin.open_positions, 2);
    assert_eq!(margin.locked, 2 * (p.locked_margin + p.settlement_fee));
}

#[test]
fn mock_terminal_position_leaves_aggregates() {
    let mut env = mock_ready_market();
    let trader = mock_ready_trader(&mut env);
    let t = trader.pubkey();
    env.mock_position_with(&t, Side::PayFixed, TEST_NOTIONAL, |p| {
        p.status = PositionStatus::Settled
    });
    assert_eq!(env.market_state().position_count, 1);
    assert_eq!(env.market_state().open_interest, 0);
    assert_eq!(env.margin_state(&t).locked, 0);
    assert_eq!(env.reserve_state().committed, 0);
}

#[test]
fn mock_daily_rates_chain_the_index() {
    let mut env = mock_market_env();
    // Mon 2026-01-05 .. Fri 2026-01-09.
    let friday = INDEX_START_DATE + 7 * SECONDS_PER_DAY;
    env.mock_daily_rates_through(friday, 40_000);
    let market = env.market_state();
    assert_eq!(market.last_rate_date, friday);
    let fri = env.daily_rate_state(friday);
    assert_eq!(fri.days_applied, 3);
    assert_eq!(market.rate_index, fri.index_after);
    let thu = env.daily_rate_state(friday - SECONDS_PER_DAY);
    assert_eq!(thu.days_applied, 1);
    assert_eq!(fri.index_before, thu.index_after);
    assert!(fri.index_after > fri.index_before);
}
