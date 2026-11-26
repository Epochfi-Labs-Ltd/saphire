//! IX-25 `withdraw_fees(amount: u64)` · A · admin · pause-ok
//!
//! Not on the cards yet; see docs/skeleton.md "Fee additions". Moves
//! protocol fees out of `fee_vault`, which no carded instruction can do.

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{constants::*, errors::FraError, state::Config};

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        has_one = authority @ FraError::NotAuthority,
        has_one = fee_vault,
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub fee_vault: InterfaceAccount<'info, TokenAccount>,

    /// Any token account of the fee mint; the admin chooses the destination.
    #[account(
        mut,
        constraint = destination.mint == fee_vault.mint @ FraError::WrongMint,
    )]
    pub destination: InterfaceAccount<'info, TokenAccount>,

    /// Needed by `transfer_checked`.
    #[account(address = fee_vault.mint @ FraError::WrongMint)]
    pub settlement_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
}

/// Validation: `amount > 0` → `InvalidParams` · `amount <= fee_vault.amount`
/// → `InsufficientFees`.
///
/// Logic:
/// 1. CPI `transfer_checked` `fee_vault → destination`, signer seeds
///    `[b"config", bump]`
/// 2. Emit `FeesWithdrawn`
///
/// Errors: `NotAuthority`, `InvalidParams`, `InsufficientFees`, `WrongMint`.
pub fn handle_withdraw_fees(_ctx: Context<WithdrawFees>, _amount: u64) -> Result<()> {
    todo!("IX-25 A")
}
