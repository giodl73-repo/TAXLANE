# Payment Integrity: What The Public Evidence Shows

The public evidence supports a useful but deliberately bounded account of
federal payment integrity. It can show what agencies reported, how the reported
categories reconcile, which periods were measured, and which methodological
questions remain open. It does not yet support a TAXLANE finding that a program
performed well or poorly, committed fraud or waste, created collectible debt,
recovered a particular share, prevented loss, or produced net savings.

That distinction matters because the numbers describe different things.
A statistical improper-payment estimate is not a transaction ledger. An
unknown payment is not automatically an improper payment. A court-confirmed
fraud table is not automatically a subset of the estimate. An operational
recovery row can use a different period and basis. An internal methodology
closure means that TAXLANE found enough evidence to answer one methodology
question; it is not a public performance grade.

## The FY2024 headline, correctly labeled

The official PaymentAccuracy annual workbook contains 68 FY2024 program rows
covering **$4,071.861 billion** in reported outlays. Across those rows it reports:

| Payment class | Amount |
|---|---:|
| Overpayments | $135.184B |
| Underpayments | $7.864B |
| Technically improper payments | $5.923B |
| **Classified improper payments** | **$148.971B** |
| Unknown payments | $12.570B |
| **Improper plus unknown** | **$161.540B** |

The arithmetic has two steps. The displayed overpayment, underpayment, and
technical values total $148,970.632 million, which reconciles within the
$0.001 million source-precision tolerance to the source-reported $148,970.633
million classified improper total. Adding
$12.570 billion unknown payments produces the $161.540 billion combined
headline. Calling the entire $161.5 billion “improper payments” erases a source
category; the accurate short label is **improper plus unknown payments across
covered programs**.

The 68 rows do not represent the full federal payment universe, and their
tested-payment periods vary. The workbook also contains 54 court-confirmed-
fraud rows and 59 agency-recovery rows, but those parallel tables are not shown
to be disjoint, same-period subsets of the statistical headline. They cannot be
subtracted, divided, or netted against it.

## Four program cards

### 1. Medicare Part D — three fields closed, five open

For payments sampled from January through December 2022, the FY2024 annual row
reports $96.521 billion in outlays and $3.575 billion improper: $3.053 billion
overpayments and $522 million underpayments, with technically improper and
unknown categories reported as zero. CMS also publishes a 95 percent confidence
interval of $3.19 billion to $4.01 billion, or 3.31 percent to 4.15 percent,
around its rounded $3.58 billion and 3.70 percent gross result.

CMS separates the corrected 3.16 percent overpayment result into a 2.70 percent
documentation component and 0.46 percent drug and pricing discrepancies. That
shows that sponsor documentation materially affects the measurement. The
sponsor-documentation dependency field is closed internally while the other
five methodology fields remain open. It does
not make every documentation failure a collectible debt. The annual-row
margin-of-error value of 0.42 has no disclosed unit or formula and must not be
forced to reconstruct the published interval.

**Public question:** What same-cohort records connect the statistical estimate
to final debt, appeal, collectibility, collection, and control cost?

### 2. Medicaid — one field closed, seven open

For payments sampled from July 2022 through June 2023, the annual row reports
$610.833 billion in outlays and $31.099 billion improper: $29.370 billion
overpayments, $124 million underpayments, and $1.605 billion technically
improper, with unknown payments reported as zero. Only the sample-period field
is closed internally. The sample design, payment universe, estimation method,
exclusions, payment-type split, state rotation and weighting, and the boundary
between improper payments and fraud or waste remain open.

**Public question:** How are state-cycle samples, payment components,
exclusions, and weights combined into the national estimate?

### 3. VA PLTSS — two fields closed, six open

The official FY2024 Purchased Long Term Services and Supports result uses an
October 2022 through September 2023 sample. The annual row reports $5.621
billion in outlays, $657 million improper, and $103 million unknown. The
improper total comprises $218.30 million overpayments, $6.41 million
underpayments, and $432.46 million technically improper payments. The official
scorecard reports the projected overpayment estimate as $218.30 million at
3.88 percent. A prior TAXLANE probe that showed $2.502 billion and 15.54 percent
was unsupported and has been corrected.

The FY2025 AFR reports a different tested-payment cycle and must not be blended
into this FY2024 result.

The scorecard also lists three operational recovery rows: $6.91 million
identified and $4.46 million recovered for recovery activity; $1.18 million and
$0.76 million for recovery audit activity; and $3.97 million and $3.71 million
for FY2023–FY2025 PIIA sample and deep-dive activity. Those rows have different
periods and bases. Same period, same definition, estimate-to-debt lineage,
debt-to-collection lineage, and row disjointness are all unestablished. Do not
sum them or divide them by $218.30 million.

**Public question:** Which privacy-safe same-cohort records connect sampled
classifications to bills, disputes, collectibility, and certified cash?

### 4. Federal Crop Insurance — four fields closed, four open

For the July 2021 through June 2022 sample period, the annual row reports
$23.867 billion in outlays and $579.36 million improper, comprising $573.93
million overpayments and $5.43 million underpayments. Sample period, payment-
type split, the data-access root-cause definition, and payment universe are
closed internally. Sample design, estimation method, exclusion rules, and the
recoverable-savings basis remain open.

The official annual-workbook value is $573.93 million overpayments. An apparent
$579.93 million figure in unaudited “Other Information” is excluded as a typo.
The workbook reports a margin-of-error value of 2.5 without disclosing its unit
in the extracted row. Neither value should be converted into identified debt,
recovery, or savings.

**Public question:** What reproducible current-cycle estimator, exclusion,
uncertainty, appeal, debt, and collection records support the reported result?

## How to compare these facts

Compare only records with matched programs, tested-payment cohorts, evidence
classes, units, and definitions. Preserve each source’s period. Keep classified
improper payments separate from unknown-payment status, statistical estimates
separate from court-confirmed fraud, and both separate from operational
recoveries. Do not rank programs by internal closure count: a 4/4 evidence
coverage status is not better program performance than a 1/7 status.

The safe public use is narrow but meaningful: explain the reported composition,
show the arithmetic, preserve uncertainty, identify corrected source errors,
and ask precisely what evidence would connect an estimate to disposition and
cash. Avoid performance rankings, fraud or waste labels, recovery percentages,
and prevented-loss or savings claims.

In compact form:

```text
statistical estimate
!= unknown-payment status
!= court-confirmed fraud
!= identified debt
!= collectible amount
!= operational recovery
!= net savings
```

The machine-readable source for this packet is
`data/derived/breadth_benchmark_matrix/payment_integrity_bounded_factual_examples.fy2024.v1.draft.json`.
The accompanying role review permits bounded, source-labeled factual reporting
only. It closes zero methodology components and zero full methodology fields;
all established public, performance, fraud, waste, debt, collectibility,
recovery, prevention, and savings gates remain false.
