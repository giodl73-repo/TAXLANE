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

`pilot_lane_selection_gate.v1.draft.json` defines criteria for the first
deterministic simulator pilot and keeps the final pilot lane unselected until
role review clears normative and source conflicts.

`deterministic_annual_update_simulator_contract.v1.draft.json` defines the
narrow annual-update simulator contract. It requires baseline, modernization,
and stress paths, prohibits optimization, keeps funds separate, and blocks
lower-rate recognition unless all floors pass.

`public_thesis_packet.v1.draft.json` defines the role-reviewed public thesis
packet for the adaptive-rate phase. It permits explanatory design language only:
overspending risk rather than unsupported waste, technology transition rather
than automatic cuts, blocked rates as valid outcomes, and no statutory-rate,
effective-rate, savings, waste, fraud, department-cut, technology-savings, pilot,
solver, public-card, or balanced-budget claim.

`pilot_lane_selection_decision.v1.draft.json` selects transportation asset
maintenance and safety under the transportation-infrastructure lane for scaffold
work only. It does not run the simulator, set target costs, calculate rates,
publish a public card, estimate savings, find waste or fraud, set floor
thresholds, or make a balanced-budget claim.

`transportation_pilot_source_plan.v1.draft.json` names official source families
and custody requirements for the transportation pilot. It captures no source
bytes, closes no custody, creates no baseline or floor thresholds, and keeps all
target-cost, rate, savings, waste, fraud, technology-savings, simulator, solver,
and balanced-budget claims blocked.

`transportation_pilot_baseline_path_contract.v1.draft.json` defines the
FY2025-FY2035 current-law baseline row requirements for the transportation
pilot. It reuses the existing FY2025 transportation depth-card anchor, keeps
baseline rows empty until source custody is captured, and leaves simulator,
target-cost, rate, savings, waste, fraud, technology-savings, solver, floor, and
balanced-budget claims blocked.

`transportation_pilot_floor_indicator_contract.v1.draft.json` defines the
required outcome-floor indicator families for the transportation pilot. It sets
no floor thresholds, records no floor passes, keeps indicator rows empty until
source custody and threshold decisions exist, and leaves simulator, target-cost,
rate, savings, waste, fraud, technology-savings, solver, and balanced-budget
claims blocked.

`transportation_pilot_modernization_path_contract.v1.draft.json` defines the
modernization-path requirements for the transportation pilot. It treats
technology as a transition path rather than an automatic cut, keeps all
productivity and fiscal effects null, requires transition/admin/monitoring
costs and same-service-or-better evidence before credit, and leaves simulator,
target-cost, rate, savings, waste, fraud, technology-savings, solver, stress,
and balanced-budget claims blocked.

`transportation_pilot_stress_path_contract.v1.draft.json` defines the stress
path requirements for the transportation pilot. Stress is an adverse
realization of the same policy, not a harsher cut; all stress values remain
null, stress rows remain empty, and simulator, target-cost, rate, savings,
waste, fraud, technology-savings, solver, and balanced-budget claims remain
blocked.

`transportation_pilot_fy2025_anchor_custody.v1.draft.json` closes custody for
the already-local OMB FY2025 transportation anchor only. It records the raw file
path, byte count, SHA-256, and FY2025 component reconciliation while leaving the
full FY2025-FY2035 baseline path, trust-fund reconciliation, floors,
modernization, stress, simulator, rate, savings, and balanced-budget claims
blocked.

`transportation_pilot_partial_federal_outlay_path.v1.draft.json` adds the
already-local OMB Public Budget Database transportation federal net-outlay path
for FY2025-FY2031. FY2032-FY2035 remain explicit null missing rows; the record
does not complete trust-fund, offset, transfer, federal/state/local, simulator,
target-cost, rate, savings, or balanced-budget work.

`transportation_pilot_trust_fund_source_custody.v1.draft.json` locks custody for
the already-local OMB Appendix Chapter 13 funds PDF used by the transportation
trust-fund work. It extracts no annual values and is not a trust-fund
reconciliation, fund-balance path, baseline, rate, savings, or balanced-budget
claim.

`transportation_pilot_trust_fund_accounting_boundary.v1.draft.json` extracts
only the OMB funds-appendix accounting boundary for transportation trust-fund
work. It keeps trust funds separate, requires explicit general-fund transfers,
and keeps all annual values, reconciliation, rates, savings, and balanced-budget
claims blocked.

`fund_group_fy2025_reconciliation_fixture.v1.draft.json` captures aggregate
FY2025 federal-fund and trust-fund OMB Appendix Chapter 13 accounting context,
including an explicit public rounding line. It is not transportation-specific
trust-fund data and does not open baseline, rate, savings, or balanced-budget
claims.

