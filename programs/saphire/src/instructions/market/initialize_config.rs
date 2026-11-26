//! IX-01 `initialize_config(protocol_fee_bps: u16, rate_ceiling: u64)` · A · admin
//! Implements: parameters for REQ-02a, 16, 33, 55.
//!
//! Writes the Config fields so the test fixture can start. Validation is
//! left as a TODO (skeleton only).

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{constants::*, events::ConfigInitialized, state::Config};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Config::INIT_SPACE,
        seeds = [SEED_CONFIG],
        bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        seeds = [SEED_FEE_VAULT],
        bump,
        token::mint = settlement_mint,
        token::authority = config,
        token::token_program = token_program,
    )]
    pub fee_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(mint::token_program = token_program)]
    pub settlement_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

/// Validation: `protocol_fee_bps <= MAX_FEE_BPS` · `rate_ceiling > 0`.
/// Logic: write fields, `paused = false` · emit `ConfigInitialized`.
/// Errors: `InvalidParams`.
pub fn handle_initialize_config(
    ctx: Context<InitializeConfig>,
    protocol_fee_bps: u16,
    rate_ceiling: u64,
) -> Result<()> {
    // TODO(A): validation — `protocol_fee_bps <= MAX_FEE_BPS` and
    // `rate_ceiling > 0`, else `FraError::InvalidParams`.

    let authority = ctx.accounts.authority.key();
    let fee_vault = ctx.accounts.fee_vault.key();
    ctx.accounts.config.set_inner(Config {
        authority,
        protocol_fee_bps,
        fee_vault,
        rate_ceiling,
        paused: false,
        bump: ctx.bumps.config,
        _reserved: [0; 32],
    });

    emit!(ConfigInitialized {
        authority,
        fee_vault,
        settlement_mint: ctx.accounts.settlement_mint.key(),
        protocol_fee_bps,
        rate_ceiling,
    });
    Ok(())
}
