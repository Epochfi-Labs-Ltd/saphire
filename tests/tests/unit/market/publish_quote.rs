//! IX-08 `publish_quote` · operator · halt-ok · A.
// TODO(A): implement these tests and remove the #[ignore]s once IX-08 is implemented.
// Build state with the `mock_*` builders (saphire_tests::mock), not the setup_* chains.

/// Fields overwritten, published_at = now, epoch += 1 (REQ-14).
#[test]
#[ignore = "TODO(A): IX-08 not implemented"]
fn happy_path_overwrites_and_advances_epoch() {
    todo!("IX-08: happy_path_overwrites_and_advances_epoch")
}

/// NotOperator: signer is not Market.operator.
#[test]
#[ignore = "TODO(A): IX-08 not implemented"]
fn rejects_non_operator() {
    todo!("IX-08: rejects_non_operator")
}

/// MarketClosed: market.status == Closed.
#[test]
#[ignore = "TODO(A): IX-08 not implemented"]
fn rejects_closed_market() {
    todo!("IX-08: rejects_closed_market")
}

/// RateOutOfRange: fixed_mid == 0.
#[test]
#[ignore = "TODO(A): IX-08 not implemented"]
fn rejects_zero_mid() {
    todo!("IX-08: rejects_zero_mid")
}

/// RateOutOfRange: fixed_mid > config.rate_ceiling (REQ-16).
#[test]
#[ignore = "TODO(A): IX-08 not implemented"]
fn rejects_mid_above_ceiling() {
    todo!("IX-08: rejects_mid_above_ceiling")
}

/// SpreadTooWide: half_spread_bps > market.max_half_spread_bps (REQ-15).
#[test]
#[ignore = "TODO(A): IX-08 not implemented"]
fn rejects_spread_above_market_max() {
    todo!("IX-08: rejects_spread_above_market_max")
}

/// InvalidParams: fixed_mid <= half_spread_bps · 100.
#[test]
#[ignore = "TODO(A): IX-08 not implemented"]
fn rejects_mid_not_above_spread() {
    todo!("IX-08: rejects_mid_not_above_spread")
}

/// InvalidParams: max_notional_at_quote outside [min_notional, max_notional].
#[test]
#[ignore = "TODO(A): IX-08 not implemented"]
fn rejects_quote_cap_outside_market_bounds() {
    todo!("IX-08: rejects_quote_cap_outside_market_bounds")
}

/// halt-ok.
#[test]
#[ignore = "TODO(A): IX-08 not implemented"]
fn allowed_while_market_halted() {
    todo!("IX-08: allowed_while_market_halted")
}
