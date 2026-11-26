use anchor_lang::prelude::*;

/// Seeds `[b"quote", market]`, overwritten on each publish. Segment A.
/// Created by IX-05 · written by IX-08 · read by IX-18, IX-20.
#[account]
#[derive(InitSpace)]
pub struct Quote {
    pub market: Pubkey,
    pub fixed_mid: u64,
    pub half_spread_bps: u16,
    pub max_notional_at_quote: u64,
    pub published_at: i64,
    /// 0 = never published; advanced by the program on every publish (REQ-14).
    pub epoch: u64,
    pub bump: u8,
    pub _reserved: [u8; 32],
}

impl Quote {
    /// Fresh when `epoch > 0` and `now − published_at <= validity_secs`.
    pub fn is_fresh(&self, now: i64, validity_secs: u32) -> bool {
        self.epoch > 0
            && now
                .checked_sub(self.published_at)
                .is_some_and(|age| age <= i64::from(validity_secs))
    }
}
