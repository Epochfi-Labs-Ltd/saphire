//! IX-19 `open_position_signed` · trader + operator signature (DECISION PENDING) · B.
// TODO(B): implement these tests and remove the #[ignore]s once IX-19 is implemented.

/// Position (origin Rfq, K = deal.fixed_rate) and UsedNonce created.
#[test]
#[ignore = "TODO(B): IX-19 not implemented"]
fn happy_path_opens_rfq_position() {
    todo!("IX-19: happy_path_opens_rfq_position")
}

/// Quote freshness and at-quote cap do not apply.
#[test]
#[ignore = "TODO(B): IX-19 not implemented"]
fn does_not_require_fresh_quote() {
    todo!("IX-19: does_not_require_fresh_quote")
}

/// MissingSignature.
#[test]
#[ignore = "TODO(B): IX-19 not implemented"]
fn rejects_missing_ed25519_instruction() {
    todo!("IX-19: rejects_missing_ed25519_instruction")
}

/// BadSigner.
#[test]
#[ignore = "TODO(B): IX-19 not implemented"]
fn rejects_signature_from_non_operator() {
    todo!("IX-19: rejects_signature_from_non_operator")
}

/// MessageMismatch.
#[test]
#[ignore = "TODO(B): IX-19 not implemented"]
fn rejects_signed_message_mismatch() {
    todo!("IX-19: rejects_signed_message_mismatch")
}

/// OfferExpired: now >= expires_at.
#[test]
#[ignore = "TODO(B): IX-19 not implemented"]
fn rejects_expired_deal() {
    todo!("IX-19: rejects_expired_deal")
}

/// PeriodStarted: now >= period_start.
#[test]
#[ignore = "TODO(B): IX-19 not implemented"]
fn rejects_period_already_started() {
    todo!("IX-19: rejects_period_already_started")
}

/// BadPeriod: dates outside ROLL_TOLERANCE_DAYS of derive_period.
#[test]
#[ignore = "TODO(B): IX-19 not implemented"]
fn rejects_inconsistent_period() {
    todo!("IX-19: rejects_inconsistent_period")
}

/// NonceUsed (surfaces as system AccountAlreadyInUse, see docs/skeleton.md).
#[test]
#[ignore = "TODO(B): IX-19 not implemented"]
fn rejects_replayed_nonce() {
    todo!("IX-19: rejects_replayed_nonce")
}

/// ProgramPaused, MarketHalted, NotEligible, NotionalOutOfBounds, OpenInterestCap, InsufficientFreeMargin, InsufficientReserve.
#[test]
#[ignore = "TODO(B): IX-19 not implemented"]
fn rejects_ix18_checks() {
    todo!("IX-19: rejects_ix18_checks")
}
