# Wave: Fund Group Review

## Goal

Extract and review OMB fund-group records so TAXLANE can explain receipt and
outlay context without overclaiming legal dedication or taxpayer-dollar tracing.

## Thesis

Receipt categories alone do not answer where money is legally or structurally
credited. OMB Table 1.4 gives a fund-group view, but it must be handled as
budget-accounting context, not proof that every broad receipt source is legally
dedicated to a program.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Fund-group schema alignment | done | Profiled OMB Table 1.4 and added explicit `interfund-transactions` controlled value. |
| 02 | Fund-group first draft rows | done | Extracted 1934 fund-group receipt/outlay/surplus-deficit draft rows and reconciled totals. |
| 03 | Fund-group source review | done | Verified Table 1.4 checksum, row anchors, source labels, signs, and extracted 1934 values. |
| 04 | Fund concept guardrail | done | Recorded Table 1.4 public-use limits, legal-dedication blockers, trust-fund caveats, and interfund transaction handling. |
| 05 | Receipt allocation blocker note | pending | Summarize how fund groups affect future taxpayer receipt allocation. |

## Success Criteria

- Table 1.4 source labels are preserved.
- `interfund-transactions` is represented explicitly.
- Fund-group totals reconcile to Table 1.1 for the reviewed years.
- Legal dedication remains `unknown` unless a budget-concept source supports it.
- No taxpayer allocation claim is made.

## Non-Goals

- Do not build a taxpayer receipt in this wave.
- Do not claim individual income taxes are legally dedicated to a fund.
- Do not use fund groups as program-level lanes.
- Do not extract all years at once.

## Validation

Run:

```powershell
git diff --check
```

For JSONL extraction pulses, parse draft records and reconcile totals.
