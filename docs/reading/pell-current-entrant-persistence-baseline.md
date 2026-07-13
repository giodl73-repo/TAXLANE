# Pell Current-Entrant Persistence Baseline

Machine record:
`data/derived/breadth_benchmark_matrix/pell_current_entrant_persistence_baseline.bps2020-2022.v1.draft.json`.

NCES DataLab query `396385`, retrieval code `zclxfu`, crosses entry-year Pell
amount `PELL20` with the five-category three-year outcome variable
`PROUT3_NEW` for BPS:20/22. The universe is **all respondents**, the analysis
uses `WTA000`, and DataLab reports balanced repeated replication standard
errors. There are no filters, no subtable, and no suppressed cells.

## Official Weighted Distribution

| Status through academic year 2021-22 | `PELL20 = 0` estimate | SE | `PELL20 > 0` estimate | SE |
|---|---:|---:|---:|---:|
| Bachelor's degree attained | 0.8150894% | 0.1008173 | 0.4971528% | 0.0834908 |
| Associate's degree attained | 7.0807806% | 0.3414563 | 6.4224743% | 0.3544924 |
| Certificate attained | 3.4187823% | 0.2497779 | 7.1063823% | 0.3445115 |
| No degree; enrolled in 2021-22 | 70.4424349% | 0.6186631 | 58.0382947% | 0.7774679 |
| No degree; not enrolled in 2021-22 | 18.2429128% | 0.5892492 | 27.9356960% | 0.7039664 |

Standard errors are percentage points. DataLab's weighted group totals are
1,782,866 for `PELL20 = 0` and 1,468,475 for `PELL20 > 0`, with a weighted
grand total of 3,251,341. These are weighted population counts, not respondent
sample counts. The machine record also preserves exact weighted cell counts,
relative standard errors, and confidence bounds.

## Category and Receipt Boundaries

`PROUT3_NEW` has five categories. Its enrolled-without-degree category combines
students across institution levels. It therefore does **not** reproduce the
six-category First Look Table A-1 distribution, which separates enrollment at
4-year and less-than-4-year institutions. The five-category total is consistent
with the same cohort but has a different published-variable definition.

The row cut records Pell receipt amount: zero dollars versus a positive amount
from $1 through the documented maximum of $9,293. Positive `PELL20` is evidence
of Pell receipt in academic year 2019-20. Zero dollars is not evidence that a
student was ineligible, did not apply, was denied, or forms a comparable
untreated group.

## Evidence Boundary

This is an official survey-weighted descriptive cross-tab, not a randomized or
adjusted causal design. The receipt groups may differ in financial need,
institution, enrollment intensity, preparation, work, family circumstances,
and other observed or unobserved characteristics. Their differences cannot be
called Pell Grant effects. The separate
[significance screen](pell-current-entrant-persistence-significance-screen.md)
applies the official DataLab independent-estimates t-test method and a
Bonferroni screen: three of five outcome differences pass that screen. It is
not covariance-aware, so covariance-aware confirmation remains blocked.

The cohort began during 2019-20, and the three-year observation window overlaps
COVID-19 disruption. Many students remained enrolled in 2021-22, so the table
is not a mature completion baseline. “No degree, not enrolled” is not a
permanent-dropout classification. The output contains no full incremental Pell
cost, compatible federal outlay allocation, fiscal return, fraud finding,
improper-payment estimate, recovery, or budget saving.

The exact official response is fixed at
`data/raw/nces/SRC-NCES-DATALAB-BPS20-22-PELL-PERSISTENCE-2026/2026-07-13/workspace-retrieve-zclxfu.json`.
Its SHA-256 is
`AEDC7781DDC8DA4A9F59942E16B398F58CFFB20128CC6FE44CF24D6F04795DC5`.
The public table is [NCES DataLab query zclxfu](https://nces.ed.gov/datalab/powerstats/table/zclxfu).

Related source and bridge context: [NCES 2024-401 First Look](https://nces.ed.gov/pubs2024/2024401.pdf),
[NCES 2026-013 data-file documentation](https://nces.ed.gov/sites/default/files/data-asset/study-program-not-applicable/2026/03/202022-beginning-postsecondary-students-longitudinal-study-bps2022/2026013.pdf),
and the [BPS first-time-student longitudinal bridge](bps-first-time-student-longitudinal-bridge.md).
