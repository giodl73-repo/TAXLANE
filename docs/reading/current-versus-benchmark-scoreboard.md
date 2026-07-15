# Current Versus Benchmark Scoreboard

## Headline

TAXLANE now has a sourced current-state top line for every question in the
breadth matrix: **17 questions across 13 policy lanes, with zero open breadth
gaps**. Five questions (29.4%) have a matched benchmark; 12 (70.6%) have a
federal top line but still need a scope- and outcome-matched expected value.

This closes breadth, not the argument. The next phase is depth: explain the
largest observed differences, test plausible causes, attach outcomes, and only
then estimate any addressable opportunity.

Machine rows:
`data/derived/breadth_benchmark_matrix/breadth_benchmark_matrix.v1.draft.jsonl`.

## Comparable Top Lines

| Metric | Current | Benchmark | Reading |
|---|---:|---:|---|
| Total health spending | 17.2% GDP | 9.3% OECD average | Large observed cost gap; not a fraud or automatically recoverable-savings estimate. |
| Public pensions | ~7.3% GDP | ~8.1% peer norm | Spending is near the peer norm; financing-base and solvency questions remain separate. |
| National defense | ~3.0% GDP | 2.0–3.5% strategic band | Inside a policy band; the benchmark is strategic, not statistically expected spending. |
| Public family support | ~1.1% GDP | ~2.2% peer norm | Below the peer comparison on the matched paper basis. |
| All-government tax revenue | 26.0% GDP | 34.1% OECD average | Below the average; the average is context, not a mandatory target. |

## Federal Top Lines Without A Matched Expected Value

| Metric | Current | Why no single expected value yet |
|---|---:|---|
| Medicare plus health-care-services outlays | 27.38% of federal outlays | OECD total-health measures include different financing and government scopes. |
| Borrowed share | 25.31% of federal outlays | The appropriate path depends on the cycle, primary balance, debt stock, rates, and policy goals. |
| Gross Treasury interest | 17.34% of federal outlays | Gross and net interest, debt structure, and GDP paths must be kept separate. |
| Complete Veterans Benefits and Services | 5.38% of federal outlays | No outcome- and eligibility-matched comparison is attached; the earlier 4.98% subtotal covered only income security plus medical care. |
| Complete federal Transportation function | 2.07% of federal outlays | Federal-only spending omits major state/local capital and maintenance; 1.44% is ground transportation alone. |
| Complete federal Education, Training, Employment, and Social Services | 1.03% of federal outlays | FY2024 FSA administrative scale, historical B&B outcomes among 2015–16 bachelor completers, an early descriptive BPS cohort, and a separate official DataLab entry-year Pell-receipt by five-category three-year persistence distribution are attached. An official independent-estimates t-test screen is also attached; three of five comparisons pass a derived Bonferroni screen, but covariance-aware confirmation remains blocked. Receipt is not eligibility, and mature outcomes, causal interpretation, cost links, a FY2025 fiscal crosswalk, and a joint efficiency benchmark remain blocked. |
| Disaster relief and insurance subfunction | 0.90% of federal outlays | Event incidence, exposure, mitigation, and supplemental funding vary sharply; do not relabel the broader parent function as disaster spending. |
| Justice Administration | 1.19% of federal outlays | State/local spending and matched safety, access, timeliness, corrections, and due-process outcomes are not yet attached. |
| Science, energy, environment, and natural resources | 2.18% of federal outlays | This is a disclosed composition of three OMB functions, not one official function or one benchmarkable outcome. |
| Agriculture | 0.68% of federal outlays | The FY2024 FCIC integrity bridge retains four internal field closures and four open fields. A narrow FY2024 governance component, historical FY2020/RY2018 selection component, and FY2025 public-evidence checkpoint add bounded context. OMB routes the actual S&EMP through secure MAX; the public record still does not expose a reproducible current estimator or exclusions. This is not a FY2025 function-350 account crosswalk; recoverability, farm, acre, risk, productivity, conservation, and peer evidence remain open. |
| International Affairs | 0.64% of federal outlays | The −$14.936B international-financial entry now reconciles exactly to ten account rows, chiefly FMS customer deposits exceeding same-year trust-fund outlays; outcome-matched component benchmarks remain open. |

## Breadth Closure And Depth Queue

All 17 matrix questions now meet at least Tier 2: a sourced current value with
its scope, period, unit, and interpretation boundary. No question remains a
Tier 3 breadth gap. The 12 Tier 2 questions are not “expected-value ready.”

The prioritized depth queue is:

1. **Health cost decomposition:** quantify price, utilization, administration,
   coverage, case mix, and outcomes behind the 17.2% versus 9.3% GDP comparison.
   The first diagnostic decomposition is now available in
   `docs/reading/health-cost-decomposition.md`; service-level scoring remains open.
