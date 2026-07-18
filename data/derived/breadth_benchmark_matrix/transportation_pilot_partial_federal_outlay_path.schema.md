# Transportation Pilot Partial Federal Outlay Path Schema

Canonical artifact:
`transportation_pilot_partial_federal_outlay_path.v1.draft.json`.

Validation rules:

- The raw OMB Public Budget Database outlays file must already exist locally.
- The recorded byte count and SHA-256 must match the local raw file.
- No new external request may be marked submitted.
- The path covers FY2025-FY2031 only; FY2032-FY2035 remain explicit missing
  rows with null values.
- Raw workbook values are thousands of dollars and must be converted to
  millions by dividing by 1,000.
- Each annual row must have component sum equal total and zero reform delta.
- FY2025 must match the FY2025 anchor custody record: 145,320 million.
- Federal/state/local translation, trust-fund reconciliation, and gross-to-net
  reconciliation remain null.
- Every output placeholder remains null.
- Only `partial_federal_net_outlay_path_published` may be true; full baseline,
  floors, modernization, stress, simulator, target-cost, rates, savings, waste,
  fraud, department-cut, technology-savings, and balanced-budget claims remain
  false.
