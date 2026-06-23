# Performance Demand Response Bundle Applied Example JSON Schema

## Purpose

This schema documents `performance-demand-response-bundle.applied-example.json` fields.
The JSON is serialized from `PerformanceDemandResponseBundleManifest` and lists `PerformanceDemandResponseBundleArtifact` rows for importer and UI/API consumers.

## Manifest Fields

| Field | Type | Required | Meaning |
|---|---|---|---|
| `artifact` | string | yes | Repo-relative path for this bundle manifest JSON. |
| `bundle_kind` | string | yes | Fixed value `applied-response-importer-fixture`. |
| `total_rows` | integer | yes | Applied response-log row count from the fixture status. |
| `updated_rows` | integer | yes | Rows changed by applying example intake. Must not exceed `total_rows`. |
| `public_claim_allowed` | integer | yes | Must remain `0` for the applied fixture bundle. |
| `public_claim_blocked` | integer | yes | Blocked public-claim row count. With allowed count, must sum to `total_rows`. |
| `artifacts` | array | yes | Ordered applied fixture artifact entries. Must include intake, applied log, applied status, dashboard, handoff, applied schema, delta Markdown, delta JSONL, and delta schema artifacts. |
| `boundary` | string | yes | Fixture-only boundary statement. |
| `use_rule` | string | yes | Response tracking use rule; must match the core response-log use rule. |

## Artifact Fields

| Field | Type | Required | Meaning |
|---|---|---|---|
| `artifact` | string | yes | Repo-relative artifact path using forward slashes. |
| `role` | string | yes | Artifact role in the applied fixture bundle. |
| `kind` | string | yes | One of `jsonl`, `json`, or `markdown`. |
| `consumer_use` | string | yes | Intended importer or UI/API use. |

## Validation Rules

- JSON must deserialize as `PerformanceDemandResponseBundleManifest`.
- Every artifact entry must validate as `PerformanceDemandResponseBundleArtifact`.
- The manifest must include all required applied fixture artifacts.
- Public claims must remain blocked for this fixture manifest.

## Public-Use Rule

The manifest is fixture metadata only. It must not be used as canonical response status, public-claim eligibility, a finding of fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.
