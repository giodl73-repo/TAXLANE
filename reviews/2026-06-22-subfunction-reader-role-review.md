# Subfunction Reader Role Review

## Scope

This review covers the first reader-facing subfunction allocation packet and
chart-spec handoff:

| Artifact | Role |
|---|---|
| `docs/reading/modeled-income-tax-subfunction-outlays.md` | Public drilldown packet. |
| `docs/charts/income-tax-outlay-subfunction-model/fy2025-top-subfunctions.vl.json` | FY2025 ranked chart spec. |
| `docs/charts/income-tax-outlay-subfunction-model/selected-subfunction-trends.vl.json` | Selected subfunction trend chart spec. |
| `docs/charts/income-tax-outlay-subfunction-model/decade-top-subfunctions.vl.json` | Decade ranked chart spec. |
| `data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.decade-long.csv` | Decade subfunction rollup. |

## Findings

| Role | Result |
|---|---|
| Taxpayer Advocate | Pass: the packet explains what the model does and gives a plain-language "use/avoid" wording pair. |
| Budget Accountant | Pass: fiscal-year basis, OMB Table 2.1/Table 3.2 source spine, and modeled-not-legal status remain visible. |
| Source Custodian | Pass: the packet names the canonical JSONL and CSV artifacts and keeps source coverage to OMB FY2027 historical tables. |
| Public-Goods Steward | Pass with caution: subfunctions improve visibility, but the packet tells readers to start with broad-model context. |
| Program Beneficiary | Pass with caution: beneficiary-facing labels are not presented as dedicated funding claims. |
| Compliance Burden | Pass: no individual tax advice, liability calculation, filing guidance, or compliance recommendation is introduced. |
| Fiscal Sustainability | Pass: net interest remains visible as financing cost rather than public-service spending. |
| Reform Skeptic | Pass: the packet states that subfunction labels raise overclaim risk and that the model is only proportional allocation. |

## Deficit Context Addendum

| Check | Result |
|---|---|
| Financing context visible | Pass: the reader packet now requires broad-model borrowed-share and income-tax-coverage context beside public subfunction charts. |
| Chart-only public use blocked | Pass: standalone subfunction charts are treated as analysis artifacts unless paired with financing context or linked broad-model context. |
| Taxpayer receipt boundary | Pass: the packet continues to block "your income taxes paid this program" wording. |

## Decade Rollup Addendum

| Check | Result |
|---|---|
| Partial-decade caveat visible | Pass: the reader packet labels the 1960s and 2020s buckets as partial and blocks ten-year comparisons for those buckets. |
| Rollup method visible | Pass: the packet describes decade shares as cumulative modeled allocation dollars within the decade, not annual-rank averages. |
| Long-run wording bounded | Pass: the packet describes a shift in modeled largest subfunctions without implying legal dedication or taxpayer-dollar tracing. |

## Decision

The subfunction reader packet is acceptable as a drilldown companion to the
broad modeled-outlay packet.

This review does not approve a taxpayer receipt, legal dedication claim,
program-level tracing claim, or reform proposal.

## Follow-Up

- Any taxpayer-facing receipt must still label allocation method and legal
  status at the point of display.
