# Transportation Pilot Baseline Path Contract

Machine record:
`data/derived/breadth_benchmark_matrix/transportation_pilot_baseline_path_contract.v1.draft.json`.

This contract defines the current-law baseline path required before the
transportation asset-maintenance and safety pilot can run a deterministic
simulator.

It is not a completed baseline path, simulator run, target-cost selection, rate calculation, rate publication, public rate card, tax proposal, savings estimate,
waste finding, fraud finding, department-cut instruction, technology-savings
claim, solver result, modernization path, stress path, floor threshold decision,
or balanced-budget claim.

The selected pilot remains transportation asset maintenance and safety under
the `transportation-infrastructure` lane.

The required baseline horizon is FY2025 through FY2035, including the baseline
year. Every annual current-law row must carry zero reform delta under unrounded
arithmetic.

The FY2025 anchor comes from the existing transportation depth card:

- total transportation outlays: $145.320B;
- ground transportation: $100.827B;
- air transportation: $29.743B;
- water transportation: $13.852B;
- other transportation: $0.898B.

The component sum equals $145.320B. This anchor is not a multi-year baseline and
does not make the simulator ready.

Every future annual row must include gross program outlays, implementation/admin
outlays, credited offsetting collections, dedicated receipts, explicit general-fund transfer, other scored fund income, reserve contribution, net cash
requirement, fund balance change, federal/state/local translation status, source
vintage, raw source path, byte count, SHA-256, unrounded value status, and zero
current-law reform delta.

Transportation trust funds remain separate. Explicit interfund transfers and
credited offsetting collections must be recorded. State, local, private, and
user-financed activity remains separate until translated.

Baseline rows remain empty. Source custody, annual rows, trust-fund
reconciliation, federal/state/local translation, unrounded values, floor
indicators, modernization path, stress path, and simulator readiness remain
blocked.

Only the baseline contract is published. Completed baseline, source custody,
simulator, target-cost, rate, public-card, savings, waste, fraud, department
cut, technology-savings, and balanced-budget claims remain false.
