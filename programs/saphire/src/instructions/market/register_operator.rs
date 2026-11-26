//! IX-03 `register_operator()` · A · admin
//! Implements: REQ-02b.

use anchor_lang::prelude::*;

use crate::{
    constants::*,
    errors::FraError,
    state::{Config, OperatorRegistration},
};

#[derive(Accounts)]
pub struct RegisterOperator<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        has_one = authority @ FraError::NotAuthority,
    )]
    pub config: Account<'info, Config>,

    /// CHECK: any wallet; recorded as `OperatorRegistration.operator`.
    pub operator: UncheckedAccount<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + OperatorRegistration::INIT_SPACE,
        seeds = [SEED_OPERATOR, operator.key().as_ref()],
        bump,
    )]
    pub operator_registration: Account<'info, OperatorRegistration>,

    pub system_program: Program<'info, System>,
}

/// Validation: none.
/// Logic: write `operator`, `active = true` · emit `OperatorRegistered`.
/// Errors: `NotAuthority`.
pub fn handle_register_operator(_ctx: Context<RegisterOperator>) -> Result<()> {
    todo!("IX-03 A")
}
