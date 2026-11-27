use anchor_lang_v2::prelude::*;

#[account]
pub struct Counter {
    pub count: u64,
    pub authority: Address,
}
