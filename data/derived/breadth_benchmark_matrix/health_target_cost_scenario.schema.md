# Health Target-Cost Scenario Boundary Schema

Machine record: `health_target_cost_scenario.v1.draft.json`.

This record hardens a mechanical CY2024 private-insurance payer-payment
sensitivity without converting it into a target cost, savings estimate,
federal score, premium forecast, or provider-revenue forecast.

Required structure:

- `contract_path`, `rubric_path`, and the ordered seven-record evidence chain;
- source custody, with missing custody forcing A1 false;
- CY2024 category bases, reference periods, exact formula, and reconciliation;
- exactly four sensitivity paths: current, modest, central, and aggressive;
- a Grade C perimeter and null federal translation on perimeter mismatch;
- null behavior, transition-cost, administration, and incidence fields;
- all five mandatory outcome-floor classes and all-false A1-A7 gates;
- explicit blockers and all-false target-cost, federal-effect, gross-savings,
  net-savings, balanced-rate, and solver-readiness booleans; and
- exclusion of the legacy $395.046B flat federal illustration from solver and
  savings outputs.

Category arithmetic uses:

```text
payer payment base * (target percent of Medicare / reference percent of Medicare - 1)
```

Covered share uses:

```text
covered private-insurance payments / total private-insurance payments * 100
```

Null means unmodeled or unavailable and must never become zero. False means a
gate has not passed and must never be inferred from mechanical arithmetic.
