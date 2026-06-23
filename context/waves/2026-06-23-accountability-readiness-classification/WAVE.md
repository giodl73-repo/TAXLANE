# Wave: Accountability Readiness Classification

## Goal

Add typed Rust readiness states for accountability evidence records.

## Thesis

TAXLANE needs to separate internal evidence review from public claims in code. A typed readiness classification lets later reports and interfaces ask whether a record is evidence-only, needs role review, or is eligible for public claim wording.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Public-claim readiness states | done | Added `PublicClaimReadiness` in `taxlane-core`, tests, schema docs, review, and VTRACE closeout. |

## Success Criteria

- Source-reviewed records classify as `NeedsRoleReview`.
- Role-reviewed official/adjudicated records classify as `PublicClaimEligible`.
- TAXLANE and VTRACE validation pass.

## Non-Goals

- Do not publish public claims.
- Do not replace exact wording review.
- Do not add scoring.

## Validation

Run:

```powershell
cargo test
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
git diff --check
```
