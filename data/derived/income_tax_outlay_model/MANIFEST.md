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
| `data/derived/README.md` | Derived data index | documentation | n/a | supporting | `f5da75bc92365e60439aff3e787539d67385eff460076386e3266f4d0e0fff26` |
| `data/derived/income_tax_outlay_model/reconciliation-review.md` | Generated-row reconciliation review | documentation | n/a | supporting | `ab200dfc8c467700964cb23d51875411603a73f9d1ea62e1beea9073449a9c8d` |
| `data/derived/income_tax_outlay_model/decade-summary.md` | Human-readable decade summary | documentation | n/a | supporting | `47d02425c28cd8f8fd0c236b46e0c997d54a0824ddca1cb125ca0a39bcfd915f` |
| `docs/reading/modeled-income-tax-outlays.md` | Reader-facing packet | documentation | n/a | supporting | `b7cdffb161572ff8e7c862747ad4172ef7a59deaa780373c6331f474d68515f7` |
| `docs/reading/modeled-income-tax-subfunction-outlays.md` | Reader-facing subfunction drilldown packet | documentation | n/a | supporting | `64b910a3b8e6665165a15a149f2fa54e160fcc21bf97608fcc7860e966f72197` |
| `reviews/2026-06-22-subfunction-reader-role-review.md` | Subfunction reader role review | documentation | n/a | supporting | `0ed026995fa850ed077b7d705a682edda687c1c9c803f4a8fa91e6b9b145e5b7` |
| `docs/research/2026-06-22-subfunction-deficit-context-note.md` | Subfunction deficit context note | documentation | n/a | supporting | `6c85eca0ba06a2e5ca0a81821b1e4b572598006a43991210063c5f6c68b86067` |
| `docs/data/README.md` | Data documentation index | documentation | n/a | supporting | `026693344431ab4eebe2c952c5476154d4b3e122b7c57485cf6471b377d32c30` |
| `docs/data/dictionary.md` | Data dictionary | documentation | n/a | supporting | `89c73a837ab30fe3fc7ed1f7acb87b2d93edb07ea176f0fa0e7543d262de621a` |
| `docs/data/accountability-evidence-schema.md` | Accountability evidence schema | documentation | n/a | supporting | `7055c188e186143f55ab688c85871413e2717e584261092fe3999304f3858608` |
| `docs/research/2026-06-23-accountability-evidence-boundary.md` | Accountability evidence boundary note | documentation | n/a | supporting | `2c221e635642267d8594323de222a5cb14beb785fd2d776a892a74fba5311ecd` |
| `docs/charts/README.md` | Chart catalog | documentation | n/a | supporting | `1964b015e1253a77c982b2916444179572933c811d7724365e4ceecfe442aa81` |
| `docs/charts/income-tax-outlay-subfunction-model/README.md` | Subfunction chart set handoff note | documentation | n/a | supporting | `3acb16eb58bf954fff8cac9fd44b840be58ca1ae21430d4f84fb756a0fe12635` |
| `docs/charts/income-tax-outlay-model/README.md` | Broad chart set handoff note | documentation | n/a | supporting | `0f20babf158a3e416382b32cecc1403115986391b6a02f0ab7d59372b0b7b1df` |
| `docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json` | Annual allocation chart spec | visualization spec | n/a | view | `dab4f7d15be91c7357c8595d14e8d58336048747bc7a3825cee756072dc05b54` |
| `docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json` | Decade allocation chart spec | visualization spec | n/a | view | `4e1bbc8b41ce38147477d7494e4a5be29b62985111e3c9be62de370501216969` |
| `docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json` | Annual financing context chart spec | visualization spec | n/a | view | `509c0061b6fc1208ed2cb43f13493662647590d2a72cb8b3fe4b12ed3589a722` |
| `docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json` | Decade financing context chart spec | visualization spec | n/a | view | `d211045006ad27f05554b1db88c858d1a4ace725d721a34f91565c5d34fbe450` |
| `docs/charts/income-tax-outlay-subfunction-model/fy2025-top-subfunctions.vl.json` | FY2025 top subfunction allocation chart spec | visualization spec | n/a | view | `0d077c437b4a4d0b2614855fca21fe0a719acffc6a0ed8640001190d595d35d3` |
| `docs/charts/income-tax-outlay-subfunction-model/selected-subfunction-trends.vl.json` | Selected subfunction trend chart spec | visualization spec | n/a | view | `e8c317b28eb77ec75223926e407063a9cbb183160068a1db63b01b36537493c6` |
| `docs/charts/income-tax-outlay-subfunction-model/decade-top-subfunctions.vl.json` | Decade top subfunction allocation chart spec | visualization spec | n/a | view | `e6dc76202c97a5003b9933e8c264d10167e803a2e6140f3b37f6d33791cc868e` |
| `data/derived/taxpayer_receipt_model/taxpayer_receipt_model.placeholder-1000.fy2025.omb-fy2027-v1.draft.json` | Placeholder visibility receipt scenario | scenario | n/a | yes | `9f44475ed9b71239215dae060237216b5a760737db085c59ba4bc3dcb5906bbf` |
| `data/derived/accountability_evidence/accountability_evidence.omb-fy2027-v1.2026-06-23.draft.jsonl` | Draft accountability evidence records | evidence record | 2 | supporting | `0f52d824722f291f862ff5a456ecc94db5f93cb357b121fb5a899dd4e5aff654` |
| `data/derived/accountability_evidence/README.md` | Accountability evidence dataset method note | documentation | n/a | supporting | `2451cda42a7c3ebbde9c700472e0f22a898e7f5dc33c96722a783912b4973471` |
| `data/derived/accountability_evidence/readiness-report.md` | Accountability evidence readiness report | documentation | n/a | supporting | `ab04c4abd9d4589d37f6fd05c0fc35d1f56eeaa851ea8beab0ed02f90b50d430` |
| `data/derived/accountability_evidence/action-queue.md` | Accountability evidence action queue | documentation | n/a | supporting | `84829f631ab7845be69f188745d2ec0262a9e5965e02f215f750e7c9ba61858f` |
| `data/derived/accountability_evidence/performance-demand-packet.md` | Accountability performance demand packet | documentation | n/a | supporting | `17dd78408f331e94d5b673bdb1ba0361c119c2cfa276fb9ff310df61e1871702` |
| `data/derived/accountability_evidence/accountability-work-items.jsonl` | Accountability machine-readable work items | work item | 2 | supporting | `a069a5fb8ec7a2d17484ffc0654d00648f0f37acf9f0254c1c6caa500bf823bc` |
| `data/derived/accountability_evidence/claim-guard-report.md` | Accountability claim guard report | documentation | n/a | supporting | `6aee74368a9650bbc47215fa81481cec178e25863f2212cd5876ca6ba78ad266` |
| `data/derived/accountability_evidence/public-questions.md` | Accountability public-safe questions | documentation | n/a | supporting | `5e15cb2cb4f30ab47d870ff9887b4e04c926be26218d8f048ebc419ccdde63c3` |
| `docs/reading/placeholder-visibility-receipt.md` | Placeholder receipt reader packet | documentation | n/a | supporting | `4be976503aabb6c21ec77a553f185918190ebcb00e3300930d37e3a7a12f962e` |
| `docs/reading/placeholder-receipt-display-packet.md` | Placeholder receipt static display packet | documentation | n/a | supporting | `aa2f3fd899fa1dd763b9482c4d4aae81fcde3e77aeb8ce01b1626d10155f35f2` |
| `reviews/2026-06-23-placeholder-display-packet-role-review.md` | Placeholder receipt display packet role review | documentation | n/a | supporting | `1a0825afa9a4db902ea1d17a0a69443e0caeb2a6ae7c805f3718282c8806fd68` |
| `docs/design/placeholder-receipt-placement-spec.md` | Placeholder receipt static placement spec | documentation | n/a | supporting | `31202dd33c7a82f87e7a32aab4a3a726021cc02abb4814f9ebe1b740f6c6a201` |
| `docs/design/README.md` | Design handoff index | documentation | n/a | supporting | `f4bc4bb5f9b5a602ad8dab45de2c1b0ce91825d5f26bdde80aa8df7786643d57` |
| `reviews/2026-06-23-placeholder-placement-spec-role-review.md` | Placeholder receipt placement spec role review | documentation | n/a | supporting | `6c793e8988db21628e8fb332f06de5a4d24d64fdaa73460e7e1cac99cea6104f` |
| `docs/design/placeholder-receipt-mockup-review-checklist.md` | Placeholder receipt mockup review checklist | documentation | n/a | supporting | `8a6fe7c31eec90739096cce0e358bd3475df86d0826c07dbab37d4f39120e7f6` |
| `reviews/2026-06-23-placeholder-mockup-checklist-role-review.md` | Placeholder receipt mockup checklist role review | documentation | n/a | supporting | `a5ce525e2999eae057bc661a7489200192737469ea310d75a401f20e78b913d5` |
| `reviews/2026-06-23-accountability-evidence-role-review.md` | Accountability evidence role review | documentation | n/a | supporting | `5f12b08f142e42aabaea34b6324a6c171b26c0962324673657064cdc068b6043` |
| `reviews/2026-06-23-accountability-evidence-source-custody-review.md` | Accountability evidence source-custody review | documentation | n/a | supporting | `6962a9c60b8d749262104ebd81b4f7e15edded62837a25bae36d79667f26fd6c` |
| `reviews/2026-06-23-accountability-validator-hardening-review.md` | Accountability validator hardening review | documentation | n/a | supporting | `5546400501b598dcda7c30ca9a66f042490de51719276d5214da6d6a71681c5b` |
| `reviews/2026-06-23-accountability-readiness-classification-review.md` | Accountability readiness classification review | documentation | n/a | supporting | `7ff2318a8e509d647bc2c0390809ba1fb198c8c8eb97c644acbeabd4087f6301` |
| `reviews/2026-06-23-accountability-readiness-report-review.md` | Accountability readiness report review | documentation | n/a | supporting | `dfe36c0b6fa5f8baf6f112c1415bd8f8cea4afdd95ac02fc407fb882f69d26af` |
| `reviews/2026-06-23-accountability-evidence-only-record-review.md` | Accountability evidence-only record review | documentation | n/a | supporting | `2d8dd4d50a31f6c775dd1c6b31b309c511b87b5e581c1dcffe42572cbcbe5d37` |
| `reviews/2026-06-23-accountability-next-action-report-review.md` | Accountability next-action report review | documentation | n/a | supporting | `958d6c2de7e20ac63e8ca4599d07e846d289065cafe6042d4573f921a0400645` |
| `reviews/2026-06-23-accountability-action-queue-review.md` | Accountability action queue review | documentation | n/a | supporting | `0854ea1918e9c3dfd912f24b36c82b22e4138b3887b0329be824e4a9ec5d07eb` |
| `reviews/2026-06-23-accountability-performance-demand-packet-review.md` | Accountability performance demand packet review | documentation | n/a | supporting | `8b20ddf872efdfd952488a7155d5944eded5f81e115dc748157afebdc5ca8cb2` |
| `reviews/2026-06-23-accountability-core-workflow-review.md` | Accountability core workflow review | documentation | n/a | supporting | `00c423e6cc811e00164a628fa450eee7a83e64b54711a0d2d195fae1a9c82cfb` |
| `reviews/2026-06-23-accountability-work-items-review.md` | Accountability work items review | documentation | n/a | supporting | `ac11be43bb84c82e2e7dd51122c2b241d23fffb853d85a552b29c5ff164b9d49` |
| `reviews/2026-06-23-accountability-claim-guard-report-review.md` | Accountability claim guard report review | documentation | n/a | supporting | `6e959a2344e61b0f4f595047d03a36933bdf4f6f3eb3acf0a6d76df7aafcfd47` |
| `reviews/2026-06-23-accountability-public-questions-review.md` | Accountability public questions review | documentation | n/a | supporting | `9eba797fdb61ce482a4458b30b1d59b8318934851988cb1abf641a0bc6395bdc` |
| `reviews/2026-06-23-rust-core-crate-architecture-review.md` | Rust core crate architecture review | documentation | n/a | supporting | `04203820d1dd05d067146fa2ca1b7f2fbe5f95346e6c0b3ccbc95d09fc8808ba` |
| `docs/vtrace/MISSION.md` | VTRACE mission | documentation | n/a | supporting | `751833c38bcc467ffe7e2c926195397ce396106830452c0d1f9b3039d6c211a6` |
| `docs/vtrace/REQUIREMENTS.md` | VTRACE requirements | documentation | n/a | supporting | `1a22419ccf0b3a10644391d410774e6ab290960913cf1e745431c68e28868913` |
| `docs/vtrace/SPECIFICATION_BASELINE.md` | VTRACE specification baseline | documentation | n/a | supporting | `dfb7c1462974eeac27fffe451e989211dfe4497b145bb56df15f6a671f010678` |
| `docs/vtrace/TRACE.md` | VTRACE trace matrix | documentation | n/a | supporting | `228d7f0bb89ed9ff3816c972741a5831ead7304deedcf6474441569ffd901940` |
| `docs/vtrace/WORK_PACKAGES.md` | VTRACE work packages | documentation | n/a | supporting | `fa17e6327be28b4b93b4d5b9cab6d93e0148aff3f32aca066b6efc6cd97f1a10` |
| `docs/vtrace/VERIFICATION.md` | VTRACE verification plan | documentation | n/a | supporting | `a5aded0406fb2a497f07a6055a26e601225dffc8ee77723fafde072abb461234` |
| `docs/vtrace/VALIDATION.md` | VTRACE validation scenarios | documentation | n/a | supporting | `ca0bb5c715e3a242071f3134211decc7b2c65f4ef6f569cfc46bf4602298fd8a` |
| `docs/vtrace/EVIDENCE.md` | VTRACE evidence ledger | documentation | n/a | supporting | `85088f1a5701c360684545fa1529876a93206b3bf856aeb263ba44e4d726c333` |
| `docs/vtrace/REVIEW.md` | VTRACE adoption review | documentation | n/a | supporting | `52ed83ae325229c3da170b7842dbf07d33f76be84c7e56d0a48aacb8595dd7ff` |
| `docs/charts/taxpayer-receipt-model/README.md` | Taxpayer receipt chart set handoff note | documentation | n/a | supporting | `29b98ce16cccf88869779c079dc652d7da15583777e015a1d2b8481e9c63250d` |
| `docs/charts/taxpayer-receipt-model/placeholder-lane-bars.vl.json` | Placeholder receipt lane bar chart spec | visualization spec | n/a | view | `f4233308571464bf6a58fb417803453a32c054fe43a2869d9ae93ffb616f3ccf` |
| `docs/charts/taxpayer-receipt-model/placeholder-financing-context.vl.json` | Placeholder receipt financing context chart spec | visualization spec | n/a | view | `064cb863095eea673ac4f907f6f9e18b798ec4208c31851f3bc16a80121c395b` |
| `Cargo.toml` | Rust workspace manifest | tooling | n/a | supporting | `405a2bba4b4deec2a77d51621f397528c0b5a86a3137df32af6633f32ffbfad0` |
| `Cargo.lock` | Rust dependency lockfile | tooling | n/a | supporting | `44b2fcc6662f4fa3cecadbd650c7991b66e42af15dbf4efbf8ff16b832717d96` |
| `crates/taxlane-core/Cargo.toml` | Rust Taxlane core crate manifest | tooling | n/a | supporting | `6469c9f3c3d01bc0c51783255082c93ed608c6c64c0d4f32f5cd2c08f2426fb9` |
| `crates/taxlane-core/src/lib.rs` | Rust Taxlane core domain library | library | n/a | supporting | `762605f81e2218ff9cc341801f9c4f98608749756a7ed224943823cf59d06272` |
| `tools/taxlane/Cargo.toml` | Rust Taxlane tools crate manifest | tooling | n/a | supporting | `b910e56a20f40547ac1e4f696e84772dde163738b03915ddbcc0d0a39427719a` |
| `tools/taxlane/src/main.rs` | Rust validation and manifest command implementation | script | n/a | supporting | `3b1ef0f08ec7575b29c521008c63694a3f00ba5021e0d1fc6c3d97079142315d` |

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
