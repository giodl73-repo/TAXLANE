# Solver input readiness rollup

Machine record:
`data/derived/breadth_benchmark_matrix/solver_input_readiness_rollup.v1.draft.json`

Pulse 107 summarizes the deterministic solver input state after the reserve,
net-interest, assigned receipt-base, and distribution placeholder contracts.

The rollup does not make the solver ready. Every solver input remains not ready and null.

Newly linked but still blocked:

- reserves path: contract and parameter gate only;
- net interest formula: formula contract only, inputs still missing;
- assigned receipt bases: inventory only, amounts and behavior still missing;
- distributional effect placeholder: placeholder only, values still missing.

Still missing or partial:

- OASDI annual fund path;
- Medicare HI annual fund path;
- transportation trust-fund annual values;
- baseline year plus ten-year unified horizon;
- explicit interfund transfer annual amounts;
- credited offsetting collections by lane and fund;
- explicit deficit gap.

This is a solver-input readiness rollup, not a solver run, not target-cost selection, not rate calculation, not a public rate card, not a tax proposal, not a savings estimate, not a waste finding, not a fraud finding, not a department-cut instruction, not a technology-savings claim, and not a balanced-budget claim.
