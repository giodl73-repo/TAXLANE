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
| `performance-demand-response-dashboard.md` | generated response dashboard | Human-readable response-log and claim-gate counts. |
| `performance-demand-response-handoff.md` | generated response handoff | Routes response artifacts by reader/implementer task. |
| `performance-demand-response-intake.md` | generated response intake template | Captures reply evidence before response-log updates. |
| `performance-demand-response-intake.schema.md` | response intake row schema | Documents the response intake field contract. |
| `performance-demand-response-intake.example.jsonl` | generated response intake example rows | Exercises the typed intake-to-log importer handoff. |
| `performance-demand-response-log.applied-example.jsonl` | generated applied response log example rows | Shows response-log rows after applying the intake example. |
| `performance-demand-response-status.applied-example.json` | generated applied response status | Counts response-log rows after applying the intake example. |
| `performance-demand-response-dashboard.applied-example.md` | generated applied response dashboard | Summarizes applied response-log counts for importer review. |
| `performance-demand-response-handoff.applied-example.md` | generated applied response handoff | Routes applied response importer fixture artifacts by task. |
| `performance-demand-response-applied-example.schema.md` | applied response fixture schema | Documents the applied importer fixture artifact contract. |
| `performance-demand-response-delta.applied-example.md` | generated applied response delta | Shows exact row-level changes after applying the intake example. |
| `performance-demand-response-delta.applied-example.jsonl` | generated applied response delta rows | Machine-readable row changes after applying the intake example. |
| `performance-demand-response-delta.applied-example.schema.md` | applied response delta row schema | Documents the applied delta JSONL row contract. |
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

The performance-demand response dashboard renders those response-log counts as
a compact human-readable summary.

The performance-demand response handoff explains which response artifact to use
for scanning, logging, classification, follow-up, and UI/API handoff.

The performance-demand response intake template records source custody,
classification, and remaining-evidence fields before response-log updates.

The performance-demand response intake schema documents the field contract for
future UI/API consumers that capture received replies.

The performance-demand response intake example JSONL is a generated importer
fixture. Validation parses it as typed intake rows, applies each row to the
matching response-log row through `taxlane-core`, and checks that the updated
log row remains blocked for public claims.

The performance-demand response log applied example JSONL shows the typed
response-log rows after applying the example intake rows. It is an importer
fixture only; it is not the canonical response log and does not authorize public
claims.

The performance-demand response status applied example JSON summarizes the
applied example rows without recomputing them downstream. It remains an importer
fixture and keeps allowed public claims at zero.

The performance-demand response dashboard applied example Markdown renders the
same applied counts for quick inspection. It remains an importer fixture and is
not the canonical response dashboard.

The performance-demand response handoff applied example Markdown routes the
applied importer fixture artifacts by task. It remains implementation guidance
for fixtures, not public accountability wording.

The performance-demand response applied example schema documents the fixture
artifact contract for importer consumers. It does not add claim authority.

The performance-demand response delta applied example compares canonical
response-log rows to applied fixture rows. It shows changed fields only and
keeps the blocked public-claim gate visible.

The performance-demand response delta applied example JSONL carries the same
changed fields as typed `PerformanceDemandResponseDeltaRow` records for future
UI/API consumers.

The performance-demand response delta applied example schema documents the
JSONL row fields and the blocked-claim guardrails for importer consumers.

The performance-demand checklist schema documents the JSONL row fields for
future UI/API consumers.

The artifact map explains which artifact to use for evidence review, workflow
handoff, or public reader questions.

The report's next-action column is operational guidance, not a public claim.
