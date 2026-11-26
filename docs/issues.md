# GitHub issues — initial backlog

Machine-readable backlog for `gh issue create`. Create the labels first, then the issues in order; `Blocked by` refers to issue keys in this file (resolve to numbers as they are created). Each issue block is:

```
### <key> · <title>
labels: ...
assignee: ...        (optional; GitHub handles to be filled in)
blocked by: ...      (optional)
---
<body, markdown>
```

Spec references point into `docs/implementation.md` (IX-nn cards, account cards) and `README.md`.

## Labels

| Label | Colour | Description |
| --- | --- | --- |
| `segment:A` | `#1a9c5b` | Market & Reserve (Ritam) |
| `segment:B` | `#2f6fdb` | Positions (Liam) |
| `segment:C` | `#e8702a` | Daily Rates & Settlement (Greg) |
| `segment:shared` | `#6b7280` | Touches the shared state contract or more than one segment |
| `type:feature` | `#0e8a16` | Instruction or account implementation |
| `type:test` | `#5319e7` | Tests, fixtures, fuzz |
| `type:decision` | `#fbca04` | Needs a team decision before code |
| `type:docs` | `#c5def5` | Documentation |
| `type:infra` | `#bfd4f2` | Toolchain, CI, keeper, SDK |
| `priority:p0` | `#b60205` | Blocks others; this week |
| `priority:p1` | `#d93f0b` | Needed for the demo |
| `priority:p2` | `#fef2c0` | After the demo |

```
gh label create "segment:A" --color 1a9c5b --description "Market & Reserve (Ritam)"
gh label create "segment:B" --color 2f6fdb --description "Positions (Liam)"
gh label create "segment:C" --color e8702a --description "Daily Rates & Settlement (Greg)"
gh label create "segment:shared" --color 6b7280 --description "Shared state contract / multi-segment"
gh label create "type:feature" --color 0e8a16
gh label create "type:test" --color 5319e7
gh label create "type:decision" --color fbca04
gh label create "type:docs" --color c5def5
gh label create "type:infra" --color bfd4f2
gh label create "priority:p0" --color b60205
gh label create "priority:p1" --color d93f0b
gh label create "priority:p2" --color fef2c0
```

Milestones: `M1 shared contract` · `M2 segments on fixture` · `M3 lifecycle demo`.

---

## Decisions (create first; p0)

### D1 · Decision: co-signer vs detached signature for open_position_signed
labels: type:decision, segment:B, segment:shared, priority:p0
---
Greg's review (24 Nov): parsing an Ed25519 instruction via the sysvar is error-prone. Proposal: operator becomes a co-signer of the trader's open transaction; runtime verifies both; `UsedNonce` is dropped; expiry via blockhash/durable nonce plus `now < period_start`. Fallback: operator posts the deal on-chain, trader accepts in a second tx.
- [ ] Decide
- [ ] Update IX-19 card, account cards, error group "Signed deal", README
- [ ] If co-signer: remove `UsedNonce`, IX-24, `NonceUsed`/`MissingSignature`/`BadSigner`/`MessageMismatch`

### D2 · Decision: index_start_date anchor for the daily rate index
labels: type:decision, segment:C, segment:A, priority:p0
---
`publish_daily_rate` requires `date == next business day after Market.last_rate_date`. Anchor options: (a) `MarketParams.index_start_date` set at `create_market` (current doc); (b) first print sets it. (a) is stricter; (b) is simpler but lets the operator start anywhere.

### D3 · Decision: test clock harness
labels: type:decision, segment:shared, type:test, priority:p0
---
Period dates are months out; every B/C test needs warp. Pick LiteSVM (`warp_to_slot` / clock sysvar set) vs solana-program-test. Rust-only tests. Decide once; fixture depends on it.

### D4 · Decision: settlement mint — SPL Token vs Token-2022 on devnet
labels: type:decision, segment:A, segment:shared, priority:p0
---
Program uses `anchor_spl::token_interface` so both work; the fixture and devnet scripts need one. Affects every vault.

