//! Segment A — Market & Reserve (Ritam).

pub mod admit_trader;
pub mod create_market;
pub mod deactivate_operator;
pub mod fund_reserve;
pub mod halt_market;
pub mod initialize_config;
pub mod pause_program;
pub mod publish_quote;
pub mod register_operator;
pub mod resume_market;
pub mod resume_program;
pub mod suspend_trader;
pub mod update_config;
pub mod withdraw_reserve;

pub use admit_trader::*;
pub use create_market::*;
pub use deactivate_operator::*;
pub use fund_reserve::*;
pub use halt_market::*;
pub use initialize_config::*;
pub use pause_program::*;
pub use publish_quote::*;
pub use register_operator::*;
pub use resume_market::*;
pub use resume_program::*;
pub use suspend_trader::*;
pub use update_config::*;
pub use withdraw_reserve::*;
