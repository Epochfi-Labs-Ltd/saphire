use anchor_lang::prelude::*;

/// Seeds `[b"eligible", market, trader]`. Segment A. Per-market whitelist.
/// Created by IX-09 · written by IX-09 (re-admit), IX-10 · read by IX-11,
/// IX-12, IX-18, IX-19 — never by withdraw, close, settle, unwind.
#[account]
#[derive(InitSpace)]
pub struct Eligibility {
    pub market: Pubkey,
    pub trader: Pubkey,
    pub active: bool,
    pub bump: u8,
    pub _reserved: [u8; 32],
}
