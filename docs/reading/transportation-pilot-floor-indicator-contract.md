# Transportation Pilot Floor Indicator Contract

Machine record:
`data/derived/breadth_benchmark_matrix/transportation_pilot_floor_indicator_contract.v1.draft.json`.

This contract defines the floor indicators that the transportation asset
maintenance and safety pilot must satisfy before any lower-cost scenario can be
treated as admissible.

It is not a floor threshold decision, floor pass finding, completed baseline
path, simulator run, target-cost selection, rate calculation, rate publication,
public rate card, tax proposal, savings estimate, waste finding, fraud finding,
department-cut instruction, technology-savings claim, solver result,
modernization path, stress path, or balanced-budget claim.

The selected pilot remains transportation asset maintenance and safety under
the `transportation-infrastructure` lane.

Every lower-cost scenario must pass access and coverage, quality and safety,
equity and distribution, adequacy and resilience, delivery feasibility, and
transportation asset condition floors before it can affect a target cost.

No thresholds are set here. No floor pass finding is made here. Missing values remain null and blocked gates remain false.

The required floor families are:

- access and coverage;
- quality and safety;
- equity and distribution;
- adequacy and resilience;
- delivery feasibility;
- transportation asset condition.

Future indicator records must include floor ID, indicator ID, source family,
source ID, retrieval date, raw source path, raw byte count, raw SHA-256, period,
unit, perimeter, observed value, threshold value, comparison direction, pass
flag, missingness reason, and federal/state/local translation status.

Source custody is not closed. Indicator records remain empty. Thresholds are
not set. Floor passes are not recorded. Baseline path, modernization path,
stress path, and simulator readiness remain blocked.

International transportation differences are not savings. No fraud inference is allowed from international comparisons, source gaps, indicator gaps, or
improper-payment estimates.

Only the floor indicator contract is published. Source custody, indicator
records, thresholds, floor pass, simulator, target-cost, rate, public-card,
savings, waste, fraud, department cut, technology-savings, and balanced-budget
claims remain false.