### D5 · Decision: gaps found in review (G1–G7)
labels: type:decision, segment:A, segment:B, priority:p1
---
- [ ] G1 `withdraw_fees` (admin) or fees locked in v1
- [ ] G2 `close_market` (operator, OI = 0) or drop `MarketStatus::Closed`
- [ ] G3 drop `Market.keeper` or keep as roadmap field
- [ ] G4 market risk params immutable vs `update_market`
- [ ] G5 keep or cut IX-02 `update_config`, IX-24 `close_used_nonce`
- [ ] G6 Position accounts never closed (rent) — accept for v1
- [ ] G7 early-close fee → reserve, protocol fee → fee vault (confirm)

### D6 · Decision: conflicts between segment docs (X1–X7)
labels: type:decision, segment:shared, priority:p1
---
- [ ] X2 rounding: one rule in `fra_math` (ceil locks/fees, truncate PnL)
- [ ] X3 quote epoch program-advanced; reword REQ-14
- [ ] X4 `publish_quote` allowed while Halted
- [ ] X5 pause blocks IX-18/19 only
- [ ] X6 Part A signer lines → operator (Ritam updates his guide)
- [ ] X7 early-close fee destination = reserve
(X1 fixing key is superseded by the daily rate index.)

### D7 · Decision: position NFTs — out of v1
labels: type:decision, segment:B, priority:p2
---
Record decision 14: not in scope; if later, non-transferable Metaplex Core receipt pointing at the Position PDA. Transferable positions break `TraderMargin` ownership and the whitelist.

---

## Shared / infra (M1)

### S1 · Freeze and tag the shared state contract
labels: segment:shared, type:feature, priority:p0
blocked by: D1, D2
---
Account structs, seeds, enums, error enum, units, rounding exactly as `docs/implementation.md` account cards. Tag `contract-v1`. Nobody writes an instruction handler before this tag.
- [ ] `state/*` match the cards field for field
- [ ] `errors.rs` grouped as in the doc
- [ ] `constants.rs` seeds
- [ ] All three owners sign off

### S2 · fra_math crate — pricing, dates, index, rounding
labels: segment:B, segment:shared, type:feature, priority:p0
---
`compute_fixed_rate`, `bps_ceil`, `year_fraction_days`, `rate_pnl`, `derive_period` (+ `Calendar` trait, `WeekendOnly`), `next_business_day`, `days_until`, `compound_index`, `period_rate_from_index`. No Solana deps, no floats, 256-bit intermediate for the index.
- [ ] Unit tests for every worked example in the cards (K 40700; 568_611_111; month-end roll; 89-day flat-rate round trip)
- [ ] proptest: index monotone; window rate within observed min/max

### S3 · Rust test fixture on LiteSVM
labels: segment:A, segment:shared, type:test, priority:p0
blocked by: D3, S1
---
`setup_fra_market`, `fund_test_reserve`, `publish_test_quote`, `admit_test_trader`, `setup_eligible_trader_with_margin`, `publish_rates_through(date, rate)`, `warp_to(ts)`. B builds on it from day 2.

### S4 · saphire-sdk crate — PDA helpers and instruction builders
labels: segment:shared, type:infra, priority:p1
blocked by: S1
---
One PDA fn per account; one builder per instruction; `publish_daily_rates_range`; Borsh `DealMessage` (until D1 resolves).

### S5 · CI: build, fmt, clippy, tests with pinned toolchain
labels: type:infra, segment:shared, priority:p1
---
`.github/workflows/ci.yml`; versions from `docs/toolchain.md`; `cargo test --workspace` only (no TS).

### S6 · Lifecycle E2E test (README "What we demo")
labels: segment:shared, type:test, priority:p1
blocked by: A5, B4, B5, B6, C1, C2, C3
---
config → register → market → fund 600k → quote → admit → deposit 50k → open at quote → open signed → close early → daily rates through period_end at 4.30% → settle → unwind a third position → withdraw all; invariants after every step.

