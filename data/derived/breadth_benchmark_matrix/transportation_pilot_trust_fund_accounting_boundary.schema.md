# Transportation pilot trust-fund accounting boundary schema

Schema for
`transportation_pilot_trust_fund_accounting_boundary.v1.draft.json`.

Required invariants:

- `record_id = transportation-pilot-trust-fund-accounting-boundary:v1`.
- The record uses the already-local OMB Appendix Chapter 13 funds PDF.
- Raw byte count and SHA-256 must be recomputed from the local source.
- `new_external_request_submitted` must remain `false`.
- The record may publish accounting-boundary concepts only; it must not publish
  annual trust-fund values.
- Trust funds remain separate and general-fund transfers must be explicit.
- Borrowing from the general fund is financing, not a receipt; repayment of that
  borrowing is not an outlay.
- The future row identity must preserve `primary_outlays`,
  `net_cash_requirement`, and `fund_balance_change`.
- Annual rows remain empty until Table 13-4 or an equivalent official annual
  source is captured locally.
- Missing numeric outputs remain `null`, never zero.
- Only `accounting_boundary_published` may be `true`; rate, savings, target,
  solver, waste, fraud, department-cut, technology-savings, and balanced-budget
  booleans must remain `false`.
