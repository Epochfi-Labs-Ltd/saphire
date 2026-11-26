//! IX-15 `pause_program()` · A · admin
//! Implements: REQ-02d, 74a, 41.

use anchor_lang::prelude::*;

use crate::{constants::*, errors::FraError, state::Config};

#[derive(Accounts)]
pub struct PauseProgram<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_CONFIG],
        bump = config.bump,
        has_one = authority @ FraError::NotAuthority,
    )]
    pub config: Account<'info, Config>,
}

/// Validation: none.
/// Logic: `paused = true` · emit `ProgramPaused`. Pause blocks IX-18/19 only.
/// Errors: `NotAuthority` (REQ-02d).
pub fn handle_pause_program(_ctx: Context<PauseProgram>) -> Result<()> {
    todo!("IX-15 A")
}
