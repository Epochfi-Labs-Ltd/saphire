# Saphire — Implementation Guide

One entry per instruction, in three parts:

1. **Accounts, PDAs, constraints** — what Anchor's account struct validates (seeds, signers, owners, mints, relationships).
2. **Additional validation** — checks that need more context than the account struct can express (clock, arithmetic, cross-account state).
3. **Business logic** — the state changes and CPIs, in order.

Conventions: amounts u64 in USDC base units; rates u64 scaled 10⁴ per percent (1 bp = 100); bps u16; dates i64 Unix seconds; every PDA stores its `bump`. All arithmetic is checked; `fra_math` is the single source of rounding. Errors are named per refusal; the shared enum is at the end.

Segment ownership: **A** Market & Reserve (Ritam) · **B** Positions (Liam) · **C** Fixing & Settlement (Greg).

---

## Shared accounts (reference)

| Account | Seeds | Owner |
| --- | --- | --- |
| `Config` | `[b"config"]` | A |
| `OperatorRegistration` | `[b"operator", operator]` | A |
| `Market` | `[b"market", benchmark_id, settlement_mint, start_offset_months, tenor_months]` | A |
| `Reserve` | `[b"reserve", market]` — owns `reserve_vault` | A |
| `Quote` | `[b"quote", market]` | A |
| `Eligibility` | `[b"eligible", market, trader]` | A |
| `TraderMargin` | `[b"margin", market, trader]` — owns `margin_vault` | B |
| `Position` | `[b"position", market, trader, position_count_le]` | B |
| `UsedNonce` | `[b"nonce", market, trader, nonce_le]` | B |
| `Fixing` | `[b"fixing", market, period_start_le, period_end_le]` | C |

Token accounts: `fee_vault` (authority = Config PDA), `reserve_vault` (authority = Reserve PDA), `margin_vault` (authority = TraderMargin PDA); all `mint = Market.settlement_mint`.

---

# Segment A — Market & Reserve

## IX-01 `initialize_config(protocol_fee_bps: u16, rate_ceiling: u64)`

**1. Accounts**
- `authority`: Signer, mut (payer)
- `config`: init, seeds `[b"config"]`, payer = authority
- `fee_vault`: init token account, authority = config, mint = settlement_mint
- `settlement_mint`: Mint
- `token_program`, `system_program`

**2. Additional validation**
- `protocol_fee_bps <= MAX_FEE_BPS` (e.g. 100) → `InvalidParams`
- `rate_ceiling > 0` → `InvalidParams`

**3. Logic**
1. Write `authority`, `protocol_fee_bps`, `fee_vault`, `rate_ceiling`, `paused = false`, `bump`.
2. Emit `ConfigInitialized`.

## IX-02 `update_config(protocol_fee_bps: Option<u16>, rate_ceiling: Option<u64>)` (optional)

**1.** `authority`: Signer, `has_one` on config → `NotAuthority`; `config`: mut, seeds.
**2.** Same bounds as IX-01.
**3.** Overwrite the provided fields; emit `ConfigUpdated`.

## IX-03 `register_operator()`

**1.** `authority`: Signer, mut, `config.authority == authority` → `NotAuthority`; `config`; `operator`: UncheckedAccount (wallet, not a signer); `operator_registration`: init, seeds `[b"operator", operator]`; `system_program`.
**2.** None.
**3.** Write `operator`, `active = true`, `bump`; emit `OperatorRegistered`.

## IX-04 `deactivate_operator()`

**1.** `authority`: Signer, `config.authority == authority`; `config`; `operator_registration`: mut, seeds `[b"operator", operator_registration.operator]`.
**2.** `active == true` → `OperatorInactive` (idempotency policy: refuse).
**3.** `active = false`; account kept; emit `OperatorDeactivated`. Existing markets, reserve and quotes untouched; `create_market` refuses this operator from now on.

## IX-05 `create_market(params: MarketParams, keeper: Pubkey)`

