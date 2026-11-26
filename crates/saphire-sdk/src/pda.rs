//! PDA derivation, one fn per account. Each returns `(address, bump)`.

use anchor_lang::prelude::Pubkey;
use saphire::constants::*;

fn find(seeds: &[&[u8]]) -> (Pubkey, u8) {
    Pubkey::find_program_address(seeds, &saphire::ID)
}

/// `[b"config"]`
pub fn config() -> (Pubkey, u8) {
    find(&[SEED_CONFIG])
}

/// `[b"fee_vault"]`
pub fn fee_vault() -> (Pubkey, u8) {
    find(&[SEED_FEE_VAULT])
}

/// `[b"operator", operator]`
pub fn operator_registration(operator: &Pubkey) -> (Pubkey, u8) {
    find(&[SEED_OPERATOR, operator.as_ref()])
}

/// `[b"market", benchmark_id, settlement_mint, start_offset_months, tenor_months]`
pub fn market(
    benchmark_id: u8,
    settlement_mint: &Pubkey,
    start_offset_months: u8,
    tenor_months: u8,
) -> (Pubkey, u8) {
    find(&[
        SEED_MARKET,
        &[benchmark_id],
        settlement_mint.as_ref(),
        &[start_offset_months],
        &[tenor_months],
    ])
}

/// `[b"reserve", market]`
pub fn reserve(market: &Pubkey) -> (Pubkey, u8) {
    find(&[SEED_RESERVE, market.as_ref()])
}

/// `[b"reserve_vault", market]`
pub fn reserve_vault(market: &Pubkey) -> (Pubkey, u8) {
    find(&[SEED_RESERVE_VAULT, market.as_ref()])
}

/// `[b"quote", market]`
pub fn quote(market: &Pubkey) -> (Pubkey, u8) {
    find(&[SEED_QUOTE, market.as_ref()])
}

/// `[b"eligible", market, trader]`
pub fn eligibility(market: &Pubkey, trader: &Pubkey) -> (Pubkey, u8) {
    find(&[SEED_ELIGIBLE, market.as_ref(), trader.as_ref()])
}

/// `[b"margin", market, trader]`
pub fn trader_margin(market: &Pubkey, trader: &Pubkey) -> (Pubkey, u8) {
    find(&[SEED_MARGIN, market.as_ref(), trader.as_ref()])
}

/// `[b"margin_vault", market, trader]`
pub fn margin_vault(market: &Pubkey, trader: &Pubkey) -> (Pubkey, u8) {
    find(&[SEED_MARGIN_VAULT, market.as_ref(), trader.as_ref()])
}

/// `[b"position", market, trader, position_count_le]`
pub fn position(market: &Pubkey, trader: &Pubkey, position_count: u64) -> (Pubkey, u8) {
    find(&[
        SEED_POSITION,
        market.as_ref(),
        trader.as_ref(),
        &position_count.to_le_bytes(),
    ])
}

/// `[b"nonce", market, trader, nonce_le]`
pub fn used_nonce(market: &Pubkey, trader: &Pubkey, nonce: u64) -> (Pubkey, u8) {
    find(&[
        SEED_NONCE,
        market.as_ref(),
        trader.as_ref(),
        &nonce.to_le_bytes(),
    ])
}

/// `[b"rate", market, date_le]`
pub fn daily_rate(market: &Pubkey, date: i64) -> (Pubkey, u8) {
    find(&[SEED_RATE, market.as_ref(), &date.to_le_bytes()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn daily_rate_matches_program_helper() {
        let market = Pubkey::new_unique();
        assert_eq!(
            daily_rate(&market, 1_767_571_200),
            saphire::DailyRate::pda(&market, 1_767_571_200)
        );
    }

    #[test]
    fn position_seeds_use_little_endian_count() {
        let (m, t) = (Pubkey::new_unique(), Pubkey::new_unique());
        assert_ne!(position(&m, &t, 1).0, position(&m, &t, 256).0);
    }
}
