# Denominator Requirements

## Purpose

This draft family records which denominator is required before TAXLANE can show a
per-person, per-worker, per-taxpayer, per-beneficiary, or per-enrollee number.

It prevents false precision. A per-person display is not valid unless the
denominator is sourced and named.

## Artifacts

| Artifact | Role |
|---|---|
| `denominator_requirements.v1.draft.jsonl` | Required denominators by display type. |
| `denominator_values.ty2022.irs-soi-1304.draft.jsonl` | Sourced TY2022 IRS SOI return-count denominators for taxpayer-basis displays. |
| `denominator_values.cy2025.cms-medicare-trustees-2026.draft.jsonl` | Sourced CY2025 Medicare Part A/B/D and total-beneficiary denominators from the Medicare Trustees report. |
| `denominator_values.cy2025.census.draft.jsonl` | Sourced CY2025 Census resident-population and household denominators. |
| `denominator_values.cy2025.ssa-trustees-2026.draft.jsonl` | Sourced rounded CY2025 Social Security covered-worker and beneficiary denominators. |
| `per_unit_display_readiness.v1.draft.jsonl` | Per-unit display readiness rows with ready, illustrative, and blocked statuses. |
| `per-unit-display-readiness.md` | Reader-facing readiness dashboard. |
| `per_unit_receipt_cards.v1.draft.jsonl` | Public-card rows derived from readiness rows. |
| `denominator_requirements.schema.md` | Field contract. |
| `docs/research/2026-06-28-denominator-source-ladder.md` | Source ladder for remaining population, worker, beneficiary, enrollee, and household denominators. |

## Validation

```powershell
git diff --check
```
