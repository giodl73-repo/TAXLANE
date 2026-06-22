# Wave: Subfunction README Validation Caveat

## Goal

Add validation and decade-rollup caveats to the generated subfunction model
README.

## Thesis

The subfunction model README is a direct handoff for generated artifacts. It
should state how to run aggregate validation and make the partial-decade
boundary visible beside the decade-long export.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Subfunction README caveats | done | Updated the Rust-generated README template with decade-rollup caveats and the validation command, then refreshed generated artifacts and manifest coverage. |

## Success Criteria

- Update the Rust README template, not only the generated Markdown.
- Regenerate the subfunction model README.
- Refresh manifest coverage.
- Keep allocation math and data rows stable.

## Non-Goals

- Do not change model data semantics.
- Do not change chart specs.
- Do not approve taxpayer receipt wording.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
