# Adaptive Rate and Performance System

## Purpose

This wave turns the guarded fiscal model into an evidence-governed operating
system for adaptive public finance.

The public thesis is:

> TAXLANE can build a transparent fiscal operating model that assigns effective
> funding rates to public functions, tests whether spending is delivering
> required outcomes, detects overspending risk without making unsupported fraud
> or savings claims, and updates rates over time as technology, delivery models,
> demographics, and performance change.

This is not yet a statutory-rate, tax-proposal, savings, waste, fraud,
department-cut, or balanced-budget claim. It is the next implementation plan.

## Starting point

The prior wave established the critical boundaries:

- the 17-row FY2025 budget ledger reconciles to $7,011.105B;
- the 15 analytical lanes are distinct from the 17 budget rows;
- revenue-solvency and payment-integrity remain non-additive overlays;
- net interest is endogenous and cannot be cut directly;
- every lower-cost scenario must pass access, quality, equity,
  adequacy/resilience, and delivery-feasibility floors;
- statutory rates remain blocked until matched receipt bases, behavior,
  incidence, distribution, and administration are modeled;
- Pulse 80 blocks balanced rates until target paths, assigned bases, solver
  outputs, endogenous interest, and the unrounded deficit gap reconcile;
- Pulse 81 blocks final closure until distributional analysis, behavioral
  sensitivity, macro feedback, interaction scoring, reserve/emergency stress
  tests, eight-role review, public-language review, and public rate cards exist.

## Current frontier

This wave is now past the initial contract phase. Pulses 82 through 193 have
turned the adaptive-rate thesis into a guarded implementation surface:

- rate, risk, modernization, public-card, pilot-selection, simulator, and public
  thesis contracts exist;
- transportation was selected for scaffold work and has partial FY2025-FY2031
  federal outlay context, trust-fund accounting boundaries, floor contracts,
  modernization contracts, stress contracts, and solver-readiness gates;
- current-law source-custody work now separates FY2025 anchor custody from
  incomplete forward horizons, fund paths, receipt paths, and solver inputs;
- lane-depth packets cover the 15 analytical lanes while preserving the
  distinction from the 17-row budget ledger and the non-additive treatment of
  revenue-solvency and payment-integrity overlays;
- receipt-base, distribution, administration, incidence, interaction, macro,
  reserve, net-interest, and balanced-rate blockers are represented as explicit
  machine-readable nulls instead of placeholders;
- income-security and family source-capture work has reached an ordered closure
  work queue, with federal program perimeter, CBO baseline/take-up, child
  poverty/income, childcare/family-service, food/nutrition, and international
  comparator lineage still open;
- Pulse 194 closes the narrow FY2025 federal account-perimeter source-custody
  step for OMB income-security function 600, while state/local translation,
  CBO/take-up, benefit package, floor, solver, rate, savings, and
  balanced-budget gates remain blocked;
- Pulse 195 records that automated CBO SNAP selected-program baseline capture is
  blocked by challenge HTML and that CBO open data does not expose the selected
  SNAP PDF as a CSV, so CBO baseline/take-up values remain unpopulated;
- Pulse 196 reuses existing OECD IDD raw custody to close international child
  relative-poverty context for the income-security/family lane, while the Census
  domestic child poverty and income-unit gate remains open;
- Pulse 197 reuses existing OECD SOCX raw custody to close public
  family-benefit total, cash, and in-kind service comparator context, while tax
  credits, childcare participation, ESSPROS/ILO, missing-country review, and
  child-outcome linkage remain open.

The public showable state is therefore not a rate proposal. It is a disciplined
operating model that can show:

- what evidence is already source-custodied;
- which artifacts are context-only, partial, or not ready;
- why values remain null;
- which gates must close before target costs, solver inputs, rates, savings, or
  balanced-budget claims can be published.

The next useful closure work should remain narrow source lineage, not a broad
public claim. The active frontier is the remaining Pulse 193 closure queue:
manual CBO SNAP baseline capture, broader CBO/take-up, Census domestic child
poverty/income, childcare and family-service, food hardship/nutrition, and the
remaining international comparator lineage. Each should close one named
source-custody gate at a time without converting it into savings, rates, or a
benefit package model.

## Design principles

1. Effective rates are outputs, not slogans.
   A rate is publishable only after its assigned base, elasticity,
   avoidance/compliance, incidence, distribution, interactions, and current-law
   and reform yields are modeled.

