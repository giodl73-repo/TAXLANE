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
| `performance-demand-checklist.jsonl` | generated machine-readable demand checklist | One JSON row per evidence demand for future UI/API handoff. |
| `performance-demand-claim-gates.json` | generated demand claim-gate summary | Counts allowed and blocked demand rows for UI/API display. |
| `performance-demand-dashboard.md` | generated demand dashboard | Human-readable claim-gate summary for demand rows. |
| `performance-demand-brief.md` | generated demand ask packet | Compact citizen-facing ask packet for blocked demand rows. |
| `performance-demand-letter.md` | generated demand letter template | Public-safe request template for evidence and reviewed wording. |
| `performance-demand-response-rubric.md` | generated response rubric | Classifies replies without turning gaps into findings. |
| `performance-demand-followup.md` | generated demand follow-up template | Public-safe follow-up for partial or unclear replies. |
| `performance-demand-response-log.md` | generated demand response log | Tracks reply status and remaining evidence gaps. |
| `performance-demand-response-log.jsonl` | generated demand response log rows | Machine-readable response status rows for future UI/API handoff. |
| `performance-demand-response-log.schema.md` | response log row schema | Documents the response log JSONL row contract. |
| `performance-demand-response-status.json` | generated response status summary | Counts response-log and claim-gate status for UI/API display. |
| `performance-demand-checklist.schema.md` | demand checklist row schema | Documents the `PerformanceDemandChecklistRecord` row contract. |
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

The performance-demand checklist JSONL carries the same demand rows in a
machine-readable form and keeps `public_claim_allowed` explicit.

The performance-demand claim-gates JSON summarizes how many demand rows are
allowed or blocked before any public claim display.

The performance-demand dashboard renders those claim gates as a compact
human-readable summary.

The performance-demand brief turns blocked demand rows into a compact ask packet
without changing public-claim eligibility.

The performance-demand letter template adapts the same blocked rows into a
public-safe request for evidence and reviewed wording.

The performance-demand response rubric classifies replies to evidence requests
without converting missing evidence into findings.

The performance-demand follow-up template asks for missing evidence after a
partial, process-only, or no-evidence response.

The performance-demand response log tracks reply status and remaining evidence
gaps without changing claim gates.

The performance-demand response log JSONL carries the same response status rows
in a machine-readable form and keeps `public_claim_allowed` explicit.

The performance-demand response log schema documents the JSONL row fields and
response classes for future UI/API consumers.

The performance-demand response status JSON summarizes response-log counts and
blocked public-claim counts without recomputing JSONL rows.

The performance-demand checklist schema documents the JSONL row fields for
future UI/API consumers.

The artifact map explains which artifact to use for evidence review, workflow
handoff, or public reader questions.

The report's next-action column is operational guidance, not a public claim.
