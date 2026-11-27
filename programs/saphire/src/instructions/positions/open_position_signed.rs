//! IX-19 `open_position_signed(deal: SignedDeal)` · B · trader + operator signature
//! Implements: REQ-42–50 (collapsed), then REQ-29–35, 37, 38, 40, 41.

use anchor_lang::prelude::*;

use crate::state::Side;

/// Deal terms submitted by the trader and signed by the operator.
/// Signature design (detached vs co-signer) is still pending.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SignedDeal {
    pub side: Side,
    pub notional: u64,
    pub fixed_rate: u64,
    pub period_start: i64,
    pub period_end: i64,
    pub expires_at: i64,
    pub nonce: u64,
}

#[derive(Accounts)]
pub struct OpenPositionSigned<'info> {
    pub signer: Signer<'info>,
}

pub fn handle_open_position_signed(
    _ctx: Context<OpenPositionSigned>,
    _deal: SignedDeal,
) -> Result<()> {
    Ok(())
}