### S7 · Fuzz suite for REQ-75–86
labels: segment:C, type:test, priority:p1
blocked by: S3
---
Random sequences of fund, withdraw, quote, deposit, open, signed open, daily rate, settle, close, unwind, halt, pause; assert all twelve rules after each action. Start against whatever exists in week 1.

---

## Segment A — Market & Reserve (M2)

### A1 · IX-01/02 initialize_config, update_config
labels: segment:A, type:feature, priority:p0
blocked by: S1
---
Cards IX-01, IX-02. Fee vault owned by Config PDA. Tests: happy, `InvalidParams`, duplicate init, `NotAuthority`.

### A2 · IX-03/04 register_operator, deactivate_operator
labels: segment:A, type:feature, priority:p0
blocked by: S1
---
Tests: `NotAuthority`, duplicate registration, deactivate twice, deactivated operator cannot create a market (A3).

### A3 · IX-05 create_market
labels: segment:A, type:feature, priority:p0
blocked by: A2, D2, D4
---
Operator-signed; `rate_index = 10^18`, `last_rate_date` anchor. Tests: `OperatorInactive`, `MarketExists`, each `InvalidParams` case (REQ-04, 05).

### A4 · IX-06/07 fund_reserve, withdraw_reserve
labels: segment:A, type:feature, priority:p0
blocked by: A3
---
Mirror assertion; `InsufficientWithdrawable` with committed > 0 (needs a hand-set `committed` in test). Tests: wrong mint, wrong owner, zero amount, `NotOperator`.

### A5 · IX-08 publish_quote
labels: segment:A, type:feature, priority:p0
blocked by: A3, D6
---
Program-advanced epoch; allowed while Halted. Tests: `RateOutOfRange`, `SpreadTooWide`, size bounds, `MarketClosed`, `NotOperator`.

### A6 · IX-09/10 admit_trader, suspend_trader
labels: segment:A, type:feature, priority:p1
blocked by: A3
---
Operator-signed (decision 1). Re-admit reactivates. Tests: `NotOperator`, suspend leaves margin and positions untouched (with B4).

### A7 · IX-13/14/15/16 halt, resume, pause, resume
labels: segment:A, type:feature, priority:p1
blocked by: A3
---
Tests: transitions, `InvalidTransition`, `MarketNotHalted`, `NotAuthority`; with B/C: opens refused, settle/unwind/withdraw still allowed.

### A8 · Modified Following date helper interface (WeekendOnly v1)
labels: segment:A, segment:B, type:feature, priority:p1
blocked by: S2
---
`Calendar` trait impl and the cases table in the guide (A9); labelled simplification; SIFMA calendar on roadmap.

### A9 · Gap items if accepted: withdraw_fees, close_market, update_market
labels: segment:A, type:feature, priority:p2
blocked by: D5
---
One sub-task per accepted item from D5, each with its card added to `docs/implementation.md`.

---

## Segment B — Positions (M2)

### B1 · IX-11 init_trader_margin
labels: segment:B, type:feature, priority:p0
blocked by: S1, S3
---
Margin vault authority = TraderMargin PDA. Tests: `NotEligible`, duplicate, wrong mint.

### B2 · IX-12 deposit_margin
labels: segment:B, type:feature, priority:p0
blocked by: B1
---
Tests: `NotEligible`, zero, wrong mint/owner, mirror; allowed while halted and paused.

### B3 · IX-17 withdraw_margin
labels: segment:B, type:feature, priority:p0
blocked by: B2
---
No eligibility/halt/pause check. Tests: `NotTrader`, `InsufficientFreeMargin` with locked > 0, suspended trader can withdraw.

### B4 · IX-18 open_position (program-priced)
labels: segment:B, type:feature, priority:p0
blocked by: B2, A5, S2
---
Refusal order as the card; worked example must reproduce exactly (K 40700, lock/commit 20,000, fee 100, OI 1,000,000). Tests: every row of the card's error list plus month-end roll.

