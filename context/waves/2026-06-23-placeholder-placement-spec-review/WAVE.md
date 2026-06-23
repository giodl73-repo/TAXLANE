# Wave: Placeholder Placement Spec Review

## Goal

Run role review on the static placeholder receipt placement spec.

## Thesis

The placement spec defines how charts, financing context, and caveats must stay
together. Before any mockup or UI work depends on it, the spec needs a role
review that records the accepted boundary and remaining blockers.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Placement spec role review | done | Added role review for the static placement spec and included it in manifest coverage. |

## Success Criteria

- Review accepts the placement spec only as a static design handoff.
- Review blocks public release, interactive UI, calculator fields, and legal
  tracing claims.
- Manifest coverage includes the new review.

## Non-Goals

- Do not build a mockup or UI.
- Do not change chart specs, receipt values, or required copy.
- Do not approve public release.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay manifest
cargo run -p taxlane-tools -- income-tax-outlay validate
git diff --check
```
