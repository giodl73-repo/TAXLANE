# Breadth And Benchmark Matrix

This family answers three portfolio questions in one controlled record set:

1. Are the important fiscal lanes covered?
2. Do we have a top-line current value and a defensible comparison?
3. Have efficiency gaps, improper payments, fraud, and savings remained
   separate?

Canonical draft rows:
`breadth_benchmark_matrix.v1.draft.jsonl`.

Public scoreboard: `docs/reading/current-versus-benchmark-scoreboard.md`.

`global_country_comparison_coverage.v1.draft.json` defines the European,
Asian, and additional peer panels, lane-specific metrics, official source
families, and claim gates for all 15 lane IDs. It is an acquisition contract,
not observed country data; its reader is
`docs/reading/global-country-comparison-coverage.md`.

`international_comparator_target_rubric.v1.draft.json` defines how an observed
panel becomes a fair reference: peer median and IQR for typical context,
favorable quartile only after outcome and transferability gates, and sustained
high performers as examples rather than automatic targets. It blocks best-
country and small-panel P85 targeting and preserves the causal savings firewall.

`program_lane_target_cost_contract.v1.draft.json` defines the next bridge from
comparison evidence to balanced rates across all 15 analytical lanes. It maps
those lanes to the existing 17-row FY2025 rate model without treating revenue-
solvency or payment integrity as additive spending and without dropping the two
offset rows. Every lane declares a target-cost method, policy levers, outcome
floors, federal translation, financing bases, and ten-year solver treatment.
All numeric target and balanced-rate gates remain false.

`balanced_rate_readiness_gate.v1.draft.json` freezes the Pulse 80 no-rate
boundary: all-receipt funding shares, residual general-fund requirement shares,
and assigned-base effective rates remain uncalculated until target paths,
assigned bases, the integrated solver, endogenous interest, and a zero
unrounded deficit gap all reconcile.

`final_closure_readiness_gate.v1.draft.json` freezes the Pulse 81 no-closure
boundary: distributional analysis, behavioral sensitivity, macro feedback,
interaction scoring, reserve/emergency stress tests, eight-role review,
public-language review, public rate cards, and a zero unrounded deficit gap all
remain required before any final public closure claim.

`adaptive_rate_system_contract.v1.draft.json` starts the adaptive-rate phase by
defining the annual update lifecycle, assigned-base requirements, denominator
definitions, and separate rate-calculation and rate-publication gates. Every
rate output remains null and every public claim gate remains false.

`overspending_risk_taxonomy.v1.draft.json` defines safe review-needed signal
classes for cost growth, unit cost, outcomes, administration, procurement,
payment integrity, and technology gaps. It blocks waste, fraud, recoverability,
savings, budget-score, and department-cut claims unless separate positive
evidence exists.

`technology_transition_operating_model.v1.draft.json` defines the modernization
scenario contract. It requires implementation, training, cybersecurity,
privacy, fallback, service-risk, phase-in, measured-productivity, and stress
fields before any lower target cost or rate effect can be considered.

`public_rate_card_v2_contract.v1.draft.json` defines the future public-card
shape for valid and blocked rates. It treats `not_calculated` and `blocked` as
first-class outcomes and keeps statutory-rate language unavailable until
publication gates pass.

`oecd_cofog_country_panel.data2022.v1.draft.json` is the first observed batch:
99 captured 2022 country/function observations and 11 explicit missing cells
across 11 countries and ten COFOG divisions. Its reader is
`docs/reading/oecd-cofog-country-panel-2022.md`.

`hybrid_cofog_country_panel.data2022.v1.draft.json` reconciles the OECD panel
with IMF GFS and uses IMF only to fill Canada's ten missing cells. It contains
109 observed cells and one unresolved U.S. environmental-protection cell.

`fiscal_country_panel.data2022.v1.draft.json` adds 2022 tax revenue and tax
mix, total revenue, overall and primary balances, gross and net debt, and direct
OECD D.41 interest payable plus separately sourced net interest for the 11 core
peers. Singapore's interest remains missing. Gross and net interest are not
substituted or treated as savings targets.

