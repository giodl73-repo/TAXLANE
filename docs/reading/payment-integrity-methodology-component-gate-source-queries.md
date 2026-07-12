# Payment Integrity Methodology Component Gate Source Queries

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_queries_q4_2025.jsonl`

This packet turns component gate source targets into concrete official-source
search queries.

## Queries

| Program | Priority | Query focus |
|---|---:|---|
| USDA Federal Crop Insurance Program | 1 | Recoveries, collections, receivables, debts, or CARS treatment. |
| USDA Federal Crop Insurance Program | 2 | Compliance review, data mining, corrective action, and preventable-dollar treatment. |
| VA PLTSS | 1 | Incorrect-amount overpayments, bills of collection, and recoverability. |
| VA PLTSS | 2 | Medical-review category splits separating recoverable and non-recoverable categories. |

The queries are only instructions for source discovery. They do not themselves
provide evidence, close fields, or support scoring.

## Boundary

These rows do not close fields, do not score programs, do not estimate savings,
do not identify waste, do not identify fraud, and do not claim recoverable
amounts.
