//! End-to-end sequence from README "What we demo".
// TODO(A/B/C): implement once every handler on the path lands.
//
// Steps (fixture helpers in brackets):
//  1. initialize_config                         [setup_config]
//  2. register_operator                         [setup_fra_market]
//  3. create_market                             [setup_fra_market]
//  4. fund_reserve                              [fund_test_reserve]
//  5. publish_quote                             [publish_test_quote]
//  6. admit_trader                              [admit_test_trader]
//  7. init_trader_margin + deposit_margin       [setup_eligible_trader_with_margin]
//  8. open_position at quote                    [open_test_position]
//  9. open_position_signed (RFQ deal)           [signed_deal_ixs]
// 10. close_position early
// 11. warp past period_end                      [warp_to]
// 12. publish_daily_rate for every business day through period_end
//                                               [publish_test_daily_rates_through]
// 13. settle_position
// 14. unwind_position (a position whose period_end print never arrives,
//     warped past period_end + rate_grace_days)
// 15. withdraw_margin (all) + withdraw_reserve (all)
// 16. withdraw_fees (all of fee_vault)         [IX-25, not on the cards yet]
//
// After every step, check the invariants from docs/implementation.md
// ("Invariants"): vault mirrors, locked/committed <= balance, and
// locked / committed / open_interest equal their sums over Open positions,
// and Market.rate_index == index_after of the latest DailyRate. Settle and
// unwind crankers each receive the position's settlement_fee.

#[test]
#[ignore = "TODO(A/B/C): lifecycle depends on every handler"]
fn full_lifecycle() {
    todo!("lifecycle: steps 1-15 with invariant checks")
}
