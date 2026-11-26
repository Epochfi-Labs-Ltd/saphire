//! IX-06 `fund_reserve(amount: u64)` · A · operator · halt-ok · pause-ok
//! Implements: REQ-07, 08.

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    errors::FraError,
    state::{Market, Reserve},
};

#[derive(Accounts)]
pub struct FundReserve<'info> {
    pub operator: Signer<'info>,

    #[account(has_one = operator @ FraError::NotOperator)]
    pub market: Account<'info, Market>,

    #[account(mut, has_one = market)]
    pub reserve: Account<'info, Reserve>,

    #[account(mut, address = reserve.vault)]
    pub reserve_vault: InterfaceAccount<'info, TokenAccount>,

    /// Source of funds.
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

/// Validation: `amount > 0`.
/// Logic:
/// 1. CPI `transfer_checked` `operator_token_account → reserve_vault`
///    (operator signs)
/// 2. `reserve.balance += amount`
/// 3. reload vault; assert `balance == reserve_vault.amount`
/// 4. Emit `ReserveFunded`
///
/// Errors: `NotOperator` (REQ-08), `InvalidParams`, `WrongMint`,
/// `MirrorMismatch`.
pub fn handle_fund_reserve(_ctx: Context<FundReserve>, _amount: u64) -> Result<()> {
    todo!("IX-06 A")
}
