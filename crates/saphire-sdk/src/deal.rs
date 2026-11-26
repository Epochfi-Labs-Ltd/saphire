//! Signed-deal helpers for IX-19 (detached-signature design; DECISION
//! PENDING, see `saphire::instructions::positions::open_position_signed`).

use anchor_lang::{prelude::Pubkey, solana_program::instruction::Instruction, AnchorSerialize};
pub use saphire::{DealMessage, SignedDeal};

/// Native Ed25519 signature-verification program.
pub const ED25519_PROGRAM_ID: Pubkey =
    anchor_lang::pubkey!("Ed25519SigVerify111111111111111111111111111");

/// `DealMessage { domain: b"FRADEAL1", program_id, market, trader, ..deal }`.
pub fn deal_message(market: &Pubkey, trader: &Pubkey, deal: &SignedDeal) -> DealMessage {
    DealMessage::new(saphire::ID, *market, *trader, deal)
}

/// The bytes the operator signs: `borsh(DealMessage)`.
pub fn deal_message_bytes(market: &Pubkey, trader: &Pubkey, deal: &SignedDeal) -> Vec<u8> {
    let mut out = Vec::new();
    // Writing into a Vec cannot fail.
    deal_message(market, trader, deal)
        .serialize(&mut out)
        .expect("borsh into Vec");
    out
}

const PUBKEY_LEN: usize = 32;
const SIGNATURE_LEN: usize = 64;
/// `num_signatures: u8`, `padding: u8`, then one 14-byte offsets struct.
const DATA_START: usize = 2 + 14;

/// Build the Ed25519 verify instruction that must immediately precede
/// `open_position_signed` in the same transaction. Layout matches
/// `solana_ed25519_program::new_ed25519_instruction_with_signature`: one
/// signature, all data inline (`instruction_index = u16::MAX`).
pub fn ed25519_verify_instruction(
    pubkey: &Pubkey,
    signature: &[u8; SIGNATURE_LEN],
    message: &[u8],
) -> Instruction {
    let public_key_offset = DATA_START;
    let signature_offset = public_key_offset + PUBKEY_LEN;
    let message_offset = signature_offset + SIGNATURE_LEN;

    let to_u16 = |v: usize| u16::try_from(v).expect("ed25519 instruction data exceeds u16");
    let this_ix = u16::MAX;

    let mut data = Vec::with_capacity(message_offset + message.len());
    data.push(1u8); // num_signatures
    data.push(0u8); // padding
    for field in [
        to_u16(signature_offset),
        this_ix,
        to_u16(public_key_offset),
        this_ix,
        to_u16(message_offset),
        to_u16(message.len()),
        this_ix,
    ] {
        data.extend_from_slice(&field.to_le_bytes());
    }
    data.extend_from_slice(pubkey.as_ref());
    data.extend_from_slice(signature);
    data.extend_from_slice(message);

    Instruction {
        program_id: ED25519_PROGRAM_ID,
        accounts: vec![],
        data,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use saphire::Side;

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
}
