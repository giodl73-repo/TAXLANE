# Performance Demand Response Applied Example Bundle

## Purpose

This generated bundle index gives importer and UI/API consumers one place to find every applied response fixture artifact.
It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.

## Applied Fixture Summary

- Response rows: 2
- Updated rows: 1
- Not-yet-received rows: 1
- Public claims currently allowed: 0
- Public claims currently blocked: 2

## Bundle Artifacts

| Artifact | Role | Consumer Use |
|---|---|---|
| `performance-demand-response-intake.example.jsonl` | Source-custodied intake fixture row. | Exercise importer parsing and record-id matching. |
| `performance-demand-response-log.applied-example.jsonl` | Response-log rows after applying example intake. | Inspect typed applied rows without changing canonical response status. |
| `performance-demand-response-status.applied-example.json` | Compact applied response counts. | Feed fixture counts into UI/API tests without recomputing rows. |
| `performance-demand-response-dashboard.applied-example.md` | Human-readable applied response counts. | Scan importer behavior without opening JSON. |
| `performance-demand-response-handoff.applied-example.md` | Task routing for the applied fixture set. | Choose the right applied artifact by implementation task. |
| `performance-demand-response-applied-example.schema.md` | Fixture artifact contract. | Confirm roles and guardrails for applied importer artifacts. |
| `performance-demand-response-delta.applied-example.md` | Human-readable changed fields. | Inspect row-level changes after applying example intake. |
| `performance-demand-response-delta.applied-example.jsonl` | Machine-readable changed fields. | Feed delta rows into UI/API diff consumers. |
| `performance-demand-response-delta.applied-example.schema.md` | Delta row field contract. | Confirm field meanings and blocked-claim guardrails. |

## Boundary

Bundle artifacts are importer fixtures, not canonical response status. Do not use them as public-claim eligibility, misconduct findings, performance findings, or proof of reform benefits.

## Use Rule

Track response status and remaining evidence gaps; do not claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.
