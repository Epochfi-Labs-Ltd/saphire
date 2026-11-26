//! LiteSVM integration tests: one module per instruction, grouped as in
//! `programs/saphire/src/instructions/`. Run `anchor build` first.
//!
//! `cargo test -p saphire-tests --test integration [market::|positions::|settlement::]`

#![allow(clippy::result_large_err)]

mod fixture;
mod lifecycle;
mod market;
mod positions;
mod settlement;
