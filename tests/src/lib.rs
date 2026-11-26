//! Shared support for the Saphire tests. The tests themselves live in two
//! targets next to this crate:
//!
//! - `tests/unit/`: pure checks with no SVM, e.g. PDA seeds, signed-deal
//!   encoding, and that the Codama client matches the program's IDL.
//!   `cargo test -p saphire-tests --test unit`
//! - `tests/integration/`: LiteSVM tests, one module per instruction,
//!   grouped as in `programs/saphire/src/instructions/` (market, positions,
//!   settlement), plus the lifecycle E2E.
//!   `cargo test -p saphire-tests --test integration`
//!
//! Integration tests load `target/deploy/saphire.so`: run `anchor build`
//! first. Tests for unimplemented handlers are named stubs, `#[ignore]`d with
//! a `TODO(<segment>)` note; run them with `-- --ignored` once filled in.
//!
//! Instructions are built with the Codama-generated `saphire-client`; the
//! helpers here only fill in the PDAs it expects to be passed.

// LiteSVM's `TransactionResult` has a large `Err` variant; it is the
// library's type, so the lint is allowed here rather than boxed per call.
#![allow(clippy::result_large_err)]

pub mod deal;
pub mod fixture;
pub mod pda;