### B5 · IX-19 open_position_signed (RFQ)
labels: segment:B, type:feature, priority:p1
blocked by: B4, D1
---
Implement whichever D1 chose. Tests: happy, expired, after period_start, bad period, replay (nonce or tx), wrong operator, non-operator signer.

### B6 · IX-20 close_position (early)
labels: segment:B, type:feature, priority:p1
blocked by: B4
---
Opposite-side quote, cap, fee to reserve, both transfer directions. Worked example: net −104.44. Tests: `NotTrader`, `NotOpen`, `UseSettle`, `QuoteStale`; allowed while halted.

### B7 · Fixture helpers for C: open_test_position, open_signed_test_position
labels: segment:B, type:test, priority:p1
blocked by: B4
---
Returns the Position address; C builds settle tests on it from end of week 1.

### B8 · Operator signing bot (TS or Rust) for RFQ deals
labels: segment:B, type:infra, priority:p2
blocked by: D1
---
Signs the Borsh deal (or co-signs the tx); refuses nonce reuse; short expiry. Only what the demo needs.

---

## Segment C — Daily Rates & Settlement (M2)

### C1 · IX-21 publish_daily_rate + Market.rate_index
labels: segment:C, type:feature, priority:p0
blocked by: S1, S2, D2
---
Sequential prints, `days_applied` check, `compound_index`. Tests: happy single day; Friday → Monday (`days_applied` 3); gap → `RateDateNotNext`; duplicate; `RateDateInFuture`; `RateOutOfRange`; `NotOperator`; several days in one tx.

### C2 · IX-22 settle_position
labels: segment:C, type:feature, priority:p0
blocked by: C1, B7
---
Two checkpoint accounts; `L` from `period_rate_from_index`; cap; both transfer directions; zero case. Worked example: 89 days of 43000 → 568.61 to the base unit (document the exact expected value from the index path). Tests: `NotOpen`, `PeriodNotEnded`, `RateCheckpointMissing` (start / end), halted and paused still settle, suspended trader still settled.

### C3 · IX-23 unwind_position
labels: segment:C, type:feature, priority:p1
blocked by: C1, B7
---
Grace boundary, `RateCheckpointExists`, `BadRateCheckpointAddress`; no token transfer; late print after unwind does not reopen.

### C4 · Keeper cron: settle/unwind
labels: segment:C, type:infra, priority:p1
blocked by: C2, C3
---
Lists Open positions past `period_end` with a `period_end` checkpoint → settle; past grace with none → unwind; batches 4–5 per tx; retries on blockhash expiry. Runs on devnet for the demo.

### C5 · Daily SOFR print script
labels: segment:C, type:infra, priority:p1
blocked by: C1
---
Reads the NY Fed publication, computes `days_applied`, submits `publish_daily_rate`; catch-up mode submits missing days in order. Fixture mode uses a constant rate.

### C6 · Settlement worked examples as tests (cap table)
labels: segment:C, type:test, priority:p1
blocked by: C2
---
The seven-row cap table from the guide (rose a little / exactly K / collapsed / beyond cap both sides / unwound) as individual tests.

---

## Docs

### X1 · Part A guide: update signer lines to operator
labels: type:docs, segment:A, priority:p1
---
`create_market`, `admit_trader`, `suspend_trader`, `halt_market`, `resume_market` are operator-signed (decision 1). Ritam updates his guide and flowcharts.

### X2 · docs/testing.md — fixture API and lifecycle steps
labels: type:docs, segment:shared, priority:p1
blocked by: S3, D3

### X3 · Requirements v2: reword REQ-14 (epoch), REQ-51–55 (daily prints), remove FloatFixing references
labels: type:docs, segment:shared, priority:p1

### X4 · docs/toolchain.md and docs/skeleton.md kept current
labels: type:docs, type:infra, priority:p2
