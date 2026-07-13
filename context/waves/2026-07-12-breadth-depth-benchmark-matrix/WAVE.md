# Wave: Breadth, Depth, And Benchmark Matrix

## Goal

Create one controlled scoreboard showing which fiscal areas TAXLANE covers,
which have current and benchmark values on matched definitions, and which still
need depth. Keep efficiency comparisons, improper payments, fraud, and
recoverable savings as distinct evidence classes.

## Status

Active in depth mode. Breadth is closed: all 17 questions across 13 lanes have a
sourced current top line and zero remain Tier 3 gaps. Five questions (29.4%)
have a matched benchmark; 12 (70.6%) have a federal top line but still need a
scope- and outcome-matched expected value.

Retained depth work includes the headline-basis crosswalk; veterans,
transportation, education, disaster, justice, science/energy/environment,
agriculture, and international-affairs cards; the health decomposition and
benchmark ladder; fiscal paths and first-order debt dynamics; policy scale
baskets; and the distribution screen. The public scoreboard is canonical for
coverage status and the prioritized depth queue.

The next bounded slice should deepen one existing Tier 2 row rather than add a
new breadth category. Priority is the payment-integrity bridge or one blocked
accounting bridge, with improper payments, confirmed fraud, recovery, and
prospective preventable loss kept separate.

The payment-integrity bridge now captures the official FY2024 annual workbook
and reconciles payment-type composition across 68 program rows. Confirmed-fraud,
recovery, and net-savings links remain blocked, as does a VA PLTSS program
headline until the annual-workbook and Q4-scorecard conflict is resolved.

## Design rules

- Never invent an expected value for a policy-dependent lane.
- Match unit, scope, and period before computing a comparison.
- Treat peer averages as comparisons, not automatic targets.
- Never infer fraud from an international cost difference.
- Never convert improper payments into recoverable savings without separate
  reviewed evidence.

## Validation

```powershell
cargo test
cargo run -p taxlane-tools -- income-tax-outlay validate
git diff --check
```

## Current pulse

`pulses/pulse-01-current-state-reconciliation.md` records the exact handoff and
next decision gate.