`solver_accounting_readiness_gate.v1.draft.json` states the only allowed solver
uses of the aggregate fund-group fixture: rounding, deficit-sign, and aggregate
fund-balance tests. It keeps solver readiness, transportation readiness, rates,
savings, and balanced-budget claims blocked.

`solver_input_inventory.v1.draft.json` names each required deterministic solver
input, any current partial/context artifact, and the missing evidence. Every row
remains not ready with a null value; no solver, rate, savings, or
balanced-budget claim is opened.

`reserve_rule_contract.v1.draft.json` defines the required reserve contribution,
withdrawal, cap, emergency override, payback, and rounding fields before any
solver run may use reserve accounting. It sets no numeric parameters and keeps
solver/rate/savings claims blocked.

`reserve_parameter_readiness_gate.v1.draft.json` names the parameter decisions
that remain blocked before reserves can enter a deterministic solver:
contribution formula, withdrawal rule, balance series, cap formula, emergency
override threshold, emergency deferral cap, future-year payback, rounding
residual, source vintage, and role review. Every value remains null.

`net_interest_formula_contract.v1.draft.json` defines the endogenous
net-interest formula boundary for the future solver. Debt stock, maturity,
rate, timing, interest-receipt, other-financing, and feedback-test inputs remain
null; net interest cannot be cut directly and no path or rate claim is opened.

`assigned_receipt_base_inventory.v1.draft.json` lists candidate receipt bases
and the required matched-year, perimeter, amount, elasticity, burden,
distribution, interaction, and yield fields. Every base value remains null and
statutory/effective rates remain blocked.

`distributional_effect_placeholder.v1.draft.json` defines the income-group,
incidence, benefit/service, interaction, macro-feedback, equity-floor, and
public-language fields required before any solver output, public rate card, tax
proposal, or balanced-budget claim. Every distribution value remains null.

`solver_input_readiness_rollup.v1.draft.json` summarizes the twelve deterministic
solver inputs after the reserve, net-interest, assigned-base, and distribution
contracts. Every input remains not ready with a null value.

`current_law_path_inventory.v1.draft.json` inventories the official annual
current-law paths required for the FY2025-through-FY2035 solver horizon. It
adds no annual values, allows no interpolation, and keeps trust funds separate.

`current_law_source_custody_preflight.v1.draft.json` defines the source-custody
packet required before current-law annual path values may be populated. It
submits no external request, captures no source values, and leaves all custody
fields null.

`lane_depth_explainability_tracker.v1.draft.json` answers whether the lane-depth
and public explainability layer is complete. It records all 15 analytical lanes,
marks every lane incomplete for full depth and public explainability, and blocks
solver, target-cost, rate, savings, waste, fraud, technology-savings, and
balanced-budget claims.

`lane_agent_work_order_plan.v1.draft.json` defines the safe scaled-agent work
order for lane-depth packets: one lane per clean worktree, common deliverables,
wave-level integration review, and the same solver/rate/savings/waste/fraud
claim firewall.

`wave1_public_topline_lane_depth_packets.v1.draft.json` executes the first
scaled lane-depth explainability scaffold for Health and Medicare, Social
Security, and National defense while keeping final depth completion, solver,
target-cost, rate, savings, waste, fraud, technology-savings, and
balanced-budget claims blocked.

`wave2_human_services_lane_depth_packets.v1.draft.json` executes the second
scaled lane-depth explainability scaffold for Income security and family,
Education and workforce, and Veterans while keeping final depth completion,
solver, target-cost, rate, savings, waste, fraud, technology-savings, and
balanced-budget claims blocked.

`wave3_public_goods_lane_depth_packets.v1.draft.json` executes the third scaled
lane-depth explainability scaffold for Disaster and resilience, Justice/courts
public safety, and Science/energy/environment while keeping final depth
completion, solver, target-cost, rate, savings, waste, fraud,
technology-savings, and balanced-budget claims blocked.

`wave4_component_and_pilot_lane_depth_packets.v1.draft.json` executes the
fourth scaled lane-depth explainability scaffold for Agriculture,
International affairs, and Transportation/infrastructure while keeping
component-heavy lanes separate, preserving trust-fund boundaries, and keeping
final depth completion, solver, target-cost, rate, savings, waste, fraud,
technology-savings, and balanced-budget claims blocked.

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

`wave5_fiscal_control_overlay_depth_packets.v1.draft.json` publishes the Wave 5
fiscal-control overlay depth scaffolds for revenue-solvency, payment integrity,
and net interest. Revenue-solvency and payment integrity remain non-additive
overlays. Net interest remains endogenous and cannot be cut directly. The packet
opens no rate, tax, solver, target-cost, savings, waste/fraud, department-cut,
technology-savings, or balanced-budget claim.

