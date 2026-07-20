# Medicare HI OMB/CMS receipt-row perimeter evidence schema

Draft schema for
`medicare_hi_omb_cms_receipt_row_perimeter_evidence.v1.draft.json`.

Required identity fields:

- `record_id = medicare-hi-omb-cms-receipt-row-perimeter-evidence:v1`
- `record_family = medicare_hi_omb_cms_receipt_row_perimeter_evidence`
- `schema_version = v1.draft`
- `pulse = 149`

Required analytical fields:

- contract, closure queue, reconciliation, payroll-tax perimeter, and official
  source-capture paths.
- custody booleans showing existing official-source custody and no external
  contact or new download.
- work-queue item 1 with `completed = false`, `ready = false`, and
  `value = null`.
- three evidence rows for CMS payroll taxes, OMB Hospital Insurance receipt
  anchor, and CMS taxation-of-OASDI-benefits context.
- recomputed difference and share formulas.
- seven perimeter-bridge requirement rows with exactly two ready context rows
  and five blocked rows.
- blocked outputs with null values.
- public warning phrases.
- claim booleans with only
  `medicare_hi_omb_cms_perimeter_evidence_published` true.
