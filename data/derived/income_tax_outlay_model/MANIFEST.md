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
| `README.md` | Repository overview | documentation | n/a | supporting | `75773ea9530ed70ad314805572a5819dde2edf3a78cbe5f3ca39767339c15167` |
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
| `data/derived/accountability_evidence/README.md` | Accountability evidence dataset method note | documentation | n/a | supporting | `7cd786d59a9de3d3fa3868068eb88028c8b892d8b39a54fbcbf3b5fa6d25b6d4` |
| `data/derived/accountability_evidence/readiness-report.md` | Accountability evidence readiness report | documentation | n/a | supporting | `ab04c4abd9d4589d37f6fd05c0fc35d1f56eeaa851ea8beab0ed02f90b50d430` |
| `data/derived/accountability_evidence/action-queue.md` | Accountability evidence action queue | documentation | n/a | supporting | `84829f631ab7845be69f188745d2ec0262a9e5965e02f215f750e7c9ba61858f` |
| `data/derived/accountability_evidence/performance-demand-packet.md` | Accountability performance demand packet | documentation | n/a | supporting | `17dd78408f331e94d5b673bdb1ba0361c119c2cfa276fb9ff310df61e1871702` |
| `data/derived/accountability_evidence/accountability-work-items.jsonl` | Accountability machine-readable work items | work item | 2 | supporting | `a069a5fb8ec7a2d17484ffc0654d00648f0f37acf9f0254c1c6caa500bf823bc` |
| `data/derived/accountability_evidence/claim-guard-report.md` | Accountability claim guard report | documentation | n/a | supporting | `6aee74368a9650bbc47215fa81481cec178e25863f2212cd5876ca6ba78ad266` |
| `data/derived/accountability_evidence/public-questions.md` | Accountability public-safe questions | documentation | n/a | supporting | `5e15cb2cb4f30ab47d870ff9887b4e04c926be26218d8f048ebc419ccdde63c3` |
| `data/derived/accountability_evidence/performance-demand-checklist.md` | Accountability performance demand checklist | documentation | n/a | supporting | `e7e666ac5bf20b0b22bb366b7175fc3b9eba09106bebf984bef615d32d5eab2d` |
| `data/derived/accountability_evidence/performance-demand-checklist.jsonl` | Accountability performance demand checklist rows | demand checklist row | 2 | supporting | `c76749f5a493429666e875ca39382b2771301a278e24a7ab45b4b8f5df0919d0` |
| `data/derived/accountability_evidence/performance-demand-claim-gates.json` | Accountability performance demand claim gates | claim gate summary | n/a | supporting | `26855bdb653a39b216c2336c9ed3bf002331bfc64ac07ad343d757219ec4caa6` |
| `data/derived/accountability_evidence/performance-demand-dashboard.md` | Accountability performance demand dashboard | documentation | n/a | supporting | `940d5b9373fa1190a83662bf92ed91d5f48b6efa68873928b46ac192a0d1bf85` |
| `data/derived/accountability_evidence/performance-demand-brief.md` | Accountability performance demand brief | documentation | n/a | supporting | `0c0c87e7f2082ef1d774ce39b355814244bfd20e6ca4978f499dd60b5e540385` |
| `data/derived/accountability_evidence/performance-demand-letter.md` | Accountability performance demand letter template | documentation | n/a | supporting | `bd0745870af31388b12eb48dc72351f866ce8c2ef0b96d514b2c52ee3864ae10` |
| `data/derived/accountability_evidence/performance-demand-response-rubric.md` | Accountability performance demand response rubric | documentation | n/a | supporting | `bc628d1a209ea0ac2aab20f1730f5e80fadda4d296c302e0ea2ad1772c19602a` |
| `data/derived/accountability_evidence/performance-demand-followup.md` | Accountability performance demand follow-up template | documentation | n/a | supporting | `fe52a87f0aaaf5107248734bed8a4331623095532355d2237a8278f47fb85898` |
| `data/derived/accountability_evidence/performance-demand-response-log.md` | Accountability performance demand response log | documentation | n/a | supporting | `44d7d18c158b0847787e9b8a26c2daa4adcfabc41e2e91bd89c8737529d590ce` |
| `data/derived/accountability_evidence/performance-demand-response-log.jsonl` | Accountability performance demand response log rows | response log row | 2 | supporting | `a9dc003107c6ef8e83a8f30cea4bab9b8603a7c446cf3952a4a3b244d6c520c1` |
| `data/derived/accountability_evidence/performance-demand-response-log.schema.md` | Accountability performance demand response log schema | documentation | n/a | supporting | `1afc7c06cab6a8474f0959880ed9b8039d37d4c7f02cee25e576ef8514ae20fe` |
| `data/derived/accountability_evidence/performance-demand-response-status.json` | Accountability performance demand response status | response status summary | n/a | supporting | `a9da5fce495e962d48c28bfb64dca15efc74dcd82cad1b8a616c8d905f48e75e` |
| `data/derived/accountability_evidence/performance-demand-response-dashboard.md` | Accountability performance demand response dashboard | documentation | n/a | supporting | `be517d17f71007f844108aaadf180980bda6f149ad0c96a19673123f03c11b08` |
| `data/derived/accountability_evidence/performance-demand-response-handoff.md` | Accountability performance demand response handoff | documentation | n/a | supporting | `2f73153a21a16f3b9320237351e081830dd390b16b6050f8a507e7878e871354` |
| `data/derived/accountability_evidence/performance-demand-response-intake.md` | Accountability performance demand response intake template | documentation | n/a | supporting | `6a70af43ab1bc37a2ae3584fca091d326fa22ff82997b7af73f890216b6b8efa` |
| `data/derived/accountability_evidence/performance-demand-response-intake.schema.md` | Accountability performance demand response intake schema | documentation | n/a | supporting | `dd4ed7eda5d7361c42efe5723d9eda862148bd5d9e5df4ac309385341e6714da` |
| `data/derived/accountability_evidence/performance-demand-checklist.schema.md` | Accountability performance demand checklist schema | documentation | n/a | supporting | `afdcb11bb20a75709acabb690655fd03ba8d3a56df522c7b09e8d4b4ffeff6d1` |
| `data/derived/accountability_evidence/artifact-map.md` | Accountability artifact map | documentation | n/a | supporting | `f5537f9c23c7c18a13529511ab1741bf58c7218bffe6228a845a5a10a8d3b6f6` |
| `docs/reading/accountability-public-brief.md` | Reader-facing accountability brief | documentation | n/a | supporting | `cc225ff43f81fa0316f85a8f8ede225f5d1a51470f81d2a296e32820e7434c35` |
| `docs/reading/README.md` | Reading packet index | documentation | n/a | supporting | `bbc87f7c681d9310bf9ec9d25e0f1f43f321a9c82e5176115bb34816bda52f6b` |
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
| `reviews/2026-06-23-accountability-public-brief-review.md` | Accountability public brief review | documentation | n/a | supporting | `db47feee4c78e64aac86f5e99f89907d62f366fe7f4e82b7a0e5a96cc87b79ef` |
| `reviews/2026-06-23-accountability-brief-discovery-review.md` | Accountability brief discovery review | documentation | n/a | supporting | `0530d8e2ee08b546bbe98dc7f231c7ace7b971962cb7a2de4ea295808c09a247` |
| `reviews/2026-06-23-accountability-artifact-map-review.md` | Accountability artifact map review | documentation | n/a | supporting | `fe6da34f643d730daa20a793b28a808c5c89e6b501fb1cbb776c3e03a9b75eed` |
| `reviews/2026-06-23-accountability-performance-demand-checklist-review.md` | Accountability performance demand checklist review | documentation | n/a | supporting | `02c95371cf8bff321aefe107a2d025c1d3a6a7f9248841d77904d4dafa390941` |
| `reviews/2026-06-23-accountability-performance-demand-checklist-jsonl-review.md` | Accountability performance demand checklist JSONL review | documentation | n/a | supporting | `f205c8dd12f84a1884737ca1f096733b141a345b97c43dfae91e4a5208f77098` |
| `reviews/2026-06-23-accountability-demand-checklist-core-contract-review.md` | Accountability demand checklist core contract review | documentation | n/a | supporting | `e0049ed8005544c484996cedd82627318ee0b656da40de153d474f980aee101a` |
| `reviews/2026-06-23-accountability-demand-checklist-jsonl-read-validation-review.md` | Accountability demand checklist JSONL read validation review | documentation | n/a | supporting | `07abda1924927288ecba5f488afa11a87585cf6f9430b4bcc8b65cef8bf0da3c` |
| `reviews/2026-06-23-accountability-demand-checklist-schema-review.md` | Accountability demand checklist schema review | documentation | n/a | supporting | `301d99afa92a417e537ae822efe7fe8be8b35148458ab17159724ff99d7e915e` |
| `reviews/2026-06-23-accountability-performance-demand-claim-gates-review.md` | Accountability performance demand claim gates review | documentation | n/a | supporting | `0941807744c50e4b8a0a765a60ef2710f8299a71e9c5aa7476d48236bf152c54` |
| `reviews/2026-06-23-accountability-performance-demand-dashboard-review.md` | Accountability performance demand dashboard review | documentation | n/a | supporting | `3627a2017625e607d60ea1d7708291620e6cf076dd93ceaa0b7afb9770cc01c9` |
| `reviews/2026-06-23-rust-core-crate-architecture-review.md` | Rust core crate architecture review | documentation | n/a | supporting | `04203820d1dd05d067146fa2ca1b7f2fbe5f95346e6c0b3ccbc95d09fc8808ba` |
| `docs/vtrace/MISSION.md` | VTRACE mission | documentation | n/a | supporting | `751833c38bcc467ffe7e2c926195397ce396106830452c0d1f9b3039d6c211a6` |
| `docs/vtrace/REQUIREMENTS.md` | VTRACE requirements | documentation | n/a | supporting | `1a22419ccf0b3a10644391d410774e6ab290960913cf1e745431c68e28868913` |
| `docs/vtrace/SPECIFICATION_BASELINE.md` | VTRACE specification baseline | documentation | n/a | supporting | `dfb7c1462974eeac27fffe451e989211dfe4497b145bb56df15f6a671f010678` |
| `docs/vtrace/TRACE.md` | VTRACE trace matrix | documentation | n/a | supporting | `a899d04808ea28e9542b2605e0fc69afe3206575789f2a88a5249cd9e7e7cf38` |
| `docs/vtrace/WORK_PACKAGES.md` | VTRACE work packages | documentation | n/a | supporting | `229f6f3a0644159ef02f17a30bc6d063b847eaaa762988eca66823a6b1e03f6f` |
| `docs/vtrace/VERIFICATION.md` | VTRACE verification plan | documentation | n/a | supporting | `50d0f8cfd896357bd036e9155b8190cfb3287d630d0cef39ce5c01e104b5a9ed` |
| `docs/vtrace/VALIDATION.md` | VTRACE validation scenarios | documentation | n/a | supporting | `62074ca0556bf40ca8bc702d60b829847bba2e85f30e7a3934942ada889339a6` |
| `docs/vtrace/EVIDENCE.md` | VTRACE evidence ledger | documentation | n/a | supporting | `294b6c1f7483a6dea7c7e959c8079b3914e51d2251a084733cc13c099b8633f6` |
| `docs/vtrace/REVIEW.md` | VTRACE adoption review | documentation | n/a | supporting | `52ed83ae325229c3da170b7842dbf07d33f76be84c7e56d0a48aacb8595dd7ff` |
| `docs/charts/taxpayer-receipt-model/README.md` | Taxpayer receipt chart set handoff note | documentation | n/a | supporting | `29b98ce16cccf88869779c079dc652d7da15583777e015a1d2b8481e9c63250d` |
| `docs/charts/taxpayer-receipt-model/placeholder-lane-bars.vl.json` | Placeholder receipt lane bar chart spec | visualization spec | n/a | view | `f4233308571464bf6a58fb417803453a32c054fe43a2869d9ae93ffb616f3ccf` |
| `docs/charts/taxpayer-receipt-model/placeholder-financing-context.vl.json` | Placeholder receipt financing context chart spec | visualization spec | n/a | view | `064cb863095eea673ac4f907f6f9e18b798ec4208c31851f3bc16a80121c395b` |
| `Cargo.toml` | Rust workspace manifest | tooling | n/a | supporting | `405a2bba4b4deec2a77d51621f397528c0b5a86a3137df32af6633f32ffbfad0` |
| `Cargo.lock` | Rust dependency lockfile | tooling | n/a | supporting | `44b2fcc6662f4fa3cecadbd650c7991b66e42af15dbf4efbf8ff16b832717d96` |
| `crates/taxlane-core/Cargo.toml` | Rust Taxlane core crate manifest | tooling | n/a | supporting | `6469c9f3c3d01bc0c51783255082c93ed608c6c64c0d4f32f5cd2c08f2426fb9` |
| `crates/taxlane-core/src/lib.rs` | Rust Taxlane core domain library | library | n/a | supporting | `871f01cd1f60b932aa2ea7eef04f3b12646bed2564c62b687338c7548269763c` |
| `tools/taxlane/Cargo.toml` | Rust Taxlane tools crate manifest | tooling | n/a | supporting | `b910e56a20f40547ac1e4f696e84772dde163738b03915ddbcc0d0a39427719a` |
| `tools/taxlane/src/main.rs` | Rust validation and manifest command implementation | script | n/a | supporting | `424691eabaa9354be47f9de188ba1a1c371518a973fb2d799ba87249127054eb` |

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
