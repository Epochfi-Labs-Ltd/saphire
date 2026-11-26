use anchor_lang::prelude::*;

/// Seeds `[b"margin", market, trader]`. Segment B.
/// Vault: token account, authority = TraderMargin PDA.
/// Created by IX-11 · written by IX-12, IX-17 (`balance`); IX-18/19 (`locked`
/// +, `balance` − fee, `open_positions` +); IX-20, IX-22, IX-23 (`locked` −,
/// `balance` ±, `open_positions` −).
///
/// Invariant: `balance == margin_vault.amount`; `locked == Σ (locked_margin +
/// settlement_fee)` over Open positions.
#[account]
#[derive(InitSpace)]
pub struct TraderMargin {
    pub market: Pubkey,
    pub trader: Pubkey,
    pub vault: Pubkey,
    pub balance: u64,
    pub locked: u64,
    pub open_positions: u32,
    pub bump: u8,
    pub _reserved: [u8; 32],
}

impl TraderMargin {
    /// `balance − locked`; `None` if the invariant `locked <= balance` is
    /// broken.
    pub fn free(&self) -> Option<u64> {
        self.balance.checked_sub(self.locked)
    }
}
