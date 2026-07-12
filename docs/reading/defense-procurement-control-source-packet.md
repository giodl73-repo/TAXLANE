# Defense Procurement Control Source Packet

## Purpose

This packet attaches reviewed source IDs to the defense cost-down backlog item:
`cost-down:defense:procurement-control`.

Machine rows:
`data/derived/efficiency_pressure/cost_down_source_packets.fy2025.v1.draft.jsonl`.

## What The Sources Support

| Source | Use |
|---|---|
| `SRC-GAO-WEAPON-SYSTEMS-2025` | Acquisition-program risk and leading-practices context. |
| `SRC-CBO-FYDP-2025` | Planned defense-cost trajectory and projection context. |
| `SRC-OMB-HIST-6-1-FY2027` | OMB national-defense outlays as percent of GDP. |
| `SRC-NATO-DEFEXP-2025` | NATO-definition defense-burden comparison. |

## What Is Still Missing

- Program-level acquisition baseline and current cost/schedule extraction.
- Readiness and strategy floor source for affected capabilities.
- A reviewed scoring method before any savings estimate.
- Role-reviewed wording before any waste, performance, or acquisition-failure
  public claim.

## Use Rule

Use this as a procurement-control source packet only. It is not a savings
estimate, not a readiness judgment, and not a finding of waste, fraud, abuse, or
poor performance.
