# Health Price Discipline Source Packet

## Purpose

This packet attaches reviewed source IDs to the first cost-down backlog item:
`cost-down:health-medicare:price-discipline`.

Machine row:
`data/derived/efficiency_pressure/cost_down_source_packets.fy2025.v1.draft.jsonl`.

## What The Sources Support

| Source | Use |
|---|---|
| `SRC-OECD-HEALTH-2025` | Health-cost benchmark pressure, with the government/compulsory scope caveat. |
| `SRC-JAMA-PAPANICOLAS-2018` | Literature support that US health cost pressure is substantially about prices and administration, not only utilization. |
| `SRC-CBO-LTBO` | Long-run federal health and Medicare budget-pressure context. |
| `SRC-CMS-MEDICARE-TRUSTEES-2026` | Medicare part-financing and enrollment context. |

## What Is Still Missing

- Program-specific CMS or HHS price/utilization extraction.
- A selected benchmark and case-mix adjustment method.
- Quality, access, and outcome floor sources for the affected population.
- A reviewed scoring method before any savings estimate.

## Use Rule

Use this as a source packet only. It is not a savings estimate, not a finding of
waste, fraud, abuse, or poor performance, and not a legal allocation of
income-tax dollars.
