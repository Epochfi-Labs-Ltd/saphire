//! PDA helpers in `saphire_tests::pda` against the program's own seeds.

use anchor_lang::prelude::Pubkey;
use saphire_tests::pda::*;

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
