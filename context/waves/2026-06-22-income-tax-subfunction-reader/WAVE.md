# Wave: Income-Tax Subfunction Reader

## Goal

Add the first reader-facing packet for the subfunction allocation model.

## Thesis

Subfunction charts are useful only if the public copy keeps the caveat visible:
subfunction labels are modeled allocation labels, not legal dedication or
program tracing. The first reader packet should make the drilldown useful while
keeping broad fiscal context first.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Subfunction reader packet | done | Added a modeled subfunction outlay packet, role review, reading index entry, and manifest coverage. |

## Success Criteria

- Add a public packet for modeled Table 3.2 subfunction allocation.
- Include source coverage, FY2025 drilldown examples, and explicit use/avoid
  wording.
- Add role review that blocks legal dedication and taxpayer receipt claims.
- Include reader/review artifacts in manifest coverage.

## Non-Goals

- Do not build a public UI.
- Do not create a taxpayer receipt.
- Do not change model math, chart specs, or extraction code.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