`wave_lane_depth_scaffold_rollup.v1.draft.json` audits Waves 1-5 after all 15
analytical lanes have scaffolded public explainability packets. It confirms
that all 15 lanes are scaffolded, zero lanes are complete, zero lanes are
solver-ready, zero lanes are rate-ready, and no solver, rate, target-cost,
savings, waste/fraud, department-cut, technology-savings, or balanced-budget
claim is open.

`post_rollup_readiness_work_queue.v1.draft.json` orders the next work after the
scaffold rollup: source custody, current-law paths, fund reconciliation, outcome
floors, policy-specific scores, receipt-base modeling, payment-integrity
lineage, net-interest feedback, and finally a deterministic solver dry run. It
publishes no values, rates, solver outputs, savings, waste/fraud findings,
department-cut instructions, technology-savings claims, or balanced-budget
claim.

`current_law_source_custody_batch_plan.v1.draft.json` breaks the first
post-rollup work item into future source-custody capture batches for the federal
baseline, trust funds, health components, and net-interest/debt paths. It
captures no sources, populates no values, contacts no agency or person, and
keeps all solver, rate, savings, waste/fraud, department-cut,
technology-savings, and balanced-budget claims blocked.

`current_law_source_custody_packet_template.v1.draft.json` defines the exact
future custody packet shape and readiness checks required before current-law
annual values may be populated. It captures no source, publishes no values, and
keeps all solver, rate, savings, waste/fraud, department-cut,
technology-savings, and balanced-budget claims blocked.

`current_law_fy2025_17_row_ledger_custody.v1.draft.json` is the first
data-bearing current-law custody packet. It records existing local OMB Historical
Table custody and publishes the FY2025 baseline-year 17-row ledger, including
the two negative fiscal reconciliation rows, while keeping ten-year paths,
solver inputs, target costs, rates, and public fiscal claims blocked.

`current_law_baseline_annual_path_partial.v1.draft.json` extends the data-bearing
current-law baseline with official OMB Public Budget Database outlay totals for
FY2025-FY2031. It leaves FY2032-FY2035, forward receipts, and forward deficits
null, prohibits interpolation, and keeps complete-horizon, solver, target-cost,
rate, and public fiscal claims blocked.

`current_law_baseline_receipts_deficit_path_partial.v1.draft.json` fills the next
official current-law baseline data gap by adding OMB Historical Table 2.1 total
receipts and explicit deficit gaps for FY2025-FY2031. It leaves FY2032-FY2035
null, does not split funds or receipt bases, and keeps solver, target-cost, rate,
and public fiscal claims blocked.

`current_law_fy2025_fund_group_path.v1.draft.json` publishes source-custodied
FY2025 OMB fund-group actuals for total, federal funds, trust funds, and
interfund transactions. It does not relabel federal funds as the general fund,
does not split named trust funds, and keeps forward fund paths and solver/rate
claims blocked.

`current_law_fy2025_dedicated_receipt_anchors.v1.draft.json` publishes
source-custodied FY2025 dedicated-receipt anchors for OASI, DI, Medicare HI,
transportation trust-fund excise receipts, and airport-and-airway excise
receipts. It is a receipt-anchor packet only, not a complete named trust-fund
path, solver input, target-cost selection, rate calculation, savings estimate,
or balanced-budget claim.

`current_law_fy2025_named_trust_fund_outlay_anchors.v1.draft.json` publishes
source-custodied FY2025 OASI, DI, OASDI-sum, and Medicare HI outlay anchors from
the OMB Public Budget Database. It is an outlay-anchor packet only, not a
complete trust-fund path, transportation trust-fund path, solver input,
target-cost selection, rate calculation, savings estimate, or balanced-budget
claim.

`current_law_named_fund_balance_transfer_gap.v1.draft.json` documents the
remaining named-fund balance and explicit-transfer source gap after the FY2025
receipt and outlay anchors. It keeps all fund-balance paths, transfer schedules,
transportation reconciliation, solver inputs, target-costs, rates, savings, and
balanced-budget claims blocked.

`assigned_receipt_base_source_gap.v1.draft.json` documents that local IRS HT23
custody supports rate/bracket source context only, not AGI, taxable-income,
taxable-payroll, or transportation fee base amounts. It quarantines the TY2022
illustrative statutory-rate file from solvers, rate outputs, public rate cards,
tax proposals, and balanced-budget claims.

`distribution_incidence_source_gap.v1.draft.json` documents the remaining source
and model gaps for distribution, incidence, administration burden, benefit and
service valuation, interaction scoring, and macro feedback. It keeps all
distributional values, solver inputs, rates, public rate cards, tax proposals,
and balanced-budget claims blocked.

