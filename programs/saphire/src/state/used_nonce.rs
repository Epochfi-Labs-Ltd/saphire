use anchor_lang::prelude::*;

/// Seeds `[b"nonce", market, trader, nonce_le]`. Segment B.
/// Created by IX-19 · closed by IX-24 (optional) · replay protection for
/// signed deals.
///
/// DECISION PENDING: dropped entirely if IX-19 moves to the co-signer design
/// (see instructions/positions/open_position_signed.rs).
#[account]
#[derive(InitSpace)]
pub struct UsedNonce {
    pub market: Pubkey,
    pub trader: Pubkey,
    pub nonce: u64,
    pub position: Pubkey,
    pub bump: u8,
    pub _reserved: [u8; 32],
}
