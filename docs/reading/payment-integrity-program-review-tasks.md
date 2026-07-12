# Payment Integrity Program Review Tasks

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_program_review_tasks_q4_2025.jsonl`

This packet turns the program-review gates into a concrete extraction queue.
Each selected PaymentAccuracy scorecard program gets four required task rows:
methodology, access floor, corrective action, and confidence limits.

## Why This Matters

Improper-payment estimates can identify control surfaces, but they cannot be
treated as savings by themselves. Before any public cost-down score, TAXLANE
needs to know how the estimate was produced, whether tighter controls would
block lawful benefits or services, what corrective action is actually underway,
and how uncertain the estimate is.

## Current Queue

Medicare Part D tasks focus on payment universe/methodology, beneficiary access
and appeals, state-data corrective actions, and uncertainty around overpayment
versus recoverable amount.

VA PLTSS tasks focus on reviewed-claim methodology, veteran access and care
continuity, targeted medical-review implementation, and the distinction between
documentation defects and recoverable overpayments.

Federal Crop Insurance tasks focus on estimate methodology, producer access and
appeals, quality-control/compliance-review implementation, and uncertainty
around agency process error versus recoverable savings.

Medicaid tasks focus on PERM methodology, beneficiary/state access floors,
state corrective-action implementation, and uncertainty around improper-payment
measurement versus fraud, waste, abuse, and recoverable savings.

## Boundary

Use these rows as extraction work items only. They do not say waste occurred and
do not estimate recoverable savings.
