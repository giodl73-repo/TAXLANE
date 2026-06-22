# Income-Tax Outlay Model Source Profile

## Source Coverage

- Model ID: `individual-income-tax-proportional-outlays-v1`
- Fiscal years emitted: 1940-2025
- Year count: 86
- Record count: 516
- Actual/projection treatment: actual years only; FY2026-FY2031 are excluded.

## Source Roles

| Source ID | Use |
|---|---|
| `SRC-OMB-HIST-1-1-FY2027` | Total receipts, total outlays, and surplus/deficit. |
| `SRC-OMB-HIST-2-1-FY2027` | Individual income-tax receipts. |
| `SRC-OMB-HIST-3-1-FY2027` | Broad outlay categories and total federal outlays. |

## Broad Categories

| Category key | OMB label | Table 3.1 row |
|---|---|---:|
| `national-defense` | National Defense | 4 |
| `human-resources` | Human resources | 5 |
| `physical-resources` | Physical resources | 14 |
| `net-interest` | Net interest | 22 |
| `other-functions` | Other functions | 25 |
| `undistributed-offsetting-receipts` | Undistributed offsetting receipts | 32 |

## Reconciliation Sample

All amounts are in millions of dollars. `Modeled sum` is the sum of
the six category allocation rows for the fiscal year.

| Fiscal year | Table 1.1 outlays | Table 3.1 outlays | Category total | Income tax receipts | Modeled sum | Deficit gap |
|---:|---:|---:|---:|---:|---:|---:|
| 1940 | 9,468 | 9,468 | 9,468 | 892 | 892.000 | 2,920 |
| 1950 | 42,562 | 42,562 | 42,562 | 15,755 | 15,755.000 | 3,119 |
| 1960 | 92,191 | 92,191 | 92,192 | 40,715 | 40,715.000 | 0 |
| 1970 | 195,649 | 195,649 | 195,649 | 90,412 | 90,412.000 | 2,842 |
| 1980 | 590,941 | 590,941 | 590,941 | 244,069 | 244,069.000 | 73,829 |
| 1990 | 1,252,993 | 1,252,993 | 1,252,995 | 466,884 | 466,884.000 | 221,035 |
| 2000 | 1,788,950 | 1,788,950 | 1,788,950 | 1,004,462 | 1,004,462.000 | 0 |
| 2010 | 3,457,079 | 3,457,079 | 3,457,079 | 898,549 | 898,549.000 | 1,294,373 |
| 2020 | 6,516,817 | 6,516,817 | 6,516,817 | 1,608,663 | 1,608,663.000 | 3,095,653 |
| 2025 | 7,011,105 | 7,011,105 | 7,011,105 | 2,656,044 | 2,656,044.000 | 1,774,684 |

## Model Caveat

These rows allocate individual income-tax receipts by reported outlay
share, normalized over the displayed broad-category rows when source
rounding creates a small difference from the displayed total. They do
not claim that income-tax dollars were legally dedicated to the listed
outlay categories.
