# Pulse 21: BPS First-Time-Student Longitudinal Bridge

## Result

Captured the NCES BPS:20/22 First Look and data-file documentation and attached
a bounded current first-time-student longitudinal bridge. Published Table
A-1 totals and A-S1 standard errors describe mutually exclusive three-year
attainment and persistence statuses for the 2019–20 first-time-beginner cohort
through June 2022. The record also maps documented same-cohort Pell, debt,
employment, and salary variables without creating unpublished contrasts.

## Decision gate

Pass for a nationally representative, survey-weighted, early descriptive
baseline that includes completers, persisters, and students no longer enrolled.
Fail for a Pell program effect, mature completion rate, permanent-dropout
classification, causal employment or salary effect, public repayment estimate,
full incremental cost, compatible fiscal return, fraud, improper payment, or
recoverable savings claim. COVID-19 context, response rates, imputation,
weighting, sampling error, and disclosure perturbation remain explicit.

## Integration status

Shared integration complete. The bridge is linked from the education
depth card, higher-education account bridge, FY2024 FSA access baseline,
experimental-Pell artifact, and B&B bachelor-completer artifact. The education
breadth row, public scoreboard, readers, wave, and source ledger now carry the
same early descriptive entrant-cohort boundary. Rust validation and the
generated manifest cover the bridge and its checksum-verified source packet.

Pulse 22 closes the narrow DataLab source-capture gate through a separate
canonical current-entrant persistence baseline. The official table provides
an unadjusted `PELL20` receipt-group by five-category `PROUT3_NEW`
distribution. It does not convert receipt into eligibility, reproduce the
six-category First Look table, establish a covariance-aware between-group
significance test, or support a mature, causal, cost, or fiscal claim.

## Next bounded action

Capture and review BPS:20/25 as the future six-year maturity gate. Before any
claim about statistically significant receipt-group differences, run and
preserve a covariance-aware DataLab test rather than inferring significance
from separate cell standard errors. Keep causal, full-cost, and fiscal gates
closed.