`administration_compliance_burden_source_gap.v1.draft.json` documents the
remaining source and model gaps for agency administration cost, taxpayer burden,
employer withholding/reporting burden, avoidance/compliance response,
implementation transition cost, and technology shift/productivity evidence. It
keeps all administration, compliance, transition, technology-productivity, rate,
solver, public-rate-card, tax-proposal, savings, and balanced-budget values
blocked.

`rate_publication_readiness_rollup.v1.draft.json` summarizes the current hard
rate-publication blockers across assigned receipt bases, distribution/incidence,
administration/compliance burden, interaction/macro feedback, and public review.
It keeps statutory rates, effective rates, assigned-base rates, public rate
cards, tax proposals, solver inputs, savings, and balanced-budget values blocked.

`receipt_base_local_source_inventory.v1.draft.json` inventories already-local
official source custody relevant to assigned receipt-base work. It records that
HT23 supports rate/bracket context only, derived denominator context cannot
replace raw source custody, and all assigned base amounts, yields, rates,
solver inputs, public rate cards, tax proposals, and balanced-budget outputs
remain blocked.

`receipt_base_source_work_queue.v1.draft.json` converts the local source
inventory into explicit source-capture and extraction tasks for assigned receipt
bases. It keeps every work item not started, not ready, and external-contact
blocked while all base amounts, yields, rates, solver inputs, public rate cards,
tax proposals, savings, and balanced-budget outputs remain null.

`omb_receipt_category_context.fy2025.v1.draft.json` extracts FY2025 social
insurance, retirement, and excise receipt-category context from the already-local
OMB Historical Table 2.4 workbook. It treats receipt categories as fiscal
context only, not legal or economic assigned receipt bases, and keeps rates,
solver inputs, public rate cards, tax proposals, savings, and balanced-budget
outputs blocked.

`receipt_base_work_item_completion.v1.draft.json` marks the OMB receipt-category
reconciliation work item as context-complete while preserving the boundary that
no assigned receipt base, rate, solver input, public rate card, tax proposal,
savings, or balanced-budget output is ready.

`transportation_receipt_base_work_item_progress.v1.draft.json` records FY2025
transportation and airport-and-airway excise receipt-yield context from existing
local OMB custody. It is not a legal or economic assigned receipt base and keeps
rates, solver inputs, public rate cards, tax proposals, savings, and
balanced-budget outputs blocked.

`receipt_base_official_source_capture.v1.draft.json` captures official public
source files for IRS TY2023 individual-income context, CMS Medicare HI
taxable-payroll context, and FHWA transportation highway-user receipt and
legal-rate context. It extracts guarded context values only and keeps matched
assigned bases, rates, solver inputs, public rate cards, tax proposals, savings,
and balanced-budget outputs blocked.

`receipt_base_reconciliation_gap.v1.draft.json` reconciles captured IRS, CMS,
FHWA, and blocked SSA source context against matched assigned-base requirements.
It explains why each work item remains not ready and keeps rates, solver inputs,
public rate cards, tax proposals, savings, and balanced-budget outputs blocked.

`medicare_hi_receipt_base_reconciliation.v1.draft.json` reconciles captured
FY2025 CMS Medicare HI taxable-payroll and payroll-tax-yield context against the
OMB Hospital Insurance receipt anchor. It publishes diagnostic reconciliation
context only, not a statutory rate, effective rate, solver input, public rate
card, tax proposal, savings estimate, or balanced-budget claim.

`medicare_hi_perimeter_bridge_requirements.v1.draft.json` defines the evidence
and modeling requirements needed before Medicare HI payroll context can become a
matched assigned receipt base. It keeps the diagnostic ratio, assigned base,
rates, solver inputs, public rate cards, tax proposals, savings, and
balanced-budget outputs blocked.

`medicare_hi_payroll_tax_perimeter_bridge.v1.draft.json` partially evidences the
CMS Medicare HI payroll-tax-yield perimeter while preserving the unresolved OMB
Hospital Insurance anchor bridge. It separates taxation of OASDI benefits from
payroll taxes and keeps assigned bases, rates, solver inputs, public rate cards,
tax proposals, savings, and balanced-budget outputs blocked.

`medicare_hi_benefits_tax_income_split.v1.draft.json` uses CMS Medicare Trustees
FY2025 HI operations to split payroll taxes, taxation of OASDI benefits, and
other non-payroll income categories. It preserves the unresolved OMB receipt-row
mapping and keeps assigned bases, rates, solver inputs, public rate cards, tax
proposals, savings, and balanced-budget outputs blocked.

