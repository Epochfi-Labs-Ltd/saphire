//! Quote-priced trading across instructions: IX-06, 08, 09, 11, 12, 18, 20, 17.
// TODO(A/B): implement once the handlers on each path land.

/// fund_reserve → publish_quote → admit_trader → init_trader_margin →
/// deposit_margin → open_position: the position, TraderMargin, Reserve and
/// Market aggregates agree, and the vault mirrors hold.
#[test]
#[ignore = "TODO(A/B): needs IX-06/08/09/11/12/18"]
fn open_at_quote_from_fresh_market() {
    todo!("flow: ready_market + ready_trader + open_test_position")
}

/// open_position → close_position early: locks, commit and open interest
/// released; PnL moved between margin and reserve vaults.
#[test]
#[ignore = "TODO(B): needs IX-18/20"]
fn open_then_close_early_releases_everything() {
    todo!("flow: open_test_position then close_position")
}

/// A republished quote prices the next open; the existing position's terms
/// do not change (REQ-83).
#[test]
#[ignore = "TODO(A/B): needs IX-08/18"]
fn requote_affects_only_new_positions() {
    todo!("flow: open, publish_quote_with(new mid), open again")
}

/// open → close → withdraw_margin of the whole free balance empties the
/// margin vault.
#[test]
#[ignore = "TODO(B): needs IX-17/18/20"]
fn trader_can_withdraw_everything_after_close() {
    todo!("flow: open, close, withdraw_margin(all)")
}
