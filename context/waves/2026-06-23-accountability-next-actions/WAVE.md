# Wave: Accountability Next Actions

## Goal

Add deterministic next actions to the accountability readiness report.

## Thesis

Evidence readiness is more useful when each record states the next review step. TAXLANE should guide users toward source review, role review, or performance-baseline collection without scoring programs or publishing allegations.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Readiness next actions | done | Added generated next-action column, review, and VTRACE closeout. |

## Success Criteria

- Readiness report includes a next-action column.
- Next actions are generated from validated record state.
- No next action claims fraud, waste, abuse, or poor performance.
- Validation passes.

## Non-Goals

- Do not add scores.
- Do not publish allegations.
- Do not bypass role review.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo test
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
git diff --check
```
