# Pulse 109 — Current-law source-custody preflight

Pulse 109 adds a preflight record for the current-law annual path source
custody required before solver values may be populated.

Artifacts:

- `data/derived/breadth_benchmark_matrix/current_law_source_custody_preflight.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/current_law_source_custody_preflight.schema.md`
- `docs/reading/current-law-source-custody-preflight.md`

Boundary:

- No external request was submitted.
- No source values were captured.
- All custody fields remain `null`.
- Every row remains `custody_ready: false`.
- Every row remains `values_may_be_populated: false`.
- Solver, rate, savings, waste, fraud, department-cut, technology-savings, and
  balanced-budget claims remain blocked.

Validation:

- Rust validator requires the exact eight current-law path rows.
- Rust validator requires null custody fields and false readiness flags.
- Rust validator requires every public warning phrase in the reader.
