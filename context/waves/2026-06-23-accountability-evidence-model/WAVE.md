# Wave: Accountability Evidence Model

## Goal

Define the evidence model for performance, waste, fraud, and abuse inquiry before TAXLANE makes public accountability claims.

## Thesis

TAXLANE should help people demand performance and investigate waste or fraud, but it must separate evidence signals from allegations. The next safe slice is a source-reviewed schema boundary and role review, not an automated fraud detector.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Accountability evidence schema | done | Added evidence-first accountability schema, boundary note, role review, manifest coverage, and VTRACE closeout for WP-TAX-002. |

## Success Criteria

- Schema distinguishes evidence kind, anomaly class, allegation status, review status, comparison basis, and due-process caveat.
- Public wording rules block unsupported fraud/waste accusations.
- VTRACE WP-TAX-002 is complete with evidence.
- TAXLANE and VTRACE validation pass.

## Non-Goals

- Do not create public fraud allegations.
- Do not add automated scoring.
- Do not add Rust validators until the domain crate split starts.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay manifest
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
git diff --check
```
