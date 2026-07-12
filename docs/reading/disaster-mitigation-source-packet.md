# Disaster Mitigation Source Packet

## Purpose

This packet attaches reviewed source IDs to the disaster-exposure cost-down
backlog item: `cost-down:disaster-exposure:mitigation`.

Machine rows:
`data/derived/efficiency_pressure/cost_down_source_packets.fy2025.v1.draft.jsonl`.

## What The Sources Support

| Source | Use |
|---|---|
| `SRC-GAO-HIGH-RISK-2025` | Federal disaster-assistance pressure, fragmentation, efficiency, and fiscal-exposure context. |
| `SRC-FEMA-BRIC` | Pre-disaster hazard-mitigation program context. |
| `SRC-FEMA-HMA-PROJECTS` | Queryable FEMA HMA project records, including project amount, federal share, benefit-cost ratio, status, program area, and geography fields. |
| `SRC-FEMA-DISASTER-DECLARATIONS` | Event-level declaration, geography, declaration type, and recovery-program context. |
| `SRC-OMB-HIST-3-2-FY2027` | FY2025 disaster relief and insurance subfunction outlay context. |

## What Is Still Missing

- Benefit-cost method details behind the query-locked FEMA mitigation project records.
- Project-level benefit-cost or avoided-loss method tied to the same hazard and
  geography.
- Event-to-account bridge from declaration records to federal outlays.
- Reviewed emergency-response and lawful-aid floor sources for affected
  communities.

## Use Rule

Use this as a disaster-mitigation source packet only. It is not a savings
estimate, not a finding that mitigation funds are waste, and not a reason to
delay emergency response or lawful recovery aid.
