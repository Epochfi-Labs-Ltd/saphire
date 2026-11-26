use anchor_lang::prelude::*;

/// Seeds `[b"reserve", market]`. Segment A. Operator inventory.
/// Vault: token account, authority = Reserve PDA.
/// Created by IX-05 · written by IX-06/07 (`balance`); IX-18/19 (`committed`
/// +); IX-20, IX-22, IX-23 (`committed` −, `balance` ±).
///
/// Invariant: `balance == reserve_vault.amount` after every transfer.
#[account]
#[derive(InitSpace)]
pub struct Reserve {
    pub market: Pubkey,
    pub vault: Pubkey,
    pub balance: u64,
    pub committed: u64,
    pub bump: u8,
    pub _reserved: [u8; 32],
}

impl Reserve {
    /// `balance − committed`; `None` if the invariant `committed <= balance`
    /// is broken.
    pub fn withdrawable(&self) -> Option<u64> {
        self.balance.checked_sub(self.committed)
    }
}