2. Fairness is tested through burden and floor analysis.
   A rate is not fair merely because it balances arithmetic. Fairness requires
   distribution by income, taxpayer/employer/agency burden, legal perimeter,
   economic incidence, and beneficiary/service-floor protection.

3. Overspending detection is a risk screen, not a finding.
   A cost, growth, benchmark, procurement, administrative, or improper-payment
   signal may create a review-needed classification. It does not prove waste,
   fraud, recoverability, causal savings, or a budget score.

4. Technology is a transition path, not an automatic cut.
   Modernization may lower future funding requirements only after the model
   names implementation cost, training, cybersecurity, privacy, delivery risk,
   fallback operations, measured productivity, and service-floor results.

5. Departments earn rate updates through evidence.
   Annual rate changes should be based on current-law baselines, outcome
   floors, audited cost movement, scored policy changes, delivery-performance
   data, and technology-transition status.

6. The model must say "blocked" loudly.
   A blocked lane is a valid outcome. Missing values stay `null`; blocked gates
   stay `false`; public language names the blocker instead of filling it with a
   guessed number.

## System architecture

### 1. Adaptive rate engine

The rate engine computes three separate outputs only when gates pass:

- all-receipt funding share;
- residual general-fund requirement share;
- effective rate on an assigned base.

For every assigned base, the engine requires:

- matched year;
- legal perimeter;
- economic perimeter;
- baseline amount;
- elasticity;
- avoidance and compliance assumptions;
- employer/taxpayer/agency burden;
- distribution by income;
- interaction with other taxes;
- current-law yield;
- reform yield.

Rates do not need to sum to 100%. Their resulting revenues must reconcile to
funded requirements, with a separate rounding line and explicit deficit gap.

### 2. Overspending-risk signal system

The risk system classifies review-needed signals without converting them into
claims:

| Signal family | Example trigger | Allowed output |
|---|---|---|
| Cost growth | Growth above historical or peer range | Descriptive anomaly |
| Unit cost | Unit cost rises without matched service volume | Efficiency pressure |
| Outcome mismatch | More spending with flat/worse outcomes | Review-needed pressure |
| Administrative load | High manual processing or overhead share | Operations review candidate |
| Procurement | Delay, overrun, or cancellation pattern | Procurement-control signal |
| Payment integrity | Improper-payment or methodology gap | Control weakness or blocked gap |
| Technology gap | Manual workflow where validated automation exists | Technology-transition candidate |

Allowed classifications:

- `descriptive_anomaly`;
- `efficiency_pressure`;
- `control_weakness`;
- `recoverability_candidate`;
- `causal_savings_candidate`;
- `blocked_no_claim`.

Disallowed shortcuts:

- no fraud inference from international comparisons;
- no savings credit from improper-payment estimates alone;
- no recoverable-savings claim without same-cohort collection or causal
  prevention lineage;
- no department cut from a technology idea before transition costs and floors
  are modeled.

### 3. Technology transition operating model

Each department or lane modernization scenario must include:

- current delivery process;
- manual workload and automatable workload;
- legal/privacy/cybersecurity constraints;
- implementation cost;
- training and change-management cost;
- procurement and vendor lock-in risk;
- fallback/resilience plan;
- service-quality and access risk;
- annual phase-in;
- measured productivity target;
- outcome-floor monitoring;
- stress case for delay, failure, higher cost, or degraded access.

Rate treatment:

- year 0: investment and transition cost may increase outlays;
- years 1-3: productivity remains provisional unless measured;
- years 3-10: lower funding requirement may be recognized only if floors pass;
- stress: adverse realization preserves remediation cost and blocks unsupported
  savings.

### 4. Outcome floors and service guarantees

No lower-cost or technology-improvement scenario is solver-eligible unless
applicable floors pass.

Required floor families:

- access;
- quality;
- equity;
- adequacy/resilience;
- delivery feasibility;
- lane-specific statutory or service floors.

Examples:

- Health: access, quality, equity, adequacy, provider/payment adequacy.
- Education/workforce: access, attainment, completion, employment, equity.
- Veterans: statutory continuity, access, health, housing, claims timeliness.
- Defense: readiness, force structure, procurement schedule.
- Social Security: adequacy, old-age poverty, trust-fund path.
- Transportation: safety, asset condition, maintenance backlog.
- Justice: rights, safety, caseload, due process.

### 5. Annual update protocol

The annual update cycle is:

