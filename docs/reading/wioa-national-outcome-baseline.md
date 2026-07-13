# WIOA National Outcome Baseline

Machine record:
`data/derived/breadth_benchmark_matrix/wioa_national_outcome_baseline.py2024.v1.draft.json`.

The Department of Labor's PY2024 national summary supplies a descriptive
worker and employment outcome layer for selected Workforce Innovation and
Opportunity Act programs. The reporting period is July 1, 2024 through June
30, 2025, and the PDF says the data are current as of November 25, 2025.

| PY2024 national program | Participants served | Received training | Employment, Q2 after exit (Youth also education/training) | Employment, Q4 after exit (Youth also education/training) | Median earnings, Q2 | Credential attainment | Measurable skill gains |
|---|---:|---:|---:|---:|---:|---:|---:|
| Adult | 250,160 | 109,506 | 72.2% | 72.3% | $8,754 | 73.6% | 74.0% |
| Dislocated Worker | 187,108 | 37,119 | 69.0% | 70.5% | $9,897 | 75.1% | 72.4% |
| Youth | 121,531 | 34,956 | 65.9% | 67.4% | $5,038 | 62.0% | 67.4% |
| Wagner-Peyser Employment Service | 2,561,317 | Not applicable | 66.8% | 67.5% | $8,558 | Not applicable | Not applicable |

For Adult, Dislocated Worker, and Wagner-Peyser, the Q2 and Q4 indicators refer
to unsubsidized employment. The Youth indicators also count participation in
education or training. Median earnings cover participants in unsubsidized
employment in the second quarter after exit. Credential attainment and
measurable skill gains apply to indicator-specific education or training
populations. The published rates therefore cannot be reproduced by dividing
the numerators by total participants served or by the received-training count.

The indicators also use different time windows. Served counts and skill gains
cover July 2024 through June 2025; Q2 employment and earnings use July 2023
through June 2024 exiters; Q4 employment and credentials use calendar-year
2023 exiters. The separately displayed total-exited period is not a common
denominator. DOL also reports 64.7% effectiveness in serving employers
(1,275,198 of 1,971,425 participants employed by the same employer in both Q2
and Q4); PY2024 is the first year of this newly defined indicator.

## Publisher Discrepancy

The underlying official PY2024 Annual Summary Report PDF reports **121,531**
Youth participants served, which is the value used in the machine record. The
Department's results-at-a-glance webpage reports **125,531** for the same
headline. This four-thousand-participant conflict remains unresolved and is
preserved rather than silently harmonized.

## Evidence Boundary

These are state-reported participant and exiter outcomes, not estimates of the
effect of WIOA services. There is no untreated comparison group, and the
participants are not representative of all U.S. workers. The scope covers WIOA
Title I Adult, Dislocated Worker, and Youth programs and Title III
Wagner-Peyser; it does not cover every training, employment, education, or
social-services program in federal function 500.

OMB reports $5.434 billion of FY2025 outlays for subfunction 504, Training and
employment. The [account bridge](training-employment-account-bridge.md) now
reconciles the topline and maps the four outcome programs to DOL budget
activities. It does not separate FY2025 actual account outlays for those four
programs or align multi-year funding with the performance cohorts. A cost per
participant or outcome is therefore blocked. The baseline does not measure
waste or fraud and cannot support a recoverable-savings estimate.

Historical causal evidence is available from the national
[WIA Gold Standard evaluation](wia-gold-standard-impact-evidence.md). Its
randomized contrasts concern access to intensive services and training under
the predecessor WIA system during 2011–13. They do not convert the PY2024
descriptive rates above into current WIOA impact estimates.

The [Census CPS education-access record](census-cps-education-access-transition-baseline.md)
describes October enrollment and employment statuses for people age 15–24
reported as graduating that year. They are not selected WIOA Youth participants
or a comparison group, and their school-enrollment status is not the statutory
WIOA Youth employment-or-education indicator.

Official references: [DOL PY2024 results at a glance](https://www.dol.gov/agencies/eta/performance/wioa-performance),
[national performance results](https://www.dol.gov/agencies/eta/performance/results/national),
and [WIOA performance indicator definitions](https://www.dol.gov/agencies/eta/performance/performance-indicators).
