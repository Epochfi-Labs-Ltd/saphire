//! Unit tests: each instruction on its own. Run `anchor build` first.
//!
//! `cargo test -p saphire-tests --test unit [market::|positions::|settlement::]`

#![allow(clippy::result_large_err)]

mod market;
mod positions;
mod settlement;
mod smoke;
