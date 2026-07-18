# Pulse 76 — Social Security scenario gate

## Decision

Add the Social Security lane scenario-readiness gate without selecting a
benefit, wage-base, rate, eligibility-age, or solvency target.

## Added

- `social_security_scenario_gate.v1.draft.json`.
- Corresponding schema.
- Public reader.
- Validator checks for FY2025 context, required missing inputs, three scenario
  states, null/false policy levers, and blocked public claims.

## Boundary

The record does not publish a target cost, balanced rate, solvency reform,
benefit cut, wage-base policy, tax rate, eligibility-age change, or savings
claim. Social Security remains a separate OASDI trust-fund lane.

## Next Gate

Load a demographic and 75-year trust-fund path, then evaluate explicit wage-base,
rate, eligibility, and formula levers against adequacy, old-age-poverty,
disability/survivor, distribution, administration, and transition-cost floors.