2. **Fiscal-path scenarios:** connect borrowing, primary balance, net interest,
   debt stock, rates, and revenue bases under explicit current-policy and
   stabilization assumptions.
   The first adjustment paths are now available in
   `docs/reading/fiscal-path-scenarios.md`; dynamic debt scoring remains open.
3. **Payment-integrity bridge:** separate reported improper payments into
   overpayments, underpayments, unknowns, documentation errors, confirmed
   fraud, recoveries, and preventable future loss by program.
4. **Benefit-and-outcome denominators:** deepen Social Security, family support,
   veterans, education, and justice with eligible population, service use,
   timeliness, accuracy, access, and outcome measures.
5. **Federalism and investment outcomes:** reconcile federal, state, and local
   scope for transportation, education, justice, disaster, agriculture, and
   environmental investment before peer ranking.
6. **Accounting bridges:** the international-financial and higher-education
   negative entries now have exact account bridges; keep gross and net interest
   separate and preserve cash-versus-credit-accounting boundaries.

Do not add the displayed federal percentages into a “share of government
covered.” Some rows use complete OMB functions, others use subfunctions or
composed orientations, and total positive functions can exceed net federal
outlays because undistributed offsetting receipts reconcile the budget. The
defensible closure statistic is question coverage (17 of 17), not a summed
outlay percentage.

## Fraud And Savings Firewall

The current government-wide integrity top line is **$161.5 billion in FY2024
reported improper plus unknown payments across covered programs**: $148.971B
classified improper plus $12.570B unknown. It is not the full federal payment
universe, not a fraud estimate, and not an automatically recoverable amount.

The draft [payment-integrity depth card](payment-integrity-depth-card.md) now
reconciles the official annual workbook into $135.184B overpayments, $7.864B
underpayments, $5.923B technically improper payments, and $12.569B unknown
payments. Confirmed-fraud and recovery tables remain parallel evidence, not
automatic subsets or savings. The VA PLTSS annual row and Q4 scorecard are now
source-reviewed and reconciled at $218.30M in FY2024 overpayments; the earlier
$2.502B probe was an extraction error, not an official-source conflict.
The same-period FY2024 PLTSS row also completely reconciles overpayment,
underpayment, technically improper, and unknown categories. That closes the
payment-type split internally; VA's FY2025 AFR corroborates the taxonomy for a
later cycle without being blended into the earlier estimate. PLTSS is now two
closed and six open methodology fields, with debt and collection lineage still
blocked.
Current VA policy now also supplies the classification and certified-return
boundary, but it does not map PLTSS-specific defects through category, bills,
disputes, collections, or recovery. This records a narrow component without
changing the two-closed/six-open aggregate.

The [Federal Crop Insurance bridge](federal-crop-insurance-payment-integrity-bridge.md)
now reconciles the FY2024 annual row's $579.36M improper-payment estimate—
$573.93M overpayments and $5.43M underpayments—to the Q4 2025 scorecard and the
RMA July 2021–June 2022 review window. The
[USDA AFR extension](federal-crop-insurance-root-cause-definition-bridge.md)
defines both FY2024 data-access root-cause categories. The
[payment-universe extension](federal-crop-insurance-payment-universe-bridge.md)
discloses premium subsidy, A&O expense, and indemnities across AIP payment tiers.
Sample period, payment type split, root-cause definition, and payment universe
are closed internally. The
[sample-design component extension](federal-crop-insurance-sample-design-component-bridge.md)
records 326 RY2022 policies, AIP-aware selection and tiering,
statistical-validity language, and independent audit review. It is a narrow
component only: compliance is not public reproducibility, and frame, allocation,
probabilities, randomization, replacement, nonresponse, weights, estimator, and
variance remain open. Sample design, estimation method, exclusion rules, and
recoverable savings basis remain open, and no amount or category is scored as fraud, waste,
identified debt, collectible recovery, prevention, or savings. The subsequent
USDA-wide Do Not Pay figures are excluded from FCIC evidence. Other Information
on printed pages 60-61 of the FCIC/RMA statements is unaudited; its apparent
$579.93M typo is excluded in favor of the annual workbook's $573.93M.

The [historical sampling-method extension](federal-crop-insurance-historical-sampling-method-bridge.md)
adds a year-bound FY2020/RY2018 benchmark: unaudited Other Information describes
simple-random policy selection, premium-subsidy, A&O-subsidy, and indemnity
coverage, and statistically valid rate and dollar estimates. It does not prove
continuity to FY2024, change the four-closed/four-open aggregate, or relax any
claim gate.