`medicare_hi_legal_base_definition_gap.v1.draft.json` records that CMS Medicare
Trustees glossary terms clarify payroll-tax and taxable-payroll terminology but
do not select a legal receipt base. It keeps additional Medicare tax treatment,
matched base amounts, rates, solver inputs, public rate cards, tax proposals,
savings, and balanced-budget outputs blocked.

`medicare_hi_economic_base_definition_gap.v1.draft.json` records the Medicare HI
economic-base blocker: employer, employee, household, wage-incidence,
distribution, administration, avoidance, and compliance models remain missing.
It keeps economic-base completion, assigned bases, rates, solver inputs, public
rate cards, tax proposals, savings, and balanced-budget outputs blocked.

`medicare_hi_solver_yield_mapping_gap.v1.draft.json` records that Medicare HI
current-law yield cannot enter solver rows until OMB/CMS receipt perimeters,
trust-fund accounting, explicit transfers, fund balances, timing/rounding, and
solver-row contracts reconcile. It keeps solver inputs, rates, public rate
cards, tax proposals, savings, and balanced-budget outputs blocked.

`medicare_hi_behavior_reform_yield_gap.v1.draft.json` records that Medicare HI
behavior and reform yield are not modeled. It blocks using current-law CMS or
OMB receipt context as reform yield and keeps elasticity, avoidance,
compliance, administration, incidence, distribution, solver inputs, rates,
public rate cards, tax proposals, savings, and balanced-budget outputs blocked.

`medicare_hi_bridge_status_rollup.v1.draft.json` summarizes all six Medicare HI
bridge components. It records two partial-context rows and four explicit gap
rows, with zero components ready, and keeps assigned bases, rates, reform yield,
solver rows, public rate cards, tax proposals, savings, and balanced-budget
outputs blocked.

`medicare_hi_bridge_closure_work_queue.v1.draft.json` orders the Medicare HI
bridge closure work from OMB/CMS receipt-row reconciliation through final
rate/solver readiness review. It keeps all seven work items not ready and
retains null/false blocks for assigned bases, rates, reform yield, solver rows,
public rate cards, tax proposals, savings, and balanced-budget outputs.

`medicare_hi_omb_cms_receipt_row_perimeter_evidence.v1.draft.json` publishes
the evidence boundary for Medicare HI closure item 1 using existing official
source custody only. It recomputes the CMS/OMB difference and keeps OMB
included/excluded receipt types, source-row crosswalk, timing/rounding, assigned
bases, rates, reform yield, solver rows, savings, and balanced-budget outputs
blocked.

`medicare_hi_income_category_omb_mapping_gap.v1.draft.json` publishes the
Medicare HI closure item 2 gap: CMS income categories are evidenced, but mapping
them to OMB Hospital Insurance receipt rows remains incomplete. It keeps the
OMB/CMS crosswalk, excluded-category list, residual explanation, assigned bases,
rates, reform yield, solver rows, savings, and balanced-budget outputs blocked.

`medicare_hi_legal_base_closure_gap.v1.draft.json` publishes Medicare HI closure
item 3: CMS terminology identifies candidate legal-base terms, but the legal
base remains unselected pending statutory perimeter text, additional Medicare
tax treatment, fiscal-year matching, and source-custodied base amount. It keeps
assigned bases, rates, reform yield, solver rows, savings, and balanced-budget
outputs blocked.

`medicare_hi_economic_base_closure_gap.v1.draft.json` publishes Medicare HI
closure item 4: economic base, incidence, distribution, administration,
avoidance, compliance, and tax-interaction components remain missing. It blocks
using CMS taxable payroll or the unselected legal base as the economic burden
base and keeps rates, solver rows, savings, and balanced-budget outputs blocked.

`medicare_hi_trust_fund_solver_yield_closure_gap.v1.draft.json` publishes
Medicare HI closure item 5: current-law yield selection, trust-fund income
fields, explicit transfers, fund-balance path, timing bridge, rounding line,
and solver-row contract remain missing. It keeps solver rows, rates, savings,
and balanced-budget outputs blocked.

`medicare_hi_policy_behavior_reform_yield_closure_gap.v1.draft.json` publishes
Medicare HI closure item 6: policy instrument, phase-in, matched base,
elasticity, avoidance, compliance, administration, incidence, distribution, and
trust-fund reform-delta mapping remain missing. It keeps reform yield, solver
rows, rates, savings, and balanced-budget outputs blocked.

`medicare_hi_rate_solver_readiness_review_closure_gap.v1.draft.json` publishes
Medicare HI closure item 7: the final rate and solver readiness review remains
blocked because zero of six prerequisite bridge items are ready. It keeps rate
publication, solver inputs, public rate cards, tax proposals, savings, and
balanced-budget outputs blocked.

