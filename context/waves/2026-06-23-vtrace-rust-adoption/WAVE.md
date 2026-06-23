# Wave: VTRACE And Rust Adoption

## Goal

Adopt VTRACE traceability for TAXLANE's current receipt/display posture and
define the Rust crate boundary needed before accountability and fraud/waste
checks become code.

## Thesis

TAXLANE is moving from explanation and static receipt handoffs toward
performance accountability and fraud/waste detection. That work needs an
auditable chain from mission to requirements, evidence, validation, and review,
plus explicit Rust crate boundaries before new analysis logic is encoded.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | VTRACE first slice | done | Added TAXLANE-specific VTRACE mission, requirements, specs, trace, work packages, evidence, verification, validation, and review. |

## Success Criteria

- VTRACE package names income-tax explanation, legibility, accountability, and
  fraud/waste evidence boundaries.
- VTRACE work packages identify accountability/fraud evidence modeling and the
  future Rust domain crate split.
- TAXLANE validation and VTRACE validation pass.
- VTRACE package is included in generated manifest coverage.

## Non-Goals

- Do not build fraud detection claims yet.
- Do not add taxpayer input or calculator surfaces.
- Do not split Rust crates before the crate boundary work package starts.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay manifest
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
git diff --check
```
