# Payment Integrity Eligibility Source Packet

## Purpose

This packet attaches reviewed source IDs to the payment-integrity cost-down
backlog item: `cost-down:payment-integrity:eligibility-accuracy`.

Machine rows:
`data/derived/efficiency_pressure/cost_down_source_packets.fy2025.v1.draft.jsonl`.

## What The Sources Support

| Source | Use |
|---|---|
| `SRC-OMB-PAYMENTACCURACY` | Program-level improper-payment rates, dollar estimates, root causes, targets, and corrective-action context. |
| `SRC-GAO-IMPROPER-PAYMENTS-2025` | Government-wide improper-payment reporting context and methodology caveats. |
| `SRC-TREASURY-DO-NOT-PAY` | Pre-payment and post-payment matching context for potential improper payments. |
| `SRC-OMB-HIST-8-5-FY2027` | Mandatory-program outlay scale for affected benefit programs. |

## What Is Still Missing

- Query-locked PaymentAccuracy.gov program extracts with fiscal year and
  methodology notes.
- Program-specific root-cause and corrective-action records.
- Access, denial, appeal, reversal, and timeliness floor evidence for affected
  beneficiaries.
- Reviewed scoring method before any net savings estimate.

## Use Rule

Use this as a payment-integrity source packet only. Improper-payment estimates
are not automatically fraud, waste, abuse, collectible savings, or a reason to
block eligible people from lawful benefits.
