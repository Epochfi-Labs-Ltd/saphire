pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang_v2::prelude::*;

pub use instructions::*;

declare_id!("7DCT2pqaqtijF2kgN8jAa9Lbftzi1NiEmGAJ8SbWCMUv");

#[program]
pub mod saphire {
    use super::*;

    pub fn initialize(ctx: &mut Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
