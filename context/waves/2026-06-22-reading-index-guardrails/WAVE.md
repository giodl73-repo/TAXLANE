# Wave: Reading Index Guardrails

## Goal

Make the public reading index match the current modeled-allocation packet set.

## Thesis

The repo now has a foundation packet, broad modeled-outlay packet, and
subfunction drilldown packet. The index should state the reading order and keep
the modeled-not-legal boundary visible before readers land directly on a
drilldown packet.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Reading order guardrail | done | Added reading order and public-use wording guardrails to the reading index and root README. |

## Success Criteria

- Add packet reading order to `docs/reading/README.md`.
- State the public-use guardrail for modeled allocation wording.
- Update the root README's reader section to point to the broad and subfunction
  packets in sequence.

## Non-Goals

- Do not change model data, chart specs, or allocation math.
- Do not add a taxpayer receipt.
- Do not approve legal dedication claims.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
