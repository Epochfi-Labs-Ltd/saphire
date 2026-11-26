//! IX-09 `admit_trader()` · A · operator
//! Implements: REQ-17, 19.

use anchor_lang::prelude::*;

use crate::{
    constants::*,
    errors::FraError,
    state::{Eligibility, Market},
};

#[derive(Accounts)]
pub struct AdmitTrader<'info> {
    #[account(mut)]
    pub operator: Signer<'info>,

    #[account(has_one = operator @ FraError::NotOperator)]
    pub market: Account<'info, Market>,

    /// CHECK: any wallet; recorded as `Eligibility.trader`.
    pub trader: UncheckedAccount<'info>,

    /// init-if-needed: re-admission reactivates the existing account.
    #[account(
        init_if_needed,
        payer = operator,
        space = 8 + Eligibility::INIT_SPACE,
        seeds = [SEED_ELIGIBLE, market.key().as_ref(), trader.key().as_ref()],
        bump,
    )]
    pub eligibility: Account<'info, Eligibility>,

    pub system_program: Program<'info, System>,
}

/// Validation: none (re-admission = reactivate).
/// Logic: write `market`, `trader`, `active = true` · emit `TraderAdmitted`.
/// Errors: `NotOperator` (REQ-19).
pub fn handle_admit_trader(_ctx: Context<AdmitTrader>) -> Result<()> {
    todo!("IX-09 A")
}
