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

That now makes all eight formula inputs and seven of nine completion steps
ready for marginal incremental feedback. The engine is usable for a future
owner-admitted primary path, but it is not the full-stock maturity model.

## What remains incompatible

The Treasury average-interest-rate artifact remains latest-month snapshot
context. It cross-checks current conditions but is not substituted for CBO's
matching-vintage annual path. The
[MSPD snapshot reconciliation](net-interest-treasury-mspd-snapshot-reconciliation.md)
now resolves Table 3 and Table 5 units and overlap and reconciles Table 3 to
Table 1 total marketable debt. The
[public-maturity envelope](net-interest-mspd-public-maturity-envelope.md) now
bounds public-holder uncertainty and publishes exact existing-stock fiscal-year
runoff without pro-rata inference. The
[empirical rollover convention](net-interest-mspd-empirical-rollover-convention.md)
separately replays 434 issue/reopening rows from twelve snapshots into an
observed term mix and deterministic marginal rollover rule. It closes the
marginal input without pretending to forecast Treasury's total issuance.
Future total issuance still prevents a full-stock interest result.

The remaining sequence is therefore substantive rather than architectural:

1. run the matching-vintage rate and empirical rollover engines only after an
   owner admits a real primary-balance path;
2. independently source future total gross issuance consistent with CBO's
   annual debt totals;
3. map effective and stress rates to those buckets; and
4. validate any later full-stock result against the already-ready marginal
   result.

No savings or rate changes follow from this audit. The five current candidates
still contribute a zero admitted primary-balance delta.
