# Pulse 77 — Income security/family scenario gate

## Decision

Add the income-security/family lane scenario-readiness gate without selecting a
benefit package, eligibility rule, take-up assumption, childcare policy, housing
support policy, tax rate, target cost, or savings claim.

## Added

- `income_security_family_scenario_gate.v1.draft.json`.
- Corresponding schema.
- Public reader.
- Validator checks for FY2025 context, missing package/take-up/federalism
  inputs, three scenario states, null/false policy fields, outcome-floor blocks,
  and blocked public claims.

## Boundary

The record does not publish a target cost, balanced rate, federal budget effect,
benefit design, child-poverty result, childcare-access result, work-transition
result, or savings claim.

## Next Gate

Load a benefit package, eligible population, take-up model, federal/state/local
translation, distribution model, administration and transition cost, and
outcome-floor thresholds for child poverty, material hardship, childcare access,
and work transition.
