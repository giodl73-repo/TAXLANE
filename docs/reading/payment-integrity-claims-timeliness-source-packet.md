# Payment Integrity Claims Timeliness Source Packet

## Purpose

This packet attaches reviewed source IDs to the payment-integrity cost-down
backlog item: `cost-down:payment-integrity:claims-timeliness`.

Machine rows:
`data/derived/efficiency_pressure/cost_down_source_packets.fy2025.v1.draft.jsonl`.

## What The Sources Support

| Source | Use |
|---|---|
| `SRC-SSA-PERFORMANCE` | SSA claims-processing and service-level context. |
| `SRC-SSA-DISABILITY-PROCESSING-TIME` | Monthly average processing-time data for combined Title II disability and Title XVI blind/disabled claims. |
| `SRC-VA-CLAIMS-DATA` | Veterans claims inventory, backlog, accuracy, and timeliness context. |
| `SRC-GAO-IMPROPER-PAYMENTS-2025` | Payment-integrity context where process quality and payment error interact. |

## What Is Still Missing

- Query-locked SSA and VA processing-time and backlog extracts.
- Administrative cost and staffing/process records tied to the same claims
  workflows.
- Accuracy, appeal, reversal, and beneficiary-access floor evidence.
- Reviewed scoring method before any savings or service-gain claim.

## Use Rule

Use this as a claims-timeliness source packet only. Faster processing is not a
savings estimate and cannot be treated as an improvement if accuracy, appeal
rights, or eligible access degrade.
