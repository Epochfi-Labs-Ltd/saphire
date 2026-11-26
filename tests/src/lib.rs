//! Shared support for the Saphire tests. The tests themselves live in two
//! targets next to this crate:
//!
//! - `tests/unit/`: each instruction tested on its own. One module per
//!   instruction, grouped as in `programs/saphire/src/instructions/`
//!   (market, positions, settlement). Every account the instruction reads is
//!   mocked (written straight into LiteSVM by [`mock`]), so exactly one
//!   handler runs per test. Also here: pure checks of the helpers (PDA seeds,
//!   signed-deal encoding, Codama client <-> program sync) and the fixture's
//!   own self-checks.
//!   `cargo test -p saphire-tests --test unit`
//! - `tests/integration/`: instructions combined. Multi-step flows and the
//!   lifecycle E2E, built with the fixture's `setup_*` / `ready_*` chains,
//!   which send the real instructions.
//!   `cargo test -p saphire-tests --test integration`
//!
//! Both targets load `target/deploy/saphire.so`: run `anchor build` first.
//! Tests for unimplemented handlers are named stubs, `#[ignore]`d with a
//! `TODO(<segment>)` note; run them with `-- --ignored` once filled in.
//!
//! Instructions are built with the Codama-generated `saphire-client`; the
//! helpers here only fill in the PDAs it expects to be passed.

// LiteSVM's `TransactionResult` has a large `Err` variant; it is the
// library's type, so the lint is allowed here rather than boxed per call.
#![allow(clippy::result_large_err)]

pub mod deal;
pub mod fixture;
pub mod mock;
pub mod pda;
