# Debt Primary Balance Source Packet

## Purpose

This packet attaches reviewed source IDs to the debt-interest cost-down backlog
item: `cost-down:debt-interest:primary-balance`.

Machine rows:
`data/derived/efficiency_pressure/cost_down_source_packets.fy2025.v1.draft.jsonl`.

## What The Sources Support

| Source | Use |
|---|---|
| `SRC-CBO-LTBO` | Long-run debt, deficit, and net-interest trajectory; projections must be labeled. |
| `SRC-OMB-HIST-1-2-FY2027` | Annual receipts, outlays, deficits, and GDP-share context. |
| `SRC-OMB-HIST-3-2-FY2027` | FY2025 gross Treasury interest outlay context. |
| `SRC-TREASURY-DEBT-PENNY` | Dynamic debt-stock context after query-date locking. |

## What Is Still Missing

- A chosen scoring baseline and projection window.
- Policy-specific revenue and spending assumptions.
- A debt-stock path and interest-rate assumptions tied to the same scenario.
- A reviewed rule for translating primary balance changes into debt-service
  changes without double counting.

## Use Rule

Use this as a fiscal-balance source packet only. It is not a savings estimate,
not a program cut, and not a finding of waste, fraud, abuse, or poor
performance.