`medicare_hi_closure_series_rollup.v1.draft.json` summarizes the Medicare HI
closure series: seven blocker packets are published, but zero bridge items are
complete or ready. It keeps assigned bases, rates, reform yield, solver rows,
public rate cards, tax proposals, savings, and balanced-budget outputs blocked.

`post_medicare_hi_next_readiness_queue.v1.draft.json` orders the next readiness
work after the Medicare HI closure blocker series. It keeps source custody,
fund reconciliation, floors, policy scores, receipt bases, payment-integrity
lineage, net-interest feedback, solver dry run, rates, savings, and public
claims blocked.

`source_custody_current_law_paths_gap.v1.draft.json` publishes the rank-1
source-custody and official current-law path blocker after the Medicare HI
closure series. It keeps all required annual path values, the 2025-2035
horizon, solver inputs, rates, savings, and balanced-budget claims blocked.

`trust_fund_fund_group_reconciliation_gap.v1.draft.json` publishes the rank-2
trust-fund and fund-group reconciliation blocker. It preserves FY2025 aggregate
fund-group context while keeping named trust-fund balances, explicit transfers,
general-fund-specific paths, forward annual paths, solver inputs, rates, and
balanced-budget claims blocked.

`outcome_floor_thresholds_gap.v1.draft.json` publishes the rank-3 outcome-floor
threshold blocker. It records that lane-specific threshold values,
baseline/policy/stress floor values, and reviewed pass/fail evidence remain
missing, so lower-cost scenarios, target costs, solver inputs, savings, and
balanced-budget claims stay blocked.

`health_outcome_floor_definition_packet.v1.draft.json` is the first
lane-specific floor definition packet. It defines health/Medicare floor classes
and health-specific floor concepts while keeping threshold values, observed
values, pass/fail findings, target costs, federal effects, solver inputs,
savings, rates, technology-savings claims, and balanced-budget claims blocked.

`social_security_outcome_floor_definition_packet.v1.draft.json` extends the
floor definition packet pattern to Social Security. It defines replacement
adequacy, old-age poverty, disability/survivor protection, trust-fund
continuity, and administration/transition feasibility floors while keeping
threshold values, pass/fail findings, demographic scores, trust-fund solvency
scores, rates, solver inputs, savings, and balanced-budget claims blocked.

`defense_outcome_floor_definition_packet.v1.draft.json` extends the floor
definition packet pattern to national defense. It defines treaty commitment,
readiness, personnel-safety, strategic-reserve, and force-structure/procurement
feasibility floors while keeping thresholds, pass/fail findings, force-structure
plans, procurement schedules, federal scores, target costs, solver inputs,
savings, and balanced-budget claims blocked.

`income_security_family_outcome_floor_definition_packet.v1.draft.json` extends
the floor definition packet pattern to income security and family programs. It
defines child-poverty, material-hardship, formal-childcare-access,
work/care-transition, and benefit-package/take-up delivery-feasibility floors
while keeping thresholds, pass/fail findings, federal scores, target costs,
solver inputs, savings, and balanced-budget claims blocked.

`income_security_family_source_readiness_gap.v1.draft.json` records the required
source families for income-security/family without treating them as raw custody
or model evidence. Benefit-package models, take-up models, floor values,
federal/state/local translation, solver inputs, rates, savings, department-cut
instructions, technology-savings claims, and balanced-budget claims remain
blocked.

`income_security_family_source_capture_queue.v1.draft.json` orders the next
official source captures for income-security/family: federal program perimeter,
CBO baseline and take-up context, child poverty and income context, childcare
and family-service context, food hardship and nutrition context, and
international comparator context. Values, models, solver inputs, rates, savings,
department-cut instructions, technology-savings claims, and balanced-budget
claims remain blocked.

`income_security_family_source_capture_status_rollup.v1.draft.json` summarizes
the post-queue income-security/family source-capture status. All six source
families remain open, with raw custody, program perimeter, benefit-package
models, take-up models, floor values, federal/state/local translation, solver
inputs, rates, savings, department-cut instructions, technology-savings claims,
and balanced-budget claims blocked.

`income_security_family_source_capture_closure_work_queue.v1.draft.json`
converts the open income-security/family source-capture status into ordered
closure gates for perimeter, baseline/take-up, child poverty, childcare,
nutrition, and comparator lineage. Closure values, lineage completion, solver
inputs, rates, savings, department-cut instructions, technology-savings claims,
and balanced-budget claims remain blocked.

`income_security_family_federal_program_perimeter_bridge.fy2025.v1.draft.json`
closes the narrow FY2025 federal account-perimeter source-custody step for OMB
income-security function 600 using the already-captured Public Budget Database
outlays workbook. It reconciles $701.609B across subfunctions 601, 602, 603,
604, 605, and 609 with a zero PBD/Table 3.2 difference, but leaves state/local
translation, benefit packages, take-up, floor values, solver inputs, rates,
savings, and balanced-budget claims blocked.

