# Wave: Broad Model README Validation Heading

## Goal

Remove duplicate validation section headings in the broad model README.

## Thesis

The broad model README already describes validation checks and later gives the
command to run aggregate validation. Those sections should have distinct names
so generated artifact documentation is easier to scan.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Validation heading cleanup | done | Renamed the final command-only validation heading and refreshed manifest coverage. |

## Success Criteria

- Keep the validation-check description intact.
- Rename the command-only heading to avoid duplicate `## Validation` sections.
- Refresh manifest coverage for the tracked README.

## Non-Goals

- Do not change model data, chart specs, or validation behavior.
- Do not change allocation math.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
