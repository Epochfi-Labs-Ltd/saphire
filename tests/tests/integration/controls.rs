//! Admin and operator controls across instructions: halt/resume (IX-13/14),
//! pause/resume (IX-15/16), suspend (IX-10), deactivate (IX-04).
// TODO(A/B/C): implement once the handlers on each path land.

/// Halt blocks opens only (REQ-74): with a market halted, open_position and
/// open_position_signed fail, while close, settle, unwind, margin and
/// reserve withdrawals still work; resume_market re-enables opens.
#[test]
#[ignore = "TODO(A/B/C): needs IX-13/14/18/20/22"]
fn halt_blocks_only_opens() {
    todo!("flow: open, halt_market, try open/close/settle, resume_market, open")
}

/// Pause blocks opens only: same as halt, program-wide.
#[test]
#[ignore = "TODO(A/B/C): needs IX-15/16/18/20/22"]
fn pause_blocks_only_opens() {
    todo!("flow: open, pause_program, try open/close/settle, resume_program, open")
}

/// A suspended trader can no longer open but can still close and withdraw.
#[test]
#[ignore = "TODO(A/B): needs IX-10/17/18/20"]
fn suspended_trader_can_exit() {
    todo!("flow: open, suspend_trader, try open, close, withdraw_margin")
}

/// A deactivated operator can no longer create markets.
#[test]
#[ignore = "TODO(A): needs IX-03/04/05"]
fn deactivated_operator_cannot_create_market() {
    todo!("flow: register_operator, deactivate_operator, create_market")
}
