use anchor_lang::prelude::*;

/// Seeds `[b"operator", operator]`. Segment A. Admin allowlist of operators.
/// Created by IX-03 · written by IX-04 · read by IX-05 (`active`).
#[account]
#[derive(InitSpace)]
pub struct OperatorRegistration {
    pub operator: Pubkey,
    pub active: bool,
    pub bump: u8,
    pub _reserved: [u8; 32],
}
