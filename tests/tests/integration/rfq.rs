//! RFQ (signed deal) flows: IX-19 with IX-20, 22, 23 and 24.
// TODO(B/C): implement once the handlers on each path land.

/// open_position_signed, then the same signed deal resubmitted fails on the
/// used nonce.
#[test]
#[ignore = "TODO(B): needs IX-19"]
fn signed_deal_cannot_be_replayed() {
    todo!("flow: signed_deal_ixs twice with the same nonce")
}

/// RFQ position closed early, then close_used_nonce reclaims the nonce
/// account's rent.
#[test]
#[ignore = "TODO(B): needs IX-19/20/24"]
fn close_used_nonce_after_close() {
    todo!("flow: open_position_signed, close_position, close_used_nonce")
}

/// Quote and RFQ positions for one trader share TraderMargin; aggregates are
/// the sum over both.
#[test]
#[ignore = "TODO(B): needs IX-18/19"]
fn quote_and_rfq_positions_share_margin() {
    todo!("flow: open_test_position + signed_deal_ixs, check sums")
}
