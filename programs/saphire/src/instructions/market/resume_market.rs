//! IX-14 `resume_market()` · A · operator
//! Implements: REQ-72, 73.

use anchor_lang::prelude::*;

use crate::{errors::FraError, state::Market};

#[derive(Accounts)]
pub struct ResumeMarket<'info> {
    pub operator: Signer<'info>,

    #[account(mut, has_one = operator @ FraError::NotOperator)]
    pub market: Account<'info, Market>,
}

/// Validation: `status == Halted`.
/// Logic: `status = Active` · emit `MarketResumed`.
/// Errors: `NotOperator` (REQ-73), `MarketNotHalted`.
pub fn handle_resume_market(_ctx: Context<ResumeMarket>) -> Result<()> {
    todo!("IX-14 A")
}
