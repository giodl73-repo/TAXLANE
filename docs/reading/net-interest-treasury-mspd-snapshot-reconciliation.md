# NET Treasury MSPD snapshot reconciliation

Machine record:
`data/derived/breadth_benchmark_matrix/net_interest_treasury_mspd_snapshot_reconciliation.v1.draft.json`.

## What the three Treasury tables mean

The 2026-06-30 MSPD records use two different amount scales. Table 1 and Table
3 report millions of dollars. Table 5 reports thousands. Dividing the Table 5
grand total by 1,000 produces $23,685,187.972 million, matching Table 3's
future-maturity notes, bonds, and TIPS total. Table 5 is therefore a STRIPS
cross-check, not another debt stock to add.

Table 3's seven future-maturity buckets sum to $31,082,178.323 million. Adding
$3,590.966 million of Federal Financing Bank debt and a separately exposed
$61.958 million matured/unallocated residual reproduces Table 1 total
marketable debt of $31,085,831.247 million. The future-maturity detail covers
99.988249% of that marketable total.

## Why the annual model is still blocked

Table 1 reports $31,065,290.818 million of marketable debt held by the public
and $20,540.429 million held intragovernmentally, but the maturity detail does
not allocate each bucket between those holders. TAXLANE does not apply a pro-
rata assumption.

Treasury's June 30 actual debt held by the public is $31,681,308.343 million.
CBO's FY2026 year-end projection is $32,095,165 million, $413,856.657 million
higher. The perimeters are compatible, but the dates differ and the intervening
issuance, redemption, and financing are not yet observed.

The result is a valid current-snapshot maturity-stock seed, not an annual
maturity schedule. Formula readiness remains 3/8, baseline completion remains
4/9, and NET remains 11/21. Rollover, bucket rates, interest feedback, savings,
and rates remain blocked.
