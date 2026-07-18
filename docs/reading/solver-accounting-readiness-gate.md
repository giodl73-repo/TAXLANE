# Solver accounting readiness gate

Machine record:
`data/derived/breadth_benchmark_matrix/solver_accounting_readiness_gate.v1.draft.json`

Pulse 100 makes the boundary explicit: the aggregate FY2025 fund-group fixture
can be used for accounting tests, but it cannot run the solver.

Allowed uses:

- test the public rounding residual line;
- test that deficit is recorded as positive financing need;
- test aggregate trust-fund group balance arithmetic.

Blocked uses:

- transportation trust-fund values;
- solver run;
- target-cost selection;
- rate calculation;
- public rate card;
- tax proposal;
- savings estimate;
- waste finding;
- fraud finding;
- department-cut instruction;
- technology-savings claim;
- balanced-budget claim.

The deterministic solver remains blocked until lane/fund annual paths exist for
OASDI, Medicare HI, transportation trust, general fund, reserves, explicit
interfund transfers, credited offsets, assigned receipt bases, and endogenous
net interest.

Endogenous net interest remains missing.

endogenous net interest remains missing.
