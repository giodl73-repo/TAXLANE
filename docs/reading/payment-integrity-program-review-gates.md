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

Medicare Part D has a reconciled scorecard overpayment probe centered on sponsor
drug, pricing, and documentation inputs rather than state data. Sample design
and estimation method now have evidence ceilings, not closures. The estimator
ceiling web-verifies official APR process text but lacks official PDF custody;
formula, weights, aggregation, simulation, PDE-to-beneficiary linkage, record
treatment, variance, same-period 5% confirmation, and reconciliation remain
open. Part D stays three closed and five open. Access, appeals, recoverability,
and control-cost gates also remain blocked.

The Part D payment-universe component now identifies the PDE/GDC measurement
object and reconciles row 828's $96,521.39 million outlays to the published
$96.52 billion denominator. The gate remains blocked because the complete
included/excluded payment taxonomy, combined plan-beneficiary liability to
federal-outlay mapping, and record-level denominator construction are still
missing. No field count or scoring gate changes.

Appendix A now closes a second payment-universe component: after a sampled
reconciliation PDE is adjusted, reconciliation-PDE-aligned documentation
remains required and linked adjustment documentation is additionally required.
The cutoff and final reconciliation target are prior context. No inclusion,
exclusion, denominator, weight, estimator, or payment effect is disclosed, so
the program gate and field count remain unchanged.

The Part D audit-closeout recovery-process component documents issued named-
audit notices requiring deletion of audit-determined improper PDE records and
publisher-described recovery, with planned notices kept separate. It supplies
no amount or cohort link to the FY2024/CY2022 estimate. Debt, appeal,
settlement, collectibility, collection, write-off, liability allocation, and
control-cost evidence remain required, so no program gate changes.

The Part D published-uncertainty output component now preserves the findings'
95% dollar and rate confidence intervals and row 828's confidence label and
0.42 margin-of-error value. The row does not disclose the 0.42 field's units or
formula, and the value is not forced to reconcile to the findings bounds. APR
custody and the full estimator mechanics remain open, so no program gate or
field count changes.

The Part D missing-document exclusion-treatment component is internally closed,
but full exclusion rules remain open for taxonomy, current counts, decision
stages, post-deadline and appeal treatment, replacement, weights, and estimator
effects. FY2020's 27 exclusions are historical comparison only. This component
does not change the program gate or the three-closed/five-open field count.

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
