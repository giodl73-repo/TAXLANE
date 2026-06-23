# Performance Demand Response Applied Example Schema

## Purpose

This schema note documents the generated response importer fixture artifacts.
It does not authorize public claims, findings, or canonical response-log updates.

## Artifact Roles

| Artifact | Role | Guardrail |
|---|---|---|
| `performance-demand-response-intake.example.jsonl` | Source-custodied intake fixture row parsed as `PerformanceDemandResponseIntakeRecord`. | Must keep `role_review_needed: true`, `public_claim_allowed: false`, and the intake use rule. |
| `performance-demand-response-log.applied-example.jsonl` | Response-log rows after applying intake fixture rows through `PerformanceDemandResponseLogRecord::apply_intake`. | Must validate as response-log records and keep `Public claim blocked.`. |
| `performance-demand-response-status.applied-example.json` | Compact counts aggregated from applied response-log rows through `PerformanceDemandResponseStatus`. | Must report zero allowed public claims and at least one updated row. |
| `performance-demand-response-dashboard.applied-example.md` | Human-readable applied status summary. | Must state fixture-only and no-finding boundaries. |
| `performance-demand-response-handoff.applied-example.md` | Task routing for importer fixture consumers. | Must not describe applied examples as canonical status or public-claim eligibility. |
| `performance-demand-response-delta.applied-example.md` | Row-level comparison between canonical response-log rows and applied example rows. | Must show changed fields while preserving blocked public-claim gates. |

## Importer Rule

Importers may use these artifacts to test response intake handling. They must not treat example rows as real agency replies, public fraud/waste/abuse findings, legal dedication of income taxes, poor-performance findings, or reform benefits.