The [public methodology evidence-ceiling extension](federal-crop-insurance-public-methodology-evidence-ceiling.md)
records that OMB requires the plan's outputs but directs the S&EMP and checklist
to secure MAX. FY2025 reporting repeats the public descriptors and 3.29-percent
rate without publishing method mechanics. Zero fields close.

The [recovery-lineage boundary extension](federal-crop-insurance-recovery-lineage-boundary-bridge.md)
tracks the same sample through case dispositions and final rate reporting while
keeping ordinary compliance findings and criminal outcomes separate. No public
source links the projected estimate to sample-specific debt or cash collection,
so recoverable-savings basis remains open.

The [appeal and collectibility governance extension](federal-crop-insurance-appeal-collectibility-governance-bridge.md)
maps CARS receipt, evidence-backed dispute, correction and possible repayment,
FCIC discretion, and setoff. Those procedures prevent a Final Finding from
being treated automatically as final collectible debt or cash collected; zero
fields close.

The [public cohort-outcome evidence-ceiling extension](federal-crop-insurance-public-cohort-outcome-evidence-ceiling.md)
shows that later public reports move to FY2025/RY2023 sample progress rather
than disclosing FY2024/RY2022 final determinations, debt, or collections.
Neither the later cohort nor separately listed ordinary compliance findings can
fill that lineage gap; zero fields close.

The [cohort-disposition request-specification extension](federal-crop-insurance-cohort-disposition-request-specification.md)
provides a bounded, privacy-aware request for existing records and aggregate or
segregable fallbacks. It remains unsent and therefore changes no evidence or
field status.

The [FOIA preflight and response-intake extension](federal-crop-insurance-foia-preflight-response-intake.md)
adds an owner approval gate and blank lifecycle record. It changes no evidence
status and prevents no-records, denial, fee, or acknowledgment events from being
misread as substantive outcomes.

The matrix enforces:

Pulse 37 adds the Medicare Part D payment-type composition bridge. The exact
FY2024 categories reconcile and close that field internally, moving Part D to
two closed and six open. No benchmark, debt, recovery, fraud, waste, or savings
claim follows from the statistical overpayment estimate.

Pulse 38 reconciles the captured Part D scorecard and records a documentation-
dependency evidence ceiling. The corrected scorecard rate is 3.16%, and its
root cause concerns sponsor drug, pricing, and documentation inputs rather than
state data. No field closes and the 2/6 aggregate is unchanged.

Pulse 39 adds same-period operational treatment from the CY2022 guide and FAQ.
Sponsor-documentation dependency closes internally, moving Part D to three
closed and five open while all scoring and recoverability gates remain blocked.

Pulse 40 records a same-period Part D sample-design evidence ceiling. PDEs are
the sampled unit and CMS describes a statistically valid stratified random
sample, but the national sample size, frame, strata and allocation, inclusion
probabilities, selection implementation, replacement and nonresponse rules,
weights, and beneficiary-simulation linkage remain unpublished. Sample design
stays open, Part D stays at three closed and five open, and no benchmark or
recoverability claim is added.

Pulse 41 records the Part D estimation-process ceiling and custody blocker.
Official APR text at printed page 88 is web-verified, but HHS denied official
PDF bytes with Akamai HTTP 403. Captured findings provide same-period output and
statistical-governance evidence; the 2026 background is current-only. Estimator
formula, weights, aggregation, simulation, sample linkage, record treatment,
variance, and reconciliation remain open. No component or field closes, Part D
remains three closed and five open, and no benchmark or recoverability claim is
added.

Pulse 42 closes a narrow Part D missing-document exclusion-treatment component.
CY2022 timely problematic files remain in review, unresolved missing evidence
leaves the PDE failed, and correction is permitted before the final deadline.
The reported FY2024 category includes invalid or missing documentation. The 27
FY2020 exclusions are historical comparison only. Full exclusion rules stay
open, Part D stays three closed and five open, and no benchmark, debt, recovery,
fraud, waste, or savings claim is added.

Pulse 43 closes a narrow Part D payment-universe measurement-object and
published-denominator component. The guide identifies sampled reconciliation
PDE records, the findings define GDC and publish a $96.52 billion denominator,
and annual row 828 supplies the exact $96,521.39 million outlays value. The full
field remains open because included/excluded streams and the bridge from
combined plan-beneficiary GDC liability to federal outlays are not disclosed.
Part D stays three closed and five open, with no benchmark, debt, recovery,
fraud, waste, or savings claim added.

