//! Shared support for the Saphire tests. The tests themselves live in two
//! targets next to this crate:
//!
//! - `tests/unit/`: each instruction tested on its own, one module per
//!   instruction, grouped as in `programs/saphire/src/instructions/`
//!   (market, positions, settlement).
//!   `cargo test -p saphire-tests --test unit`
//! - `tests/integration/`: multi-instruction flows and the lifecycle E2E.
//!   `cargo test -p saphire-tests --test integration`
//!
//! Both targets load `target/deploy/saphire.so`: run `anchor build` first.
//! Tests are named stubs, `#[ignore]`d with a `TODO(<segment>)` note; fill
//! them in alongside each handler.

// LiteSVM's `TransactionResult` has a large `Err` variant; it is the
// library's type, so the lint is allowed here rather than boxed per call.
#![allow(clippy::result_large_err)]

pub mod fixture;
