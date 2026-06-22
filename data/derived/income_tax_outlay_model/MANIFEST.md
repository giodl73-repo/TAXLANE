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
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl` | Canonical annual model rows | fiscal year by broad category | 516 | yes | `01c0fec63836f3652fdd83b03a608159ad58f7614fc214ae6691421990f3a85e` |
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-summary.jsonl` | Canonical decade summary rows | decade by broad category | 54 | yes | `088cb5d55cc6a37ab52dd543d8a92bf09908cda265e9c9f93324b840eb652fb3` |
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv` | Chart-ready annual wide view | fiscal year | 86 | no | `a385e4215a20c16f0344cc8e17982c9e4c78933caac93f274ea5b6080027d81a` |
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-wide.csv` | Chart-ready decade wide view | decade | 9 | no | `5e243eb2912b7aaed26e22016fd1907bf4cee25d60b3855f1c2627e7f4b9a2aa` |
| `data/derived/income_tax_outlay_model/README.md` | Model method and schema note | documentation | n/a | supporting | `118b94fb3b97cd5d6fa16d516d8601e8f59198e7793545e4418ea07be0a063ad` |
| `data/derived/income_tax_outlay_model/source-profile.md` | Source coverage and reconciliation sample | documentation | n/a | supporting | `03ea7eee098ee6e1f3cc4a83cd89adec009f9cbb96d050f2e7118e06dcf2e70f` |
| `data/derived/income_tax_outlay_model/reconciliation-review.md` | Generated-row reconciliation review | documentation | n/a | supporting | `ab200dfc8c467700964cb23d51875411603a73f9d1ea62e1beea9073449a9c8d` |
| `data/derived/income_tax_outlay_model/decade-summary.md` | Human-readable decade summary | documentation | n/a | supporting | `47d02425c28cd8f8fd0c236b46e0c997d54a0824ddca1cb125ca0a39bcfd915f` |
| `docs/reading/modeled-income-tax-outlays.md` | Reader-facing packet | documentation | n/a | supporting | `b7cdffb161572ff8e7c862747ad4172ef7a59deaa780373c6331f474d68515f7` |
| `docs/charts/README.md` | Chart catalog | documentation | n/a | supporting | `243edecd34c1f7feae2ab4269b8979a5dcd8935c10926d4a68c2f8100c437a9b` |
| `docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json` | Annual allocation chart spec | visualization spec | n/a | view | `dab4f7d15be91c7357c8595d14e8d58336048747bc7a3825cee756072dc05b54` |
| `docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json` | Decade allocation chart spec | visualization spec | n/a | view | `4e1bbc8b41ce38147477d7494e4a5be29b62985111e3c9be62de370501216969` |
| `docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json` | Annual financing context chart spec | visualization spec | n/a | view | `509c0061b6fc1208ed2cb43f13493662647590d2a72cb8b3fe4b12ed3589a722` |
| `docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json` | Decade financing context chart spec | visualization spec | n/a | view | `d211045006ad27f05554b1db88c858d1a4ace725d721a34f91565c5d34fbe450` |
| `Cargo.toml` | Rust workspace manifest | tooling | n/a | supporting | `b8735e92f7248a16a5d8657ff9ee6b693fbe8b47f1959152a245b54466060a12` |
| `Cargo.lock` | Rust dependency lockfile | tooling | n/a | supporting | `0f07ae499ffdcbad872159c22f94cf6c6813846e8559f0991f8ab27093dbf800` |
| `tools/taxlane/Cargo.toml` | Rust Taxlane tools crate manifest | tooling | n/a | supporting | `c0e554cc6980590d82aaafe1c0daa2c7e4a1de337bf66b78dc22ca09264df42b` |
| `tools/taxlane/src/main.rs` | Rust validation and manifest command implementation | script | n/a | supporting | `ddf4c6f7c07c1fe0e86126642ff1814be64b3fb25b00c98bc2d31fedfc60a178` |

## Regeneration Order

1. `build_income_tax_outlay_model.py`
2. `build_decade_summary.py`
3. `cargo run -p taxlane-tools -- income-tax-outlay export`
4. `cargo run -p taxlane-tools -- income-tax-outlay manifest`

Run validation after regeneration:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
```
