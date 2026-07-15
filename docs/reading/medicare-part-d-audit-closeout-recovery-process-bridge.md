# Medicare Part D Audit-Closeout Recovery-Process Bridge

Machine record:
`data/derived/breadth_benchmark_matrix/medicare_part_d_audit_closeout_recovery_process_bridge.q4-2025.v1.draft.json`.

## Current process component

The captured PaymentAccuracy Part D scorecard describes a current Q4 2025 audit
and recovery process. On PDF file page 1, CMS says it conducted plan-sponsor
audits focused on drugs at high risk of overpayment, with the stated aims of
education and identifying, reducing, and recovering overpayments.

On PDF file page 2, the recovery-plan table identifies `Recovery Audit` as the
method. It says CMS issued closeout notices for the Adcirca, Revatio, and Cialis
national audit requiring sponsors to delete PDE records determined improper
under Medicare Part D. The publisher describes deletion as resulting in
recovery of those payments to the program.

The same table separately describes future closeout notifications for the
Durable Medical Equipment and Tepezza national audits. Those planned notices
would instruct plans to delete improper PDE records. They are not treated as
already issued or completed actions.

This closes one current-process component internally: the scorecard documents
an audit-closeout pathway from an audit determination to required PDE deletion
and publisher-described recovery.

## Period and amount firewall

The process description is from Q4 2025. It is not same-period evidence for the
FY2024 estimate based on CY2022 payments. The scorecard supplies no PDE, cohort,
amount, or period linkage between the named audits and that statistical
estimate. It therefore supplies no recovery percentage and no recoverable or
collectible amount.

The remaining evidence need includes PDE identifiers, counts, and amounts;
corrected-GDC-to-recoverable-payment calculation; debt establishment and
notice, dispute, appeal, or reversal treatment; the mechanics connecting PDE
deletion to reconciliation, offset, receivable, refund, or cash; gross
recoverable, collectible, collected, waived, compromised, written-off, and
outstanding amounts; liability allocation; process completeness; and control
cost.

## Guardrails and status

“Resulting in recovery” is the publisher's process description, not proof that
cash was received. PDE deletion does not itself establish debt, collectibility,
cash collection, or full federal recovery. Gross Drug Cost combines plan and
beneficiary liability, and the source does not map statutory government subsidy
or federal recovery to that liability basis.

The scorecard's fraud, waste, and abuse education objective is not a fraud or
waste finding. No debt, collectible amount, recovery amount, prevention amount,
control-cost estimate, or savings estimate is inferred.

One component closes internally, but the full `overpayment versus recoverable
amount basis` field remains open. Medicare Part D stays three fields closed and
five open. Every public, field-closure, scoring, fraud, waste, debt,
collectibility, recovery, prevention, and savings gate remains false.
