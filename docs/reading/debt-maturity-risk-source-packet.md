# Debt Maturity Risk Source Packet

## Purpose

This packet attaches reviewed source IDs to the debt-interest cost-down backlog
item: `cost-down:debt-interest:maturity-risk`.

Machine rows:
`data/derived/efficiency_pressure/cost_down_source_packets.fy2025.v1.draft.jsonl`.

## What The Sources Support

| Source | Use |
|---|---|
| `SRC-CBO-LTBO` | Projected net-interest pressure and interest-rate sensitivity context. |
| `SRC-TREASURY-DEBT-PENNY` | Dynamic debt-stock context after query-date locking. |
| `SRC-TREASURY-AVG-INTEREST` | Dynamic Treasury average-interest-rate context after query-date locking. |
| `SRC-OMB-HIST-3-2-FY2027` | FY2025 gross Treasury interest outlay context. |

## What Is Still Missing

- Query-locked Treasury debt-stock and average-rate extracts.
- Maturity distribution or refinancing exposure source suitable for the scenario.
- Reviewed CBO or Treasury interest-rate assumptions for the same horizon.
- A scoring method that turns maturity/rate exposure into risk context without
  claiming savings.

## Use Rule

Use this as a fiscal-risk source packet only. It is not a savings estimate, not
a recommendation to delay debt service, and not a finding of waste, fraud,
abuse, or poor performance.
