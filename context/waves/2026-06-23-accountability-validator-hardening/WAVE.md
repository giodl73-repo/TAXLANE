# Wave: Accountability Validator Hardening

## Goal

Harden Rust validation so accountability records can support fraud, waste, abuse, and performance inquiry without allowing premature public accusations.

## Thesis

TAXLANE should let people demand performance on public money, but the core validator must enforce allegation boundaries before public wording or possible-misconduct labels are accepted.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Core claim guardrails | done | Added Rust guardrails for draft possible-misconduct signals and accusation-style public wording, plus tests and VTRACE closeout. |

## Success Criteria

- Draft `possible_fraud`, `possible_waste`, and `possible_abuse` records are rejected.
- Accusation-style public wording is rejected without official/adjudicated status.
- Reviewed, non-accusatory evidence signals remain allowed.
- TAXLANE and VTRACE validation pass.

## Non-Goals

- Do not add scoring.
- Do not claim fraud or waste from variance alone.
- Do not replace role review with phrase matching.

## Validation

Run:

```powershell
cargo test
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
git diff --check
```
