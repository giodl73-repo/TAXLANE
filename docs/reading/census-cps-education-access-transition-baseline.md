# Census CPS Education Access and Transition Baseline

Machine record:
`data/derived/breadth_benchmark_matrix/census_cps_education_access_transition_baseline.oct2024.v1.draft.json`.

The Census Bureau's October 2024 Current Population Survey School Enrollment
Supplement supplies a broad national student denominator beside TaxLane's
administrative K-12 and higher-education records. Table 1 covers the civilian
noninstitutionalized population age 3 and older. Counts are in thousands.

| Measure | Population | Enrolled | Enrollment rate |
|---|---:|---:|---:|
| Age 3 and older | 321,500 | 75,110 | 23.4% |
| Age 3–4 | 7,432 | 4,368 | 58.8% |
| Age 5–6 | 7,657 | 7,073 | 92.4% |
| Age 7–9 | 12,210 | 11,820 | 96.8% |
| Age 10–13 | 16,470 | 16,080 | 97.7% |
| Age 14–15 | 8,477 | 8,245 | 97.3% |
| Age 16–17 | 8,973 | 8,201 | 91.4% |
| Age 18–19 | 8,660 | 5,724 | 66.1% |
| Age 20–21 | 8,507 | 4,351 | 51.1% |
| Age 22–24 | 12,930 | 3,419 | 26.4% |

The published enrolled population comprises 8,510 thousand nursery or
kindergarten students, 32,090 thousand elementary students, 17,120 thousand
high school students, and 17,400 thousand undergraduate or graduate college
students. Those displayed components sum to 75,120 thousand, 10 thousand above
the separately published 75,110 thousand total. This is a published-count
rounding difference, not an inconsistency to repair.

## Recent High School Graduate Transition

Table 7 provides an October snapshot for high school graduates age 15 to 24.
Among the 3,250 thousand people reported as graduating this year, the published
mutually exclusive statuses were:

| October status | Count, thousands |
|---|---:|
| Two-year college, full time | 537 |
| Two-year college, part time | 130 |
| Four-year college, full time | 1,303 |
| Four-year college, part time | 60 |
| Graduate school | 6 |
| Vocational school | 52 |
| Not enrolled, employed | 613 |
| Not enrolled, not employed | 549 |

Adding the six listed school categories gives 2,088 thousand, or 64.246154%
of the rounded 3,250-thousand cohort. College categories excluding vocational
school give 2,036 thousand, or 62.646154%. The not-enrolled-and-not-employed
status is 549 / 3,250, or 16.892308%. These percentages are TaxLane
calculations from rounded published counts, not Census-published rates.

The [Pell short-training randomized evaluation](pell-short-training-impact-evidence.md)
provides program-specific evidence for interested FAFSA applicants at volunteer
schools. Those randomized applicants are not the CPS population or the recent-
graduate snapshot, so the two records cannot be joined into treated and control
cohorts.

The [FY2024 FSA Title IV baseline](fsa-title-iv-student-access-baseline.md)
describes an administrative aid universe. Its FAFSA, recipient, award, and
disbursement measures are not denominators for this CPS population or recent-
graduate transition snapshot.

## Evidence Boundary

The two tables retain their different universes. Table 1 covers people age 3
and older; Table 7 covers high school graduates age 15 to 24, with the selected
rows limited to those reported as graduating this year. They are related CPS
estimates, not person-level linked records or an administrative student census.

Enrollment is a participation snapshot, not proof of regular attendance,
quality, completion, unmet demand, or long-run employment. The recent-graduate
statuses do not identify WIOA, WIA, or other program participation and provide
no treatment or counterfactual. No compatible spending cohort is attached, so
cost per student, graduate, enrollee, worker, or outcome remains blocked. The
baseline cannot support program attribution, causal effects, fraud findings,
or savings estimates.

Official references: [Census 2024 CPS enrollment tables](https://www.census.gov/data/tables/2024/demo/school-enrollment/2024-cps.html),
[Table 1 XLSX](https://www2.census.gov/programs-surveys/demo/tables/school-enrollment/2024/2024-cps/enroll01_2024_01.xlsx),
[Table 7 XLSX](https://www2.census.gov/programs-surveys/demo/tables/school-enrollment/2024/2024-cps/enroll07_2024.xlsx),
and [October 2024 technical documentation](https://www2.census.gov/programs-surveys/cps/techdocs/cpsoct24.pdf).
