# Wave: Placeholder Display Packet Manifest

## Goal

Bring the static placeholder receipt display packet into manifest coverage.

## Thesis

The display packet is now part of the artifact chain that explains how the
placeholder receipt chart specs may be used. It should be tracked by the
generated manifest so later display changes update the canonical artifact
inventory and validation catches stale hashes.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Display packet manifest coverage | done | Added the display packet to the Rust manifest inventory and refreshed chart reading order. |

## Success Criteria

- Manifest generation includes the display packet.
- Chart catalog points users from the prototype packet to the stricter static
  display handoff.
- Validation passes without changing model values or chart specs.

## Non-Goals

- Do not change receipt amounts or chart inline values.
- Do not build a UI or calculator.
- Do not approve public release.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay manifest
cargo run -p taxlane-tools -- income-tax-outlay validate
git diff --check
```
