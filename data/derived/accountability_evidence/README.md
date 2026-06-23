# Accountability Evidence Records

## Purpose

This directory contains draft `accountability_evidence` records used to prove
source custody and validator behavior before TAXLANE publishes accountability
claims.

## Current Artifact

| Artifact | Grain | Status |
|---|---|---|
| `accountability_evidence.omb-fy2027-v1.2026-06-23.draft.jsonl` | reviewed and draft evidence records | Draft; source-custody and baseline-gap bootstrap only. |
| `readiness-report.md` | one generated readiness and next-action summary | Generated from draft evidence records. |
| `action-queue.md` | one generated reviewer action queue | Groups draft records by the next evidence or review task. |
| `performance-demand-packet.md` | one generated accountability question packet | Turns blockers into evidence-request questions without making claims. |
| `accountability-work-items.jsonl` | generated machine-readable work items | One JSON row per evidence record for future UI/API handoff. |
| `claim-guard-report.md` | one generated claim-readiness summary | Counts allowed and blocked public claims without scoring records. |
| `public-questions.md` | one generated public-safe question packet | Strips records down to evidence-request questions and blockers. |
| `performance-demand-checklist.md` | one generated public evidence checklist | Turns blockers into concrete evidence requests before accepting claims. |
| `artifact-map.md` | one generated accountability artifact map | Routes each artifact to its audience, use, and public-use boundary. |

## Public-Use Rule

These records are not fraud findings, waste findings, abuse findings, or
performance scores. Public wording must preserve `allegation_status`,
`review_status`, source IDs, comparison basis, and due-process caveat.

Rust readiness classification uses:

- `EvidenceOnly` for internal evidence records.
- `NeedsRoleReview` for reviewed records that still require public wording
  review.
- `PublicClaimEligible` only when role review and official/adjudicated status
  both exist.

The readiness report is generated from the JSONL records and checked by:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
```

The action queue is also generated from the same JSONL records and checked by
the same validation command. Queue entries are internal review tasks, not public
claims.

The performance-demand packet uses the same validated records to ask what
evidence, reviewed wording, or official finding is still needed. It is a
question packet, not an accusation packet.

The work-items JSONL is generated from `taxlane-core` workflow helpers. It is
the machine-readable handoff for future UI or API code and carries
`public_claim_allowed` as an explicit boolean.

The claim-guard report summarizes the same work items for review leads. It
counts readiness and blockers; it does not publish findings.

The public questions packet is the narrowest outward-facing handoff. It exposes
questions and blockers only, not the full evidence records.

The performance-demand checklist turns those blockers into evidence requests a
citizen can make before accepting performance or misconduct claims.

The artifact map explains which artifact to use for evidence review, workflow
handoff, or public reader questions.

The report's next-action column is operational guidance, not a public claim.
