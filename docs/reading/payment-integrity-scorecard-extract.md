# Payment Integrity Scorecard Extract

## Purpose

This packet records the first PaymentAccuracy Q4 2025 scorecard probe rows for
program-level payment-integrity work.

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_scorecards_q4_2025_first_pass.jsonl`.

## Extracted Scorecards

| Program/activity | FY2024 overpayment amount | FY2024 overpayment rate | Primary extracted root-cause amount |
|---|---:|---:|---:|
| Medicare Prescription Drug Benefit (Part D) | $3.053B | 3.16% | $3.053B |
| VA Purchased Long Term Services and Supports | $218.30M | 3.88% | $218.30M |
| Federal Crop Insurance Program | $574.0M | 2.40% | $467.0M |
| Medicaid | $29.370B | 4.81% | $29.370B |

For Part D, the scorecard amount now sits behind explicit sample-design and
estimation-method ceilings. The estimation record web-verifies official APR
process text but lacks official PDF custody; captured findings publish outputs
and confidence limits but not formula, weights, aggregation, simulation, sample
linkage, record treatment, variance, or reconciliation. The 2026 CMS background
page is non-same-period corroboration only. Part D remains three closed and five
open.

The separate Pulse43 bridge identifies the sampled PDE/GDC measurement object
and reconciles the published denominator to row 828's $96,521.39 million
outlays. It does not define the complete payment universe or map combined
plan-beneficiary GDC liability to federal outlays. The scorecard remains a probe
only, and all public, debt, recovery, fraud, waste, and savings claims stay
blocked.

Pulse44 uses the scorecard's recovery-plan table to close one current-process
component: issued named-audit notices require deletion of audit-determined
improper PDE records, while DME and Tepezza notices remain planned. The Q4 2025
process is not linked by cohort or amount to the FY2024/CY2022 estimate and
therefore does not establish debt, collectibility, actual recovery, or savings.

The separate Pulse45 bridge preserves the findings' published 95% intervals of
$3.19B-$4.01B and 3.31%-4.15% around its rounded $3.58B and 3.70% result, plus
row 828's $3,575.09M, 0.037039355 rate, `95% to <100%` confidence label, and
0.42 margin-of-error value. Because the row discloses neither units nor a
formula for 0.42, the bridge does not force reconciliation or close the full
estimation method. It establishes no debt, recovery, fraud, waste, or savings.

Pulse46 separately closes the two-track documentation treatment after a sampled
reconciliation PDE is adjusted: reconciliation-PDE-aligned documentation stays
required and linked adjustment documentation is added. The rule supplies no
inclusion, exclusion, denominator, weight, estimator, or payment effect and
does not change the scorecard probe, counts, or claim gates.

Part D exclusion rules also has only a component closure. The CY2022 sources
resolve missing-document review, unresolved failed status, and predeadline cure,
but not the full taxonomy, counts, decision stages, later treatment, replacement,
weights, or estimator effects. The FY2020 exclusions are historical comparison
only. None of this turns the scorecard estimate into debt or recovery.

## Boundary

These are scorecard probes, not the full PaymentAccuracy program-year dataset.
The VA row is source-reviewed and reconciles to the FY2024 annual workbook; its
estimate uses an October 2022 through September 2023 sample. The probes do not
prove fraud, waste, abuse, poor performance, or savings. Further work still
needs methodology detail, corrective-action status, recovery basis, control
cost, and beneficiary/service-access floor evidence.
