# Transportation pilot trust-fund source custody schema

Schema for `transportation_pilot_trust_fund_source_custody.v1.draft.json`.

Required invariants:

- `record_id = transportation-pilot-trust-fund-source-custody:v1`.
- `record_family = transportation_pilot_trust_fund_source_custody`.
- The source must be the already-local OMB Appendix Chapter 13 funds PDF.
- Raw byte count and SHA-256 must be recomputed from the local file.
- `new_external_request_submitted` must remain `false`.
- `custody_complete_for_local_fund_source` may be `true`, but
  `custody_complete_for_trust_fund_reconciliation` must remain `false`.
- Trust funds must remain separate, explicit general-fund transfers must be
  required, and credited offsetting collections must be required.
- Extracted value rows remain empty until annual amounts are extracted and
  reconciled.
- Missing annual trust-fund values, fund-balance identities, and crosschecks
  remain blocked; missing values remain `null`, never zero.
- Only `local_source_custody_published` may be `true`; all rate, savings, target,
  solver, fraud, waste, technology-savings, department-cut, and balanced-budget
  booleans must remain `false`.
