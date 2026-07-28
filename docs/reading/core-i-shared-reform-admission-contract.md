# CORE-I Shared Reform Admission Contract

Machine record:
`data/derived/breadth_benchmark_matrix/core_i_shared_reform_admission_contract.v1.draft.json`.

TRN-C exposed four reusable requirements that did not belong in transportation
alone: conditional legislative status, bounded annual score values, cost-only
reform admission, and applicability-based policy/stress floors. CORE-I now
implements those interfaces in `taxlane-core`.

The shared engine admits a sourced real reform only when its annual horizon is
unique and contiguous and every applicable floor passes both policy and stress.
It preserves bounded values, rejects fabricated non-applicable passes, and
keeps target costs blocked unless a supported lower-cost claim exists.

CORE-I does not decide policy merit, assign financing, or run a lane solver.
