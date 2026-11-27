//! Reserve and fee flows: IX-06, 07 and 18.
// TODO(A/B): implement once the handlers on each path land.

/// With a position open, withdraw_reserve is capped at balance − committed;
/// after the position closes the operator can withdraw the rest.
#[test]
#[ignore = "TODO(A/B): needs IX-06/07/18/20"]
fn reserve_withdrawal_respects_commitments() {
    todo!("flow: fund, open, withdraw_reserve(too much) fails, close, withdraw all")
}

/// Protocol fees taken at open land in fee_vault.
#[test]
#[ignore = "TODO(A/B): needs IX-18"]
fn protocol_fees_accrue() {
    todo!("flow: open N positions, fee_vault == Σ fee_paid")
}

/// Program-held tokens equal Σ margin balances + reserve.balance + fee_vault
/// (REQ-75) after each step of fund → open → close → withdraw.
#[test]
#[ignore = "TODO(A/B): needs IX-06/07/17/18/20"]
fn conservation_across_flow() {
    todo!("flow: check REQ-75 after every step")
}
