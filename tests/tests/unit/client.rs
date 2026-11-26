//! The Codama client (`saphire-client`) must stay in sync with the program.
//! A failure here means the IDL changed: run `pnpm generate` and commit
//! `clients/rust/src/generated`.

use anchor_lang::{prelude::Pubkey, Discriminator, InstructionData, ToAccountMetas};
use saphire::{accounts as acc, instruction as program_ix};
use saphire_client::{accounts as client_acc, instructions as client_ix, types::Side};
use saphire_tests::fixture::{default_market_params, TOKEN_PROGRAM_ID};

#[test]
fn program_id_matches() {
    assert_eq!(saphire_client::PROGRAM_ID, saphire::ID);
}

#[test]
fn account_discriminators_match() {
    assert_eq!(
        client_acc::CONFIG_DISCRIMINATOR,
        saphire::Config::DISCRIMINATOR
    );
    assert_eq!(
        client_acc::MARKET_DISCRIMINATOR,
        saphire::Market::DISCRIMINATOR
    );
    assert_eq!(
        client_acc::POSITION_DISCRIMINATOR,
        saphire::Position::DISCRIMINATOR
    );
    assert_eq!(
        client_acc::DAILY_RATE_DISCRIMINATOR,
        saphire::DailyRate::DISCRIMINATOR
    );
}

#[test]
fn update_config_options_encode_like_anchor() {
    let authority = Pubkey::new_unique();
    let config = Pubkey::new_unique();
    let client = client_ix::UpdateConfigBuilder::new()
        .authority(authority)
        .config(config)
        .rate_ceiling(123)
        .instruction();
    let anchor = program_ix::UpdateConfig {
        protocol_fee_bps: None,
        rate_ceiling: Some(123),
    };
    assert_eq!(client.data, anchor.data());
    assert_eq!(
        client.accounts,
        acc::UpdateConfig { authority, config }.to_account_metas(None)
    );
}

#[test]
fn create_market_flattened_params_encode_like_anchor() {
    let p = default_market_params();
    let keeper = Pubkey::new_unique();
    let k = |_| Pubkey::new_unique();
    let (operator, config, registration, mint, market, reserve, vault, quote) =
        (k(0), k(1), k(2), k(3), k(4), k(5), k(6), k(7));
    let client = client_ix::CreateMarketBuilder::new()
        .operator(operator)
        .config(config)
        .operator_registration(registration)
        .settlement_mint(mint)
        .market(market)
        .reserve(reserve)
        .reserve_vault(vault)
        .quote(quote)
        .token_program(TOKEN_PROGRAM_ID)
        .benchmark_id(p.benchmark_id)
        .start_offset_months(p.start_offset_months)
        .tenor_months(p.tenor_months)
        .day_count_basis(p.day_count_basis)
        .business_day_convention(p.business_day_convention)
        .initial_margin_bps(p.initial_margin_bps)
        .operator_commit_bps(p.operator_commit_bps)
        .min_notional(p.min_notional)
        .max_notional(p.max_notional)
        .open_interest_cap(p.open_interest_cap)
        .quote_validity_secs(p.quote_validity_secs)
        .rate_grace_days(p.rate_grace_days)
        .early_close_fee_bps(p.early_close_fee_bps)
        .max_half_spread_bps(p.max_half_spread_bps)
        .index_start_date(p.index_start_date)
        .settlement_fee(p.settlement_fee)
        .keeper(keeper)
        .instruction();
    let anchor_accounts = acc::CreateMarket {
        operator,
        config,
        operator_registration: registration,
        settlement_mint: mint,
        market,
        reserve,
        reserve_vault: vault,
        quote,
        token_program: TOKEN_PROGRAM_ID,
        system_program: anchor_lang::system_program::ID,
    };
    assert_eq!(
        client.data,
        program_ix::CreateMarket { params: p, keeper }.data()
    );
    assert_eq!(client.accounts, anchor_accounts.to_account_metas(None));
}

#[test]
fn open_position_side_encodes_like_anchor() {
    let data = |side| {
        client_ix::OpenPositionBuilder::new()
            .trader(Pubkey::new_unique())
            .config(Pubkey::new_unique())
            .market(Pubkey::new_unique())
            .quote(Pubkey::new_unique())
            .eligibility(Pubkey::new_unique())
            .trader_margin(Pubkey::new_unique())
            .margin_vault(Pubkey::new_unique())
            .reserve(Pubkey::new_unique())
            .fee_vault(Pubkey::new_unique())
            .position(Pubkey::new_unique())
            .settlement_mint(Pubkey::new_unique())
            .token_program(TOKEN_PROGRAM_ID)
            .side(side)
            .notional(42)
            .instruction()
            .data
    };
    let anchor = |side| program_ix::OpenPosition { side, notional: 42 }.data();
    assert_eq!(data(Side::PayFixed), anchor(saphire::Side::PayFixed));
    assert_eq!(
        data(Side::ReceiveFixed),
        anchor(saphire::Side::ReceiveFixed)
    );
}
