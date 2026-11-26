//! Unit tests: each instruction on its own. Run `anchor build` first.
//!
//! Every account an instruction reads is mocked with the `mock_*` builders
//! (`saphire_tests::mock`), written straight into LiteSVM, so each test
//! sends exactly one Saphire instruction and fails only when that handler
//! does. Do not use the fixture's `setup_*` / `ready_*` chains here: they
//! send real instructions and belong in the integration tests.
//!
//! `cargo test -p saphire-tests --test unit [market::|positions::|settlement::|helpers::]`

#![allow(clippy::result_large_err)]

mod fixture;
mod helpers;
mod market;
mod positions;
mod settlement;
