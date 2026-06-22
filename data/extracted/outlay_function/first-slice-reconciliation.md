# OMB Outlay Function First Slice Reconciliation

This note documents the initial 1940-1942 slice. The current full actual-year
Table 3.1 extraction is documented in `table-3-1-profile.md`.

## Source

- Outlay source: `SRC-OMB-HIST-3-1-FY2027`
- Fiscal spine: `SRC-OMB-HIST-1-1-FY2027`
- Schema: `docs/data/outlays-lanes-schema.md`

## Slice

The first draft outlay extraction covers fiscal years 1940-1942 from OMB
Historical Table 3.1:

- National Defense
- Human resources
- Physical resources
- Net interest
- Other functions
- Undistributed offsetting receipts
- Total, Federal outlays

All records are `draft-extracted`.

## Reconciliation

| Fiscal year | Table 3.1 total outlays | Table 1.1 total outlays | Difference |
|---:|---:|---:|---:|
| 1940 | 9,468 | 9,468 | 0 |
| 1941 | 13,653 | 13,653 | 0 |
| 1942 | 35,137 | 35,137 | 0 |

Amounts are in millions of dollars.

## Extraction Decisions

- Net interest is extracted as its own visible outlay function.
- Undistributed offsetting receipts are extracted as negative amounts with
  `offsetting_treatment = "undistributed-offsetting-receipts"`.
- Function codes are TAXLANE slugs because Table 3.1 uses labels, not OMB
  numeric function codes.
- No public lane allocation should use these draft rows.
