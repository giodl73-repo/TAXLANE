# Disaster Supplemental Tracking Source Packet

## Purpose

This packet attaches reviewed source IDs to the disaster-exposure cost-down
backlog item: `cost-down:disaster-exposure:supplemental-tracking`.

Machine rows:
`data/derived/efficiency_pressure/cost_down_source_packets.fy2025.v1.draft.jsonl`.

## What The Sources Support

| Source | Use |
|---|---|
| `SRC-OMB-HIST-3-2-FY2027` | Annual disaster relief and insurance subfunction outlay context. |
| `SRC-TREASURY-MTS` | Current-period receipt and outlay context after query-date locking. |
| `SRC-USASPENDING` | Program, account, and award exploration after query parameters are recorded. |
| `SRC-FEMA-DISASTER-DECLARATIONS` | Event identifiers, declaration type, dates, programs, and geography. |
| `SRC-GAO-HIGH-RISK-2025` | Federal disaster-assistance fragmentation, efficiency, and data-sharing context. |

## What Is Still Missing

- Query-locked Treasury, USAspending, and FEMA extracts with periods and
  parameters recorded.
- Event-to-account and supplemental-to-base budget crosswalk.
- Definition of recurring exposure versus one-time emergency response reviewed
  against agency sources.
- Reviewed scoring method before treating better tracking as savings.

## Use Rule

Use this as a supplemental-tracking source packet only. It is not a savings
estimate, not a finding of waste, and not a reason to slow emergency response or
lawful recovery aid.
