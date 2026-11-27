//! Settlement flows: IX-18 with IX-21, 22 and 23.
// TODO(B/C): implement once the handlers on each path land.

/// open → warp past period_end → publish every daily print → settle_position:
/// settled amount from the period_start/period_end checkpoints; aggregates
/// released.
#[test]
#[ignore = "TODO(B/C): needs IX-18/21/22"]
fn open_publish_rates_settle() {
    todo!("flow: open_test_position, publish_test_daily_rates_through, settle_position")
}

/// The same flow for both sides at the same rate path: one gains what the
/// other loses (before caps).
#[test]
#[ignore = "TODO(B/C): needs IX-18/21/22"]
fn pay_and_receive_fixed_settle_symmetrically() {
    todo!("flow: open PayFixed + ReceiveFixed, publish rates, settle both")
}

/// period_end print never arrives; after rate_grace_days, unwind_position
/// releases everything with no PnL.
#[test]
#[ignore = "TODO(B/C): needs IX-18/21/23"]
fn unwind_when_period_end_print_missing() {
    todo!("flow: open, publish up to period_end - 1, warp past grace, unwind_position")
}

/// Settling a position does not change other open positions or the market's
/// rate index.
#[test]
#[ignore = "TODO(B/C): needs IX-18/21/22"]
fn settle_one_of_many() {
    todo!("flow: two positions, settle one, check the other untouched")
}
