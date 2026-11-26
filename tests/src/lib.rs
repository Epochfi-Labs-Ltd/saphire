//! Saphire integration tests (LiteSVM). Run `anchor build` first: the
//! fixture loads `target/deploy/saphire.so`.
//!
//! One module per instruction card. Tests for unimplemented handlers are
//! named stubs, `#[ignore]`d with a `TODO(<segment>)` note; run them with
//! `cargo test -p saphire-tests -- --ignored` once filled in.

// LiteSVM's `TransactionResult` has a large `Err` variant; it is the
// library's type, so the lint is allowed here rather than boxed per call.
#![allow(clippy::result_large_err)]

pub mod fixture;

#[cfg(test)]
mod ix01_initialize_config;
#[cfg(test)]
mod ix02_update_config;
#[cfg(test)]
mod ix03_register_operator;
#[cfg(test)]
mod ix04_deactivate_operator;
#[cfg(test)]
mod ix05_create_market;
#[cfg(test)]
mod ix06_fund_reserve;
#[cfg(test)]
mod ix07_withdraw_reserve;
#[cfg(test)]
mod ix08_publish_quote;
#[cfg(test)]
mod ix09_admit_trader;
#[cfg(test)]
mod ix10_suspend_trader;
#[cfg(test)]
mod ix11_init_trader_margin;
#[cfg(test)]
mod ix12_deposit_margin;
#[cfg(test)]
mod ix13_halt_market;
#[cfg(test)]
mod ix14_resume_market;
#[cfg(test)]
mod ix15_pause_program;
#[cfg(test)]
mod ix16_resume_program;
#[cfg(test)]
mod ix17_withdraw_margin;
#[cfg(test)]
mod ix18_open_position;
#[cfg(test)]
mod ix19_open_position_signed;
#[cfg(test)]
mod ix20_close_position;
#[cfg(test)]
mod ix21_publish_daily_rate;
#[cfg(test)]
mod ix22_settle_position;
#[cfg(test)]
mod ix23_unwind_position;
#[cfg(test)]
mod ix24_close_used_nonce;
#[cfg(test)]
mod ix25_withdraw_fees;
#[cfg(test)]
mod lifecycle;