**1. Accounts**
- `operator`: Signer, mut (payer)
- `config`: seeds `[b"config"]`
- `operator_registration`: seeds `[b"operator", operator]`, `constraint = active` → `OperatorInactive`
- `settlement_mint`: Mint
- `market`: init, seeds `[b"market", params.benchmark_id, settlement_mint, params.start_offset_months, params.tenor_months]` (duplicate → init fails = `MarketExists`)
- `reserve`: init, seeds `[b"reserve", market]`
- `reserve_vault`: init token account, authority = reserve, mint = settlement_mint
- `quote`: init, seeds `[b"quote", market]`
- `token_program`, `system_program`

**2. Additional validation**
- `initial_margin_bps > 0 && operator_commit_bps > 0` → `InvalidParams` (REQ-04)
- `min_notional <= max_notional` → `InvalidParams` (REQ-05)
- `benchmark_id`, `day_count_basis`, `business_day_convention` in the supported set → `InvalidParams`
- `max_half_spread_bps`, `quote_validity_secs`, `fixing_grace_days`, `open_interest_cap` > 0

**3. Logic**
1. Write every `Market` field; `operator = signer`, `keeper`, `status = Active`, `open_interest = 0`, `position_count = 0`.
2. `reserve.market = market`, `vault = reserve_vault`, `balance = 0`, `committed = 0`.
3. `quote.market = market`, zeroed fields, `epoch = 0` (unpublished).
4. Emit `MarketCreated`.

## IX-06 `fund_reserve(amount: u64)`

**1.** `operator`: Signer, `market.operator == operator` → `NotOperator`; `market`; `reserve`: mut, seeds, `has_one = market`; `reserve_vault`: mut, `== reserve.vault`; `operator_token_account`: mut, `mint == market.settlement_mint`, `owner == operator` → `WrongMint`; `token_program`.
**2.** `amount > 0` → `InvalidParams`.
**3.**
1. CPI `transfer(operator_token_account → reserve_vault, amount)` signed by operator.
2. `reserve.balance += amount`.
3. `require!(reserve.balance == reserve_vault.amount)` after reload → `MirrorMismatch`.
4. Emit `ReserveFunded`.

## IX-07 `withdraw_reserve(amount: u64)`

**1.** As IX-06 (destination token account instead of source).
**2.** `amount > 0`; `withdrawable = balance − committed` (checked); `amount <= withdrawable` → `InsufficientWithdrawable` (REQ-10). No halt or pause check (REQ-74).
**3.**
1. CPI `transfer(reserve_vault → operator_token_account, amount)` signed by reserve PDA `[b"reserve", market, bump]`.
2. `reserve.balance −= amount`; mirror assert.
3. Emit `ReserveWithdrawn`.

## IX-08 `publish_quote(fixed_mid: u64, half_spread_bps: u16, max_notional_at_quote: u64)`

**1.** `operator`: Signer, `market.operator == operator`; `config`; `market`; `quote`: mut, seeds, `has_one = market`; `clock`.
**2.**
- `market.status != Closed` → `MarketClosed` (publishing while Halted is allowed, decision 6)
- `0 < fixed_mid <= config.rate_ceiling` → `RateOutOfRange` (REQ-16)
- `half_spread_bps <= market.max_half_spread_bps` → `SpreadTooWide` (REQ-15)
- `fixed_mid > half_spread_bps * 100` (receive-fixed rate stays positive) → `InvalidParams`
- `market.min_notional <= max_notional_at_quote <= market.max_notional` → `InvalidParams`
**3.** Overwrite `fixed_mid`, `half_spread_bps`, `max_notional_at_quote`, `published_at = clock.unix_timestamp`, `epoch += 1` (program-advanced, decision 5). Emit `QuotePublished`.

## IX-09 `admit_trader()`

**1.** `operator`: Signer, mut, `market.operator == operator`; `market`; `trader`: UncheckedAccount; `eligibility`: init_if_needed, seeds `[b"eligible", market, trader]`; `system_program`.
**2.** None (re-admission of a suspended trader is the reactivate path).
**3.** Write `market`, `trader`, `active = true`, `bump`; emit `TraderAdmitted`.

## IX-10 `suspend_trader()`

