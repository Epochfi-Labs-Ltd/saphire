//! IX-17 `withdraw_margin(amount: u64)` · B · trader · halt-ok · pause-ok
//! Implements: REQ-23–26, 74.
//!
//! No `eligibility`, `config` or `market` status read — a suspended trader
//! withdraws free margin (REQ-26).

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{errors::FraError, state::TraderMargin};

#[derive(Accounts)]
pub struct WithdrawMargin<'info> {
    pub trader: Signer<'info>,

    #[account(mut, has_one = trader @ FraError::NotTrader)]
    pub trader_margin: Account<'info, TraderMargin>,

    #[account(mut, address = trader_margin.vault)]
    pub margin_vault: InterfaceAccount<'info, TokenAccount>,

    /// Destination of funds.
    #[account(
        mut,
        constraint = trader_token_account.mint == margin_vault.mint @ FraError::WrongMint,
        constraint = trader_token_account.owner == trader.key() @ FraError::WrongMint,
    )]
    pub trader_token_account: InterfaceAccount<'info, TokenAccount>,

    /// Not on the card: needed by `transfer_checked`. Taken from the vault
    /// because the card forbids reading `market`.
    #[account(address = margin_vault.mint @ FraError::WrongMint)]
    pub settlement_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
}

/// Validation: `amount > 0` · `amount <= balance − locked` (REQ-24).
/// Logic:
/// 1. CPI `transfer_checked` `margin_vault → trader_token_account`, signer
///    seeds `[b"margin", market, trader, bump]`
/// 2. `balance −= amount`; mirror assert
/// 3. Emit `MarginWithdrawn`
///
/// Errors: `NotTrader` (REQ-25), `InsufficientFreeMargin` (REQ-24),
/// `InvalidParams`.
pub fn handle_withdraw_margin(_ctx: Context<WithdrawMargin>, _amount: u64) -> Result<()> {
    todo!("IX-17 B")
}