Pulse 44 closes a current Part D audit-closeout recovery-process component. The
Q4 2025 scorecard documents issued named-audit notices requiring deletion of
audit-determined improper PDE records and publisher-described recovery, while
separating planned DME and Tepezza notices. No amount or cohort link connects
that later process to the FY2024/CY2022 estimate. Full recoverable-amount basis
stays open, Part D stays three closed and five open, and no benchmark, debt,
collection, recovery-amount, fraud, waste, prevention, or savings claim is
added.

Pulse 45 closes a same-period Part D published uncertainty-output component.
Findings report the 95% dollar and rate bounds, and row 828 reports its
confidence label and 0.42 margin-of-error field. Because that row discloses no
units or formula, no reconciliation to the findings bounds is forced. Full
estimator mechanics and APR custody stay open, Part D stays three closed and
five open, and no benchmark, debt, recovery, fraud, waste, prevention, or
savings claim is added.

Pulse 46 closes a narrow Part D reconciliation-PDE adjustment-documentation
component. After a sampled reconciliation PDE is adjusted, Appendix A requires
both reconciliation-PDE-aligned documentation and additional linked adjustment
documentation. The cutoff and final reconciliation target are prior context,
not new closures. The guide supplies no inclusion, exclusion, denominator,
weight, estimator, or payment effect, so full payment universe remains open,
Part D stays three closed and five open, and no claim or scoring gate changes.

Pulse 47 records a zero-closure Part D sampling-and-estimation-plan access
ceiling. OMB M-21-19 directs agency S&EMPs and checklists to secure MAX, while
the prior validated Part D bridges inventory the public evidence still short of
a reproducible method. Secure MAX does not prove exemption, withholding,
nonexistence, or public unavailability, and the governmentwide rule supplies no
Part D-specific method. No request was submitted. Counts and every claim and
scoring gate remain unchanged.

Pulse 48 creates an unsent, privacy-aware existing-records request
specification for the final CY2022/FY2024 Part D S&EMP package and operative
version records. CMS filing instructions and 45 CFR Part 5 define the route and
process; the draft excludes claims, PHI, identifiers, credentials, raw inputs,
and the separate recovery track and accepts redaction and segregable release.
Owner, requester, fee, scope, and one-channel preflight remain unresolved. No
request or outbound action occurred, zero components or fields close, and all
counts and gates remain unchanged.

Pulse 49 adds the hard submission preflight and blank response-intake state
machine for that request. All preflight booleans remain false, the lifecycle is
closed-world, and later events require append-only local custody. Routing,
acknowledgment, fees, estimates, no-records, redaction, denial, and appeal do
not constitute Part D methodology evidence. No outbound or agency event exists;
zero components or fields close and all counts and gates remain unchanged.

Pulse 50 completes the bounded role review of the FY2024 PaymentAccuracy annual
extraction. Raw custody, metadata, schema, five extracted artifacts, exact
1/68/54/59 row counts, reconciliation, measurement-period, fraud-definition,
recovery-scope, and null-preservation checks are recorded. The review allows
exact source-labeled factual reporting; it does not allow evidence classes to
be joined or converted into fraud, waste, debt, recovery, prevention, or
savings. One review action closes, zero methodology components and fields
close, and every program count and gate remains unchanged.

Pulse 51 records the VA PLTSS same-cohort debt and collection lineage evidence
ceiling from six existing checksum-verified sources. It preserves the $218.30
million statistical estimate and three operational recovery rows as separate
period-and-basis records. The later AFR and current policies cannot assign
historical dispositions, and absence from this bounded inventory does not prove
nonexistence, withholding, zero debt, zero collection, or noncollectibility.
Zero components and fields close; PLTSS remains two closed and six open with
every gate false.

Pulse 52 turns the reviewed evidence into the bounded public handoff
[Payment Integrity: What The Public Evidence Shows](payment-integrity-bounded-factual-examples.md).
Its seven examples retain source, period, evidence class, wording, and caveat;
the four program cards preserve Part D 3/5, Medicaid 1/7, PLTSS 2/6, and FCIC
4/4 internal methodology counts. Zero components and fields close. Bounded
factual reporting is allowed, but every established public, performance,
fraud, waste, debt, collectibility, recovery, prevention, and savings gate
remains false.

```text
international efficiency gap != improper payments != fraud != recoverable savings
```

World comparisons can identify an efficiency question. Fraud requires evidence
of willful misrepresentation at the relevant program or transaction grain.
Recoverable savings requires a separate reviewed estimate that accounts for
recoverability, control cost, behavioral response, access, due process, and
outcome floors.

## Comparability Grades

| Grade | Meaning |
|---|---|
| A | Same concept, unit, and period on an authoritative harmonized source. |
| B | Useful comparison with a disclosed scope, year, or definition adjustment. |
| C | Directional only; do not headline without additional reconciliation. |
| Not scored | No matched benchmark is claimed. |

