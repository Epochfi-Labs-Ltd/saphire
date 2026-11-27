//! Shared state contract (docs/implementation.md, "Account cards"). One file
//! per account; the enums shared across accounts live here.
//!
//! Every account carries `bump: u8` and `_reserved: [u8; 32]` so fields can be
//! added later without a realloc.

use anchor_lang::prelude::*;

mod config;
mod daily_rate;
mod eligibility;
mod market;
mod operator_registration;
mod position;
mod quote;
mod reserve;
mod trader_margin;
mod used_nonce;

pub use config::*;
pub use daily_rate::*;
pub use eligibility::*;
pub use market::*;
pub use operator_registration::*;
pub use position::*;
pub use quote::*;
pub use reserve::*;
pub use trader_margin::*;
pub use used_nonce::*;

/// `Market.status`. Halted blocks IX-18/19 only (REQ-74).
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum MarketStatus {
    Active,
    Halted,
    Closed,
}

/// Trader's side of the FRA.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum Side {
    /// Pays fixed K, receives float L: gains when L > K.
    PayFixed,
    /// Receives fixed K, pays float L: gains when L < K.
    ReceiveFixed,
}

/// How a position was priced.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum Origin {
    /// IX-18: at the published quote.
    Quote,
    /// IX-19: from a signed RFQ deal.
    Rfq,
}

/// `Position.status`. Moves only Open -> terminal (REQ-84). Forward/Live are
/// phases of Open split by the clock at `period_start`, not stored.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PositionStatus {
    Open,
    Closed,
    Settled,
    Unwound,
}
