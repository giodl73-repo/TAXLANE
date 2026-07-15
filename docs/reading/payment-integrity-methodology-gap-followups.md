# Payment Integrity Methodology Gap Followups

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_gap_followups_q4_2025.jsonl`

This packet turns Medicare Part D, Medicaid, VA PLTSS, and USDA Federal Crop
Insurance methodology field reviews into a concrete source-work queue.

## Current Queue

Part D payment type split now has exact same-period annual support and is closed
internally alongside sample period and sponsor documentation dependency
treatment. The five remaining followups cover sample design, payment universe,
estimation method, exclusion rules, and overpayment-versus-recoverable-amount
basis.

The payment-universe followup now preserves both the closed PDE/GDC
measurement-object and published-denominator component and the separate
reconciliation-PDE adjustment-documentation component. For an adjustment after
reconciliation, the latter requires both reconciliation-PDE-aligned documents
and additional linked adjustment documents. It does not disclose inclusion,
exclusion, denominator, weight, estimator, or payment effects. The followup
therefore still requests the full stream taxonomy, adjustment and settlement
rules, covered populations and phases, double-count controls, and the mapping
from combined plan-beneficiary liability to federal outlays and row 828's
$96,521.39 million denominator.

The overpayment-versus-recoverable followup preserves the current
audit-closeout PDE-deletion process component while seeking same-cohort lineage
from the FY2024/CY2022 estimate through audit determination, debt and appeal,
deletion and settlement, collectibility, actual collection, write-off, and
control cost.

The sample-design followup is now narrower: preserve the supported PDE unit,
stratified-random description, sponsor routing, reconciliation cutoff, and
statistical-governance components, then seek the national sample size, frame
and coverage, stratum definitions, allocation, inclusion probabilities,
within-stratum selection implementation, replacement and nonresponse rules,
weights, and PDE-to-beneficiary-simulation linkage. Do not reopen the supported
components or treat them as full design closure.

The estimation-method followup now preserves one closed published-output
component. Findings publish the 95% dollar and rate intervals, while annual row
828 reports its confidence label and 0.42 margin-of-error field without units or
formula. Preserve the web-verified APR process observations and custody blocker.
Seek formula, weights, PDE-to-beneficiary linkage, simulation, record treatment,
variance and confidence-limit construction, margin-of-error definition,
same-period 5% confirmation, and rounding mechanics. The 2026 background page is
corroborative only.

The exclusion-rules followup now preserves a closed missing-document treatment
component without closing the field. Do not re-research whether timely
problematic files are reviewed, whether an unresolved Missing Documentation
Form remains failed, or whether predeadline cure is permitted. Seek the complete
CY2022 taxonomy, current counts and decision stages, distinctions among no,
late, missing, invalid, and incomplete submissions, post-deadline and appeal
treatment, replacement and weights, estimator effects, and the reason for any
change from FY2020. Treat FY2020's 27 exclusions as historical comparison only.

For the inherited state-data item, the CY2022 submission guide and FAQ now
establish sponsor-documentation treatment. The captured scorecard does not
support a state-data characterization, so that field is closed internally under
the corrected label.

Medicaid priorities 1, 3, and 4 are unsupported by the captured result set:
sample design, estimation method, and exclusion rules. Medicaid priorities 2
and 5 through 8 have partial support but still need precise source capture or
reviewer decisions.

The VA PLTSS payment-type followup now has a complete category reconciliation.
The other priorities still need PLTSS-specific source detail; exclusion rules
remain unsupported and need source discovery.

USDA Federal Crop Insurance priorities 1 through 3, 5, and 8 have partial
support but still need FCIC-specific detail before closure. Priorities 4 and 6
remain unsupported. Priority 7 flags a source mismatch: the current scorecard
uses data-access/outside-agency-control root-cause wording, not the older
agency-process-error framing.

## Boundary

These rows are follow-up tasks only. They do not close methodology fields, do
not estimate savings, and do not make a waste finding.
