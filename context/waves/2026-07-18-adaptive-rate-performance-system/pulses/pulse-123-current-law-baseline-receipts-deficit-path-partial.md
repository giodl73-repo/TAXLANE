# Pulse 123 — Current-law baseline receipts and deficit path partial

## Scope

Add FY2025-FY2031 official current-law receipts and deficit-gap values using local OMB custody.

## Added

- `data/derived/breadth_benchmark_matrix/current_law_baseline_receipts_deficit_path_partial.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/current_law_baseline_receipts_deficit_path_partial.schema.md`
- `docs/reading/current-law-baseline-receipts-deficit-path-partial.md`
- Rust validator and focused regression test

## Data populated

| FY | Outlays | Receipts | Deficit |
|---:|---:|---:|---:|
| 2025 | `$7,011.105B` | `$5,236.421B` | `$1,774.684B` |
| 2026 | `$7,540.434B` | `$5,475.705B` | `$2,064.729B` |
| 2027 | `$8,092.860B` | `$5,920.951B` | `$2,171.909B` |
| 2028 | `$8,445.361B` | `$6,288.407B` | `$2,156.954B` |
| 2029 | `$8,653.223B` | `$6,660.321B` | `$1,992.902B` |
| 2030 | `$8,996.290B` | `$7,137.281B` | `$1,859.009B` |
| 2031 | `$9,279.779B` | `$7,559.389B` | `$1,720.390B` |

## Boundary

This pulse uses existing local OMB custody and the already validated Pulse 122 outlay path. No external request was submitted and no agency or person was contacted.

FY2032 through FY2035 remain null. The artifact is not complete-horizon-ready, fund-split-ready, or solver-ready.

No target cost, rate, public rate card, tax proposal, savings estimate, waste finding, fraud finding, department-cut instruction, technology-savings claim, or balanced-budget claim is made.
