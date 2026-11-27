//! The program loads and an instruction round-trips through LiteSVM.

use anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas};
use saphire_tests::fixture::TestEnv;
use solana_signer::Signer;

#[test]
fn program_loads_and_accepts_an_instruction() {
    let mut env = TestEnv::new();
    let admin = env.admin.insecure_clone();
    let ix = Instruction {
        program_id: saphire::ID,
        accounts: saphire::accounts::PauseProgram {
            signer: admin.pubkey(),
        }
        .to_account_metas(None),
        data: saphire::instruction::PauseProgram {}.data(),
    };
    env.send(&[ix], &[&admin])
        .expect("pause_program stub succeeds");
}
