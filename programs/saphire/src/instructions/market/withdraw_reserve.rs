//! IX-07 `withdraw_reserve(amount: u64)` · A · operator · halt-ok · pause-ok
//! Implements: REQ-09, 10, 11, 74.

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    errors::FraError,
    state::{Market, Reserve},
};

/// As IX-06; `operator_token_account` is the destination.
#[derive(Accounts)]
pub struct WithdrawReserve<'info> {
    pub operator: Signer<'info>,

    #[account(has_one = operator @ FraError::NotOperator)]
    pub market: Account<'info, Market>,

    #[account(mut, has_one = market)]
    pub reserve: Account<'info, Reserve>,

    #[account(mut, address = reserve.vault)]
    pub reserve_vault: InterfaceAccount<'info, TokenAccount>,

    /// Destination of funds.
    #[account(
        mut,
        constraint = operator_token_account.mint == market.settlement_mint @ FraError::WrongMint,
        constraint = operator_token_account.owner == operator.key() @ FraError::WrongMint,
    )]
    pub operator_token_account: InterfaceAccount<'info, TokenAccount>,

    /// Not on the card: needed by `transfer_checked` (Token-2022 safe).
    #[account(address = market.settlement_mint @ FraError::WrongMint)]
    pub settlement_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
}

/// Validation: `amount > 0` · `amount <= balance − committed` (REQ-10).
/// Logic:
/// 1. CPI `transfer_checked` `reserve_vault → operator_token_account`, signer
///    seeds `[b"reserve", market, bump]`
/// 2. `reserve.balance −= amount`; mirror assert
/// 3. Emit `ReserveWithdrawn`
///
/// Errors: `NotOperator` (REQ-11), `InsufficientWithdrawable` (REQ-10),
/// `InvalidParams`.
pub fn handle_withdraw_reserve(_ctx: Context<WithdrawReserve>, _amount: u64) -> Result<()> {
    todo!("IX-07 A")
}
