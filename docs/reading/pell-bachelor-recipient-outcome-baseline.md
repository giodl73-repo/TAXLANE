# Pell Bachelor-Recipient Outcome Baseline

Machine record:
`data/derived/breadth_benchmark_matrix/pell_bachelor_recipient_outcome_baseline.bb2016-2020.v1.draft.json`.

NCES followed about 17,160 respondents representing roughly 2 million people
who completed bachelor's degree requirements in 2015–16. All degrees came from
Title IV eligible institutions in the 50 states, District of Columbia, or
Puerto Rico and were awarded by June 30, 2017. Outcomes were observed four
years after degree completion in 2020.

The Pell split is **ever received Pell**, not Pell receipt in 2015–16. The
`PELLCUM` variable covers any Pell receipt from award year 1993–94 through
2015–16 using NSLDS and student records. Pell recipients were 50.9 percent of
the bachelor-completer cohort; nonrecipients were 49.1 percent.

## Employment and Enrollment

| 2020 status | Ever received Pell | Never received Pell |
|---|---:|---:|
| Employed only | 69.0% | 71.7% |
| Employed and enrolled | 11.8% | 11.0% |
| Enrolled only | 4.2% | 7.4% |
| Unemployed | 4.2% | 2.7% |
| Out of the labor force | 10.8% | 7.2% |

These are descriptive distributions among people who had already completed a
bachelor's degree. The groups were not randomized or adjusted into a causal
counterfactual.

## Federal Student Loans and Repayment

Among federal student-loan borrowers, ever-Pell recipients borrowed an average
of $42,900 and had an average owed-to-borrowed ratio of 85.3 percent. Their
median values were $33,700 and 100.0 percent. For never-Pell borrowers, the
corresponding average values were $38,500 and 66.3 percent, and medians were
$27,000 and 76.0 percent.

NCES classified 33.1 percent of ever-Pell borrowers and 18.7 percent of
never-Pell borrowers as in repayment. Among borrowers in repayment, average
monthly payments were $240 and $230, respectively; both medians were $200.
The March 13, 2020 COVID-19 emergency administrative forbearance likely
affected the repayment percentages. Parent PLUS is excluded, and income-driven
plans requiring a zero-dollar payment are included.

Loan amounts, balances, and payments are borrower outcomes. They are not Pell
Grant amounts, program costs, FY2025 outlays, or a cost-effectiveness numerator.

## Hours and Earnings Among Workers

Among graduates working for pay, 87.3 percent of ever-Pell recipients and 88.1
percent of never-Pell recipients worked full time. Full-time workers averaged
41.2 and 42.0 hours per week, respectively. Average annualized earned income
was $55,500 for full-time ever-Pell workers and $64,200 for full-time never-Pell
workers; medians were $50,000 and $57,900. Part-time averages were $18,700 and
$19,900, with a $15,600 median in both groups.

These earnings exclude nonworkers and describe the current or most recent job
held for at least four months. They are not all-person income or a causal return
to Pell receipt.

## Financial Well-Being

| 2020 measure | Ever received Pell | Never received Pell |
|---|---:|---:|
| Owned a home | 33.7% | 27.8% |
| Had a retirement account | 69.0% | 79.2% |
| Reported negative net worth | 42.9% | 24.0% |
| Did not meet essential expenses in prior 12 months | 14.8% | 6.2% |

## Evidence Boundary

This is an observational bachelor-completer outcome baseline. Conditioning on
bachelor's completion means it cannot estimate Pell effects on persistence or
completion and does not represent noncompleters. Lifetime Pell receipt reflects
financial need and other selection factors. No displayed difference can be
attributed causally to Pell.

The 2020 outcome window overlaps the COVID-19 pandemic, and loan repayment was
affected by CARES Act administrative forbearance. Worker, borrower, and
repayment subuniverses must remain separate. This source supplies no Pell award
amount, full incremental cost, compatible federal outlay, fiscal return, fraud
finding, or recoverable savings estimate.

The [BPS first-time-student longitudinal bridge](bps-first-time-student-longitudinal-bridge.md)
instead begins with 2019–20 first-time entrants and follows early outcomes
through June 2022 without conditioning on bachelor completion. The separate
[Pell current-entrant persistence baseline](pell-current-entrant-persistence-baseline.md)
adds the official DataLab entry-year receipt-group distribution. Its five
categories, receipt-not-eligibility boundary, and pandemic-era early window do
not provide a mature completion or post-completion outcome, causal estimate,
cost link, or fiscal link.

The linked [significance screen](pell-current-entrant-persistence-significance-screen.md)
applies only to that early entrant cohort. It is not a covariance-aware test
and does not extend to bachelor-completer postcompletion outcomes.

Official reference: [NCES 2022-241](https://nces.ed.gov/pubs2022/2022241.pdf).