`revenue_solvency_outcome_floor_definition_packet.v1.draft.json` extends the
floor definition packet pattern to the non-additive revenue-solvency overlay. It
defines distributional-progressivity, revenue-stability, administrability,
growth-sensitivity, and matched base/behavior/incidence readiness floors while
keeping receipt bases, rates, tax proposals, solver inputs, savings, and
balanced-budget claims blocked.

`net_interest_outcome_floor_definition_packet.v1.draft.json` extends the floor
definition packet pattern to net interest. It preserves the rule that net
interest is endogenous and cannot be cut directly, and it defines debt-service,
endogenous-formula, debt/maturity/rate path, primary-balance feedback, and
stress-resilience floors while keeping direct cuts, savings, solver inputs,
solver runs, and balanced-budget claims blocked.

`payment_integrity_outcome_floor_definition_packet.v1.draft.json` extends the
floor definition packet pattern to the non-additive payment-integrity overlay.
It defines due-process, timely-payment, access, false-positive, and causal
prevention/same-cohort collection lineage floors while keeping fraud findings,
waste findings, recoverable savings credits, solver inputs, savings, and
balanced-budget claims blocked.

`veterans_outcome_floor_definition_packet.v1.draft.json` extends the floor
definition packet pattern to veterans programs. It defines earned-eligibility,
health-access, claims-timeliness, housing/employment-outcome, and eligible
cohort/service-package feasibility floors while keeping cohort models, service
package models, statutory continuity findings, target costs, savings, solver
inputs, and balanced-budget claims blocked.

`transportation_infrastructure_outcome_floor_definition_packet.v1.draft.json`
extends the floor definition packet pattern to the transportation/infrastructure
lane. It defines asset-condition, fatalities, reliability, access,
climate-resilience, and asset-inventory/maintenance-gap delivery-feasibility
floors while keeping asset inventories, maintenance-gap estimates,
federal/state/local translations, simulator runs, target costs, savings, solver
inputs, and balanced-budget claims blocked.

`education_workforce_outcome_floor_definition_packet.v1.draft.json` extends the
floor definition packet pattern to the education/workforce lane. It defines
attainment, completion/persistence, access/affordability, employment/earnings
transition, equity/distribution, and federal/state/local-translation
delivery-feasibility floors while keeping program-to-outlay allocation, cohort
timing, thresholds, pass/fail findings, target costs, savings, solver inputs,
and balanced-budget claims blocked.

`disaster_resilience_outcome_floor_definition_packet.v1.draft.json` extends the
floor definition packet pattern to the disaster/resilience lane. It defines
life-safety/response, recovery-continuity, mitigation/resilience,
equity/unmet-need, reserve-adequacy, and exposure-normalized-loss-distribution
delivery-feasibility floors while keeping event-normalized outlays,
base/supplemental bridges, reserve rules, target costs, savings, solver inputs,
and balanced-budget claims blocked.

`justice_courts_public_safety_outcome_floor_definition_packet.v1.draft.json`
extends the floor definition packet pattern to the justice/courts/public-safety
lane. It defines due-process/court-access, public-safety/victimization,
court-timeliness/caseload, correctional-safety/recidivism, civil-rights/equity,
and federalism/caseload delivery-feasibility floors while keeping all-government
translation, component policy paths, target costs, savings, solver inputs, and
balanced-budget claims blocked.

`science_energy_environment_outcome_floor_definition_packet.v1.draft.json`
extends the floor definition packet pattern to the science/energy/environment
lane. It defines separate science, energy, environment, equity, scope, and
delivery-feasibility floors while preserving that the composed subtotal is not
one OMB function, one program, one performance surface, or one composite target.
Component scenarios, composite targets, target costs, savings, solver inputs,
and balanced-budget claims remain blocked.

`agriculture_outcome_floor_definition_packet.v1.draft.json` extends the floor
definition packet pattern to agriculture. It defines farm-resilience,
crop-insurance payment-integrity boundary, research/services/productivity,
conservation/environmental crossover, nutrition-handoff, and
component-account-denominator delivery-feasibility floors while keeping
component paths, account crosswalks, target costs, savings, solver inputs, and
balanced-budget claims blocked.

`international_affairs_outcome_floor_definition_packet.v1.draft.json` extends
the floor definition packet pattern to international affairs. It defines
diplomacy, ODA/development, humanitarian, security-assistance, financial-
instrument/accounting, and component-commitment-outlay delivery-feasibility
floors while keeping component policy paths, target costs, savings, solver
inputs, and balanced-budget claims blocked.