`qpsd_maturity_country_panel.2022q4.v1.draft.json` adds a joint World Bank–IMF
general-government maturity snapshot. Ten peers report short-term debt by
original maturity, but only six also report long-term-original debt due within
one year. A combined near-term stock is limited to those six; missing maturity
components are never treated as zero.

`socx_oldage_family_country_panel.data2022.v1.draft.json` adds matched OECD
public old-age/survivors and family-benefit spending for seven countries, with
cash and service components kept separate. Five countries remain missing;
mixed-year family tax-break figures are not spliced into the 2022 panel.

`pension_replacement_country_panel.model2024.v1.draft.json` captures modeled
gross and net mandatory-scheme replacement rates for an average-earning male
worker entering at age 22 in 2024. It is a future entitlement model, not a
current-retiree outcome or a spending-efficiency score.

`age_relative_poverty_country_panel.v1.draft.json` adds actual older-person and
child relative-income-poverty observations with every country year exposed.
The measure uses each country's own median-income threshold and remains separate
from spending and modeled pension entitlements.

The health depth phase begins with
`health_cost_decomposition.v1.draft.json`, which separates price, volume and
intensity, administration, coverage and case mix, and outcomes without treating
different years or denominators as additive savings.

`health_service_price_volume_bridge.cy2024.v1.draft.json` then decomposes
CY2024 growth for hospital, physician/clinical, and retail-drug spending while
keeping unmatched category peer benchmarks blocked.

`health_category_benchmark_ladder.v1.draft.json` records which hospital,
physician, and retail-drug comparisons are matched spending measures, domestic
price references, or mechanism evidence—and why none is yet a savings target.

`health_target_admissibility.v1.draft.json` tests whether Medicare-relative
references can become scenario anchors using current access, margin, quality,
and payment-adequacy evidence; it blocks a universal target.

`health_medicare_relative_scenarios.v1.draft.json` defines low, central, and
high policy paths while blocking dollar scoring until a matched commercial
allowed-spending base and behavioral/access model exist.

`health_commercial_sample_sensitivity.v1.draft.json` applies those paths only
to Milliman's cited analytical data volumes, exposing basis sensitivity while
blocking national and net-savings claims.

`health_national_phi_sensitivity.v1.draft.json` bridges the scenarios to CMS
CY2024 private-insurance hospital and physician/clinical payments, labels the
cross-source result Grade C, and blocks savings and federal-budget claims.

`fiscal_path_scenarios.v1.draft.json` translates CBO's 2036 primary-deficit
baseline into partial-closure, balance, and surplus adjustment equivalents
without claiming that primary balance automatically stabilizes debt.

`fiscal_debt_dynamics_2026_2036.v1.draft.json` extends those targets across an
annual CBO baseline and adds first-order interest feedback, while preserving a
hard boundary between transparent scenario arithmetic and a policy score.

`fiscal_policy_scale_baskets.v1.draft.json` compares the cumulative paths with
CBO-scored policy-option magnitudes. Its arithmetic baskets demonstrate scale;
they are not additive package scores or recommendations.

`fiscal_policy_distribution_screen.v1.draft.json` maps statutory channels to
likely burden bearers and protection gates without inventing quantified
incidence. A joint microsimulation remains required before package claims.

`payment_integrity_depth_card.fy2024.v1.draft.json` reconciles the official
FY2024 annual workbook's improper, unknown, overpayment, underpayment, and
technically-improper totals while keeping court-confirmed-fraud and agency-
recovery tables parallel until program, period, and definitions match.

`payment_integrity_bounded_factual_examples.fy2024.v1.draft.json` packages
seven reviewed examples, a five-class evidence legend, and ordered Part D,
Medicaid, PLTSS, and FCIC cards for bounded source-labeled public explanation.
It closes zero components and fields and leaves every established public,
performance, fraud, waste, debt, recovery, prevention, and savings gate false.

