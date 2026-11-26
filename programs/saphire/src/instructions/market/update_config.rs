//! IX-02 `update_config(protocol_fee_bps?: u16, rate_ceiling?: u64)` · A · admin · optional
//! Implements: REQ-02a.

use anchor_lang::prelude::*;

use crate::{constants::*, errors::FraError, state::Config};

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_CONFIG],
        bump = config.bump,
        has_one = authority @ FraError::NotAuthority,
    )]
    pub config: Account<'info, Config>,
}

/// Validation: as IX-01 bounds (`protocol_fee_bps <= MAX_FEE_BPS`,
/// `rate_ceiling > 0`) on each provided field.
/// Logic: overwrite provided fields · emit `ConfigUpdated`.
/// Errors: `NotAuthority`, `InvalidParams`.
pub fn handle_update_config(
    _ctx: Context<UpdateConfig>,
    _protocol_fee_bps: Option<u16>,
    _rate_ceiling: Option<u64>,
) -> Result<()> {
    todo!("IX-02 A")
}
