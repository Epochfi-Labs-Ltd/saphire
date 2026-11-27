//! Integration tests: instructions combined. Run `anchor build` first.
//!
//! Each test drives a flow through several real instructions.
//! Single-instruction behaviour belongs in the unit tests (`tests/unit`).
//!
//! `cargo test -p saphire-tests --test integration`

#![allow(clippy::result_large_err)]

mod controls;
mod lifecycle;
mod quote_trading;
mod reserve_and_fees;
mod rfq;
mod settlement;