`federal_crop_insurance_payment_integrity_bridge.fy2024-q4-2025.v1.draft.json`
reconciles the FCIC annual row, Q4 2025 scorecard, and RMA review-period
evidence. The linked
`federal_crop_insurance_root_cause_definition_bridge.fy2024.v1.draft.json`
adds same-period USDA definitions for the failure-to-access and inability-to-
access categories. The linked
`federal_crop_insurance_payment_universe_bridge.fy2024.v1.draft.json` adds the
official FY2024 payment categories and AIP payment tiers. Four fields are closed
internally while sample design, estimation method, exclusion rules, and
recoverable-savings basis remain open. Every scoring, public-claim, fraud,
waste, recovery, and savings gate
remain blocked. USDA-wide Do Not Pay figures following the FCIC section are not
part of the bridges. Other Information on printed pages 60-61 of the FCIC/RMA
financial statements is unaudited; its apparent $579.93M typo is excluded in
favor of the annual workbook's $573.93M.

`federal_crop_insurance_sample_design_component_bridge.fy2024.v1.draft.json`
adds a narrow component-only decision for the disclosed FY2024 sampling
governance and independent methodology review. It does not change the four-open
field count: frame, allocation, probabilities, randomization, replacement,
nonresponse, weights, estimator, and variance remain undisclosed, and compliance
is not public reproducibility.

`federal_crop_insurance_historical_sampling_method_bridge.fy2020.v1.draft.json`
preserves a separate historical benchmark from FY2020 Other Information. For
reinsurance year 2018, the source discloses simple-random policy selection, all
three named payment categories, and statistically valid rate and dollar
estimates. The section is unaudited, and nothing in it proves that the method
continued through FY2024. It closes no current field, changes no 4/4 count, and
relaxes no scoring or claim gate.

`federal_crop_insurance_public_methodology_evidence_ceiling.fy2025.v1.draft.json`
records why the remaining method cannot be reproduced from the public sources.
OMB M-21-19 requires point and confidence-interval estimates but directs the
S&EMP and checklist to an agency secure MAX page. The later FY2025 FCIC report
repeats categories, AIP tiers, annual statistical-validity language, and a
3.29-percent actual rate, but publishes no estimator, weights, variance, or
exclusions. This closes zero fields, preserves the 4/4 aggregate, and blocks all
score and claim gates.

`federal_crop_insurance_recovery_lineage_boundary_bridge.fy2024.v1.draft.json`
follows the same 326-policy sample across no-finding closure, Initial and Final
Findings, review completion, and rate reporting. RMA's Manager's Reports keep
ordinary compliance findings, IPERIA sample progress, and criminal outcomes in
separate sections. The bridge closes a narrow disposition and non-additivity
component, not the recoverable-savings field: no public source reconciles the
sample to debt, appeals, collections, prevention, or control cost.

`federal_crop_insurance_appeal_collectibility_governance_bridge.fy2024.v1.draft.json`
adds the published post-Finding state transitions. CARS receipt starts a 45-day
evidence-backed dispute path; the 2022 SRA provides possible correction,
payment, repayment discretion, and setoff while preserving administrative
appeal. This closes a narrow governance component, not collectibility or
recoverable savings. Contractual authority is not an exercised remedy or cash
collection.

`federal_crop_insurance_public_cohort_outcome_evidence_ceiling.fy2024.v1.draft.json`
records the end of the public Manager's Report path for the FY2024/RY2022
cohort. After completion and rate reporting, the September and November 2024
reports move to the 388-policy FY2025/RY2023 cohort and keep ordinary compliance
findings separate. No retrospective appeal, final-debt, setoff, or collection
lineage is published. Zero fields close and the 4/4 aggregate remains.

`federal_crop_insurance_cohort_disposition_request_specification.fy2024.v1.draft.json`
turns the ceiling into an existing-records-only request contract. It specifies
the cohort, likely custodians, disposition states, privacy exclusions,
deidentified or aggregate fallbacks, electronic formats, segregability, and
owner placeholders. The linked request template is unsent; it creates no fee
commitment and closes no field.

`federal_crop_insurance_foia_response_intake_contract.v1.draft.json` adds the
owner preflight and response lifecycle. It records timing, clarification, fees,
productions, denials, no-records responses, and appeals while preserving an
interpretation firewall. The blank intake template begins at
`draft_not_submitted`; no external action or field closure occurs.

`va_pltss_payment_type_composition_bridge.fy2025.v1.draft.json` closes the
PLTSS payment-type split internally from the same-period FY2024 annual row and
uses the FY2025 AFR only as later-cycle taxonomy corroboration. PLTSS moves to
two closed and six open fields. Statistical overpayments remain separate from
bills, receivables, collectible debt, collections, recovery, and savings.

