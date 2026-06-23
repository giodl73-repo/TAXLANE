# Accountability Performance Demand Packet Review

## Scope

This review covers the generated accountability performance demand packet:

| Artifact | Role |
|---|---|
| `data/derived/accountability_evidence/performance-demand-packet.md` | Generated questions for asking what evidence is needed before performance claims. |
| `tools/taxlane/src/main.rs` | Demand-question generation and stale-output validation. |

## Decision

Accept the packet as a safe public-accountability question surface.

The packet helps a reader demand performance evidence and reviewed wording
without claiming that fraud, waste, abuse, or poor performance occurred.

## Role Findings

| Role | Finding |
|---|---|
| Taxpayer Advocate | Pass: questions are plain enough for a citizen to ask and keep the evidence gap visible. |
| Reform Skeptic | Pass: the packet asks for evidence instead of making reform or misconduct claims. |
| Source Custodian | Pass: questions are generated from records that validation already ties to ledgered sources. |
| Maintainer | Pass: validation checks the generated packet for staleness. |

## Guardrails

- Use the packet to ask for reviewed evidence, not to assert outcomes.
- Do not convert missing evidence into a performance failure claim.
- Do not convert source custody into public wording until role review approves the exact text.
