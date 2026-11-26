//! IX-24 `close_used_nonce` · trader · optional (DECISION PENDING) · B/C.
// TODO(B/C): implement these tests and remove the #[ignore]s once IX-24 is implemented.

/// UsedNonce closed, rent to trader.
#[test]
#[ignore = "TODO(B/C): IX-24 not implemented"]
fn happy_path_closes_and_refunds_rent() {
    todo!("IX-24: happy_path_closes_and_refunds_rent")
}

/// NotTrader.
#[test]
#[ignore = "TODO(B/C): IX-24 not implemented"]
fn rejects_non_owner() {
    todo!("IX-24: rejects_non_owner")
}

/// PositionStillOpen.
#[test]
#[ignore = "TODO(B/C): IX-24 not implemented"]
fn rejects_while_position_open() {
    todo!("IX-24: rejects_while_position_open")
}
