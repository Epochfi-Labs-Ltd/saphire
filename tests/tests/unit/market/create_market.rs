//! IX-05 `create_market` · operator · A.
// TODO(A): implement these tests and remove the #[ignore]s once IX-05 is implemented.
// Build state with the `mock_*` builders (saphire_tests::mock), not the setup_* chains.

/// Market (rate_index = 10^18, last_rate_date = index_start_date), Reserve,
/// reserve_vault and Quote (epoch 0) created.
#[test]
#[ignore = "TODO(A): IX-05 not implemented"]
fn happy_path_creates_market_reserve_vault_quote() {
    todo!("IX-05: happy_path_creates_market_reserve_vault_quote")
}

/// OperatorInactive: operator registration deactivated.
#[test]
#[ignore = "TODO(A): IX-05 not implemented"]
fn rejects_inactive_operator() {
    todo!("IX-05: rejects_inactive_operator")
}

/// OperatorInactive (surfaces as AccountNotInitialized, see docs/skeleton.md).
#[test]
#[ignore = "TODO(A): IX-05 not implemented"]
fn rejects_unregistered_operator() {
    todo!("IX-05: rejects_unregistered_operator")
}

/// MarketExists (surfaces as system AccountAlreadyInUse, see docs/skeleton.md).
#[test]
#[ignore = "TODO(A): IX-05 not implemented"]
fn rejects_duplicate_market() {
    todo!("IX-05: rejects_duplicate_market")
}

/// InvalidParams: initial_margin_bps or operator_commit_bps == 0 (REQ-04).
#[test]
#[ignore = "TODO(A): IX-05 not implemented"]
fn rejects_zero_margin_or_commit_bps() {
    todo!("IX-05: rejects_zero_margin_or_commit_bps")
}

/// InvalidParams: min_notional > max_notional (REQ-05).
#[test]
#[ignore = "TODO(A): IX-05 not implemented"]
fn rejects_min_notional_above_max() {
    todo!("IX-05: rejects_min_notional_above_max")
}

/// InvalidParams: benchmark_id, day_count_basis or business_day_convention unsupported.
#[test]
#[ignore = "TODO(A): IX-05 not implemented"]
fn rejects_unsupported_conventions() {
    todo!("IX-05: rejects_unsupported_conventions")
}

/// InvalidParams: open_interest_cap, quote_validity_secs, rate_grace_days or max_half_spread_bps == 0.
#[test]
#[ignore = "TODO(A): IX-05 not implemented"]
fn rejects_zero_caps_and_windows() {
    todo!("IX-05: rejects_zero_caps_and_windows")
}

/// Market.settlement_fee = params.settlement_fee; 0 is allowed (no crank bounty).
#[test]
#[ignore = "TODO(A): IX-05 not implemented"]
fn stores_settlement_fee() {
    todo!("IX-05: stores_settlement_fee")
}
