# Pulse 124 — Current-law FY2025 fund-group path

## Scope

Add source-custodied FY2025 OMB fund-group actuals.

## Added

- `data/derived/breadth_benchmark_matrix/current_law_fy2025_fund_group_path.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/current_law_fy2025_fund_group_path.schema.md`
- `docs/reading/current-law-fy2025-fund-group-path.md`
- Rust validator and focused regression test

## Data populated

| Fund group | Receipts | Outlays | Surplus/deficit |
|---|---:|---:|---:|
| total | `$5,236.421B` | `$7,011.105B` | `-$1,774.684B` |
| federal funds | `$3,413.497B` | `$5,284.502B` | `-$1,871.005B` |
| trust funds | `$3,009.025B` | `$2,912.704B` | `$96.321B` |
| interfund transactions | `-$1,186.101B` | `-$1,186.101B` | null |

## Boundary

No external request was submitted and no agency or person was contacted.

Federal funds are not relabeled as general fund. Trust funds are not split into OASDI, Medicare HI, or transportation trust funds. Forward fund values remain blocked.

No solver input, target cost, rate, public rate card, tax proposal, savings estimate, waste finding, fraud finding, department-cut instruction, technology-savings claim, or balanced-budget claim is made.