**1.** `operator`: Signer, `market.operator == operator`; `market`; `eligibility`: mut, seeds `[b"eligible", market, eligibility.trader]`, `has_one = market`.
**2.** None.
**3.** `active = false`; nothing else touched (REQ-18); emit `TraderSuspended`.

## IX-13 `halt_market()` / IX-14 `resume_market()`

**1.** `operator`: Signer, `market.operator == operator` → `NotOperator`; `market`: mut.
**2.** halt: `status == Active` → `InvalidTransition`; resume: `status == Halted` → `MarketNotHalted`.
**3.** Set status; emit `MarketHalted` / `MarketResumed`. Halt blocks IX-18/19 only.

## IX-15 `pause_program()` / IX-16 `resume_program()`

**1.** `authority`: Signer, `config.authority == authority`; `config`: mut.
**2.** None.
**3.** `paused = true/false`; emit event. Pause blocks IX-18/19 only (decision 7).

---

# Segment B — Positions

## IX-11 `init_trader_margin()`

**1. Accounts**
- `trader`: Signer, mut (payer)
- `market`
- `eligibility`: seeds `[b"eligible", market, trader]`, `constraint = active` → `NotEligible`
- `trader_margin`: init, seeds `[b"margin", market, trader]`
- `margin_vault`: init token account, authority = trader_margin, mint = market.settlement_mint
- `settlement_mint`: `== market.settlement_mint`
- `token_program`, `system_program`

**2.** None.
**3.** Write `market`, `trader`, `vault`, `balance = 0`, `locked = 0`, `open_positions = 0`, `bump`.

## IX-12 `deposit_margin(amount: u64)`

**1.** `trader`: Signer; `market`; `eligibility`: seeds, `active` → `NotEligible` (REQ-21); `trader_margin`: mut, seeds `[b"margin", market, trader]`; `margin_vault`: mut, `== trader_margin.vault`; `trader_token_account`: mut, `mint == market.settlement_mint`, `owner == trader`; `token_program`.
**2.** `amount > 0` → `InvalidParams` (REQ-22). Allowed while halted or paused.
**3.**
1. CPI `transfer(trader_token_account → margin_vault, amount)` signed by trader.
2. `balance += amount`; reload vault; `require!(balance == margin_vault.amount)` → `MirrorMismatch`.
3. Emit `MarginDeposited`.

## IX-17 `withdraw_margin(amount: u64)`

**1.** `trader`: Signer, `trader_margin.trader == trader` → `NotTrader` (REQ-25); `trader_margin`: mut, seeds; `margin_vault`: mut, `== trader_margin.vault`; `trader_token_account`: mut, mint/owner checks; `token_program`. No `eligibility`, no `config`, no `market` status read (REQ-26, 74).
**2.** `amount > 0`; `free = balance − locked`; `amount <= free` → `InsufficientFreeMargin` (REQ-24).
**3.**
1. CPI `transfer(margin_vault → trader_token_account, amount)` signed by `[b"margin", market, trader, bump]`.
2. `balance −= amount`; mirror assert.
3. Emit `MarginWithdrawn`.

## IX-18 `open_position(side: Side, notional: u64)`

**1. Accounts**
- `trader`: Signer, mut (payer)
- `config`: seeds `[b"config"]`
- `market`: mut
- `quote`: seeds `[b"quote", market]`, `has_one = market`
- `eligibility`: seeds, `constraint = active` → `NotEligible` (REQ-35)
- `trader_margin`: mut, seeds `[b"margin", market, trader]`
- `margin_vault`: mut, `== trader_margin.vault`
- `reserve`: mut, seeds `[b"reserve", market]`
- `fee_vault`: mut, `== config.fee_vault`
- `position`: init, seeds `[b"position", market, trader, market.position_count.to_le_bytes()]`
- `clock`, `token_program`, `system_program`

