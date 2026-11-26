//! IX-16 `resume_program()` · A · admin
//! Implements: REQ-02d, 74a, 41.

use anchor_lang::prelude::*;

use crate::{constants::*, errors::FraError, state::Config};

#[derive(Accounts)]
pub struct ResumeProgram<'info> {
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
/// Logic: `paused = false` · emit `ProgramResumed`.
/// Errors: `NotAuthority` (REQ-02d).
pub fn handle_resume_program(_ctx: Context<ResumeProgram>) -> Result<()> {
    todo!("IX-16 A")
}
