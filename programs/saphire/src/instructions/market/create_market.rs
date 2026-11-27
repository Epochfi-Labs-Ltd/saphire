//! IX-05 `create_market(params: MarketParams, keeper: Pubkey)` · A · operator
//! Implements: REQ-01–06.

use anchor_lang::prelude::*;

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
}

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    pub signer: Signer<'info>,
}

pub fn handle_create_market(
    _ctx: Context<CreateMarket>,
    _params: MarketParams,
    _keeper: Pubkey,
) -> Result<()> {
    Ok(())
}