**2. Additional validation (in this order — cheap refusals first)**
1. `!config.paused` → `ProgramPaused`; `market.status == Active` → `MarketHalted` (REQ-41)
2. `now − quote.published_at <= market.quote_validity_secs` and `quote.epoch > 0` → `QuoteStale` (REQ-36)
3. `market.min_notional <= notional <= min(market.max_notional, quote.max_notional_at_quote)` → `NotionalOutOfBounds` (REQ-39)
4. `market.open_interest + notional <= market.open_interest_cap` → `OpenInterestCap` (REQ-40)
5. `lock = bps_ceil(notional, market.initial_margin_bps)`; `commit = bps_ceil(notional, market.operator_commit_bps)`; `fee = bps_ceil(notional, config.protocol_fee_bps)`
6. `trader_margin.free() >= lock + fee` → `InsufficientFreeMargin` (REQ-37)
7. `reserve.balance − reserve.committed >= commit` → `InsufficientReserve` (REQ-38)

**3. Logic**
1. `K = fra_math::compute_fixed_rate(quote.fixed_mid, quote.half_spread_bps, side)` (REQ-27/28).
2. `(period_start, period_end) = fra_math::derive_period(now, market.start_offset_months, market.tenor_months, calendar)`; `days = year_fraction_days(period_start, period_end)` (REQ-29/30).
3. CPI `transfer(margin_vault → fee_vault, fee)` signed by TraderMargin PDA; `trader_margin.balance −= fee` (REQ-33).
4. `trader_margin.locked += lock`; `open_positions += 1` (REQ-31).
5. `reserve.committed += commit` (REQ-32).
6. `nonce = market.position_count`; `market.position_count += 1`; `market.open_interest += notional` (REQ-34).
7. Write `Position { market, trader, nonce, side, notional, fixed_rate: K, trade_date: now, period_start, period_end, day_count_days: days, locked_margin: lock, committed_reserve: commit, fee_paid: fee, origin: Quote, status: Open, settled_amount: 0, settled_at: 0, bump }`.
8. Emit `PositionOpened`.

## IX-19 `open_position_signed(deal: SignedDeal)`

> **Under review (Greg, 24 Nov):** replace the detached signature with the operator as a co-signer of this transaction. That version drops `instructions_sysvar`, `used_nonce` and steps 2.1–2.3 below: accounts gain `operator: Signer` with `market.operator == operator`, and the runtime proves both parties agreed. The deal fields, expiry and period-date checks stay. The text below is the current (detached-signature) design.

`SignedDeal { side, notional, fixed_rate, period_start, period_end, expires_at, nonce }`

**1. Accounts** — as IX-18 **minus** `quote`, **plus**:
- `instructions_sysvar`: `address = sysvar::instructions::ID`
- `used_nonce`: init, seeds `[b"nonce", market, trader, deal.nonce.to_le_bytes()]` (duplicate → `NonceUsed`)

**2. Additional validation**
1. Load the instruction at `current_index − 1` from the sysvar; `program_id == ed25519_program::ID` → `MissingSignature`. Parse the Ed25519 header; exactly one signature; `pubkey == market.operator` → `BadSigner`; `message == borsh(DealMessage { domain: b"FRADEAL1", program_id, market, trader, ..deal })` → `MessageMismatch`.
2. `now < deal.expires_at` → `OfferExpired`
3. `now < deal.period_start` → `PeriodStarted`
4. `deal.period_end − deal.period_start` matches tenor and `deal.period_start − now` matches the start offset, both within `ROLL_TOLERANCE` (3 business days) via `fra_math::derive_period` → `BadPeriod`
5. Then IX-18 checks 1, 3 (market bounds only — the at-quote cap does not apply), 4, 5, 6, 7. Quote freshness is not checked.

**3. Logic**
1. `K = deal.fixed_rate`; `period_start/end = deal.*`; `days` computed.
2. IX-18 steps 3–8 with `origin = Rfq`.
3. `used_nonce = { market, trader, nonce, position, bump }`.
4. Emit `PositionOpened`, `DealNonceUsed`.

## IX-20 `close_position()`

