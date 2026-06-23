# Wave: Accountability Evidence-Only Record

## Goal

Add a second accountability evidence record that demonstrates the `EvidenceOnly` readiness path.

## Thesis

TAXLANE should let researchers record missing performance evidence before making performance claims. A baseline-gap record can show where accountability work is needed without implying fraud, waste, abuse, or poor performance.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Evidence-only baseline gap | done | Added health-lane baseline-gap record, refreshed readiness report, review, and VTRACE closeout. |

## Success Criteria

- Readiness report includes both `EvidenceOnly` and `NeedsRoleReview` rows.
- New record validates through source custody and core shape checks.
- Public wording remains non-accusatory.

## Non-Goals

- Do not claim health spending is wasteful or fraudulent.
- Do not add performance scores.
- Do not publish public accusations.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo test
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
git diff --check
```
