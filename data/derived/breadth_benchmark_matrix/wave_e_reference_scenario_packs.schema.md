# Wave E Reference Scenario Packs Schema

This schema governs `wave_e_reference_scenario_packs.v1.draft.json`.

## Completion boundary

Wave E is complete when all fifteen analytical lanes have one current-policy
continuation reference calibration with:

- every declared component represented by a current-policy continuation treatment;
- a policy value copied by deterministic identity from the source-custodied Wave D anchor;
- a synthetic adverse stress value exactly one reported increment beyond the no-regression boundary;
- deterministic central and stress pass/fail results;
- populated policy-instrument, phase-in, behavior, transition/admin-cost, incidence, and score-provenance boundaries;
- an eight-role review approving the pack for comparator calibration only.

A reference calibration is a scored scenario pack because its performance value and pass/fail result are deterministic and reproducible. It is not a reform scenario or fiscal score.

## Required top-level fields

- identity: `record_id`, `record_family`, `version`, `status`, `pulse`, and `as_of_date`;
- lineage: schema, Wave D, Wave E rollup, and role-review paths;
- `completion_contract` and `calibration_rules`;
- exactly fifteen `lane_scenarios`;
- `aggregate_status`, `claim_booleans`, and `public_warning`.

## Lane scenario requirements

Each lane scenario must contain:

- stable scenario and lane identity;
- additivity treatment and every required component;
- one anchor floor with measure, unit, source packet, comparator, threshold, baseline, and reporting increment;
- a current-policy continuation reference with populated scenario-boundary fields;
- a same-policy synthetic adverse stress case;
- central pass, stress fail, and comparator-verification results;
- readiness and blocked-output objects;
- an explicit claim boundary.

For `at_or_above`, the central value passes when it is greater than or equal to the threshold and the stress value must equal threshold minus one reporting increment. For `at_or_below`, the central value passes when it is less than or equal to the threshold and the stress value must equal threshold plus one reporting increment.

## Claim boundary

Reference policy values are identity projections, and adverse stress values are comparator tests. Fiscal effects, reform scores, forecasts, lower-cost admissibility, target costs, solver inputs, rates, savings, technology-savings claims, and balanced-budget claims remain null or false.
