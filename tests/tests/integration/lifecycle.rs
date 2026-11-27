//! End-to-end sequence from README "What we demo".
// TODO(A/B/C): implement once every handler on the path lands.
//
// Steps:
//  1. initialize_config
//  2. register_operator
//  3. create_market
//  4. fund_reserve
//  5. publish_quote
//  6. admit_trader
//  7. init_trader_margin + deposit_margin
//  8. open_position at quote
//  9. open_position_signed (RFQ deal)
// 10. close_position early
// 11. warp past period_end
// 12. publish_daily_rate for every business day through period_end
// 13. settle_position
// 14. unwind_position (a position whose period_end print never arrives,
//     warped past period_end + rate_grace_days)
// 15. withdraw_margin (all) + withdraw_reserve (all)
//
// After every step, check the invariants from docs/implementation.md
// ("Invariants"): vault mirrors, locked/committed <= balance, and
// locked / committed / open_interest equal their sums over Open positions,
// and Market.rate_index == index_after of the latest DailyRate.

#[test]
#[ignore = "TODO(A/B/C): lifecycle depends on every handler"]
fn full_lifecycle() {
    todo!("lifecycle: steps 1-15 with invariant checks")
}
