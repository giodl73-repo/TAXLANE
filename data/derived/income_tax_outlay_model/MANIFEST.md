# Income-Tax Outlay Model Artifact Manifest

## Purpose

This manifest records the artifact chain for modeled allocations of
ordinary individual income-tax receipts by OMB outlay share.

The annual, decade, and subfunction JSONL files are canonical model
outputs. CSV files, Markdown notes, and chart specs are derived or
supporting views.

## Model

- Broad model ID: `individual-income-tax-proportional-outlays-v1`
- Subfunction model ID: `individual-income-tax-proportional-subfunction-outlays-v1`
- Broad coverage: fiscal years 1940-2025 for annual actual-year rows
- Subfunction coverage: fiscal years 1962-2025 for Table 3.2 actual-year rows
- Projection treatment: FY2026-FY2031 excluded
- Legal status: modeled allocation, not legal dedication

## Artifacts

| Path | Role | Grain | Rows | Canonical | SHA-256 |
|---|---|---|---:|---|---|
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl` | Canonical annual model rows | fiscal year by broad category | 516 | yes | `c50c2fdf4c5eab5e2903d8bbc12e581cb2923d3a065a77f6c790f3c7bf397f59` |
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-summary.jsonl` | Canonical decade summary rows | decade by broad category | 54 | yes | `36a28934e3bcac104e84b5389b3a47fe97657f066a8359707c00ee6521ef0d60` |
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv` | Chart-ready annual wide view | fiscal year | 86 | no | `a385e4215a20c16f0344cc8e17982c9e4c78933caac93f274ea5b6080027d81a` |
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-wide.csv` | Chart-ready decade wide view | decade | 9 | no | `5e243eb2912b7aaed26e22016fd1907bf4cee25d60b3855f1c2627e7f4b9a2aa` |
| `data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.draft.jsonl` | Canonical annual subfunction model rows | fiscal year by Table 3.2 subfunction | 4691 | yes | `0aaa49392b7b51fd28be99f7b236369c1f1ac7f92092fbff49bc1469e3d109c7` |
| `data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.annual-long.csv` | Chart-ready annual subfunction long view | fiscal year by Table 3.2 subfunction | 4691 | no | `ad131a5f5b881f7437a7d083cde3d70524c9a180b3d8752f480214d51f7be72a` |
| `data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.decade-long.csv` | Chart-ready decade subfunction long view | decade by Table 3.2 subfunction | 521 | no | `88682d381058e9409a2d224c2f188a740a1cf51b00647635e10dcd65d999f5ce` |
| `data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.fy2025-top-subfunctions.csv` | Chart-ready FY2025 top subfunction view | ranked FY2025 subfunction | 25 | no | `7490dcd469ef70169d4037c41037793339540189c711d37e95b52e308faa04ff` |
| `data/derived/income_tax_outlay_subfunction_model/README.md` | Subfunction model method and schema note | documentation | n/a | supporting | `bf055c8fbd297e98b28c1a47318209f3a7fbdf65fd118b8231a1f42dffff0a85` |
| `data/derived/income_tax_outlay_subfunction_model/source-profile.md` | Subfunction source coverage and reconciliation sample | documentation | n/a | supporting | `2e61e02b034f77cb3219ab3e83cb1173bd15fd2bff628ebae27e01200091541b` |
| `data/derived/income_tax_outlay_model/README.md` | Model method and schema note | documentation | n/a | supporting | `b96bda607dc1bfa450fd4b35bc20291ff13d5382edb82ca6938ca69156963e86` |
| `data/derived/income_tax_outlay_model/source-profile.md` | Source coverage and reconciliation sample | documentation | n/a | supporting | `7d0dff2c0c671e4e1bbc80aee6a72a6ad09890758e1d8e7c92999a6d093456a2` |
| `data/derived/income_tax_outlay_model/reconciliation-review.md` | Generated-row reconciliation review | documentation | n/a | supporting | `9c1898df3c6d7498a285887c8a64665d320afb1d492e8cc4b69bb27d22b30be8` |
| `data/derived/income_tax_outlay_model/decade-summary.md` | Human-readable decade summary | documentation | n/a | supporting | `8ee633a570b3974a77463abcd56f33642affd7944f0173562ce7913ff9a389da` |
| `docs/reading/modeled-income-tax-outlays.md` | Reader-facing packet | documentation | n/a | supporting | `26f5c340e1ffadc4efa716bbce33772c4963df54635982bd550a3392aae0579b` |
| `docs/reading/modeled-income-tax-subfunction-outlays.md` | Reader-facing subfunction drilldown packet | documentation | n/a | supporting | `2d9688ba0ac6522979b9a2eb0cac6c969ed7813f06ec89f23b9d0358ad284390` |
| `reviews/2026-06-22-subfunction-reader-role-review.md` | Subfunction reader role review | documentation | n/a | supporting | `50bb3107dc11b9bd68e7315333e38882d015eeb57a83126d601d5b752ec474e0` |
| `docs/research/2026-06-22-subfunction-deficit-context-note.md` | Subfunction deficit context note | documentation | n/a | supporting | `75421ddbe7808ef4f98ba06d7414c6103a71ee065524a8e328f8e8efce4049de` |
| `docs/charts/README.md` | Chart catalog | documentation | n/a | supporting | `09d8f1d7b08742a84275a6bd9a7bfec41b2fa9d008e0e794e5b2b8d49dbad09d` |
| `docs/charts/income-tax-outlay-subfunction-model/README.md` | Subfunction chart set handoff note | documentation | n/a | supporting | `e18e0744f33d7b0117fee2e7c9195be0477d529d23744f9d893068dcaf93950b` |
| `docs/charts/income-tax-outlay-model/README.md` | Broad chart set handoff note | documentation | n/a | supporting | `f70477105e401fabeaa5a15e807a414343465ad7b8706b91b6f8ba84f95fc180` |
| `docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json` | Annual allocation chart spec | visualization spec | n/a | view | `e0f7c39a6d4f392fc3d60f5f9f6c0561f1bc7706f2dab6019158945b66ca68b6` |
| `docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json` | Decade allocation chart spec | visualization spec | n/a | view | `e53433c4304830465e8633b4d5e6b88c8f1e54f517785916ff8191540c1bcb5b` |
| `docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json` | Annual financing context chart spec | visualization spec | n/a | view | `0f73917e9e372abce44ff1d1050b0a71d6358bb4de28b5538838e4d87a590011` |
| `docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json` | Decade financing context chart spec | visualization spec | n/a | view | `a3b513e30738ef3070d0e2f1fcc3c2e925d3dc0dc3f0381a8827a5cdd1a781ff` |
| `docs/charts/income-tax-outlay-subfunction-model/fy2025-top-subfunctions.vl.json` | FY2025 top subfunction allocation chart spec | visualization spec | n/a | view | `fc4cd48e5a1ed5761e05be28ddb636365f26966909b15b3bb0b6ed88e676ff6a` |
| `docs/charts/income-tax-outlay-subfunction-model/selected-subfunction-trends.vl.json` | Selected subfunction trend chart spec | visualization spec | n/a | view | `a5b10152d4e828854b6cf8ed056846ed7e130c404f25fa8ad000a542a579154d` |
| `docs/charts/income-tax-outlay-subfunction-model/decade-top-subfunctions.vl.json` | Decade top subfunction allocation chart spec | visualization spec | n/a | view | `71189f4429f4ab980f82c19d689fe50ad86a0a086b6dd79a87be316d87665eba` |
| `Cargo.toml` | Rust workspace manifest | tooling | n/a | supporting | `9ebc5854ae2e4979a5e9b86f65a60cb29c5a88f587877b126ed9cbd8457532cb` |
| `Cargo.lock` | Rust dependency lockfile | tooling | n/a | supporting | `98b37b03e7430d5538eb8c663022926d896f9b31194915f43b654556fb16716f` |
| `tools/taxlane/Cargo.toml` | Rust Taxlane tools crate manifest | tooling | n/a | supporting | `44e924bb7e71d35948fa75dcc0583ef58203a0e0f52180cf03072ef64aa67c33` |
| `tools/taxlane/src/main.rs` | Rust validation and manifest command implementation | script | n/a | supporting | `ccdd3c3587736f499aaba8e21aecda75f524e327fd48963c6697f85cbdd8ad7b` |

## Regeneration Order

1. `cargo run -p taxlane-tools -- income-tax-outlay model`
2. `cargo run -p taxlane-tools -- income-tax-outlay summary`
3. `cargo run -p taxlane-tools -- income-tax-outlay export`
4. `cargo run -p taxlane-tools -- income-tax-outlay subfunction-model`
5. `cargo run -p taxlane-tools -- income-tax-outlay subfunction-export`
6. `cargo run -p taxlane-tools -- income-tax-outlay manifest`

Run validation after regeneration:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
```
