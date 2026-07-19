# Current-law baseline annual path partial schema

Draft schema for `current_law_baseline_annual_path_partial.v1.draft.json`.

Required top-level fields:

- `record_id`, `record_family`, `version`, `status`, and `pulse`.
- Paths to the custody batch plan, custody packet template, FY2025 17-row ledger custody, and current-law path inventory.
- `path_id = baseline_plus_ten_year_horizon`.
- `baseline_year = 2025`, eleven `required_years` from FY2025 through FY2035, populated years, missing years, and `interpolation_used = false`.
- `source_custody_status` with official-source, no-contact, source-ready, partial-outlay-ready, incomplete-horizon, and solver-blocked flags.
- `source_packets` for each official local raw source used.
- `annual_rows`, exactly one row per required fiscal year.
- `reconciliation`, including FY2025 PBD-to-ledger equality and complete-horizon false status.
- `blocked_outputs`, where the full required horizon, forward receipts/deficits, fund paths, solver inputs, policy deltas, target costs, rates, and public rate cards remain null.
- `claim_booleans`, allowing partial current-law outlay path publication while blocking complete baseline, solver, target-cost, rate, savings, waste, fraud, department-cut, technology-savings, and balanced-budget claims.

Validation requirements:

- Raw and metadata files must exist.
- Raw byte counts and SHA-256 values must match the local files.
- Annual rows must cover FY2025 through FY2035 exactly once.
- FY2025 through FY2031 must have official outlay values.
- FY2032 through FY2035 outlays must remain null unless an official local source is added.
- FY2026 through FY2035 receipts and deficits must remain null in this partial artifact.
- FY2025 PBD outlays must equal the FY2025 17-row ledger total of `7,011,105` million dollars.
- No interpolation is allowed.
- Solver and public fiscal claims must remain blocked.
