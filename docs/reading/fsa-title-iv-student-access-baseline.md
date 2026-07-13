# FSA Title IV Student Access Baseline

Machine record:
`data/derived/breadth_benchmark_matrix/fsa_title_iv_student_access_baseline.fy2024.v1.draft.json`.

Federal Student Aid's FY 2024 Annual Report provides a national administrative
baseline for access to Title IV loans, grants, and Federal Work-Study. FSA
reported processing **more than 17.6 million** FAFSA forms and delivering
approximately **$120.8 billion** in Title IV aid to **more than 9.9 million**
postsecondary students and their families through 5,378 active participating
institutions. The “more than” and “approximately” qualifiers are part of the
published measures and are retained.

## Table 4 disbursements

Table 4 reports fiscal-year aid disbursed in millions of dollars.

| Program | FY 2024 | FY 2023 | Difference | Change |
|---|---:|---:|---:|---:|
| Direct Loans | $85,802.4 | $83,295.3 | $2,507.1 | 3.0% |
| Pell Grants | $32,995.7 | $28,689.2 | $4,306.5 | 15.0% |
| Supplemental Educational Opportunity Grants | $871.5 | $893.8 | -$22.3 | -2.5% |
| TEACH Grants | $42.3 | $82.1 | -$39.8 | -48.5% |
| Iraq and Afghanistan Service Grants | $0.7 | $0.6 | $0.1 | (16.7)% printed; +16.7% arithmetically |
| Federal Work-Study | $1,103.5 | $1,150.2 | -$46.7 | -4.1% |
| Grant subtotal | $33,910.2 | $29,665.7 | $4,244.5 | (14.3)% printed; +14.3% arithmetically |
| Grand total | $120,816.1 | $114,111.2 | $6,704.9 | 5.9% |

At the table's displayed one-decimal precision, the four grant rows sum
exactly to the published grant subtotal in both years. The six program rows
sum exactly to the published grand total, and $120,816.1 million minus
$114,111.2 million equals the published $6,704.9 million difference.

The printed percentage column contains two sign contradictions. It shows
parenthesized negative changes for the Iraq and Afghanistan Service Grant row
and the grant subtotal, although each FY 2024 amount and displayed dollar
difference is higher than FY 2023. The machine record preserves the printed
strings and separately records the positive arithmetic changes; it does not
silently normalize the source.

## Pell and other access measures

The report describes approximately $33.0 billion in FY 2024 Pell Grant
disbursements, averaging **$5,218** for **more than 6.3 million** students. The
maximum Pell award was **$7,395** in award year 2023–24 and remained $7,395 in
award year 2024–25. The average and maximum are grant amounts, not tuition,
full cost of attendance, program delivery cost, or cost per outcome.

Other report descriptions retain the same published qualifiers: more than
$85.8 billion in net Direct Loans to more than 6.7 million recipients;
approximately $33.9 billion in grants to more than 6.3 million recipients;
more than 1.6 million FSEOG awards; more than 23,000 TEACH Grants; more than
100 Iraq and Afghanistan Service Grant awards; and more than 600,000 Federal
Work-Study awards.

## Period and evidence boundary

The report explicitly says Table 4 amounts come from financial systems and
are fiscal-year amounts, while recipient and award counts come from multiple
sources and are based on award year. Fiscal year 2024 overlaps portions of two
award years, so these amounts and counts are not treated as a single aligned
student cohort. TaxLane does not reverse-engineer exact recipients from the
published Pell average or divide fiscal-year dollars by award-year counts.

This is a descriptive access and disbursement baseline. It contains no
student outcome cohort, untreated comparison group, complete program cost,
OMB function/subfunction or account-row crosswalk, or evidence supporting a
causal effect. Year-over-year changes do not establish changes in access,
educational quality, completion, employment, or earnings. The record does not
support cost per student or outcome, fraud or waste findings, recoverable
amounts, or savings estimates.

The [B&B Pell bachelor-recipient baseline](pell-bachelor-recipient-outcome-baseline.md)
describes historical outcomes among 2015–16 bachelor completers grouped by
lifetime Pell receipt. It is not the FY2024 Title IV recipient or disbursement
cohort and supplies no person-level bridge to these administrative totals.

The [BPS first-time-student longitudinal bridge](bps-first-time-student-longitudinal-bridge.md)
provides early descriptive outcomes for a 2019–20 entrant cohort, not the
FY2024 recipient or disbursement population. The separate [Pell current-entrant
persistence baseline](pell-current-entrant-persistence-baseline.md) supplies an
official DataLab receipt-group cross-tab, but receipt is not eligibility and
the five-category, pandemic-era result is neither causal nor mature. It forms
no FY2024 population, cost, or fiscal link.

The linked [significance screen](pell-current-entrant-persistence-significance-screen.md)
does not change that boundary. Its independent-estimates method has three of
five Bonferroni-screened differences, while covariance-aware confirmation and
any FY2024 cohort link remain blocked.

Official references: [Department annual plans and reports](https://www.ed.gov/about/ed-overview/annual-performance-reports/annual-plans-and-reports)
and the [FY 2024 FSA Annual Report PDF](https://www.ed.gov/media/document/2024-fsa-annual-report-108481.pdf).
