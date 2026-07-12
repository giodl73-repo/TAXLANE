# Payment Integrity Methodology Component Gate Progress Source Queries

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_queries_q4_2025.jsonl`

This packet turns component gate progress source targets into concrete
official-source search queries.

## Queries

| Program | Priority | Query focus |
|---|---:|---|
| VA PLTSS | 1 | Bills of collection, debts, receivables, recoveries, and collectible overpayments. |
| VA PLTSS | 2 | Medical-review recoverability category split for incorrect-amount findings. |

The queries are only instructions for source discovery. They do not themselves
provide evidence, close fields, or support scoring.

## Boundary

These rows do not close fields, do not score PLTSS, do not estimate savings, do
not identify waste, do not identify fraud, and do not claim recoverable amounts.
