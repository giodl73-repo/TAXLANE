# Accountability Evidence Records

## Purpose

This directory contains draft `accountability_evidence` records used to prove
source custody and validator behavior before TAXLANE publishes accountability
claims.

## Current Artifact

| Artifact | Grain | Status |
|---|---|---|
| `accountability_evidence.omb-fy2027-v1.2026-06-23.draft.jsonl` | reviewed and draft evidence records | Draft; source-custody and baseline-gap bootstrap only. |
| `readiness-report.md` | one generated readiness summary | Generated from draft evidence records. |

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
