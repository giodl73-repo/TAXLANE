# Current-law baseline receipts and deficit path partial schema

Draft schema for `current_law_baseline_receipts_deficit_path_partial.v1.draft.json`.

Required fields:

- Identity fields and links to Pulse 122 partial outlay path, custody batch plan, custody template, and path inventory.
- `path_id = baseline_plus_ten_year_horizon`.
- Required years FY2025-FY2035, populated years FY2025-FY2031, missing years FY2032-FY2035, and `interpolation_used = false`.
- Source custody status with official-source, no-contact, source-ready, partial-value-ready, complete-horizon-false, fund-path-false, and solver-false flags.
- Source packets for OMB Historical Table 2.1 and the prior validated Pulse 122 outlay path.
- Annual rows with total outlays, total receipts, and deficit for FY2025-FY2031, and null values for FY2032-FY2035.
- Reconciliation status showing seven populated receipt/deficit years and four missing years.
- Blocked outputs for full horizon values, receipt bases, rates, fund paths, solver inputs, policy deltas, target costs, and public rate cards.
- Claim booleans permitting only partial current-law receipts/deficit path publication and source custody readiness.

Validation requirements:

- Raw OMB Table 2.1 and Pulse 122 source artifact hashes must match.
- FY2025-FY2031 deficits must equal `total_outlays_musd - total_receipts_musd`.
- FY2032-FY2035 values must remain null.
- Missing values must not be zero-filled.
- No fund split, solver input, target cost, rate, savings, waste, fraud, department-cut, technology-savings, tax-proposal, public-rate-card, or balanced-budget claim may be true.
