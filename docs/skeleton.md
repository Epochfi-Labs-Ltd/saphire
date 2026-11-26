# Saphire skeleton

What exists, how to run it, and where the code departs from the cards in `docs/implementation.md`.

**Scope:** structure only, on the daily rate index design (IX-21 `publish_daily_rate`, `DailyRate` checkpoints, `Market.rate_index`). Each account struct carries its card's account checks (seeds, signer, `has_one`, `constraint`, `init`, token checks). Every handler is `todo!("IX-nn <segment>")`, and the card's Validation and Logic are copied into the handler's doc comment. IX-01 is the one exception: it writes the Config fields so the fixture can start, but its validation is still a TODO. In `fra_math`, the daily-rate index functions are implemented and unit-tested (`next_business_day`, `days_until`, `compound_index`, `period_rate_from_index`, plus `WeekendOnly`); `compute_fixed_rate`, `bps_ceil`, `year_fraction_days`, `rate_pnl` and `derive_period` are still `todo!()`. Tests are named, ignored stubs, one per card case.

## Toolchain

Anchor 1.2.2 · Solana CLI 4.1.2 (Agave) · host rustc 1.89.0 · SBF platform-tools v1.54 (rustc 1.89.0) · LiteSVM 0.15.0. Details and pinning are in [toolchain.md](toolchain.md).

## Running

```sh
anchor build                                  # builds target/deploy/saphire.so (tests load it)
cargo test --workspace                        # what `anchor test` runs ([scripts] test)
cargo test -p saphire-tests -- --ignored      # the stubbed tests, once filled in
cargo fmt --all --check
cargo clippy --workspace -- -D warnings
```

Current results: `anchor build` succeeds (25 instructions, 41 errors, 25 events in the IDL); `cargo test --workspace` passes 36 tests (27 `fra_math` index/calendar, 3 IX-01, 2 fixture self-checks, 4 SDK) with 168 ignored (157 integration stubs, 11 `fra_math` stubs); fmt and clippy are clean, including clippy `--all-targets`. CI (`.github/workflows/ci.yml`) has not been run yet.

## Layout

```
programs/saphire/src/
  lib.rs            declare_id!, #[program] with one fn per instruction -> handle_<name>
  constants.rs      SEED_* (cards + vault seeds), MAX_FEE_BPS, ROLL_TOLERANCE_DAYS, DEAL_DOMAIN, supported-set ids
  errors.rs         FraError, grouped and commented as in the guide
  events.rs         one event per instruction
  state/            one file per account + enums in mod.rs
  instructions/     market/ (A), positions/ (B), settlement/ (C)
crates/fra_math/    no_std, no Solana deps: index.rs (rate index, implemented), date.rs (calendar + print sequencing implemented; derive_period todo), lib.rs (pricing/rounding todo)
crates/saphire-sdk/ pda.rs (one fn per account), ix.rs (one builder per instruction), deal.rs (DealMessage bytes, Ed25519 verify ix)
tests/src/          fixture.rs, ixNN_<name>.rs per card, lifecycle.rs
keeper/             prints "todo"
```

## Instruction card -> file

| IX | Instruction | Seg | Program | Tests |
| --- | --- | --- | --- | --- |
| IX-01 | initialize_config | A | `programs/saphire/src/instructions/market/initialize_config.rs` | `tests/src/ix01_initialize_config.rs` |
| IX-02 | update_config | A | `…/market/update_config.rs` | `tests/src/ix02_update_config.rs` |
| IX-03 | register_operator | A | `…/market/register_operator.rs` | `tests/src/ix03_register_operator.rs` |
| IX-04 | deactivate_operator | A | `…/market/deactivate_operator.rs` | `tests/src/ix04_deactivate_operator.rs` |
| IX-05 | create_market | A | `…/market/create_market.rs` | `tests/src/ix05_create_market.rs` |
| IX-06 | fund_reserve | A | `…/market/fund_reserve.rs` | `tests/src/ix06_fund_reserve.rs` |
| IX-07 | withdraw_reserve | A | `…/market/withdraw_reserve.rs` | `tests/src/ix07_withdraw_reserve.rs` |
| IX-08 | publish_quote | A | `…/market/publish_quote.rs` | `tests/src/ix08_publish_quote.rs` |
| IX-09 | admit_trader | A | `…/market/admit_trader.rs` | `tests/src/ix09_admit_trader.rs` |
| IX-10 | suspend_trader | A | `…/market/suspend_trader.rs` | `tests/src/ix10_suspend_trader.rs` |
| IX-11 | init_trader_margin | B | `…/positions/init_trader_margin.rs` | `tests/src/ix11_init_trader_margin.rs` |
| IX-12 | deposit_margin | B | `…/positions/deposit_margin.rs` | `tests/src/ix12_deposit_margin.rs` |
| IX-13 | halt_market | A | `…/market/halt_market.rs` | `tests/src/ix13_halt_market.rs` |
| IX-14 | resume_market | A | `…/market/resume_market.rs` | `tests/src/ix14_resume_market.rs` |
| IX-15 | pause_program | A | `…/market/pause_program.rs` | `tests/src/ix15_pause_program.rs` |
| IX-16 | resume_program | A | `…/market/resume_program.rs` | `tests/src/ix16_resume_program.rs` |
| IX-17 | withdraw_margin | B | `…/positions/withdraw_margin.rs` | `tests/src/ix17_withdraw_margin.rs` |
| IX-18 | open_position | B | `…/positions/open_position.rs` | `tests/src/ix18_open_position.rs` |
| IX-19 | open_position_signed | B | `…/positions/open_position_signed.rs` | `tests/src/ix19_open_position_signed.rs` |
| IX-20 | close_position | B | `…/positions/close_position.rs` | `tests/src/ix20_close_position.rs` |
| IX-21 | publish_daily_rate | C | `…/settlement/publish_daily_rate.rs` | `tests/src/ix21_publish_daily_rate.rs` |
| IX-22 | settle_position | C | `…/settlement/settle_position.rs` | `tests/src/ix22_settle_position.rs` |
| IX-23 | unwind_position | C | `…/settlement/unwind_position.rs` | `tests/src/ix23_unwind_position.rs` |
| IX-24 | close_used_nonce | B/C | `…/positions/close_used_nonce.rs` | `tests/src/ix24_close_used_nonce.rs` |
| IX-25 | withdraw_fees *(not on the cards yet)* | A | `…/market/withdraw_fees.rs` | `tests/src/ix25_withdraw_fees.rs` |

