//! Thin client for the Saphire program.
//!
//! - [`pda`]: one derivation fn per account, mirroring the seeds in
//!   `saphire::constants`.
//! - [`ix`]: one builder per instruction, returning an [`Instruction`]
//!   (`solana_instruction::Instruction`, the same type `solana_sdk`
//!   re-exports as `solana_sdk::instruction::Instruction`).
//! - [`deal`]: Borsh [`DealMessage`] encoding and the Ed25519 verify
//!   instruction for `open_position_signed` (IX-19).

pub mod deal;
pub mod ix;
pub mod pda;

pub use anchor_lang::prelude::Pubkey;
pub use anchor_lang::solana_program::instruction::Instruction;
pub use saphire::{
    DealMessage, MarketParams, MarketStatus, Origin, PositionStatus, Side, SignedDeal,
    ID as PROGRAM_ID,
};

/// SPL Token program id.
pub const TOKEN_PROGRAM_ID: Pubkey = anchor_spl::token::ID;
/// Token-2022 program id.
pub const TOKEN_2022_PROGRAM_ID: Pubkey = anchor_spl::token_2022::ID;

/// Everything a market-scoped builder needs to know about the market.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MarketRef {
    pub market: Pubkey,
    pub settlement_mint: Pubkey,
    /// SPL Token or Token-2022, whichever owns `settlement_mint`.
    pub token_program: Pubkey,
}

impl MarketRef {
    /// Derive the market address from its seeds.
    pub fn new(
        benchmark_id: u8,
        settlement_mint: Pubkey,
        start_offset_months: u8,
        tenor_months: u8,
        token_program: Pubkey,
    ) -> Self {
        Self {
            market: pda::market(
                benchmark_id,
                &settlement_mint,
                start_offset_months,
                tenor_months,
            )
            .0,
            settlement_mint,
            token_program,
        }
    }
}
