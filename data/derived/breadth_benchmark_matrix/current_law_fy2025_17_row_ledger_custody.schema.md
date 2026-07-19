# Current-law FY2025 17-row ledger custody schema

Draft schema for `current_law_fy2025_17_row_ledger_custody.v1.draft.json`.

Required top-level fields:

- `record_id`, `record_family`, `version`, `status`, and `pulse`.
- Paths to the current-law custody template, batch plan, and path inventory.
- `path_id = full_17_row_fy2025_ledger` and `batch_id = batch_1_federal_baseline_and_17_row_ledger`.
- `fiscal_year = 2025`, `year_basis = fiscal_year`, and `unit = millions_of_dollars`.
- `source_custody_status` with official-source, no-contact, source-ready, and blocked solver flags.
- `source_packets`, one row per official raw source, with packet ID, path ID, batch ID, source ID, official publisher, source vintage, retrieval date, raw path, byte count, SHA-256, metadata path, extraction method, annual coverage, component mapping, review status, custody readiness, value-population readiness, and claim booleans.
- `source_value_lineage` identifying the existing local value artifact used for the FY2025 rows.
- `ledger_rows`, exactly 17 rows, including the two negative fiscal reconciliation rows.
- `reconciliation` with row counts, the unrounded row sum, required total outlays, rounding residual, receipts, deficit, net interest, offset-row retention, and net-interest direct-cut block.
- `blocked_outputs`, where non-baseline-year paths, solver inputs, policy deltas, target costs, rates, and public rate cards remain null.
- `claim_booleans`, where source custody and baseline-year current-law value publication may be true, but solver, target-cost, rate, savings, waste, fraud, department-cut, technology-savings, and balanced-budget claims remain false.

Validation requirements:

- Raw source files and metadata files must exist.
- Raw byte counts and SHA-256 values must match the local captured files.
- Ledger row count must equal 17.
- Positive row count must equal 15 and negative offset row count must equal 2.
- Row sum must equal `7,011,105` million dollars with zero rounding residual.
- Commerce/housing credit and undistributed offsetting receipts must remain in fiscal reconciliation.
- Current-law reform deltas must all be zero.
- Blocked output values must remain null.
- Net interest must remain visible and cannot be marked directly cuttable.
