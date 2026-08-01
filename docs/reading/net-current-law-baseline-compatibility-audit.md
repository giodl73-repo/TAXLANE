# NET current-law baseline compatibility audit

Machine record:
`data/derived/breadth_benchmark_matrix/net_current_law_baseline_compatibility_audit.v1.draft.json`.

## What is now usable

The already-admitted CORE-G CBO spine closes more of NET's baseline than the
earlier completion plan recognized. Its eleven FY2025–FY2035 rows provide
source-custodied annual debt held by the public, net interest, and explicit
other-financing and timing values. The existing deterministic validator also
replays every topline and debt identity with a zero policy delta.

CORE-G also contains the matching-vintage FY2025–FY2035 projected average
interest-rate path. Combined with the explicit borrowing-timing convention,
that supports a reduced-form feedback engine which preserves the CBO baseline
and calculates only policy deltas. Incremental interest receipts default to
zero for a public-borrowing shock; any nonzero candidate effect requires
evidence. A mechanical nonzero regression now proves later debt and interest
movement.

That makes seven of eight formula inputs and six of nine completion steps
ready. The reduced-form engine is usable for a future admitted primary path,
but it is not the full-stock maturity model.

## What remains incompatible

The Treasury average-interest-rate artifact remains latest-month snapshot
context. It cross-checks current conditions but is not substituted for CBO's
matching-vintage annual path. The
[MSPD snapshot reconciliation](net-interest-treasury-mspd-snapshot-reconciliation.md)
now resolves Table 3 and Table 5 units and overlap and reconciles Table 3 to
Table 1 total marketable debt. The snapshot still does not allocate buckets
between public and intragovernmental holders or define annual rollover, so it
cannot yet drive interest feedback.

The remaining sequence is therefore substantive rather than architectural:

1. allocate Treasury maturity detail to debt held by the public without a pro-
   rata inference and bridge June actuals to fiscal year end;
2. define annual maturity buckets and rollover rules;
3. map effective and stress rates to those buckets;
4. validate the full-stock result against the already-ready reduced-form result;
   and
5. run both only after an owner admits a real primary-balance path.

No savings or rate changes follow from this audit. The five current candidates
still contribute a zero admitted primary-balance delta.
