# Accountability Public Brief Review

## Scope

This review covers the generated reader-facing accountability public brief:

| Artifact | Role |
|---|---|
| `docs/reading/accountability-public-brief.md` | Reader-facing bridge between modeled income-tax visibility and accountability questions. |
| `data/derived/accountability_evidence/public-questions.md` | Public-safe source questions. |
| `tools/taxlane/src/main.rs` | Public brief generation and stale-output validation. |

## Decision

Accept the brief as the current public-facing accountability handoff.

The brief states what TAXLANE can safely say now: income-tax allocation is a
modeled visibility tool, accountability records are not findings, and the public
can ask for reviewed performance evidence or role-approved wording.

## Role Findings

| Role | Finding |
|---|---|
| Taxpayer Advocate | Pass: the brief gives a concise use/avoid table and plain questions. |
| Reform Skeptic | Pass: the brief blocks legal dedication claims and unsupported misconduct claims. |
| Source Custodian | Pass: accountability questions remain generated from validated evidence records. |
| Maintainer | Pass: validation checks the generated brief for staleness. |

## Guardrails

- Keep this brief as a question surface until records become public-claim eligible.
- Do not add program performance scores without reviewed performance evidence.
- Do not add fraud, waste, or abuse claims without official/adjudicated status and role-approved wording.