`…` = `programs/saphire/src/instructions`. IX-24 (B/C) is in `positions/` because it closes a Segment B account.

## Card account rules not expressible as Anchor constraints (handler-level TODO)

On each of these paths Anchor fails before any custom check can run, so the card's error comes back as a framework or system error instead. The stub tests note which error currently surfaces. To return the card's error, the handler has to take the account as `UncheckedAccount` and check it by hand.

| Card | Card error | What surfaces today |
| --- | --- | --- |
| IX-05 | `OperatorInactive` for a never-registered operator | `AccountNotInitialized` (registration PDA missing) |
| IX-05 | `MarketExists` | system `AccountAlreadyInUse` (custom 0) from `init` |
| IX-11, 12, 18, 19 | `NotEligible` for a never-admitted trader | `AccountNotInitialized` |
| IX-19 | `NonceUsed` | system `AccountAlreadyInUse` from `init` of `used_nonce` |
| IX-21 | duplicate date (the card notes it "also fails at init") | system `AccountAlreadyInUse` from `init` of `daily_rate` |
| IX-22 | `RateCheckpointMissing` (either checkpoint absent) | `AccountNotInitialized` |
| IX-22 | checkpoint for the wrong date (card: "seeds enforce the match") | Anchor `ConstraintSeeds` |
| IX-06, 07, 12, 17, 18, 19, 20, 22 | vault `== reserve.vault` / `== trader_margin.vault` / `== config.fee_vault` (card names no error) | Anchor `ConstraintAddress` |

These ones are expressible, and the card's error is wired in now: every signer check (`has_one … @ NotAuthority / NotOperator / NotTrader`), the `active` checks (`@ OperatorInactive`, `@ NotEligible`), `WrongMint` on token-account mint/owner, and `BadRateCheckpointAddress` (IX-23, a `constraint` against `DailyRate::pda`). Every **Validation** line on the cards is left to the handler, as the cards themselves put it there.

## Departures from the cards

- **`settlement_mint` account added** to IX-06, 07, 12, 17, 18, 19, 20, 22. `transfer_checked` needs the mint, and plain `transfer` is unsafe for Token-2022. In IX-17 it is checked against `margin_vault.mint`, because that card forbids reading `market`.
- **Vault PDAs.** The cards give no vault seeds, so the vaults use: `fee_vault = [b"fee_vault"]`, `reserve_vault = [b"reserve_vault", market]`, `margin_vault = [b"margin_vault", market, trader]`.
- **`has_one = market` on `reserve`** in IX-18, 19, 20, 22 and 23. The cards say only "reserve W"; without this, a foreign reserve could be committed.
- **IX-20 account order.** `position` comes second, so a foreign position fails with `NotTrader` rather than a seeds error on `trader_margin`. The position seeds use `position.trader`, which is equivalent once `has_one = trader` passes.
- **`OperatorRegistration`** (`state/operator_registration.rs`) is added. It is missing from the brief's file list but on the account cards.
- **Event names I chose** (the cards give none): `TraderMarginInitialized` (IX-11), `ProgramPaused` / `ProgramResumed` (IX-15/16), `UsedNonceClosed` (IX-24).
- **`MarketParams.index_start_date`** is appended as the last field. `Market.rate_index` and `last_rate_date` follow the card's field order.
- **`compound_index`** computes the growth term as `rate × days_applied × 10^12 / 360`. That is the same rational as the card's `rate × days_applied × 10^18 / (10^6 × 360)`, so it truncates identically, and it cannot overflow `u128`. The 256-bit `index × factor` product uses a hand-written two-limb multiply plus long division (`fra_math::index::mul_div`), so there is no `ruint` dependency.
- **`next_business_day` and `days_until` return `Result`**, like the rest of `fra_math`; the card lists bare `i64` / `u8`. `next_business_day` gives up after 31 non-business days rather than loop forever.
- **Placeholder constants:** `MAX_FEE_BPS = 100` and `ROLL_TOLERANCE_DAYS = 3`. The docs name both but give no value. `BENCHMARK_SOFR = 0`, `DAY_COUNT_ACT_360 = 360` and `BDC_MODIFIED_FOLLOWING = 0` are the "supported set" ids for IX-05.
- **`clock` sysvar accounts** are kept wherever a card lists them, even though `Clock::get()` would do.
- **IX-19** has the detached-signature design (instructions sysvar + `UsedNonce`); neither design is implemented. The co-signer alternative is in a `DECISION PENDING` header comment, which is repeated on `UsedNonce` and IX-24.
- **SDK `Instruction`** is `solana_instruction::Instruction`, the type `solana_sdk::instruction::Instruction` re-exports. Depending on all of `solana-sdk` was not needed.
- **Integration tests** are one LiteSVM test binary with one module per card, instead of one binary per file, to avoid linking LiteSVM 25 times. The fixture writes SPL mint and token-account state directly, so it works for SPL Token and Token-2022 alike (IX-01 has a Token-2022 test). `litesvm-token` is not used: its only cached release (0.16) would add a second LiteSVM version.

