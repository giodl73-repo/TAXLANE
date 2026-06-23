# Wave: Accountability Readiness Report

## Goal

Generate a readable accountability evidence readiness report from validated records.

## Thesis

TAXLANE needs a visible review surface that shows what evidence exists, what still needs role review, and what is eligible for public claim wording without treating evidence records as allegations.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Generated readiness report | done | Added generated readiness report, stale-report validation, review, manifest coverage, and VTRACE closeout. |

## Success Criteria

- Report lists each accountability evidence record.
- Report shows readiness state and public summary.
- Validation detects stale report content.
- TAXLANE and VTRACE validation pass.

## Non-Goals

- Do not publish allegations.
- Do not add scoring.
- Do not replace role review.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo test
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
git diff --check
```
