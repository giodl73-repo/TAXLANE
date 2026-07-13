# BLS CPS Worker Baseline

Machine record:
`data/derived/breadth_benchmark_matrix/bls_cps_worker_baseline.cy2024.v1.draft.json`.

BLS's 2024 Current Population Survey annual averages add a population-level
worker baseline beside the WIOA participant outcomes. Table 5.1 reports the
following education gradients for people age 25 and older.

| Highest educational attainment | Median usual weekly earnings | Unemployment rate |
|---|---:|---:|
| Doctoral degree | $2,278 | 1.2% |
| Professional degree | $2,363 | 1.3% |
| Master's degree | $1,840 | 2.2% |
| Bachelor's degree | $1,543 | 2.5% |
| Associate's degree | $1,099 | 2.8% |
| Some college, no degree | $1,020 | 3.8% |
| High school diploma | $930 | 4.2% |
| Less than a high school diploma | $738 | 6.2% |
| Total | $1,221 | 3.3% |

The two columns do not have the same universe. Earnings are for full-time wage
and salary workers, while unemployment rates are for the civilian labor force.
Educational attainment means the highest level completed and does not capture
apprenticeships or other on-the-job training. “Usual weekly earnings” are the
before-tax median for the main job, including usual overtime, commissions, and
tips but excluding self-employment, employer benefits, and non-wage income.
They are not annual earnings. The professional-degree median is also higher
than the doctoral-degree median, so the published rows are not strictly
monotonic.

## Employment and Population Denominators

CPS annual-average Table 7 reports population employment measures for the
total civilian noninstitutional population age 25 and older. Counts are in
thousands.

| Educational attainment | Population | Labor force | Employed | Employment-population ratio | Unemployment rate |
|---|---:|---:|---:|---:|---:|
| Less than high school | 19,295 | 9,153 | 8,589 | 44.5% | 6.2% |
| High school graduate, no college | 63,705 | 36,257 | 34,725 | 54.5% | 4.2% |
| Some college or associate's, total | 56,987 | 35,839 | 34,627 | 60.8% | 3.4% |
| Some college, no degree | 32,179 | 19,489 | 18,740 | 58.2% | 3.8% |
| Associate's degree | 24,808 | 16,350 | 15,887 | 64.0% | 2.8% |
| Bachelor's degree and higher, total | 89,612 | 65,080 | 63,571 | 70.9% | 2.3% |
| Bachelor's degree only | 55,131 | 39,897 | 38,885 | 70.5% | 2.5% |
| Advanced degree | 34,481 | 25,183 | 24,686 | 71.6% | 2.0% |

The two “total” columns are parent categories, not additional groups: each is
the sum of the two child columns that follow it. Counts are rounded to
thousands and rates to one decimal place, so recomputed identities can differ
slightly at the displayed precision.

## Evidence Boundary

These cross-sectional associations do not estimate the causal return to a
degree, training, or federal program. Table 7's employment-population ratios
use the civilian noninstitutional population; Table 5.1's earnings use only
full-time wage and salary workers. Those denominators are intentionally not
merged.

The WIOA PY2024 record describes selected program participants and exiters.
The CPS population is adjacent context, not a counterfactual cohort for WIOA.
Selection, age, education, timing, and outcome definitions differ. OMB's
$5.434 billion FY2025 subfunction 504 topline is likewise not crosswalked to
these population measures, so cost per worker or outcome is blocked. The
baseline does not measure fraud and cannot support a savings estimate.

The [Census CPS education-access record](census-cps-education-access-transition-baseline.md)
is a separate October 2024 snapshot for age enrollment and recent graduates
age 15–24. It is not the annual-average age-25-and-older worker population used
here, and the records are not linked into a longitudinal transition cohort.

Official references: [BLS Table 5.1](https://www.bls.gov/emp/tables/unemployment-earnings-education.htm),
[CPS 2024 annual-average Table 7](https://www.bls.gov/cps/data/aa2024/cpsaat07.htm),
and [CPS definitions](https://www.bls.gov/cps/definitions.htm).
