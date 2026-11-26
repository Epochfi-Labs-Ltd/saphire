//! IX-04 `deactivate_operator()` · A · admin
//! Implements: REQ-02b, 02c.

use anchor_lang::prelude::*;

use crate::{
    constants::*,
    errors::FraError,
    state::{Config, OperatorRegistration},
};

#[derive(Accounts)]
pub struct DeactivateOperator<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        has_one = authority @ FraError::NotAuthority,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_OPERATOR, operator_registration.operator.as_ref()],
        bump = operator_registration.bump,
    )]
    pub operator_registration: Account<'info, OperatorRegistration>,
}

/// Validation: `active == true`.
/// Logic: `active = false`; account kept; existing markets untouched · emit
/// `OperatorDeactivated`.
/// Errors: `NotAuthority`, `OperatorInactive`.
pub fn handle_deactivate_operator(_ctx: Context<DeactivateOperator>) -> Result<()> {
    todo!("IX-04 A")
}
