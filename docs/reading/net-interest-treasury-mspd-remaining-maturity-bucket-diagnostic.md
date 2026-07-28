# Net Interest Treasury MSPD Remaining Maturity Bucket Diagnostic

Machine record:
`data/derived/breadth_benchmark_matrix/net_interest_treasury_mspd_remaining_maturity_bucket_diagnostic.v1.draft.json`

This packet uses the captured Treasury MSPD Table 3 and Table 5 CSVs to record
latest-month remaining-maturity bucket diagnostics for rows dated 2026-06-30.
Rows are included only when `outstanding_amt` is non-null and `maturity_date`
is not before the record date.

Diagnostic coverage:

- MSPD Table 3: 1,088 latest-month rows, 462 usable rows, raw outstanding
  diagnostic total `31082178.32343828`.
- MSPD Table 5: 409 latest-month rows, 405 usable rows, raw outstanding
  diagnostic total `23685187972.23828`.
- Buckets are `<=1 year`, `>1 and <=3 years`, `>3 and <=5 years`,
  `>5 and <=10 years`, `>10 and <=20 years`, `>20 and <=30 years`, and
  `>30 years`.

The tables are not combined. Many Table 5 bucket amounts are approximately
1,000 times the corresponding Table 3 raw amounts, while Table 3 has additional
short-bill coverage. That is useful diagnostic evidence, but it requires unit,
perimeter, CUSIP, STRIPS, reconstitution, and debt-stock reconciliation before
it can become a model input.

Treasury MSPD remaining-maturity bucket diagnostics are recorded for
2026-06-30 latest-month rows, but Table 3 and Table 5 are not combined. This is
not a weighted average maturity, not a remaining-maturity schedule, not a
debt-stock projection, not a CBO/OMB fiscal-year projection bridge, not a rate
path, not primary-balance feedback, not solver input, not a solver run, not a
rate calculation, not a public rate card, not a savings estimate, and not a
balanced-budget claim. This is not a balanced-budget claim.