## Fee additions (not on the cards yet)

Added to the skeleton on request. `docs/implementation.md` does not describe them yet, so the segment owners need to write the cards. Everything below is an account struct, a field, an error, an event or a test stub; every handler is still `todo!()`. Each touched file marks the addition with "not on the cards yet".

**Settlement fee (crank bounty).** A flat per-position fee that pays whoever cranks settle or unwind, so "anyone can settle" pays for itself.

| Where | Change |
| --- | --- |
| `Market.settlement_fee: u64` | Flat amount in settlement-mint base units, from `MarketParams.settlement_fee` (appended last). 0 means no bounty. |
| `Position.settlement_fee: u64` | Copied from the market at open, so later changes don't affect open positions. |
| IX-18 / IX-19 (docs only) | Free-margin check becomes `free >= lock + fee + settlement_fee`; `locked += lock + settlement_fee`. No transfer at open. |
| IX-20 (docs only) | Early close releases `settlement_fee` from `locked` back to the trader. No transfer. |
| IX-22 | New account `payer_token_account` (mint = settlement mint, owner = payer, `@ WrongMint`). Logic: `margin_vault → payer_token_account`, then release from `locked`. |
| IX-23 | New accounts `margin_vault`, `payer_token_account`, `settlement_mint`, `token_program` for the same payout. Unwind still moves no settlement PnL (REQ-69), but it now moves the bounty. |
| Events | `PositionOpened.settlement_fee`; `PositionSettled` / `PositionUnwound` gain `cranker` and `settlement_fee`. |
| Invariant | REQ-78 becomes `locked == Σ (locked_margin + settlement_fee)` over Open positions. |

**IX-25 `withdraw_fees(amount)` · admin · pause-ok.** Without it, protocol fees are stuck in `fee_vault`. Accounts: `authority` (signer, `has_one` on Config `@ NotAuthority`), `config` (`has_one = fee_vault`), `fee_vault` (mut), `destination` (mut, mint = fee-vault mint `@ WrongMint`, any owner), `settlement_mint`, `token_program`. Validation: `amount > 0` (`InvalidParams`) and `amount <= fee_vault.amount` (new `InsufficientFees`). Event: `FeesWithdrawn`.

**Choices the owners should confirm before carding:**
1. **Operator sets the settlement fee, per market.** Market-level because the fee is in the market's mint. The alternative is admin-set, which would need a per-mint Config field or a cap such as a `MAX_SETTLEMENT_FEE`.
2. **Bounty paid from trader margin.** Locked at open, so the trader funds the crank for their own position. Alternatives: the reserve pays, or the fee is split.
3. **Errors appended as a new last group ("Fees").** This keeps every existing code stable; the card list says codes follow enum order, so the doc's order should match.
4. **Unwind pays the bounty.** Arguably the cranker shouldn't be paid when the operator failed to publish the period_end print; the alternative is to release the fee to the trader on unwind.

## Spec questions surfaced while scaffolding

1. **IX-01 can be front-run.** Any signer can call `initialize_config` first and become `Config.authority`. Consider requiring the program's upgrade authority (`program_data`).
2. **The mirror assert is a DoS vector.** `balance == vault.amount` after every transfer means anyone who sends tokens straight to a reserve or margin vault makes every later fund, withdraw, settle and close fail with `MirrorMismatch`. Consider `>=`, or sweeping the surplus.
3. **`MarketStatus::Closed` is never set** by any instruction, yet IX-08 checks for it.
4. **One `fee_vault` / mint per program.** IX-01 fixes a single fee-vault mint, while markets are keyed by `settlement_mint`. A market on a different mint could not pay fees.
