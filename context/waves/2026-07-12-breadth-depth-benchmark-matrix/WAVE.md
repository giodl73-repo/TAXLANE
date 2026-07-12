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
The third slice deepens veterans coverage from a two-component 4.98% subtotal
to the complete $377.163B / 5.38% function, with five components, service probes,
and explicit beneficiary/outcome gates before benchmarking.
The fourth slice deepens transportation from the 1.44% ground-only subtotal to
the complete $145.320B / 2.07% federal function and records the state/local,
trust-fund, asset-condition, project-delivery, and outcome gates for comparison.
The fifth slice reconciles the complete $72.042B / 1.03% education-work-social
services function and makes the −$35.005B higher-education net entry an explicit
account-level research gate rather than a performance or savings claim.

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
