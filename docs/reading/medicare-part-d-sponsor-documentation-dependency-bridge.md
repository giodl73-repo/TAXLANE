# Medicare Part D Sponsor-Documentation Dependency Bridge

Machine record:
`data/derived/breadth_benchmark_matrix/medicare_part_d_sponsor_documentation_dependency_bridge.fy2024.v1.draft.json`.

## Result

The same-period CY2022 submission guide and FAQ establish how sponsor
documentation is treated. Selected reconciliation PDEs require aligned Claim
Detail Files and prescription or medication-order support. A Missing
Documentation Form records unsuccessful retrieval but does not replace valid
evidence; the PDE remains `fail`. Corrections require an approved reset or
resubmission before the April 19, 2024 final deadline, and all timely files are
reviewed even when incomplete.

The process also fixes the target record at the June 29, 2023 reconciliation
PDE cutoff and keeps successor sponsors responsible for predecessor records
under the federal ten-year retention rule. CMS's FY2024 findings then classify
2.70% as documentation-related overpayment error and 0.46% as drug/pricing
error, reconciling to the scorecard's 3.16% overpayment rate.

## Decision

Close the inherited `state-data dependency treatment` field under the accurate
label `sponsor documentation dependency treatment`. Part D moves from two
closed and six open fields to three closed and five open fields. Sample design,
payment universe, estimation method, exclusion rules, and recoverable-amount
basis remain open.

## Claim firewall

A failed PDE is a measurement status, not proof of monetary error. The 2.70%
documentation component is statistical, not identified debt. Outside-agency-
control taxonomy is not a legal nonrecoverability finding. No score or public
claim about collectibility, recovery, fraud, waste, or savings is allowed.
