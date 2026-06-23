# Accountability Artifact Map

## Purpose

This map shows which accountability artifact to use for evidence review, performance-demand questions, and public-safe reader handoff.
It is not a list of fraud, waste, abuse, or performance findings.

## Use Order

1. Start with the draft JSONL records for source custody.
2. Use readiness, queue, demand, work-item, and claim-guard artifacts for internal review workflow.
3. Use public questions and the public brief only for outward-facing questions and handoff wording.

## Artifacts

| Artifact | Audience | Use | Avoid |
|---|---|---|---|
| `accountability_evidence.omb-fy2027-v1.2026-06-23.draft.jsonl` | Internal evidence reviewers | Validate source-custodied evidence shape. | Do not publish as findings. |
| `readiness-report.md` | Accountability researchers | See readiness and next action per record. | Do not treat readiness as a performance score. |
| `action-queue.md` | Review leads | Work records by next task. | Do not publish queue rows as claims. |
| `performance-demand-packet.md` | Accountability researchers | Ask what evidence, reviewed wording, or official finding is missing. | Do not allege misconduct. |
| `accountability-work-items.jsonl` | Product implementers | Feed future UI/API workflow from structured fields. | Do not infer public eligibility except from `public_claim_allowed`. |
| `claim-guard-report.md` | Review leads | Check allowed versus blocked public claims. | Do not publish findings from blocked records. |
| `public-questions.md` | Citizen readers | Ask safe public questions about performance evidence. | Do not expose raw draft evidence as claims. |
| `performance-demand-checklist.md` | Citizen readers | Demand source, performance, official-finding, wording, and claim-gate evidence. | Do not treat demand rows as findings. |
| `performance-demand-dashboard.md` | Citizen readers | Scan demand-row claim gates before public use. | Do not publish blocked rows as claims. |
| `performance-demand-brief.md` | Citizen readers | Use a compact ask packet for current blocked demand rows. | Do not present the brief as a finding or scorecard. |
| `performance-demand-letter.md` | Citizen readers | Adapt a public-safe evidence request template. | Do not send it as an accusation or legal conclusion. |
| `performance-demand-response-rubric.md` | Citizen readers | Classify replies to evidence requests. | Do not turn incomplete replies into findings. |
| `performance-demand-followup.md` | Citizen readers | Send a narrower follow-up for missing evidence. | Do not escalate missing evidence into accusations. |
| `performance-demand-response-log.md` | Citizen readers | Track replies and remaining missing evidence. | Do not treat log status as a finding. |
| `performance-demand-checklist.jsonl` | Product implementers | Feed demand rows into future UI/API surfaces. | Do not infer public eligibility except from `public_claim_allowed`. |
| `performance-demand-claim-gates.json` | Product implementers | Display allowed versus blocked demand-row counts. | Do not recompute or override claim gates downstream. |
| `performance-demand-checklist.schema.md` | Product implementers | Inspect the demand checklist row contract. | Do not add UI/API fields that weaken the use rule. |
| `docs/reading/accountability-public-brief.md` | Citizen readers | Read the current public handoff. | Do not describe modeled allocation as legal dedication. |

## Public-Use Rule

Public artifacts may ask for performance evidence and official findings. They must not claim fraud, waste, abuse, legal dedication of income taxes, or program performance without reviewed evidence and claim eligibility.
