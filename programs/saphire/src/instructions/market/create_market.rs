//! IX-05 `create_market(params: MarketParams, keeper: Pubkey)` · A · operator
//! Implements: REQ-01–06.

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    constants::*,
    errors::FraError,
    state::{Config, Market, OperatorRegistration, Quote, Reserve},
};

/// Market conventions and risk parameters written to `Market` at creation.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct MarketParams {
    pub benchmark_id: u8,
    pub start_offset_months: u8,
    pub tenor_months: u8,
    pub day_count_basis: u16,
    pub business_day_convention: u8,
    pub initial_margin_bps: u16,
    pub operator_commit_bps: u16,
    pub min_notional: u64,
    pub max_notional: u64,
    pub open_interest_cap: u64,
    pub quote_validity_secs: u32,
    pub rate_grace_days: u16,
    pub early_close_fee_bps: u16,
    pub max_half_spread_bps: u16,
    /// 00:00 UTC of the business day before the first daily print; written
    /// to `Market.last_rate_date`.
    pub index_start_date: i64,
    /// Flat per-position crank bounty (see `Market.settlement_fee`).
    /// Not on the cards yet; see docs/skeleton.md "Fee additions".
    pub settlement_fee: u64,
}

#[derive(Accounts)]
#[instruction(params: MarketParams)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub operator: Signer<'info>,

    #[account(seeds = [SEED_CONFIG], bump = config.bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(
        seeds = [SEED_OPERATOR, operator.key().as_ref()],
        bump = operator_registration.bump,
        constraint = operator_registration.active @ FraError::OperatorInactive,
    )]
    pub operator_registration: Box<Account<'info, OperatorRegistration>>,

    #[account(mint::token_program = token_program)]
    pub settlement_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = operator,
        space = 8 + Market::INIT_SPACE,
        seeds = [
            SEED_MARKET,
            &[params.benchmark_id],
            settlement_mint.key().as_ref(),
            &[params.start_offset_months],
            &[params.tenor_months],
        ],
        bump,
    )]
    pub market: Box<Account<'info, Market>>,

    #[account(
        init,
        payer = operator,
        space = 8 + Reserve::INIT_SPACE,
        seeds = [SEED_RESERVE, market.key().as_ref()],
        bump,
    )]
    pub reserve: Box<Account<'info, Reserve>>,

    #[account(
        init,
        payer = operator,
        seeds = [SEED_RESERVE_VAULT, market.key().as_ref()],
        bump,
        token::mint = settlement_mint,
        token::authority = reserve,
        token::token_program = token_program,
    )]
    pub reserve_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = operator,
        space = 8 + Quote::INIT_SPACE,
        seeds = [SEED_QUOTE, market.key().as_ref()],
        bump,
    )]
    pub quote: Box<Account<'info, Quote>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

/// Validation:
/// - `initial_margin_bps > 0 && operator_commit_bps > 0` (REQ-04)
/// - `min_notional <= max_notional` (REQ-05)
/// - `benchmark_id`, `day_count_basis`, `business_day_convention` in
///   supported set (`BENCHMARK_SOFR`, `DAY_COUNT_ACT_360`,
///   `BDC_MODIFIED_FOLLOWING`)
/// - `open_interest_cap`, `quote_validity_secs`, `rate_grace_days`,
///   `max_half_spread_bps` > 0
///
/// Logic:
/// 1. Write Market: `operator = signer`, `keeper`, params, `status = Active`,
///    `open_interest = 0`, `position_count = 0`, `rate_index = INDEX_SCALE`
///    (10^18), `last_rate_date = params.index_start_date`,
///    `settlement_fee = params.settlement_fee` (0 allowed)
/// 2. Reserve: `market`, `vault = reserve_vault`, `balance = committed = 0`
/// 3. Quote: `market`, zeroed, `epoch = 0` (unpublished)
/// 4. Emit `MarketCreated`
///
/// Errors: `OperatorInactive` (REQ-02/02c), `MarketExists` (init, REQ-03),
/// `InvalidParams`.
pub fn handle_create_market(
    _ctx: Context<CreateMarket>,
    _params: MarketParams,
    _keeper: Pubkey,
) -> Result<()> {
    todo!("IX-05 A")
}