**1. Accounts**
- `trader`: Signer, `position.trader == trader` → `NotTrader` (REQ-65)
- `market`: mut
- `quote`: seeds, `has_one = market`
- `trader_margin`: mut, seeds `[b"margin", market, trader]`
- `margin_vault`: mut, `== trader_margin.vault`
- `reserve`: mut, seeds; `reserve_vault`: mut, `== reserve.vault`
- `position`: mut, seeds `[b"position", market, trader, position.nonce.to_le_bytes()]`, `has_one = market`
- `clock`, `token_program`

**2. Additional validation**
1. `position.status == Open` → `NotOpen` (REQ-66)
2. `now < position.period_end` → `UseSettle` (REQ-67)
3. quote fresh (as IX-18 check 2) → `QuoteStale`
No halt/pause/eligibility check (allowed during a halt; needs a fresh quote).

**3. Logic**
1. `close_rate = compute_fixed_rate(quote.fixed_mid, quote.half_spread_bps, opposite(position.side))`.
2. `raw = fra_math::rate_pnl(notional, close_rate, K, days)`; `pnl = side == PayFixed ? raw : −raw`.
3. `pnl = clamp(pnl, −locked_margin, +committed_reserve)`.
4. `fee = bps_ceil(notional, market.early_close_fee_bps)`; `net = pnl − fee`.
5. `net > 0`: CPI `transfer(reserve_vault → margin_vault, net)` signed by Reserve PDA; `reserve.balance −= net`; `trader_margin.balance += net`. `net < 0`: CPI `transfer(margin_vault → reserve_vault, |net|)` signed by TraderMargin PDA; mirrors updated the other way. (`|net| <= locked_margin` holds by the cap plus fee ≤ lock; assert.)
6. `trader_margin.locked −= locked_margin`; `open_positions −= 1`; `reserve.committed −= committed_reserve`; `market.open_interest −= notional`.
7. `position.status = Closed`; `settled_amount = net`; `settled_at = now`.
8. Assert both mirrors; emit `PositionClosed`.

## `fra_math` (crate, owned by B, used by A and C)

| fn | Rule |
| --- | --- |
| `compute_fixed_rate(mid, half_spread_bps, side) -> u64` | PayFixed `mid + bps·100`; ReceiveFixed `mid − bps·100` (checked) |
| `bps_ceil(amount, bps) -> u64` | `ceil(amount · bps / 10_000)` in u128 |
| `year_fraction_days(start, end) -> u16` | `(end − start) / 86_400`, must be > 0 |
| `rate_pnl(notional, rate_a, rate_b, days) -> i128` | `notional · (rate_a − rate_b) · days / (10⁶ · 360)`, divide last, truncate toward zero |
| `derive_period(ts, offset_m, tenor_m, cal) -> (i64, i64)` | add months, then Modified Following each date; `cal` is a trait (weekend-only in v1) |

---

# Segment C — Fixing & Settlement

## IX-21 `publish_fixing(period_start: i64, period_end: i64, rate: u64, source_hash: [u8; 32])`

**1. Accounts**
- `operator`: Signer, mut (payer), `market.operator == operator` → `NotOperator` (REQ-52)
- `config`
- `market`
- `fixing`: init, seeds `[b"fixing", market, period_start.to_le_bytes(), period_end.to_le_bytes()]` (duplicate → `FixingExists`, REQ-53)
- `clock`, `system_program`

**2. Additional validation**
- `now >= period_end` → `PeriodNotEnded` (REQ-54)
- `period_end > period_start` → `InvalidParams`
- `0 < rate <= config.rate_ceiling` → `RateOutOfRange` (REQ-55)
No halt/pause check.

**3.** Write `market`, `period_start`, `period_end`, `rate`, `published_at = now`, `source_hash`, `bump`; emit `FixingPublished`.

## IX-22 `settle_position()`

**1. Accounts**
- `payer`: Signer (anyone)
- `market`: mut
- `reserve`: mut, seeds; `reserve_vault`: mut, `== reserve.vault`
- `fixing`: seeds `[b"fixing", market, position.period_start.to_le_bytes(), position.period_end.to_le_bytes()]` → `FixingMismatch` / missing = `FixingMissing` (REQ-61)
- `position`: mut, seeds `[b"position", market, position.trader, position.nonce.to_le_bytes()]`, `has_one = market`
- `trader_margin`: mut, seeds `[b"margin", market, position.trader]`
- `margin_vault`: mut, `== trader_margin.vault`
- `clock`, `token_program`

