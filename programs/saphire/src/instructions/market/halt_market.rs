//! IX-13 `halt_market()` · A · operator
//! Implements: REQ-72, 73.

use anchor_lang::prelude::*;

use crate::{errors::FraError, state::Market};

#[derive(Accounts)]
pub struct HaltMarket<'info> {
    pub operator: Signer<'info>,

    #[account(mut, has_one = operator @ FraError::NotOperator)]
    pub market: Account<'info, Market>,
}

/// Validation: `status == Active`.
/// Logic: `status = Halted` · emit `MarketHalted`. Halt blocks IX-18/19
/// only; everything else continues (REQ-74).
/// Errors: `NotOperator` (REQ-73), `InvalidTransition`.
pub fn handle_halt_market(_ctx: Context<HaltMarket>) -> Result<()> {
    todo!("IX-13 A")
}
