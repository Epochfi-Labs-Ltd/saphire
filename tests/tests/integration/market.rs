//! Segment A: config, operators, markets, reserve, quotes, eligibility,
//! halt/pause, fees.

mod admit_trader; // IX-09
mod create_market; // IX-05
mod deactivate_operator; // IX-04
mod fund_reserve; // IX-06
mod halt_market; // IX-13
mod initialize_config; // IX-01
mod pause_program; // IX-15
mod publish_quote; // IX-08
mod register_operator; // IX-03
mod resume_market; // IX-14
mod resume_program; // IX-16
mod suspend_trader; // IX-10
mod update_config; // IX-02
mod withdraw_fees;
mod withdraw_reserve; // IX-07 // IX-25
