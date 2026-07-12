# Defense Audit Control Source Packet

## Purpose

This packet attaches reviewed source IDs to the defense cost-down backlog item:
`cost-down:defense:audit-control-closure`.

Machine rows:
`data/derived/efficiency_pressure/cost_down_source_packets.fy2025.v1.draft.jsonl`.

## What The Sources Support

| Source | Use |
|---|---|
| `SRC-DODIG-FY2025-AUDIT` | DoD FY2025 audit-control context, including material weaknesses and related audit signals. |
| `SRC-GAO-WEAPON-SYSTEMS-2025` | Acquisition-control context that can be paired with audit-control closure work. |
| `SRC-OMB-HIST-6-1-FY2027` | National-defense scale and GDP-share context. |

## What Is Still Missing

- A detailed audit finding/control inventory with corrective-action status.
- Repeat-finding and closure-date extraction.
- Mission/readiness floor evidence for affected control areas.
- A reviewed scoring method before treating control closure as savings.

## Use Rule

Use this as an audit-control source packet only. Material weaknesses are control
signals; they are not automatically savings estimates or findings of waste,
fraud, abuse, or poor performance.
