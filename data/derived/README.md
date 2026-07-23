# Derived Records

This directory is reserved for model outputs and transformed records derived
from reviewed extracted data.

Examples of future derived records:

- Constant-dollar versions of receipt or outlay records.
- Lane rollups from OMB function and subfunction records.
- Taxpayer receipt allocation scenarios.
- Program-linked tax design scenarios.

Derived records are blocked until their input records, derivation method,
allocation labels, and review status are documented.

## Current Derived Models

| Directory | Model | Status |
|---|---|---|
| `taxpayer_receipt_model/` | Draft placeholder visibility receipt scenarios derived from lane crosswalks and modeled outlay shares. | Draft; not a taxpayer calculator. |
| `accountability_evidence/` | Draft source-custody accountability evidence records. | Draft; not fraud findings, waste findings, abuse findings, or performance scores. |
| `lane_crosswalk/` | Draft mapping from OMB Table 3.1 functions and Table 3.2 subfunctions to TAXLANE public-purpose lane IDs. | Draft; method note documented. |
| `income_tax_outlay_model/` | Yearly individual income-tax receipts allocated across broad OMB outlay categories by proportional outlay share, with decade summaries derived from annual rows. | Draft; method and source profile documented. |
| `income_tax_outlay_subfunction_model/` | Yearly individual income-tax receipts allocated across OMB Table 3.2 subfunctions by proportional outlay share. | Draft; method and source profile documented. |
| `spend_category_map/` | Top FY2025 OMB subfunction spend categories with source-routing and accountability-question metadata. | Draft; question surface only, not performance findings. |
| `efficiency_pressure/` | Draft pressure rows for high-outlay surfaces that deserve cost/performance scrutiny over time. | Draft; not fraud, waste, abuse, or poor-performance findings. |
| `contribution_alignment/` | Draft lane alignment rows for pay-in/pay-out, per-person denominator, and Medicare split questions, plus the OMB Medicare source-boundary check and CMS Trustees part-financing split. | Draft; design surface only, not individual liability or legal allocation. |
| `denominator_requirements/` | Draft denominator requirements plus sourced TY2022 tax-return, CY2025 Medicare enrollment, CY2025 Census civic, and CY2025 Social Security denominator values, with per-unit display readiness and receipt-card rows. | Draft; blocks false per-person precision until sources are extracted. |
| `breadth_benchmark_matrix/` | Typed breadth/depth inventory with current values, matched benchmarks, comparison grades, explicit coverage gaps, separate improper-payment/fraud/savings fields, and the current `taxlane_showcase_readiness_summary.v1.draft.json` machine handoff. | Draft; showcase-ready as a source-custody/readiness guardrail, not a solver, rate, savings, public-card, department-cut, technology-savings, or balanced-budget output. |
| `headline_basis_crosswalk/` | Typed measure-selection crosswalk for gross/net, function/subfunction, federal/system, and outlay/GDP headlines. | Draft; measures are explicitly not interchangeable. |
