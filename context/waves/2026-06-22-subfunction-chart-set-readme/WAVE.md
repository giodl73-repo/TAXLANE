# Wave: Subfunction Chart Set README

## Goal

Add local handoff guidance beside the subfunction chart specs.

## Thesis

The top-level chart catalog has the public guardrails, but future UI or analysis
work may open the subfunction chart directory directly. A local README should
carry the drilldown boundary, reader links, wording rule, and partial-decade
caveat beside the specs themselves.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Subfunction chart README | done | Added local chart-set guidance and manifest coverage for the subfunction chart handoff. |

## Success Criteria

- Add `docs/charts/income-tax-outlay-subfunction-model/README.md`.
- Point the top-level chart catalog to the chart-set README.
- Include the README in generated manifest coverage.
- Preserve modeled-not-legal, financing-context, and partial-decade caveats.

## Non-Goals

- Do not change chart specs or generated data.
- Do not add UI rendering code.
- Do not approve taxpayer receipt wording.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
