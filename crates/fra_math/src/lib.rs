//! `fra_math`: the only place in Saphire where rates are priced, amounts are
//! rounded, and period dates are derived. Pure Rust, `no_std`, no Solana deps,
//! so the program, the keeper and the tests all compute identically.
//! Owned by Segment B; called by all segments.
//!
//! SKELETON: module layout only; nothing is implemented yet.
//!
//! Units (docs/implementation.md, "Conventions"):
//! - amounts: `u64` base units of the settlement mint
//! - rates: `u64` scaled 10^4 per percent (4.0700% = 40_700, 1 bp = 100)
//! - bps: `u16`
//! - dates: `i64` Unix seconds

#![cfg_attr(not(test), no_std)]

pub mod date;
pub mod index;