`va_pltss_documentation_recoverability_boundary.fy2025.v1.draft.json` records
current VA classification and certified-return rules. Documentation failure is
not automatically monetary loss, unknown and technical/non-monetary categories
remain separate, and recovery requires returned funds. No source maps PLTSS
causes through category, bill, dispute, collection, or certified recovery, so
the full field remains open and the 2/6 aggregate is unchanged.

`va_pltss_same_cohort_debt_collection_lineage_evidence_ceiling.fy2024-q4-2025.v1.draft.json`
inventories the six existing checksum-verified sources without a new search or
outbound action. The first three are reviewed evidence; the AFR and policies
are later/current context that cannot assign historical dispositions. The
bounded inventory contains no same-cohort link from sampled payment through
debt, dispute, collectibility, collection, and certified cash. Zero components
and fields close, PLTSS remains two closed and six open, and all gates remain
false.

The matrix deliberately includes coverage-gap rows. An explicit missing value
is more useful than a false benchmark.

The first Tier 2 depth artifact is
`veterans_depth_card.fy2025.v1.draft.json`, which reconciles the complete
Veterans Benefits and Services function and keeps service probes separate from
performance, fraud, and savings claims.

`transportation_depth_card.fy2025.v1.draft.json` reconciles all four federal
transportation subfunctions and blocks peer comparisons until state/local,
trust-fund, asset, delivery, and outcome scopes are matched.

`education_depth_card.fy2025.v1.draft.json` reconciles function 500 and keeps
the negative higher-education net entry blocked behind account-level analysis.

`medicare_part_d_payment_type_composition_bridge.fy2024.v1.draft.json` uses
the exact annual row and CMS FY2024 findings to close the payment-type split
internally. At that bridge stage, Part D was two closed and six open; no debt, collectibility,
recovery, fraud, waste, or savings claim is allowed.

`medicare_part_d_sponsor_documentation_dependency_evidence_ceiling.fy2024.v1.draft.json`
reconciles the captured scorecard to the annual row and CMS findings, corrects
the stale Part D rate and root-cause transcription, and reframes the remaining
state-data field around sponsor documentation. At that evidence-ceiling stage, no field closes; Part D remains
two closed and six open.

`medicare_part_d_sponsor_documentation_dependency_bridge.fy2024.v1.draft.json`
adds the same-period guide and FAQ, closes sponsor-documentation dependency
treatment internally, and moves Part D to three closed and five open. Review
failure remains separate from debt, recovery, fraud, waste, and savings.

`medicare_part_d_sample_design_evidence_ceiling.fy2024.v1.draft.json` records
the same-period sample-design ceiling. It supports PDEs as the sampled unit, a
statistically valid stratified-random selection description, sponsor-contract
routing, reconciliation-PDE timing, and statistical-governance boundaries. It
does not publish the national sample size, frame, stratum definitions,
allocation, inclusion probabilities, selection implementation, replacement or
nonresponse rules, weights, or the PDE-to-beneficiary-simulation relationship.
Sample design stays open and Part D remains three closed and five open.

`medicare_part_d_estimation_method_evidence_ceiling.fy2024.v1.draft.json`
records the estimation-process evidence and custody blocker. Web extraction
verifies the official HHS FY2024 Annual Performance Report passage assigning a
GDC error to each audited PDE and describing representative-beneficiary
simulation, but Akamai HTTP 403 prevented official PDF custody. The captured
FY2024 findings supply same-period statistical-governance, output, confidence-
limit, and GDC-direction evidence; the 2026 CMS background remains current-only
corroboration. No component or field closes, and Part D remains three closed and
five open pending formula, weights, aggregation, simulation, sample linkage,
record-treatment, variance, and reconciliation mechanics.

`medicare_part_d_missing_document_exclusion_treatment_bridge.fy2024.v1.draft.json`
closes one narrow internal component. Timely incomplete or problematic files
remain in CY2022 review; an unresolved Missing Documentation Form leaves the PDE
failed; and missing or invalid evidence can be corrected before the final
deadline. The FY2024 findings count invalid or missing documentation in the
reported category. FY2020's 27 exclusions are comparison-only. Full exclusion
rules remain open, so Part D stays three closed and five open and all scoring,
fraud, waste, debt, recovery, and savings gates remain blocked.

