use anchor_lang::prelude::*;

/// Seeds `[b"config"]`, one per program. Segment A.
/// Created by IX-01 · written by IX-02, IX-15, IX-16 · read by IX-18, IX-19
/// (paused, fee, fee_vault), IX-08, IX-21 (rate_ceiling).
#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub protocol_fee_bps: u16,
    /// Token account, authority = Config PDA, mint = settlement mint.
    pub fee_vault: Pubkey,
    pub rate_ceiling: u64,
    pub paused: bool,
    pub bump: u8,
    pub _reserved: [u8; 32],
}
