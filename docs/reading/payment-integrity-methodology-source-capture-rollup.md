# Payment Integrity Methodology Source Capture Rollup

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_source_capture_rollup_q4_2025.jsonl`

This packet summarizes the Medicare Part D, Medicaid, VA PLTSS, and USDA
Federal Crop Insurance methodology source-capture pass.

## Current Status

All eight Part D methodology gap-followup rows, all eight Medicaid methodology
gap-followup rows, all eight VA PLTSS methodology gap-followup rows, and all
eight USDA Federal Crop Insurance gap-followup rows now have linked
source-capture rows. The rollup converts those captures into reviewer actions:

- decide whether the capture is enough for field closure; or
- queue a narrower source gap for missing details.

Closure-readiness rows are built for all four programs. Part D has internal
closure decisions for sample period, payment type split, and sponsor
documentation dependency treatment, leaving five fields needing narrower
source work. VA PLTSS has sample-period and payment-type-split decisions with
six fields remaining. Part D's exact annual category reconciliation does not
establish recoverable dollars.

The Part D payment-universe rollup now records two component closures, not a
field closure. The guide and findings identify the PDE/GDC measurement
object, and row 828's $96,521.39 million outlays value rounds to the published
$96.52 billion denominator. Appendix A also requires both reconciliation-PDE-
aligned documentation and additional linked adjustment documentation after a
sampled reconciliation PDE is adjusted. It discloses no inclusion, exclusion,
denominator, weight, estimator, or payment effect. Complete stream taxonomy and
combined-liability-to-federal-outlay mapping remain unresolved, so the three-
closed/five-open field count is unchanged. The existing source-capture identity
and rollup foreign key are preserved.

The overpayment-versus-recoverable rollup likewise records only a component.
The scorecard documents a current audit-closeout PDE-deletion recovery process,
including issued named-audit notices and separately planned notices. It does not
link that Q4 2025 process by cohort or amount to the FY2024/CY2022 estimate, so
debt, collectibility, collection, write-off, and control-cost lineage remains
open and the field count is unchanged.

The captured Part D scorecard also resolves the stale rate and root-cause probe:
3.16% and $3,053 million, centered on sponsor documentation rather than state
data. The CY2022 guide and FAQ provide the same-period treatment detail used to
close that field internally.

The sample-design capture now has a bounded evidence ceiling rather than a
generic partial-support label. It supports the PDE unit, stratified-random
description, sponsor routing, reconciliation cutoff, and statistical
governance. It does not supply the national sample size, frame, stratum
definitions, allocation, inclusion probabilities, selection implementation,
replacement or nonresponse rules, weights, or beneficiary-simulation linkage.
The five-open-field count is unchanged.

The estimation-method capture now also has a custody-blocked ceiling. Official
APR text is web-verified at printed page 88, while checksum-custodied findings
and row 828 now close the published confidence-interval and margin-of-error
output component. The APR bytes remain unavailable and the 2026 background page
is current-only. Formula, weights, aggregation, simulation,
PDE-to-beneficiary linkage, record treatment, variance and confidence-limit
construction, margin-of-error definition, same-period 5% confirmation, and
rounding remain open. No field closes, so Part D remains three closed and five
open.

The exclusion-rules capture now closes one component internally. Same-period
sources resolve timely problematic-file review, unresolved Missing Documentation
Form fail treatment, predeadline cure, and FY2024 inclusion of invalid or
missing documentation in the reported category. They do not disclose the full
taxonomy, current counts, decision stages, submission-state distinctions,
post-deadline or appeal treatment, replacement, weights, or estimator effects.
FY2020's 27 exclusions remain comparison-only. The five-open-field count is
unchanged.

## Boundary

These rows do not close methodology fields. They do not estimate savings and do
not make a waste finding.
