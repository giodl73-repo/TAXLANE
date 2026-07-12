# Wave: Efficiency Pressure System

## Goal

Turn "there must be waste somewhere" into a disciplined TAXLANE workflow:
identify high-outlay efficiency targets, route each target to the right evidence,
track whether the issue is price, volume, administration, eligibility, interest,
procurement, duplication, or event-driven cost, and keep public claims blocked
until a reviewed source supports them.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Efficiency pressure framework | done | Added a research note that separates efficiency pressure from waste findings. |
| 02 | Candidate pressure records | done | Added draft rows for major FY2025 pressure surfaces and required evidence. |
| 03 | Public accountability bridge | done | Added a reader packet and machine-readable cost-down backlog for pressure-linked levers. |
| 04 | Validation hardening | done | Added typed validation for efficiency pressure and cost-down backlog records. |

## Status

Active. Pulses 01-04 establish the frame, pressure rows, public backlog packet,
and Rust validation. Next work should attach reviewed source packets for one
lever at a time before any savings estimate is allowed.

## Design Rules

- "Waste" requires a reviewed finding or source-specific evidence.
- "Efficiency pressure" can be assigned from outlay size, benchmark gap, growth,
  solvency risk, or missing performance evidence.
- Every pressure row must identify the evidence needed to drive costs down.
- Do not treat cuts as savings unless the record protects the public purpose,
  outcome floor, or legal obligation.

## Validation

```powershell
cargo test
cargo run -p taxlane-tools -- income-tax-outlay validate
git diff --check
```
