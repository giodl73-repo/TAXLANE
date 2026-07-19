# Current-law FY2025 fund-group path schema

Draft schema for `current_law_fy2025_fund_group_path.v1.draft.json`.

Required fields:

- Identity fields, source links, path IDs, batch IDs, fiscal year, year basis, and unit.
- `source_custody_status` with official-source, no-contact, FY2025-value-ready, named-fund-blocked, forward-year-blocked, and solver-blocked flags.
- `source_packet` for OMB Historical Table 1.4 with raw artifact, metadata, extracted artifact, byte counts, SHA-256 values, extraction method, annual coverage, component mapping, review status, and custody readiness.
- `fy2025_fund_group_rows` for total, federal funds, trust funds, and interfund transactions.
- `reconciliation` showing totals reconcile to Pulse 123 FY2025 values and that interfund transactions are preserved.
- `blocked_outputs` for general fund, named trust funds, forward fund values, interfund transfer schedule, solver inputs, policy deltas, target costs, rates, and public rate cards.
- `claim_booleans` allowing only source custody and FY2025 fund-group publication.

Validation requirements:

- Raw, metadata, and extracted source files must exist and match byte/hash values.
- Total receipts must equal `5,236,421` million dollars.
- Total outlays must equal `7,011,105` million dollars.
- Total deficit must equal `1,774,684` million dollars.
- Federal funds plus trust funds plus interfund transactions must reconcile for receipts and outlays.
- Federal funds cannot be marked as the general fund.
- Trust funds cannot be marked as named OASDI, Medicare HI, or transportation trust fund paths.
- Missing named-fund and forward-year fields must remain null.
