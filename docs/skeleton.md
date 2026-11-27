# Saphire skeleton

A minimal skeleton: the workspace, the toolchain, the account structs and one file per instruction. Nothing is implemented. Each segment owner fills in their instruction files from the cards in `docs/implementation.md`.

**Scope:**
- **Account structs** (`programs/saphire/src/state/`) carry the fields from the account cards. They have no methods.
- **Instruction files** have a header comment (IX number, signature, REQs) and an `Accounts` context with only a `signer`. Each handler returns `Ok(())`. The instructions are wired into `lib.rs` with their card arguments, so the IDL lists them all.
- **Not included yet:** real account contexts (seeds, `has_one`, token constraints), errors, events, constants/seeds and handler logic. Each owner adds these with their instructions.
- **`fra_math`** has its module layout only (`date`, `index`).
- **Tests:** a minimal LiteSVM fixture, one smoke test that loads the program and sends an instruction, and named `#[ignore]`d stubs, one per card case.

## Toolchain

Anchor 1.2.2 · Solana CLI 4.1.2 (Agave) · host rustc 1.89.0 · SBF platform-tools v1.54 (rustc 1.89.0) · LiteSVM 0.15.0. Details and pinning are in [toolchain.md](toolchain.md).

## Running

```sh
anchor build                                         # builds target/deploy/saphire.so (tests load it)
cargo test --workspace                               # what `anchor test` runs ([scripts] test)
cargo test -p saphire-tests --test unit              # one instruction per test
cargo test -p saphire-tests --test unit market::     # one segment (market / positions / settlement)
cargo test -p saphire-tests --test integration       # instructions combined: flows + lifecycle E2E
cargo test -p saphire-tests -- --ignored             # the stubbed tests, once filled in
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

## Layout

```
programs/saphire/src/
  lib.rs            declare_id!, #[program] with one fn per instruction -> handle_<name>
  state/            one file per account + shared enums in mod.rs
  instructions/     market/ (A), positions/ (B), settlement/ (C); signer-only contexts, Ok(()) handlers
crates/fra_math/    no_std, no Solana deps: date.rs, index.rs (empty)
tests/src/          fixture.rs (load program, send transactions)
tests/tests/unit/          smoke.rs + market/, positions/, settlement/ (one <name>.rs per instruction, stubs)
tests/tests/integration/   quote_trading.rs, rfq.rs, settlement.rs, controls.rs, reserve_and_fees.rs, lifecycle.rs (stubs)
keeper/             prints "todo"
```

## Instruction card -> file

| IX | Instruction | Seg | Program | Tests |
| --- | --- | --- | --- | --- |
| IX-01 | initialize_config | A | `programs/saphire/src/instructions/market/initialize_config.rs` | `tests/tests/unit/market/initialize_config.rs` |
| IX-02 | update_config | A | `…/market/update_config.rs` | `tests/tests/unit/market/update_config.rs` |
| IX-03 | register_operator | A | `…/market/register_operator.rs` | `tests/tests/unit/market/register_operator.rs` |
| IX-04 | deactivate_operator | A | `…/market/deactivate_operator.rs` | `tests/tests/unit/market/deactivate_operator.rs` |
| IX-05 | create_market | A | `…/market/create_market.rs` | `tests/tests/unit/market/create_market.rs` |
| IX-06 | fund_reserve | A | `…/market/fund_reserve.rs` | `tests/tests/unit/market/fund_reserve.rs` |
| IX-07 | withdraw_reserve | A | `…/market/withdraw_reserve.rs` | `tests/tests/unit/market/withdraw_reserve.rs` |
| IX-08 | publish_quote | A | `…/market/publish_quote.rs` | `tests/tests/unit/market/publish_quote.rs` |
| IX-09 | admit_trader | A | `…/market/admit_trader.rs` | `tests/tests/unit/market/admit_trader.rs` |
| IX-10 | suspend_trader | A | `…/market/suspend_trader.rs` | `tests/tests/unit/market/suspend_trader.rs` |
| IX-11 | init_trader_margin | B | `…/positions/init_trader_margin.rs` | `tests/tests/unit/positions/init_trader_margin.rs` |
| IX-12 | deposit_margin | B | `…/positions/deposit_margin.rs` | `tests/tests/unit/positions/deposit_margin.rs` |
| IX-13 | halt_market | A | `…/market/halt_market.rs` | `tests/tests/unit/market/halt_market.rs` |
| IX-14 | resume_market | A | `…/market/resume_market.rs` | `tests/tests/unit/market/resume_market.rs` |
| IX-15 | pause_program | A | `…/market/pause_program.rs` | `tests/tests/unit/market/pause_program.rs` |
| IX-16 | resume_program | A | `…/market/resume_program.rs` | `tests/tests/unit/market/resume_program.rs` |
| IX-17 | withdraw_margin | B | `…/positions/withdraw_margin.rs` | `tests/tests/unit/positions/withdraw_margin.rs` |
| IX-18 | open_position | B | `…/positions/open_position.rs` | `tests/tests/unit/positions/open_position.rs` |
| IX-19 | open_position_signed | B | `…/positions/open_position_signed.rs` | `tests/tests/unit/positions/open_position_signed.rs` |
| IX-20 | close_position | B | `…/positions/close_position.rs` | `tests/tests/unit/positions/close_position.rs` |
| IX-21 | publish_daily_rate | C | `…/settlement/publish_daily_rate.rs` | `tests/tests/unit/settlement/publish_daily_rate.rs` |
| IX-22 | settle_position | C | `…/settlement/settle_position.rs` | `tests/tests/unit/settlement/settle_position.rs` |
| IX-23 | unwind_position | C | `…/settlement/unwind_position.rs` | `tests/tests/unit/settlement/unwind_position.rs` |
| IX-24 | close_used_nonce | B/C | `…/positions/close_used_nonce.rs` | `tests/tests/unit/positions/close_used_nonce.rs` |

`…` = `programs/saphire/src/instructions`. IX-24 (B/C) is in `positions/` because it closes a Segment B account.

## Spec questions surfaced while scaffolding

1. **IX-01 can be front-run.** Any signer can call `initialize_config` first and become `Config.authority`. Consider requiring the program's upgrade authority (`program_data`).
2. **The mirror assert is a DoS vector.** `balance == vault.amount` after every transfer means anyone who sends tokens straight to a reserve or margin vault makes every later fund, withdraw, settle and close fail with `MirrorMismatch`. Consider `>=`, or sweeping the surplus.
3. **`MarketStatus::Closed` is never set** by any instruction, yet IX-08 checks for it.
4. **One `fee_vault` / mint per program.** IX-01 fixes a single fee-vault mint, while markets are keyed by `settlement_mint`. A market on a different mint could not pay fees.
