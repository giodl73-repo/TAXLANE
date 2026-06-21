# OMB Fund Group Budget Interpretation

## Sources

- Numeric fund-group rows: `SRC-OMB-HIST-1-4-FY2027`
- Fiscal reconciliation: `SRC-OMB-HIST-1-1-FY2027`
- Budget concepts: `SRC-OMB-AP-13-FUNDS-FY2027`
- Concept records:
  `data/extracted/budget_concept/SRC-OMB-AP-13-FUNDS-FY2027/budget_concept.SRC-OMB-AP-13-FUNDS-FY2027.2026-06-21.source-reviewed.jsonl`

## Interpretation Decisions

| Fund group | `legal_dedication` | `appropriation_required` | Reason |
|---|---|---|---|
| `total` | `mixed` | `mixed` | Total rows combine federal-funds, trust-funds, and interfund components. |
| `federal-funds` | `mixed` | `mixed` | AP13 says Federal funds include the general fund plus special and revolving funds. |
| `trust-funds` | `dedicated` | `unknown` | AP13 says trust funds are designated by law and receive collections dedicated by law for specific purposes, but it does not define the appropriation rule for each trust fund. |
| `interfund-transactions` | `unknown` | `unknown` | Table 1.4 reports these as transactions between fund groups; AP13 concept rows do not make them ordinary receipts or outlays. |

## Public Use

These interpreted rows support statements that OMB reports 1934 receipts,
outlays, and surplus/deficit by fund group and that the group labels have
budget-accounting meaning. They still do not support tracing an individual
income-tax payment to a program or treating Federal Funds as identical to the
General Fund.

The general-fund concept is now source-reviewed, but Table 1.4 does not expose a
separate 1934 general-fund column. General-fund statements should be made from
receipt-source and budget-concept records, not by relabeling Table 1.4 Federal
Funds rows.