`medicare_part_d_payment_universe_measurement_object_denominator_bridge.fy2024.v1.draft.json`
closes one narrow internal component. It identifies sampled reconciliation PDE
records as the reviewed object, corrected-versus-reported GDC as the error
basis, and row 828's $96,521.39 million outlays as the exact value rounding to
CMS's $96.52 billion Part D Denominator. It does not enumerate the complete
payment universe or map combined plan-beneficiary liability to federal outlays.
Payment universe stays open, Part D remains three closed and five open, and all
claim and scoring gates remain blocked.

`medicare_part_d_audit_closeout_recovery_process_bridge.q4-2025.v1.draft.json`
closes one current-process component. The scorecard documents issued named-
audit closeout notices requiring deletion of audit-determined improper PDE
records and describes that deletion as recovery, while keeping planned DME and
Tepezza notices distinct. The Q4 2025 process has no amount or cohort linkage to
the FY2024/CY2022 estimate, so recoverable-amount basis stays open, Part D
remains three closed and five open, and all gates remain blocked.

`medicare_part_d_published_uncertainty_output_bridge.fy2024.v1.draft.json`
closes one same-period published-output component. Findings provide the 95%
dollar and rate confidence bounds, while annual row 828 preserves its confidence
label and 0.42 margin-of-error value. The row does not define units or a formula
for 0.42, and the bridge forces no reconciliation. Full estimator and APR
custody remain open, Part D stays three closed and five open, and all gates stay
blocked.

`medicare_part_d_reconciliation_pde_adjustment_documentation_bridge.cy2022.v1.draft.json`
closes only the two-track documentation treatment after a sampled reconciliation
PDE is adjusted. Reconciliation-PDE-aligned documentation remains required and
linked adjustment documentation is additionally required. The cutoff and final
reconciliation target are prior Pulse 39 and Pulse 43 context. No inclusion,
exclusion, denominator, weight, estimator, or payment effect is disclosed. Full
payment universe remains open, Part D remains three closed and five open, and
all gates remain blocked.

`medicare_part_d_sampling_estimation_plan_access_evidence_ceiling.fy2024.v1.draft.json`
records the governmentwide secure-MAX location for agency S&EMPs and OMB
checklists, inventories the prior validated Part D public evidence, and defines
the exact plan, checklist, appendices, version-history, sample, universe,
estimator, and exclusion details still needed. Secure MAX does not prove
exemption, withholding, nonexistence, or public unavailability. No request was
submitted and no outbound state changed. This ceiling closes zero components
and zero fields; Part D remains three closed and five open with all gates false.

`medicare_part_d_sampling_estimation_methodology_plan_request_specification.fy2024.v1.draft.json`
turns the access ceiling into an unsent, privacy-aware existing-records draft
for the final Part D S&EMP package and operative version records. It uses the
CMS non-claims filing route and HHS rules, excludes person-level and security
material, and requests native/searchable and reasonably segregable output
without predicting release. Owner, requester, fee, scope, and single-channel
preflight remain blocked. Zero components and fields close, Part D remains
three closed and five open, and all gates stay false.

`medicare_part_d_sampling_estimation_methodology_plan_foia_response_intake_contract.fy2024.v1.draft.json`
adds a hard all-false submission preflight, a closed-world 19-state lifecycle,
append-only event rules, a blank intake, administrative timing capture, an
appeal taxonomy, and production-review gates. Owner authorization is necessary
but insufficient. Administrative events and unreviewed productions cannot
close methodology fields. The request remains unsent, zero components and
fields close, Part D remains three closed and five open, and all gates stay
false.

`reviews/2026-07-14-payment-integrity-fy2024-annual-extraction-role-review.md`
closes the pending review action for the official FY2024 PaymentAccuracy
extraction. It checks raw custody, metadata, schema, all five extraction
artifacts, exact row counts, payment-class reconciliation, program-specific
measurement periods, the confirmed-fraud definition, the agency-recovery scope,
and preserved nulls. It permits bounded source-labeled factual reporting only.
Zero methodology components and fields close, program counts remain unchanged,
and every scoring and claim gate stays blocked.
