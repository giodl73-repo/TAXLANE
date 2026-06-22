# Table 3.1 Outlay Function Profile

## Source Coverage

- Outlay source: `SRC-OMB-HIST-3-1-FY2027`
- Reconciliation source: `SRC-OMB-HIST-1-1-FY2027`
- Fiscal years emitted: 1940-2025
- Year count: 86
- Record count: 602
- Actual/projection treatment: actual years only; FY2026-FY2031 are excluded.

## Extracted Rows

| Function code | OMB label | Table 3.1 row |
|---|---|---:|
| `national-defense` | National Defense | 4 |
| `human-resources` | Human resources | 5 |
| `physical-resources` | Physical resources | 14 |
| `net-interest` | Net interest | 22 |
| `other-functions` | Other functions | 25 |
| `undistributed-offsetting-receipts` | Undistributed offsetting receipts | 32 |
| `total-federal-outlays` | Total, Federal outlays | 35 |

## Reconciliation Sample

Amounts are in millions of dollars. Broad category total is the sum of the six visible Table 3.1 rows above.

| Fiscal year | Table 1.1 outlays | Table 3.1 total | Broad category total | Table total diff | Broad category diff |
|---:|---:|---:|---:|---:|---:|
| 1940 | 9,468 | 9,468 | 9,468 | 0 | 0 |
| 1950 | 42,562 | 42,562 | 42,562 | 0 | 0 |
| 1980 | 590,941 | 590,941 | 590,941 | 0 | 0 |
| 2000 | 1,788,950 | 1,788,950 | 1,788,950 | 0 | 0 |
| 2025 | 7,011,105 | 7,011,105 | 7,011,105 | 0 | 0 |

## Extraction Decisions

- Net interest is extracted as its own visible outlay function.
- Undistributed offsetting receipts are extracted as negative amounts with `offsetting_treatment = "undistributed-offsetting-receipts"`.
- Function codes are TAXLANE slugs because Table 3.1 uses labels, not OMB numeric function codes.
- No public lane allocation should use these draft rows.
