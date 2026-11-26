//! IX-10 `suspend_trader()` · A · operator
//! Implements: REQ-18, 19.

use anchor_lang::prelude::*;

use crate::{
    errors::FraError,
    state::{Eligibility, Market},
};

#[derive(Accounts)]
pub struct SuspendTrader<'info> {
    pub operator: Signer<'info>,

    #[account(has_one = operator @ FraError::NotOperator)]
    pub market: Account<'info, Market>,

    #[account(mut, has_one = market)]
    pub eligibility: Account<'info, Eligibility>,
}

/// Validation: none.
/// Logic: `active = false`; margin and positions untouched (REQ-18) · emit
/// `TraderSuspended`.
/// Errors: `NotOperator`.
pub fn handle_suspend_trader(_ctx: Context<SuspendTrader>) -> Result<()> {
    todo!("IX-10 A")
}
