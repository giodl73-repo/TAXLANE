# NET empirical marginal-rollover convention

Machine record:
`data/derived/breadth_benchmark_matrix/net_interest_mspd_empirical_rollover_convention.v1.draft.json`.

## What is now usable

Twelve source-custodied Treasury MSPD snapshots contain **434 distinct
marketable issue or reopening rows** with issue dates from July 2025 through
June 2026. They total **$31.594618 trillion of gross issuance**. That large
number is not debt outstanding: short bills can mature and be issued again
several times in one year.

The corrected extraction includes securities issued on the first calendar day
of a month. An earlier working calculation accidentally used a strict month-
start comparison, omitted 11 such rows, and returned 423 rows. The validator
now enforces the inclusive rule. Same-CUSIP rows are retained when they are
distinct issues or reopenings with different issue dates, amounts, or yields.

The trailing-12-month central mix places 57.815% of gross issuance at 92 days
or less and 24.634% between 93 and 183 days. The remaining 17.551% spans the
six longer-term buckets. The artifact also retains the observed short-heavy
November 2025 month and long-heavy June 2026 month as sensitivity rails.

## What the feature does

The dedicated `taxlane-net-interest` feature crate allocates a signed marginal
financing change across the empirical mix, matures each cohort monthly, and
immediately refinances it under the selected mix. It preserves signed principal
exactly and publishes fiscal-year snapshots by remaining term.

For a mechanical one-time **$100 billion FY2026 primary improvement**, the
model carries negative principal—avoided borrowing—through FY2035. Short-term
principal rolls repeatedly: the FY2026 rollover counter is `$135.200 billion`
and the FY2027 counter is `$192.665 billion`, even though outstanding avoided
principal remains exactly `$100 billion`. Those counters measure repeated
refinancing, not additional savings or interest.

## Decision boundary

This closes NET's eighth formula input for the **marginal incremental-feedback
mode** and makes the maturity/rollover completion step ready in that mode. The
matching-vintage CBO baseline remains unchanged.

It does not forecast Treasury's total future issuance, assign public versus
intragovernmental holders, build a full-stock debt model, supply bucket-specific
rates, admit a spending candidate, or change a tax rate. A fiscal result still
begins with an owner-admitted primary-balance path.
