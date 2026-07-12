# Payment Integrity Methodology Gap Followups

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_gap_followups_q4_2025.jsonl`

This packet turns Medicare Part D, Medicaid, VA PLTSS, and USDA Federal Crop
Insurance methodology field reviews into a concrete source-work queue.

## Current Queue

Part D priorities 1 through 5 are unsupported by the captured scorecard result
and need source discovery: sample design, payment universe, estimation method,
exclusion rules, and payment type split.

Part D priorities 6 through 8 have partial support but still need precise
citation or reviewer decisions before any field can be closed: sample period,
state-data dependency treatment, and overpayment-versus-recoverable-amount
basis.

Medicaid priorities 1, 3, and 4 are unsupported by the captured result set:
sample design, estimation method, and exclusion rules. Medicaid priorities 2
and 5 through 8 have partial support but still need precise source capture or
reviewer decisions.

VA PLTSS priorities 1 through 3 and 5 through 8 have partial support but still
need PLTSS-specific source detail before closure. Priority 4, exclusion rules,
is unsupported by the captured result set and needs source discovery.

USDA Federal Crop Insurance priorities 1 through 3, 5, and 8 have partial
support but still need FCIC-specific detail before closure. Priorities 4 and 6
remain unsupported. Priority 7 flags a source mismatch: the current scorecard
uses data-access/outside-agency-control root-cause wording, not the older
agency-process-error framing.

## Boundary

These rows are follow-up tasks only. They do not close methodology fields, do
not estimate savings, and do not make a waste finding.
