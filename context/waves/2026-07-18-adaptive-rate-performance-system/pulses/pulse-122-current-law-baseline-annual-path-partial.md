# Pulse 122 — Current-law baseline annual path partial

## Scope

Add the first forward-year official current-law annual outlay path using existing local OMB Public Budget Database custody.

## Added

- `data/derived/breadth_benchmark_matrix/current_law_baseline_annual_path_partial.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/current_law_baseline_annual_path_partial.schema.md`
- `docs/reading/current-law-baseline-annual-path-partial.md`
- Rust validator and focused regression test

## Data populated

- FY2025 outlays: `$7,011.105B`
- FY2026 outlays: `$7,540.434B`
- FY2027 outlays: `$8,092.860B`
- FY2028 outlays: `$8,445.361B`
- FY2029 outlays: `$8,653.223B`
- FY2030 outlays: `$8,996.290B`
- FY2031 outlays: `$9,279.779B`

## Boundary

This pulse uses existing local OMB custody. No external request was submitted and no agency or person was contacted.

FY2032 through FY2035 remain null. FY2026 through FY2035 receipts and deficits remain null. The artifact is not complete-horizon-ready or solver-ready.

No target cost, rate, public rate card, tax proposal, savings estimate, waste finding, fraud finding, department-cut instruction, technology-savings claim, or balanced-budget claim is made.
