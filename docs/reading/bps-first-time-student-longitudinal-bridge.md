# BPS First-Time-Student Longitudinal Bridge

Machine record:
`data/derived/breadth_benchmark_matrix/bps_first_time_student_longitudinal_bridge.ay2019-2022.v1.draft.json`.

NCES followed a nationally representative cohort of students who first began
postsecondary education in 2019–20 at Title IV eligible institutions. The
current fixed evidence covers three academic years through June 2022, rather
than conditioning the cohort on eventual degree completion.

The BPS:20/22 documentation reports about 34,240 sampled cohort members and
22,320 respondents. The follow-up response rate was 65.2 percent unweighted
and 59.6 percent weighted; the overall response rate was 52.1 percent
unweighted and 49.8 percent weighted. Estimates use analysis weight `WTA000`
and bootstrap replicate weights `WTA001` through `WTA200`.

## Published Three-Year Status

| Mutually exclusive status through June 2022 | Estimate | Standard error |
|---|---:|---:|
| Certificate attained | 5.1% | 0.23 percentage points |
| Associate's degree attained | 6.8% | 0.24 percentage points |
| Bachelor's degree attained | 0.7% | 0.07 percentage points |
| No credential; enrolled at a 4-year institution | 47.9% | 0.51 percentage points |
| No credential; enrolled at a less-than-4-year institution | 16.9% | 0.42 percentage points |
| No credential; not enrolled | 22.6% | 0.48 percentage points |

These categories sum to 100 percent. The no-credential categories describe
2021–22 enrollment status and the last institution level when enrolled. They
are not permanent-dropout classifications. The bachelor's estimate is an
especially early measure because a three-year window is shorter than the
normal duration of many bachelor's programs.

## Same-Cohort Variable Map

NCES documents `PELL20` for entry-year Pell amount and `PROUT3_NEW` for
cumulative attainment and persistence anywhere through 2021–22. It also
documents `STFCUM22` for cumulative Direct Subsidized and Unsubsidized Loan
borrowing, `T4TDUE22` and `T4XDUE22` for Title IV amounts owed with and without
Parent PLUS, `JOBST22` for 2022 employment status, and `SALARY22` for annual
salary in the year-three job.

That shared cohort and variable availability remains a bridge. The published
First Look Table A-1 has no Pell-status rows, but Pulse 22 separately captured
the official [Pell Current-Entrant Persistence Baseline](pell-current-entrant-persistence-baseline.md).
Its saved DataLab query crosses `PELL20` receipt amount with the five-category
`PROUT3_NEW` outcome using `WTA000`, BRR standard errors, no filters, and
retrieval code `zclxfu`. This closes the source-capture gate without turning an
unadjusted receipt-group distribution into a causal Pell effect.

The standalone baseline deliberately remains separate because its five
`PROUT3_NEW` categories combine enrolled students across institution levels,
whereas First Look Table A-1 has six categories and separates 4-year from
less-than-4-year enrollment. `PELL20 > 0` records receipt; `PELL20 = 0` does
not establish ineligibility, nonapplication, denial, or a valid untreated
counterfactual. The separate
[significance screen](pell-current-entrant-persistence-significance-screen.md)
uses DataLab's official independent-estimates t-test method. Three of five
outcomes pass a Bonferroni screen, but covariance-aware confirmation remains
blocked and the result is still noncausal.

This bridge and the separate current-entrant baseline are attached to the
education depth card, higher-education account bridge, FY2024 FSA access
baseline, experimental-Pell evidence, and B&B bachelor-completer baseline as
early descriptive entrant-cohort context. Those links do not create a mature
outcome record, causal estimate, cohort-compatible cost, or fiscal crosswalk.

## Evidence Boundary

This is a survey-weighted descriptive baseline from an early, pandemic-era
window. It is not a randomized or adjusted causal design. About half of the
overall weighted sample responded; NCES applies nonresponse and
poststratification adjustments. Report variables were imputed, and disclosure
protection included recoding, suppression, sanitization, and targeted swapping.

`SALARY22` is a survey job measure, not mature post-completion administrative
earnings for every cohort member. Restricted NSLDS repayment histories do not
constitute a public repayment estimate in this artifact. Borrowing and amounts
owed are student financial outcomes, not full incremental Pell costs or
compatible federal outlays. No program effect, cost-effectiveness ratio,
fiscal return, fraud finding, improper payment, or recoverable savings estimate
is supported.

The next maturity gate is the BPS:20/25 six-year follow-up. It must be captured
and reviewed before this cohort is treated as a mature completion or
labor-market outcome baseline.

Official references: [NCES 2024-401](https://nces.ed.gov/pubs2024/2024401.pdf)
and [NCES 2026-013](https://nces.ed.gov/sites/default/files/data-asset/study-program-not-applicable/2026/03/202022-beginning-postsecondary-students-longitudinal-study-bps2022/2026013.pdf).
