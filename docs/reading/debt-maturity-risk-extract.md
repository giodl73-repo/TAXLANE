# Debt Maturity Risk Extract

## Purpose

This packet records the first Treasury Fiscal Data extract for the debt maturity
and rate-risk queue item.

Machine rows:
`data/derived/efficiency_pressure/extracts/debt_maturity_risk_first_pass.jsonl`.

## Extracted Context

| Source | Record date | Probe |
|---|---|---:|
| Debt to the Penny | 2026-06-29 | $39.345T total public debt outstanding |
| Debt to the Penny | 2026-06-29 | $31.621T debt held by the public |
| Average Interest Rates | 2026-05-31 | 3.386% total marketable average rate |
| Average Interest Rates | 2026-05-31 | 3.690% Treasury bills average rate |
| Average Interest Rates | 2026-05-31 | 7.577% Domestic Series average rate |

## Boundary

This is rate-risk context only. It is not a maturity distribution, not a
refinancing-risk score, not a savings estimate, and not a debt-management
recommendation. The next extract needs security amounts by maturity, refinancing
exposure, and CBO/Treasury scenario assumptions.
