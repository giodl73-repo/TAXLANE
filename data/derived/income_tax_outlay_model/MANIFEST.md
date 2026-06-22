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
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl` | Canonical annual model rows | fiscal year by broad category | 516 | yes | `01c0fec63836f3652fdd83b03a608159ad58f7614fc214ae6691421990f3a85e` |
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-summary.jsonl` | Canonical decade summary rows | decade by broad category | 54 | yes | `088cb5d55cc6a37ab52dd543d8a92bf09908cda265e9c9f93324b840eb652fb3` |
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv` | Chart-ready annual wide view | fiscal year | 86 | no | `a385e4215a20c16f0344cc8e17982c9e4c78933caac93f274ea5b6080027d81a` |
| `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-wide.csv` | Chart-ready decade wide view | decade | 9 | no | `5e243eb2912b7aaed26e22016fd1907bf4cee25d60b3855f1c2627e7f4b9a2aa` |
| `data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.draft.jsonl` | Canonical annual subfunction model rows | fiscal year by Table 3.2 subfunction | 4691 | yes | `d5bdad37e2d0bb3e5b880152b2d0a03dfb0c335c678c0457784d8c156dd454f4` |
| `data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.annual-long.csv` | Chart-ready annual subfunction long view | fiscal year by Table 3.2 subfunction | 4691 | no | `da40593d51f0488b0e00c4f194011da5da222fb4335cc258a9ff11dbdd1e4faa` |
| `data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.decade-long.csv` | Chart-ready decade subfunction long view | decade by Table 3.2 subfunction | 521 | no | `de49fc6fe73573ca59fdcb83a1c57d42c45fb0c244b0f9a6c5576162c7e48912` |
| `data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.fy2025-top-subfunctions.csv` | Chart-ready FY2025 top subfunction view | ranked FY2025 subfunction | 25 | no | `1943a3c07cd65eba12a714886587e81774dbb9df2b459d59f8939cbc9b67a641` |
| `data/derived/income_tax_outlay_subfunction_model/README.md` | Subfunction model method and schema note | documentation | n/a | supporting | `3f5b4eb04416ce0adec2f95807e65a5857b458fe29d7e42b9170c00acd7f955b` |
| `data/derived/income_tax_outlay_subfunction_model/source-profile.md` | Subfunction source coverage and reconciliation sample | documentation | n/a | supporting | `32bee81ceca6430c056dbe116ca3f0a0dec4d9bed1bbbdaeadf7d4eb5988ab69` |
| `data/derived/income_tax_outlay_subfunction_model/reconciliation-review.md` | Subfunction generated-row reconciliation review | documentation | n/a | supporting | `39cb25bdc54db649b32fe4edf8a7c89731858b62b37daadff4d5215555e66d24` |
| `data/derived/income_tax_outlay_model/README.md` | Model method and schema note | documentation | n/a | supporting | `38ffa2bf317e8d0659ed0a75f761f4da17cc4062750e09774a94f87532fd0e06` |
| `data/derived/income_tax_outlay_model/source-profile.md` | Source coverage and reconciliation sample | documentation | n/a | supporting | `03ea7eee098ee6e1f3cc4a83cd89adec009f9cbb96d050f2e7118e06dcf2e70f` |
| `data/derived/income_tax_outlay_model/reconciliation-review.md` | Generated-row reconciliation review | documentation | n/a | supporting | `ab200dfc8c467700964cb23d51875411603a73f9d1ea62e1beea9073449a9c8d` |
| `data/derived/income_tax_outlay_model/decade-summary.md` | Human-readable decade summary | documentation | n/a | supporting | `47d02425c28cd8f8fd0c236b46e0c997d54a0824ddca1cb125ca0a39bcfd915f` |
| `docs/reading/modeled-income-tax-outlays.md` | Reader-facing packet | documentation | n/a | supporting | `b7cdffb161572ff8e7c862747ad4172ef7a59deaa780373c6331f474d68515f7` |
| `docs/reading/modeled-income-tax-subfunction-outlays.md` | Reader-facing subfunction drilldown packet | documentation | n/a | supporting | `64b910a3b8e6665165a15a149f2fa54e160fcc21bf97608fcc7860e966f72197` |
| `reviews/2026-06-22-subfunction-reader-role-review.md` | Subfunction reader role review | documentation | n/a | supporting | `0ed026995fa850ed077b7d705a682edda687c1c9c803f4a8fa91e6b9b145e5b7` |
| `docs/research/2026-06-22-subfunction-deficit-context-note.md` | Subfunction deficit context note | documentation | n/a | supporting | `6c85eca0ba06a2e5ca0a81821b1e4b572598006a43991210063c5f6c68b86067` |
| `docs/charts/README.md` | Chart catalog | documentation | n/a | supporting | `e7f870d703d997b6e3aaa12c5fc48f14e8cf4b36c39118a31bf89af2871c7ede` |
| `docs/charts/income-tax-outlay-subfunction-model/README.md` | Subfunction chart set handoff note | documentation | n/a | supporting | `3acb16eb58bf954fff8cac9fd44b840be58ca1ae21430d4f84fb756a0fe12635` |
| `docs/charts/income-tax-outlay-model/README.md` | Broad chart set handoff note | documentation | n/a | supporting | `0f20babf158a3e416382b32cecc1403115986391b6a02f0ab7d59372b0b7b1df` |
| `docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json` | Annual allocation chart spec | visualization spec | n/a | view | `dab4f7d15be91c7357c8595d14e8d58336048747bc7a3825cee756072dc05b54` |
| `docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json` | Decade allocation chart spec | visualization spec | n/a | view | `4e1bbc8b41ce38147477d7494e4a5be29b62985111e3c9be62de370501216969` |
| `docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json` | Annual financing context chart spec | visualization spec | n/a | view | `509c0061b6fc1208ed2cb43f13493662647590d2a72cb8b3fe4b12ed3589a722` |
| `docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json` | Decade financing context chart spec | visualization spec | n/a | view | `d211045006ad27f05554b1db88c858d1a4ace725d721a34f91565c5d34fbe450` |
| `docs/charts/income-tax-outlay-subfunction-model/fy2025-top-subfunctions.vl.json` | FY2025 top subfunction allocation chart spec | visualization spec | n/a | view | `0d077c437b4a4d0b2614855fca21fe0a719acffc6a0ed8640001190d595d35d3` |
| `docs/charts/income-tax-outlay-subfunction-model/selected-subfunction-trends.vl.json` | Selected subfunction trend chart spec | visualization spec | n/a | view | `e8c317b28eb77ec75223926e407063a9cbb183160068a1db63b01b36537493c6` |
| `docs/charts/income-tax-outlay-subfunction-model/decade-top-subfunctions.vl.json` | Decade top subfunction allocation chart spec | visualization spec | n/a | view | `e6dc76202c97a5003b9933e8c264d10167e803a2e6140f3b37f6d33791cc868e` |
| `Cargo.toml` | Rust workspace manifest | tooling | n/a | supporting | `b8735e92f7248a16a5d8657ff9ee6b693fbe8b47f1959152a245b54466060a12` |
| `Cargo.lock` | Rust dependency lockfile | tooling | n/a | supporting | `ce3e305db156da23b235b0fd67ba32e2f18041c64a2b530fe9f850c38ab2ab80` |
| `tools/taxlane/Cargo.toml` | Rust Taxlane tools crate manifest | tooling | n/a | supporting | `72d0be55c8b61b0922a3140e18e5756a1643095dd389c90bd2219ba6d31c96b1` |
| `tools/taxlane/src/main.rs` | Rust validation and manifest command implementation | script | n/a | supporting | `1c647f554a80805ee07afc029c628910a51b70ba0d433fed02330ccabf08e796` |

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