1. Freeze current-law baseline.
2. Update official cost, receipt, debt, and fund data.
3. Update outcome and performance indicators.
4. Run overspending-risk screens.
5. Run technology-transition scenario updates.
6. Run distributional analysis.
7. Run behavioral sensitivity.
8. Run macro feedback.
9. Run interaction scoring.
10. Recompute net interest endogenously.
11. Reconcile all funds, reserves, offsets, and emergencies.
12. Calculate proposed rates only where gates pass.
13. Publish public rate cards only after role review.
14. Name blocked lanes and missing evidence.

## Public rate-card target shape

Each public rate card should show:

- lane or budget row;
- current-law cost;
- current-law receipts and fund treatment;
- target cost if valid;
- assigned base;
- effective rate if valid;
- all-receipt funding share;
- residual general-fund requirement share;
- who bears the burden;
- distribution by income;
- why the rate changed;
- outcome floors;
- technology-transition status;
- overspending-risk classification if any;
- evidence grade;
- blockers;
- public-claim status.

## Implementation pulses

### Pulse 82 — Adaptive rate system contract

Create the machine-readable contract for the annual update protocol, rate
lifecycle, gate sequence, and public-claim boundaries.

Acceptance:

- distinguishes rate calculation from rate publication;
- requires assigned-base fields;
- preserves all-receipt vs residual-general-fund denominators;
- keeps rate outputs null until gates pass.

### Pulse 83 — Overspending-risk taxonomy

Create the risk-signal taxonomy and schema.

Acceptance:

- includes descriptive anomaly, efficiency pressure, control weakness,
  recoverability candidate, causal savings candidate, and blocked/no-claim;
- prohibits fraud and savings inference from benchmark or improper-payment
  signals alone;
- defines required evidence before moving between classes.

### Pulse 84 — Technology transition operating model

Create the modernization scenario contract.

Acceptance:

- records implementation, training, cybersecurity, privacy, fallback, and
  service-risk costs;
- defines baseline, transition, measured-productivity, and stress phases;
- blocks lower target cost unless floors pass.

### Pulse 85 — Public rate-card v2 contract

Create a public-card schema that can show valid rates and blocked rates.

Acceptance:

- includes current cost, target cost, assigned base, rate, burden, distribution,
  floors, technology status, risk signals, evidence grade, and blockers;
- labels "not calculated" and "blocked" as first-class outcomes;
- avoids statutory-rate language unless publication gates pass.

### Pulse 86 — Pilot-lane selection gate

Choose the first pilot lane only after role review.

Recommended initial candidates:

- transportation asset maintenance and safety;
- disaster administration and mitigation reserve operations;
- claims-processing modernization.

Avoid as first pilots:

- Social Security;
- Medicare;
- broad health;
- veterans statutory commitments;
- any lane requiring immediate normative distribution choices.

Acceptance:

- names pilot-selection criteria;
- blocks the final pilot choice if role review finds normative or source
  conflicts.

### Pulse 87 — Deterministic annual update simulator

Build a narrow simulator for one pilot lane.

Acceptance:

- baseline, modernization, and stress paths;
- no optimization;
- net interest and fund effects treated according to the solver contract;
- floors can block lower-rate recognition.

### Pulse 88 — Role-reviewed public thesis packet

Create a public explanation that says what the system can and cannot prove.

Acceptance:

- public language says "overspending risk" rather than "waste" unless evidence
  supports a stronger claim;
- explains technology-transition timing;
- explains why some rates are blocked;
- survives eight-role review.

## Current blockers

- No public effective rates are calculated by this plan.
- No statutory rates are proposed.
- No balanced-budget claim is made.
- No department is identified as wasteful.
- No technology transition is credited as savings.
- No rate reduction is solver-eligible without floors and score provenance.
- No income-security/family benefit package is modeled yet.
- Only the narrow FY2025 federal account-perimeter source-custody step is ready
  for income-security/family; all other income-security/family source-capture
  gates remain open.
- No transportation simulator run is publishable until the remaining baseline,
  floor, modernization, stress, fund, solver, and review gates pass.

## Validation posture

Every pulse in this phase should preserve:

- `cargo fmt --all -- --check`;
- `cargo test --workspace --no-fail-fast`;
- `cargo run -p taxlane-tools -- income-tax-outlay validate`;
- `cargo run -p taxlane-tools -- income-tax-outlay manifest`;
- `cargo run -p taxlane-tools -- income-tax-outlay manifest --check`;
- `git diff --check`.

For planning-only pulses, the Rust validator may be limited to file presence,
required phrases, and claim-booleans/blocked-gates if a machine record is added.
