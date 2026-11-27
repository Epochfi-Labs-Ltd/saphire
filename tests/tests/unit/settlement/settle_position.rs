//! IX-22 `settle_position` · anyone · halt-ok · pause-ok · C.
// TODO(C): implement these tests and remove the #[ignore]s once IX-22 is implemented.

/// L derived from the period_start/period_end checkpoints; reserve pays trader;
/// locks released; status Settled.
#[test]
#[ignore = "TODO(C): IX-22 not implemented"]
fn happy_path_pay_fixed_gain() {
    todo!("IX-22: happy_path_pay_fixed_gain")
}

/// Trader pays reserve.
#[test]
#[ignore = "TODO(C): IX-22 not implemented"]
fn happy_path_receive_fixed_loss() {
    todo!("IX-22: happy_path_receive_fixed_loss")
}

/// settled clamped to −locked_margin (REQ-58).
#[test]
#[ignore = "TODO(C): IX-22 not implemented"]
fn loss_capped_at_locked_margin() {
    todo!("IX-22: loss_capped_at_locked_margin")
}

/// settled clamped to +committed_reserve (REQ-58).
#[test]
#[ignore = "TODO(C): IX-22 not implemented"]
fn gain_capped_at_committed_reserve() {
    todo!("IX-22: gain_capped_at_committed_reserve")
}

/// REQ-56/57.
#[test]
#[ignore = "TODO(C): IX-22 not implemented"]
fn zero_pnl_moves_no_tokens() {
    todo!("IX-22: zero_pnl_moves_no_tokens")
}

/// Any signer may submit.
#[test]
#[ignore = "TODO(C): IX-22 not implemented"]
fn anyone_can_crank() {
    todo!("IX-22: anyone_can_crank")
}

/// NotOpen (REQ-62).
#[test]
#[ignore = "TODO(C): IX-22 not implemented"]
fn rejects_not_open() {
    todo!("IX-22: rejects_not_open")
}

/// PeriodNotEnded.
#[test]
#[ignore = "TODO(C): IX-22 not implemented"]
fn rejects_before_period_end() {
    todo!("IX-22: rejects_before_period_end")
}

/// RateCheckpointMissing: no period_end print yet (surfaces as
/// AccountNotInitialized, see docs/skeleton.md).
#[test]
#[ignore = "TODO(C): IX-22 not implemented"]
fn rejects_missing_end_checkpoint() {
    todo!("IX-22: rejects_missing_end_checkpoint")
}

/// RateCheckpointMissing: no period_start print, e.g. the market's index
/// started after period_start (surfaces as AccountNotInitialized).
#[test]
#[ignore = "TODO(C): IX-22 not implemented"]
fn rejects_missing_start_checkpoint() {
    todo!("IX-22: rejects_missing_start_checkpoint")
}

/// Passing a checkpoint for another date fails the seeds check (REQ-61).
#[test]
#[ignore = "TODO(C): IX-22 not implemented"]
fn rejects_checkpoint_for_other_date() {
    todo!("IX-22: rejects_checkpoint_for_other_date")
}

/// halt-ok.
#[test]
#[ignore = "TODO(C): IX-22 not implemented"]
fn allowed_while_market_halted() {
    todo!("IX-22: allowed_while_market_halted")
}

/// pause-ok.
#[test]
#[ignore = "TODO(C): IX-22 not implemented"]
fn allowed_while_program_paused() {
    todo!("IX-22: allowed_while_program_paused")
}
