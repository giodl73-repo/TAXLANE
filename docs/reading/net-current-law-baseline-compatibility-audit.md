# NET current-law baseline compatibility audit

Machine record:
`data/derived/breadth_benchmark_matrix/net_current_law_baseline_compatibility_audit.v1.draft.json`.

## What is now usable

The already-admitted CORE-G CBO spine closes more of NET's baseline than the
earlier completion plan recognized. Its eleven FY2025–FY2035 rows provide
source-custodied annual debt held by the public, net interest, and explicit
other-financing and timing values. The existing deterministic validator also
replays every topline and debt identity with a zero policy delta.

That makes three of eight formula inputs and four of nine completion steps
ready. This is a real accounting advance: NET now has a bounded annual baseline
to reconcile against rather than only a list of missing inputs.

## What remains incompatible

The Treasury average-interest-rate artifact is latest-month aggregate context,
not an annual effective-rate path by maturity bucket. The MSPD artifact exposes
maturity-bearing securities and diagnostic buckets, but Table 3 and Table 5
remain unreconciled and the amounts are not bridged to CBO debt held by the
public. Neither artifact can yet drive rollover or interest feedback.

The remaining sequence is therefore substantive rather than architectural:

1. reconcile the Treasury security perimeter to CBO debt held by the public;
2. define annual maturity buckets and rollover rules;
3. map effective and stress rates to those buckets;
4. define new-borrowing timing;
5. bridge gross interest, credited receipts, and net interest; and
6. prove that an admitted primary-balance shock changes later debt and interest.

No savings or rate changes follow from this audit. The five current candidates
still contribute a zero admitted primary-balance delta.
