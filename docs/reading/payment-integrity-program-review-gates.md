# Payment Integrity Program Review Gates

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_program_review_gates_q4_2025.jsonl`

This packet converts the selected PaymentAccuracy scorecard probes into a
review gate. The goal is to identify what must be extracted before an improper
payment estimate can support a scoreable cost-down review.

## Gate Fields

Each selected program remains blocked before a savings score until four evidence
families are attached:

- methodology and sample-design evidence
- access, denial, appeal, reversal, and timeliness floors
- corrective-action owner, milestone, and implementation status
- confidence limits or other uncertainty fields

## Program Notes

Medicare Part D has a scorecard overpayment probe and a state-data root-cause
signal. The next useful artifact is a methodology and access-floor extract that
separates better data matching from benefit disruption risk.

VA Purchased Long Term Services and Supports has a larger overpayment-rate probe
and a documentation root-cause signal. The next useful artifact must distinguish
documentation defects, medical-review controls, veteran access, and recoverable
overpayments.

Federal Crop Insurance has a smaller but concrete scorecard probe. The next
useful artifact is a compliance-review and quality-control extract that keeps
producer access, appeals, and program-delivery floors visible.

Medicaid has a larger CMS scorecard probe and a PERM/state-cycle methodology
risk. The next useful artifact is a methodology and access-floor extract that
separates improper-payment measurement from fraud, waste, abuse, and recoverable
savings claims.

## Boundary

These rows do not say money has been wasted. They do not estimate recoverable
savings. They only say which safeguards and methodology fields must be extracted
before any public savings discussion.