**2. Additional validation**
1. `position.status == Open` → `NotOpen` (REQ-62)
2. `now >= position.period_end` → `PeriodNotEnded`
No halt, pause or eligibility check (REQ-63, 74).

**3. Logic**
1. `raw = rate_pnl(notional, fixing.rate, position.fixed_rate, day_count_days)`; `pnl = side == PayFixed ? raw : −raw`.
2. `settled = clamp(pnl, −locked_margin, +committed_reserve)` (REQ-58).
3. `settled > 0`: CPI `transfer(reserve_vault → margin_vault, settled)` signed by Reserve PDA; `reserve.balance −= settled`; `trader_margin.balance += settled`. `settled < 0`: the reverse, signed by TraderMargin PDA. `settled == 0`: no transfer (REQ-56/57).
4. `trader_margin.locked −= locked_margin`; `open_positions −= 1` (REQ-59).
5. `reserve.committed −= committed_reserve`.
6. `market.open_interest −= notional` (REQ-60).
7. Assert both mirrors → `MirrorMismatch`.
8. `position.status = Settled`; `settled_amount = settled`; `settled_at = now`; emit `PositionSettled`.

## IX-23 `unwind_position()`

**1. Accounts**
- `payer`: Signer
- `market`: mut
- `reserve`: mut, seeds
- `position`: mut, seeds, `has_one = market`
- `trader_margin`: mut, seeds `[b"margin", market, position.trader]`
- `fixing_address`: UncheckedAccount — must equal the derived Fixing PDA for the position's period → `BadFixingAddress`
- `clock`

**2. Additional validation**
1. `position.status == Open` → `NotOpen`
2. `now >= position.period_end + market.fixing_grace_days · 86_400` → `GraceNotElapsed` (REQ-70)
3. `fixing_address.lamports() == 0 && data_is_empty()` → `FixingExists` (REQ-71)

**3. Logic**
1. `trader_margin.locked −= locked_margin`; `open_positions −= 1`; `reserve.committed −= committed_reserve`; `market.open_interest −= notional`. No token transfer (REQ-69).
2. `position.status = Unwound`; `settled_amount = 0`; `settled_at = now`; emit `PositionUnwound`.

## IX-24 `close_used_nonce()` (optional)

**1.** `trader`: Signer, mut; `position`: `position.trader == trader`; `used_nonce`: mut, close = trader, `used_nonce.position == position`.
**2.** `position.status != Open` → `PositionStillOpen`.
**3.** Close account, rent to trader.

---

## Error enum

```rust
#[error_code]
pub enum FraError {
    NotAuthority, NotOperator, NotTrader, NotEligible,
    OperatorInactive, MarketExists, MarketClosed, MarketHalted, MarketNotHalted, InvalidTransition, ProgramPaused,
    InvalidParams, WrongMint, MirrorMismatch,
    InsufficientWithdrawable, InsufficientFreeMargin, InsufficientReserve,
    QuoteStale, SpreadTooWide, RateOutOfRange, NotionalOutOfBounds, OpenInterestCap,
    MissingSignature, BadSigner, MessageMismatch, OfferExpired, PeriodStarted, BadPeriod, NonceUsed,
    NotOpen, UseSettle, PeriodNotEnded, FixingExists, FixingMissing, FixingMismatch, GraceNotElapsed, BadFixingAddress,
    PositionStillOpen, MathOverflow,
}
```

## Invariants (fuzz suite, Segment C)

After every instruction: `Σ margin balances + reserve.balance + fee_vault == program-held USDC` (REQ-75); `locked <= balance` and `committed <= balance` (76, 77); `locked == Σ locked_margin` and `committed == Σ committed_reserve` and `open_interest == Σ notional` over Open positions (78–80); no position moves backwards or terminates twice (84, 85); no fixing or position terms change after creation (82, 83).
