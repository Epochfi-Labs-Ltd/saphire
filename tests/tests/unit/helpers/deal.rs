//! Signed-deal encoding and the Ed25519 verify instruction (IX-19).

use anchor_lang::prelude::Pubkey;
use saphire::{Side, SignedDeal};
use saphire_tests::deal::*;

fn sample_deal() -> SignedDeal {
    SignedDeal {
        side: Side::PayFixed,
        notional: 1_000_000_000_000,
        fixed_rate: 40_700,
        period_start: 1_000,
        period_end: 2_000,
        expires_at: 500,
        nonce: 7,
    }
}

#[test]
fn deal_message_starts_with_domain_then_program_id() {
    let (m, t) = (Pubkey::new_unique(), Pubkey::new_unique());
    let bytes = deal_message_bytes(&m, &t, &sample_deal());
    assert_eq!(&bytes[..8], b"FRADEAL1");
    assert_eq!(&bytes[8..40], saphire::ID.as_ref());
    assert_eq!(&bytes[40..72], m.as_ref());
    assert_eq!(&bytes[72..104], t.as_ref());
    // side(1) + 6 x 8-byte fields
    assert_eq!(bytes.len(), 104 + 1 + 6 * 8);
}

#[test]
fn ed25519_layout() {
    let pk = Pubkey::new_unique();
    let sig = [9u8; 64];
    let msg = b"hello";
    let ix = ed25519_verify_instruction(&pk, &sig, msg);
    assert_eq!(ix.program_id, ED25519_PROGRAM_ID);
    assert_eq!(ix.data[0], 1);
    assert_eq!(&ix.data[16..48], pk.as_ref());
    assert_eq!(&ix.data[48..112], &sig);
    assert_eq!(&ix.data[112..], msg);
    // message_data_size
    assert_eq!(u16::from_le_bytes([ix.data[12], ix.data[13]]), 5);
}
