# NET MSPD public-maturity envelope

Machine record:
`data/derived/breadth_benchmark_matrix/net_interest_mspd_public_maturity_envelope.v1.draft.json`.

The reconciled June 30, 2026 Treasury snapshot contains `$31.082178 trillion`
of marketable securities with a future maturity date. TAXLANE now groups all
462 rows by federal fiscal year. Securities already outstanding schedule
`$24.907413 trillion` of maturities from FY2026 through FY2035 and `$6.174765
trillion` from FY2036 through FY2056.

Treasury reports `$20.540429 billion` of all marketable debt as
intragovernmental but does not identify its maturity rows. Instead of assigning
that amount pro rata, TAXLANE publishes an interval for each group. The lower
bound assumes all marketable intragovernmental debt sits in that group; the
upper bound assumes none does. For all future-detail rows together, the public-
held amount is bounded between `$31.061638 trillion` and `$31.065291 trillion`.
The narrow `$3.652924 billion` aggregate width reflects the separately
reconciled Federal Financing Bank and matured/unallocated amounts.

Annual intervals are not additive: the same `$20.540429 billion` cannot be
subtracted once per year. Any multi-year question must sum its marketable
amounts first and apply the envelope once.

This closes exact existing-stock fiscal-year runoff and a public-holder
envelope without pro-rata inference. It does not supply future gross issuance,
issuance term mix, annual rollover, a full-stock maturity model, savings,
solver results, or rate changes.
