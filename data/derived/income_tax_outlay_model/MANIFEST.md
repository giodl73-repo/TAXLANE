# Income-Tax Outlay Model Artifact Manifest

## Purpose

This manifest records the artifact chain for the modeled allocation of
ordinary individual income-tax receipts by broad OMB outlay share.

The annual and decade JSONL files are canonical model outputs. CSV files,
Markdown notes, and chart specs are derived or supporting views.

## Model

- Model ID: `individual-income-tax-proportional-outlays-v1`
- Coverage: fiscal years 1940-2025 for annual actual-year rows
- Projection treatment: FY2026-FY2031 excluded
- Legal status: modeled allocation, not legal dedication

## Artifacts

| Path | Role | Grain | Rows | Canonical | SHA-256 |
|---|---|---|---:|---|---|
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl` | Canonical annual model rows | fiscal year by broad category | 516 | yes | `c50c2fdf4c5eab5e2903d8bbc12e581cb2923d3a065a77f6c790f3c7bf397f59` |
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-summary.jsonl` | Canonical decade summary rows | decade by broad category | 54 | yes | `36a28934e3bcac104e84b5389b3a47fe97657f066a8359707c00ee6521ef0d60` |
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv` | Chart-ready annual wide view | fiscal year | 86 | no | `a385e4215a20c16f0344cc8e17982c9e4c78933caac93f274ea5b6080027d81a` |
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-wide.csv` | Chart-ready decade wide view | decade | 9 | no | `5e243eb2912b7aaed26e22016fd1907bf4cee25d60b3855f1c2627e7f4b9a2aa` |
| `data/derived/income_tax_outlay_model/README.md` | Model method and schema note | documentation | n/a | supporting | `f20e3e7ddc97da3835a604988ee54f0c7b62445739e373815a1f51707b7f3d36` |
| `data/derived/income_tax_outlay_model/source-profile.md` | Source coverage and reconciliation sample | documentation | n/a | supporting | `7d0dff2c0c671e4e1bbc80aee6a72a6ad09890758e1d8e7c92999a6d093456a2` |
| `data/derived/income_tax_outlay_model/reconciliation-review.md` | Generated-row reconciliation review | documentation | n/a | supporting | `9c1898df3c6d7498a285887c8a64665d320afb1d492e8cc4b69bb27d22b30be8` |
| `data/derived/income_tax_outlay_model/decade-summary.md` | Human-readable decade summary | documentation | n/a | supporting | `8ee633a570b3974a77463abcd56f33642affd7944f0173562ce7913ff9a389da` |
| `docs/reading/modeled-income-tax-outlays.md` | Reader-facing packet | documentation | n/a | supporting | `26f5c340e1ffadc4efa716bbce33772c4963df54635982bd550a3392aae0579b` |
| `docs/charts/README.md` | Chart catalog | documentation | n/a | supporting | `50ca7d9d36660ef5d98061766e647c84bd7ce3731bb6f6df97a48af578926eda` |
| `docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json` | Annual allocation chart spec | visualization spec | n/a | view | `e0f7c39a6d4f392fc3d60f5f9f6c0561f1bc7706f2dab6019158945b66ca68b6` |
| `docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json` | Decade allocation chart spec | visualization spec | n/a | view | `e53433c4304830465e8633b4d5e6b88c8f1e54f517785916ff8191540c1bcb5b` |
| `docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json` | Annual financing context chart spec | visualization spec | n/a | view | `0f73917e9e372abce44ff1d1050b0a71d6358bb4de28b5538838e4d87a590011` |
| `docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json` | Decade financing context chart spec | visualization spec | n/a | view | `a3b513e30738ef3070d0e2f1fcc3c2e925d3dc0dc3f0381a8827a5cdd1a781ff` |
| `Cargo.toml` | Rust workspace manifest | tooling | n/a | supporting | `9ebc5854ae2e4979a5e9b86f65a60cb29c5a88f587877b126ed9cbd8457532cb` |
| `Cargo.lock` | Rust dependency lockfile | tooling | n/a | supporting | `98b37b03e7430d5538eb8c663022926d896f9b31194915f43b654556fb16716f` |
| `tools/taxlane/Cargo.toml` | Rust Taxlane tools crate manifest | tooling | n/a | supporting | `44e924bb7e71d35948fa75dcc0583ef58203a0e0f52180cf03072ef64aa67c33` |
| `tools/taxlane/src/main.rs` | Rust validation and manifest command implementation | script | n/a | supporting | `c72dc121a2984a257360d234b9ba210412cafcf6153f3ad18bf9c2897d2e2cc6` |

## Regeneration Order

1. `cargo run -p taxlane-tools -- income-tax-outlay model`
2. `cargo run -p taxlane-tools -- income-tax-outlay summary`
3. `cargo run -p taxlane-tools -- income-tax-outlay export`
4. `cargo run -p taxlane-tools -- income-tax-outlay manifest`

Run validation after regeneration:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
```
