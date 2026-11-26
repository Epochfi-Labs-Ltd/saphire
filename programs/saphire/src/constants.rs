//! PDA seeds and protocol constants. Seeds mirror the account cards in
//! docs/implementation.md; `crates/saphire-sdk` derives the same addresses.

// ── Account seeds (docs/implementation.md, "Account cards") ────────────────
pub const SEED_CONFIG: &[u8] = b"config";
pub const SEED_OPERATOR: &[u8] = b"operator";
pub const SEED_MARKET: &[u8] = b"market";
pub const SEED_RESERVE: &[u8] = b"reserve";
pub const SEED_QUOTE: &[u8] = b"quote";
pub const SEED_ELIGIBLE: &[u8] = b"eligible";
pub const SEED_MARGIN: &[u8] = b"margin";
pub const SEED_POSITION: &[u8] = b"position";
pub const SEED_NONCE: &[u8] = b"nonce";
pub const SEED_RATE: &[u8] = b"rate";

// ── Vault seeds ────────────────────────────────────────────────────────────
// The cards say only "token account, authority = <PDA>". The vaults are PDAs
// of this program so every address is derivable (docs/skeleton.md, "Gaps").
/// `[b"fee_vault"]`, authority = Config.
pub const SEED_FEE_VAULT: &[u8] = b"fee_vault";
/// `[b"reserve_vault", market]`, authority = Reserve.
pub const SEED_RESERVE_VAULT: &[u8] = b"reserve_vault";
/// `[b"margin_vault", market, trader]`, authority = TraderMargin.
pub const SEED_MARGIN_VAULT: &[u8] = b"margin_vault";

// ── Protocol bounds ────────────────────────────────────────────────────────
/// Upper bound on `Config.protocol_fee_bps` (IX-01, IX-02).
/// PLACEHOLDER: the docs name the constant but not its value; 100 bps = 1%.
pub const MAX_FEE_BPS: u16 = 100;

/// How far an RFQ deal's period dates may sit from `derive_period(now, ..)`
/// (IX-19 validation 4).
/// PLACEHOLDER: the docs name `ROLL_TOLERANCE` but not its value.
pub const ROLL_TOLERANCE_DAYS: i64 = 3;

pub const SECONDS_PER_DAY: i64 = fra_math::SECONDS_PER_DAY;

/// Rate-index scale: 1.0 == 10^18. `Market.rate_index` starts here.
pub const INDEX_SCALE: u128 = 1_000_000_000_000_000_000;

/// Domain separator in the Borsh-encoded `DealMessage` (IX-19).
pub const DEAL_DOMAIN: [u8; 8] = *b"FRADEAL1";

// ── Supported market conventions (IX-05 "in supported set") ────────────────
/// v1: USD SOFR compounded in arrears.
pub const BENCHMARK_SOFR: u8 = 0;
/// v1: ACT/360.
pub const DAY_COUNT_ACT_360: u16 = 360;
/// v1: Modified Following.
pub const BDC_MODIFIED_FOLLOWING: u8 = 0;
