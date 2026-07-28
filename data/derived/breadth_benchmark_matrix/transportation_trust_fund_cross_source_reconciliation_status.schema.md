# Transportation trust-fund cross-source reconciliation status schema

`transportation_trust_fund_cross_source_reconciliation_status.v1.draft.json`
rolls up transportation trust-fund reconciliation status across OMB Table 13-4,
CBO trust-fund open data, and Treasury MTS context.

Required checks:

- The record links the OMB Table 13-4 context, aggregate context, identity
  diagnostic, Treasury MTS FY2025 anchor, and CBO FY2032-FY2035 balance
  extension context.
- Source custody includes OMB, CBO, and Treasury local artifact paths, byte
  counts, SHA-256 values, retrieval dates, and review statuses.
- The record states that OMB Table 13-4 internally reconciles for
  FY2025-FY2031 and that CBO FY2032-FY2035 rows are balance context only.
- The FY2031 OMB/CBO overlap boundary blocks stitching CBO balances onto OMB
  Table 13-4 as a continuous path.
- Complete path, FY2032-FY2035 income/outgo, Function 400 mapping, solver,
  rate, savings, technology-savings, and balanced-budget claims remain blocked.
