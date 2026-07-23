# Income-security/family federal program perimeter bridge schema

`income_security_family_federal_program_perimeter_bridge.fy2025.v1.draft.json`
records the narrow FY2025 federal account perimeter for OMB function 600 using
the already-captured Public Budget Database outlays workbook.

Required shape:

- `record_family` is `income_security_family_federal_program_perimeter_bridge`.
- `source_custody` records raw path, byte count, SHA-256, metadata path,
  retrieval date, source unit, and record unit.
- `perimeter_definition` states included subfunctions and the federal versus
  state/local boundary.
- `reconciliation` reconciles OMB Historical Table 3.2 and PBD totals.
- `subfunction_totals` includes subfunctions 601, 602, 603, 604, 605, and 609.
- `claim_booleans.fy2025_federal_account_perimeter_source_custody_ready` may be
  true, but all downstream model, floor, solver, rate, savings, department-cut,
  technology-savings, and balanced-budget claims must remain false.

This record is source custody for a federal account perimeter only. It does not
complete source capture, federal/state/local translation, benefit package
design, take-up modeling, floor values, pass/fail findings, target costs, solver
inputs, rates, savings, or balanced-budget claims.
