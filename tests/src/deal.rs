//! Signed-deal helpers for `open_position_signed` (IX-19; detached-signature
//! design, DECISION PENDING, see
//! `saphire::instructions::positions::open_position_signed`).

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
