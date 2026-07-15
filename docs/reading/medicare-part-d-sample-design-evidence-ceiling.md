# Medicare Part D Sample-Design Evidence Ceiling

Machine record:
`data/derived/breadth_benchmark_matrix/medicare_part_d_sample_design_evidence_ceiling.fy2024.v1.draft.json`.

## Supported components

The captured CMS sources establish that FY2024 reporting measures CY2022 Part
D payments and that the review unit is an individual Prescription Drug Event
(PDE) record. CMS assigns each selected record a PDE ID, routes sampled records
to the corresponding sponsor contract through the HPMS Part D IPM Module, and
provides ten identifying elements for each record to be validated. Documentation
must match the PDE version submitted for reconciliation by the June 29, 2023
cutoff.

The FY2024 Improper Payments Fact Sheet further describes the selection as a
statistically valid stratified random sample of PDEs. CMS also states that the
FY2024 Part D measure complied with OMB sampling and estimation-plan guidance
and used a statistically valid methodology. FY2024 follows methodology changes
implemented for FY2023, is treated as a baseline, and is not comparable with
reporting years before FY2023.

## Evidence ceiling

The captured sources do not disclose the actual national sample size; sampling-
frame definition, coverage, or size; stratum definitions and allocation; the
within-stratum selection procedure; inclusion probabilities; randomization
implementation; replacement, duplicate, ineligible-unit, or nonresponse rules; or the
relationship between the sampled PDE review and any separately described
beneficiary sample. Sample weights and their connection to projection, variance,
and confidence limits are also unavailable.

The guide's examples containing two or five PDEs explain upload mechanics. They
are not evidence of the national sample size. Likewise, the statistically valid
designation and sponsor-specific HPMS lists do not disclose a publicly
reproducible sample design.

## Decision and claim firewall

Record a bounded component-level evidence ceiling and keep `sample design`
open. No new field closes: Medicare Part D remains three fields closed and five open.
Methodology scoring and every public, fraud, waste, debt, recovery, and savings gate remain blocked.
Operational sampling and documentation rules do
not establish recoverable loss or savings.
