//! Minimal LiteSVM fixture: loads `target/deploy/saphire.so` (run
//! `anchor build` first) and sends transactions. Grow it as handlers land.

use anchor_lang::solana_program::instruction::Instruction;
use litesvm::{types::TransactionResult, LiteSVM};
use solana_keypair::Keypair;
use solana_message::{Message, VersionedMessage};
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;

pub type TxResult = TransactionResult;

pub const SOL: u64 = 1_000_000_000;

fn program_bytes() -> Vec<u8> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/deploy/saphire.so");
    std::fs::read(path).unwrap_or_else(|e| {
        panic!("cannot read {path}: {e}. Run `anchor build` before `cargo test`.")
    })
}

pub struct TestEnv {
    pub svm: LiteSVM,
    pub admin: Keypair,
}

impl Default for TestEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl TestEnv {
    /// Fresh SVM with the program loaded and a funded admin keypair.
    pub fn new() -> Self {
        let mut svm = LiteSVM::new();
        svm.add_program(saphire::ID, &program_bytes())
            .expect("load saphire.so");
        let admin = Keypair::new();
        svm.airdrop(&admin.pubkey(), 100 * SOL)
            .expect("airdrop admin");
        Self { svm, admin }
    }

    /// Sign and send `ixs`; `signers[0]` pays. The blockhash is expired first
    /// so identical instructions can be resent.
    pub fn send(&mut self, ixs: &[Instruction], signers: &[&Keypair]) -> TxResult {
        self.svm.expire_blockhash();
        let payer = signers.first().expect("at least one signer").pubkey();
        let msg = Message::new_with_blockhash(ixs, Some(&payer), &self.svm.latest_blockhash());
        let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), signers)
            .expect("sign transaction");
        self.svm.send_transaction(tx)
    }
}
