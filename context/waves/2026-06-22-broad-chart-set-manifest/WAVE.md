# Wave: Broad Chart Set Manifest

## Goal

Include the broad chart-set README in artifact manifest coverage.

## Thesis

The broad modeled-outlay chart directory already has local handoff guidance, but
the generated manifest did not track it. Manifest coverage should include both
chart-set README files so chart consumers can verify the guidance that travels
with each spec family.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Broad chart README manifest coverage | done | Added the broad chart-set README to manifest coverage and linked its local handoff role from the catalog. |

## Success Criteria

- Include `docs/charts/income-tax-outlay-model/README.md` in generated manifest
  coverage.
- Point the top-level chart catalog to local handoff rules for the broad chart
  set.
- Preserve existing chart specs and generated data.

## Non-Goals

- Do not change chart specs or model output.
- Do not add new chart families.
- Do not approve taxpayer receipt wording.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
