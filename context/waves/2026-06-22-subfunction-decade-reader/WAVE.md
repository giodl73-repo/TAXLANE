# Wave: Subfunction Decade Reader

## Goal

Fold the subfunction decade rollup into the reader-facing drilldown packet.

## Thesis

The decade export and chart spec are useful only if public wording keeps the
same modeled-not-legal boundary as the annual subfunction views. The reader
packet should name the decade artifact, explain cumulative-decade shares, and
label partial decades before the chart is used outside analysis notes.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Decade reader guardrails | done | Added decade artifact coverage, long-run wording, partial-decade caveats, and role-review checks. |

## Success Criteria

- Add the decade-long CSV to the subfunction reader's data map.
- Add concise decade interpretation language backed by the generated rollup.
- Extend the role review to cover the decade chart and partial-decade caveats.
- Refresh manifest hashes for tracked reader and review artifacts.

## Non-Goals

- Do not change generated model data or chart specs.
- Do not introduce taxpayer receipt wording.
- Do not approve a reform proposal.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
