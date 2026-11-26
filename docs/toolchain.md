# Toolchain

Recorded on 2026-11-26 from `avm install latest && avm use latest`. Bump all of these together.

| Tool | Version | Where it is pinned |
| --- | --- | --- |
| Anchor CLI (`anchor-cli`) | **1.2.2** | `Anchor.toml` `[toolchain] anchor_version`, CI |
| `anchor-lang` / `anchor-spl` crates | **1.2.2** | root `Cargo.toml` `[workspace.dependencies]` |
| Solana CLI (Agave) | **4.1.2** | `Anchor.toml` `[toolchain] solana_version`, CI |
| `cargo-build-sbf` | 4.1.0 (ships with Solana 4.1.2) | — |
| SBF platform-tools | v1.54 (rustc 1.89.0) | — (comes with `cargo-build-sbf`) |
| Host rustc | **1.89.0** | `rust-toolchain.toml`, CI |
| avm | 1.2.2 | — |
| LiteSVM | 0.15.0 | root `Cargo.toml` (matches the Anchor 1.2.2 test template) |

Notes

- The host toolchain is pinned to 1.89.0 to match the SBF platform-tools compiler, as the Anchor 1.2.2 template does. The machine default (`rustc 1.99.0`) is not used inside this repo.
- The workspace uses `resolver = "3"` (MSRV-aware), so cargo picks dependency versions compatible with `rust-version = 1.89.0`. Under resolver 2, adding a dependency pulled `agave-*` / `solana-*` 4.4.1, which need rustc 1.98. Keep `Cargo.lock` committed.
- `anchor-spl` is used with `default-features = false`, so its `solana-v3` cohort feature is named explicitly; it must match `anchor-lang`'s default (`solana-v3`).
- `avm use 1.2.2` also switched the machine's active Solana release from 2.3.0 to 4.1.2.
