//! IX-11 `init_trader_margin()` · B · trader
//! Implements: REQ-21 (gate).

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    constants::*,
    errors::FraError,
    state::{Eligibility, Market, TraderMargin},
};

#[derive(Accounts)]
pub struct InitTraderMargin<'info> {
    #[account(mut)]
    pub trader: Signer<'info>,

    pub market: Box<Account<'info, Market>>,

    #[account(
        seeds = [SEED_ELIGIBLE, market.key().as_ref(), trader.key().as_ref()],
        bump = eligibility.bump,
        constraint = eligibility.active @ FraError::NotEligible,
    )]
    pub eligibility: Box<Account<'info, Eligibility>>,

    #[account(
        init,
        payer = trader,
        space = 8 + TraderMargin::INIT_SPACE,
        seeds = [SEED_MARGIN, market.key().as_ref(), trader.key().as_ref()],
        bump,
    )]
    pub trader_margin: Box<Account<'info, TraderMargin>>,

    #[account(
        init,
        payer = trader,
        seeds = [SEED_MARGIN_VAULT, market.key().as_ref(), trader.key().as_ref()],
        bump,
        token::mint = settlement_mint,
        token::authority = trader_margin,
        token::token_program = token_program,
    )]
    pub margin_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        address = market.settlement_mint @ FraError::WrongMint,
        mint::token_program = token_program,
    )]
    pub settlement_mint: Box<InterfaceAccount<'info, Mint>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

/// Validation: none.
/// Logic: write `market`, `trader`, `vault`, zeros · emit
/// `TraderMarginInitialized`.
/// Errors: `NotEligible`.
pub fn handle_init_trader_margin(_ctx: Context<InitTraderMargin>) -> Result<()> {
    todo!("IX-11 B")
}
