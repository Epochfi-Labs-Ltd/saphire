//! IX-11 `init_trader_margin` · trader · B.
// TODO(B): implement these tests and remove the #[ignore]s once IX-11 is implemented.
// Build state with the `mock_*` builders (saphire_tests::mock), not the setup_* chains.

/// TraderMargin (zeros) and margin_vault (authority = TraderMargin) created.
#[test]
#[ignore = "TODO(B): IX-11 not implemented"]
fn happy_path_creates_margin_and_vault() {
    todo!("IX-11: happy_path_creates_margin_and_vault")
}

/// NotEligible: eligibility.active == false.
#[test]
#[ignore = "TODO(B): IX-11 not implemented"]
fn rejects_suspended_trader() {
    todo!("IX-11: rejects_suspended_trader")
}

/// NotEligible (surfaces as AccountNotInitialized, see docs/skeleton.md).
#[test]
#[ignore = "TODO(B): IX-11 not implemented"]
fn rejects_never_admitted_trader() {
    todo!("IX-11: rejects_never_admitted_trader")
}
