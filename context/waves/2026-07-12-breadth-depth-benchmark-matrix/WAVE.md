# Wave: Breadth, Depth, And Benchmark Matrix

## Goal

Create one controlled scoreboard showing which fiscal areas TAXLANE covers,
which have current and benchmark values on matched definitions, and which still
need depth. Keep efficiency comparisons, improper payments, fraud, and
recoverable savings as distinct evidence classes.

## Status

Active. The first slice defines the typed matrix, initial dominant-lane
comparisons, Tier 2 toplines, explicit breadth gaps, public scoreboard, and Rust
claim-safety validation. The second slice adds a headline-basis crosswalk for
interest, defense, and health so gross/net, function/subfunction, federal/system,
and outlay/GDP measures cannot be silently substituted.

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
