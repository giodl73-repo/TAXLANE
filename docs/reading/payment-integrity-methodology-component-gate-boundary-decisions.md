# Payment Integrity Methodology Component Gate Boundary Decisions

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_boundary_decisions_q4_2025.jsonl`

This packet records internal boundary decisions over component gate source
capture rollups.

## Decisions

| Program | Priority | Decision |
|---|---:|---|
| USDA Federal Crop Insurance Program | 1 | additional positive basis required |
| USDA Federal Crop Insurance Program | 2 | additional positive basis required |
| VA PLTSS | 1 | narrow process boundary supported internally |
| VA PLTSS | 2 | additional positive basis required |

The VA PLTSS process boundary only says incorrect-amount post-payment reviews
can result in bills of collection. It does not quantify recoverable dollars and
does not unlock scoring.

Priority 2 additionally records the VA-wide classification rules, but no PLTSS
cause is assigned to a recoverability or collection outcome.

## Boundary

These rows do not close fields, do not score programs, do not estimate savings,
do not identify waste, do not identify fraud, and do not claim recoverable
amounts.
