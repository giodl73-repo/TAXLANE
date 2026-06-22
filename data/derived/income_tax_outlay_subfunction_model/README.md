# Individual Income-Tax Outlay Subfunction Model

## Purpose

This derived model estimates, by fiscal year and OMB Table 3.2 subfunction, how ordinary individual income-tax receipts would be allocated if allocated in proportion to that year's reported subfunction outlays.

This is a visibility model. It is not a legal dedication, appropriation rule, or program-financing claim.

## Model ID

`individual-income-tax-proportional-subfunction-outlays-v1`

## Inputs

| Source ID | Role |
|---|---|
| `SRC-OMB-HIST-2-1-FY2027` | Individual income-tax receipt amount by fiscal year. |
| `SRC-OMB-HIST-3-2-FY2027` | Function and subfunction outlays by fiscal year. |

## Coverage

The first draft model covers fiscal years 1962-2025, the overlap between Table 3.2 actual-year subfunction rows and Table 2.1 individual income-tax receipt rows.

## Artifacts

| Artifact | Role |
|---|---|
| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.draft.jsonl` | Canonical annual modeled allocation rows by Table 3.2 subfunction. |
| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.annual-long.csv` | Chart-ready long CSV view with one row per fiscal year and subfunction. |
| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.decade-long.csv` | Chart-ready decade rollup by subfunction. |
| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.fy2025-top-subfunctions.csv` | Chart-ready FY2025 ranked view for the largest modeled subfunction allocations. |

## Method

For each fiscal year and emitted Table 3.2 subfunction:

```text
outlay_share_percent = subfunction_outlays / total_federal_outlays * 100
allocation_share_percent = subfunction_outlays / sum_of_subfunction_outlays * 100
modeled_income_tax_allocation = individual_income_tax_receipts
                                * subfunction_outlays
                                / sum_of_subfunction_outlays
```

The allocation denominator uses the emitted subfunction rows so modeled rows sum back to individual income-tax receipts. Small differences from displayed total outlays are source rounding.

## Regeneration

```powershell
cargo run -p taxlane-tools -- income-tax-outlay subfunction-model
cargo run -p taxlane-tools -- income-tax-outlay subfunction-model --check
cargo run -p taxlane-tools -- income-tax-outlay subfunction-export
cargo run -p taxlane-tools -- income-tax-outlay subfunction-export --check
```