`lane_floor_readiness_rollup.v1.draft.json` summarizes post-Pulse-175 floor
packet coverage across all fifteen analytical lanes. It records that every lane
has an outcome-floor definition packet, but zero lanes have selected thresholds,
sourced floor values, floor passage, component policy paths, behavior/incidence/
transition models, solver readiness, or public-rate readiness.

`lane_floor_source_work_queue.v1.draft.json` converts that coverage status into
an official-source work queue for threshold and observed floor-value capture.
It names source families and next capture actions for all fifteen analytical
lanes while keeping thresholds, values, pass/fail findings, solver inputs,
savings, rates, technology-savings claims, and balanced-budget claims blocked.

`health_floor_source_capture_status.v1.draft.json` starts the source-capture
layer for the first prioritized lane. It recognizes existing OMB FY2025 fiscal
custody for Medicare, non-Medicare health, and the Medicare HI receipt anchor,
but keeps health floor indicator custody, thresholds, observed floor values,
pass/fail findings, policy scores, solver inputs, savings, rates, and balanced-
budget claims blocked.

`health_medicare_trustees_source_capture_status.v1.draft.json` adds local CMS
Medicare Trustees custody for CY2025 Medicare financing and enrollment context.
It closes that source-family context gap only; NHE, CBO baseline, quality/access
indicator custody, thresholds, floor values, pass/fail findings, solver inputs,
savings, rates, and balanced-budget claims remain blocked.

`health_nhe_source_custody_gap.v1.draft.json` records that CMS NHE source IDs
appear in derived health sensitivity artifacts but are not yet backed by local
raw NHE custody in this packet. It keeps NHE source capture, floor thresholds,
observed floor values, pass/fail findings, solver inputs, savings, rates, and
balanced-budget claims blocked.

`health_cbo_source_custody_gap.v1.draft.json` records that CBO source IDs appear
in derived health context artifacts but are not yet backed by local raw CBO
health-baseline custody in this packet. It keeps CBO source capture, federal
policy translation, behavior and incidence modeling, pass/fail findings, solver
inputs, savings, rates, and balanced-budget claims blocked.

`health_quality_access_indicator_source_gap.v1.draft.json` records that
health/Medicare quality, access, risk-adjusted outcome, rural-capacity, and
safety-net-capacity indicator families are needed but are not yet backed by local
raw source custody in this packet. It keeps thresholds, observed floor values,
pass/fail findings, solver inputs, savings, rates, and balanced-budget claims
blocked.

`health_source_readiness_rollup.v1.draft.json` summarizes the post-Pulse-182
health/Medicare source-custody state. It marks OMB fiscal custody and CMS
Medicare Trustees custody as context-only ready while NHE, CBO, and
quality/access source families remain custody gaps; floor passage, solver
inputs, savings, rates, and balanced-budget claims remain blocked.

`social_security_source_readiness_gap.v1.draft.json` records existing CY2025
derived OASDI denominator context while keeping SSA raw custody, annual OASDI
fund paths, 75-year solvency paths, taxable payroll bases, floor values,
solver inputs, rates, savings, and balanced-budget claims blocked.

`social_security_source_capture_queue.v1.draft.json` orders the next official
source captures for Social Security: OASDI annual fund path, 75-year solvency
path, taxable payroll base, benefit adequacy floors, old-age poverty floors, and
SSA administration/transition capacity. Values, solver inputs, rates, savings,
and balanced-budget claims remain blocked.

`defense_source_readiness_gap.v1.draft.json` records that defense source IDs
appear in context/source packets but are not yet backed by local raw custody for
force structure, readiness, procurement schedules, strategy commitments, or audit
controls. Force-structure plans, readiness floors, procurement schedules, solver
inputs, rates, savings, and balanced-budget claims remain blocked.

`defense_source_capture_queue.v1.draft.json` orders the next official source
captures for defense: force structure, readiness indicators, procurement
schedules, policy-commitment/comparator context, audit-control context, and
transition/industrial-base capacity. Values, solver inputs, rates, savings,
department-cut instructions, technology-savings claims, and balanced-budget
claims remain blocked.

`defense_source_capture_status_rollup.v1.draft.json` summarizes the post-queue
defense source-capture status. All six source families remain open, with raw
custody, context readiness, force-structure plans, readiness values, procurement
schedules, solver inputs, rates, savings, department-cut instructions,
technology-savings claims, and balanced-budget claims blocked.

`defense_source_capture_closure_work_queue.v1.draft.json` converts the open
defense source-capture status into ordered closure gates for custody lineage,
perimeter review, audit-control boundary review, and transition/industrial-base
lineage. Closure values, lineage completion, solver inputs, rates, savings,
department-cut instructions, technology-savings claims, and balanced-budget
claims remain blocked.
