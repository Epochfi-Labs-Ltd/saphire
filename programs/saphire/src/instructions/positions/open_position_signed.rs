//! IX-19 `open_position_signed(deal: SignedDeal)` · B · trader + operator signature
//! Implements: REQ-42–50 (collapsed), then REQ-29–35, 37, 38, 40, 41.
//!
//! DECISION PENDING (24 Nov, see README "Open design question"): make the
//! operator a co-signer of this transaction instead of verifying a detached
//! signature. Co-signer version: add `operator` · signer ·
//! `market.operator == operator`; drop `instructions_sysvar`, `used_nonce`
//! and validation steps 1 and 5; keep expiry and period checks. One
//! transaction, runtime-verified, no `UsedNonce` account (and IX-24 goes
//! away). Neither design is implemented yet. The account struct below is the
//! current detached-signature design from the card.

use anchor_lang::{prelude::*, solana_program::sysvar::SysvarId};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    constants::*,
    errors::FraError,
    state::{Config, Eligibility, Market, Position, Reserve, Side, TraderMargin, UsedNonce},
};

/// Deal terms submitted by the trader; the operator signed the matching
/// [`DealMessage`].
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SignedDeal {
    pub side: Side,
    pub notional: u64,
    pub fixed_rate: u64,
    pub period_start: i64,
    pub period_end: i64,
    pub expires_at: i64,
    pub nonce: u64,
}

/// The exact bytes the operator signs: `borsh(DealMessage)`. Binding the
/// domain, program, market and trader stops a signature being replayed on
/// another program, market or trader.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct DealMessage {
    /// Always [`DEAL_DOMAIN`] (`b"FRADEAL1"`).
    pub domain: [u8; 8],
    pub program_id: Pubkey,
    pub market: Pubkey,
    pub trader: Pubkey,
    pub side: Side,
    pub notional: u64,
    pub fixed_rate: u64,
    pub period_start: i64,
    pub period_end: i64,
    pub expires_at: i64,
    pub nonce: u64,
}

impl DealMessage {
    pub fn new(program_id: Pubkey, market: Pubkey, trader: Pubkey, deal: &SignedDeal) -> Self {
        Self {
            domain: DEAL_DOMAIN,
            program_id,
            market,
            trader,
            side: deal.side,
            notional: deal.notional,
            fixed_rate: deal.fixed_rate,
            period_start: deal.period_start,
            period_end: deal.period_end,
            expires_at: deal.expires_at,
            nonce: deal.nonce,
        }
    }
}

/// As IX-18 **minus** `quote`, **plus** `instructions_sysvar` and `used_nonce`.
#[derive(Accounts)]
#[instruction(deal: SignedDeal)]
pub struct OpenPositionSigned<'info> {
    #[account(mut)]
    pub trader: Signer<'info>,

    #[account(seeds = [SEED_CONFIG], bump = config.bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(mut)]
    pub market: Box<Account<'info, Market>>,

    #[account(
        seeds = [SEED_ELIGIBLE, market.key().as_ref(), trader.key().as_ref()],
        bump = eligibility.bump,
        constraint = eligibility.active @ FraError::NotEligible, // REQ-35
    )]
    pub eligibility: Box<Account<'info, Eligibility>>,

    #[account(
        mut,
        seeds = [SEED_MARGIN, market.key().as_ref(), trader.key().as_ref()],
        bump = trader_margin.bump,
    )]
    pub trader_margin: Box<Account<'info, TraderMargin>>,

    #[account(mut, address = trader_margin.vault)]
    pub margin_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Card: "reserve W". `has_one = market` added so a foreign reserve
    /// cannot be committed.
    #[account(mut, has_one = market)]
    pub reserve: Box<Account<'info, Reserve>>,

    #[account(mut, address = config.fee_vault)]
    pub fee_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = trader,
        space = 8 + Position::INIT_SPACE,
        seeds = [
            SEED_POSITION,
            market.key().as_ref(),
            trader.key().as_ref(),
            &market.position_count.to_le_bytes(),
        ],
        bump,
    )]
    pub position: Box<Account<'info, Position>>,

    /// CHECK: address-constrained to the Instructions sysvar; the handler
    /// reads the preceding Ed25519 instruction from it.
    #[account(address = Instructions::id())]
    pub instructions_sysvar: UncheckedAccount<'info>,

    /// Replay protection: init fails if this nonce was already used.
    #[account(
        init,
        payer = trader,
        space = 8 + UsedNonce::INIT_SPACE,
        seeds = [
            SEED_NONCE,
            market.key().as_ref(),
            trader.key().as_ref(),
            &deal.nonce.to_le_bytes(),
        ],
        bump,
    )]
    pub used_nonce: Box<Account<'info, UsedNonce>>,

    /// Not on the card: needed by `transfer_checked` for the fee.
    #[account(address = market.settlement_mint @ FraError::WrongMint)]
    pub settlement_mint: Box<InterfaceAccount<'info, Mint>>,

    pub clock: Sysvar<'info, Clock>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

/// Validation:
/// 1. Previous instruction in the tx is `ed25519_program`; one signature;
///    `pubkey == market.operator`; `message == borsh(DealMessage { domain:
///    b"FRADEAL1", program_id, market, trader, ..deal })`
/// 2. `now < deal.expires_at`
/// 3. `now < deal.period_start`
/// 4. period dates consistent with offset/tenor within `ROLL_TOLERANCE_DAYS`
///    via `derive_period`
/// 5. `used_nonce` init succeeded (replay)
/// 6. IX-18 checks 1, 3 (market bounds only), 4, 5, 6, 7 — quote freshness
///    and the at-quote cap do not apply
///
/// Logic: `K = deal.fixed_rate`, dates from the deal, `days` computed ·
/// IX-18 logic 3–8 with `origin = Rfq` (including the settlement-fee lock) · `used_nonce = { market, trader,
/// nonce, position }` · emit `PositionOpened`, `DealNonceUsed`.
///
/// Errors: `MissingSignature`, `BadSigner`, `MessageMismatch`,
/// `OfferExpired`, `PeriodStarted`, `BadPeriod`, `NonceUsed` + IX-18 set.
pub fn handle_open_position_signed(
    _ctx: Context<OpenPositionSigned>,
    _deal: SignedDeal,
) -> Result<()> {
    todo!("IX-19 B")
}
