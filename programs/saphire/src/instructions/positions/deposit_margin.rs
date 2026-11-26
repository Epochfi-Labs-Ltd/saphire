//! IX-12 `deposit_margin(amount: u64)` · B · trader · halt-ok · pause-ok
//! Implements: REQ-20, 21, 22.

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    constants::*,
    errors::FraError,
    state::{Eligibility, Market, TraderMargin},
};

#[derive(Accounts)]
pub struct DepositMargin<'info> {
    pub trader: Signer<'info>,

    pub market: Box<Account<'info, Market>>,

    #[account(
        seeds = [SEED_ELIGIBLE, market.key().as_ref(), trader.key().as_ref()],
        bump = eligibility.bump,
        constraint = eligibility.active @ FraError::NotEligible,
    )]
    pub eligibility: Box<Account<'info, Eligibility>>,

    #[account(
        mut,
        seeds = [SEED_MARGIN, market.key().as_ref(), trader.key().as_ref()],
        bump = trader_margin.bump,
    )]
    pub trader_margin: Box<Account<'info, TraderMargin>>,

    #[account(mut, address = trader_margin.vault)]
    pub margin_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Source of funds.
    #[account(
        mut,
        constraint = trader_token_account.mint == market.settlement_mint @ FraError::WrongMint,
        constraint = trader_token_account.owner == trader.key() @ FraError::WrongMint,
    )]
    pub trader_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Not on the card: needed by `transfer_checked` (Token-2022 safe).
    #[account(address = market.settlement_mint @ FraError::WrongMint)]
    pub settlement_mint: Box<InterfaceAccount<'info, Mint>>,

    pub token_program: Interface<'info, TokenInterface>,
}

/// Validation: `amount > 0` (REQ-22).
/// Logic:
/// 1. CPI `transfer_checked` `trader_token_account → margin_vault` (trader
///    signs)
/// 2. `balance += amount`; reload; mirror assert
/// 3. Emit `MarginDeposited`
///
/// Errors: `NotEligible` (REQ-21), `InvalidParams`, `WrongMint`,
/// `MirrorMismatch`.
pub fn handle_deposit_margin(_ctx: Context<DepositMargin>, _amount: u64) -> Result<()> {
    todo!("IX-12 B")
}
