# Program-Linked Tax Model

## Decision supported

This note defines the first TAXLANE reform-design boundary. It shows how
TAXLANE can advocate public-purpose visibility without falsely claiming that
today's ordinary income-tax dollars are already legally assigned to programs.

## Research question

How can TAXLANE model visible tax lanes by public purpose while preserving
federal budget-accounting accuracy, fungibility warnings, beneficiary continuity,
and compliance-burden review?

## Sources

- `SRC-OMB-AP-6-CONCEPTS-FY2027`
- `SRC-OMB-AP-13-FUNDS-FY2027`
- `SRC-GAO-BUDGET-GLOSSARY-2005`
- `SRC-GAO-TRUST-DEDICATED-FUNDS-2020`
- `SRC-GAO-USER-FEE-DESIGN-2008`
- `SRC-CBO-BUDGET-TERMS-2021`
- `SRC-CBO-TRUST-FUNDS-2020`
- `docs/research/2026-06-21-budget-accounting-explainer.md`
- `docs/research/2026-06-21-public-purpose-lane-taxonomy.md`
- `reviews/2026-06-21-plan-role-review.md`

## Core distinction

TAXLANE needs three separate model tiers:

| Tier | What changes legally? | What it can say | What it must not imply |
|---|---|---|---|
| Visibility receipt | Nothing. It is an explanatory model. | "Here is a labeled allocation of current taxes, outlays, and borrowing." | That current income-tax dollars are legally dedicated to each lane. |
| Budget presentation lane | Reporting and budget display change, but tax law may not. | "This public-purpose category is tracked with consistent source and fiscal labels." | That the category self-funds or bypasses appropriations. |
| Statutory program-linked tax lane | Congress creates a tax, fund, formula, or dedication rule. | "This receipt source is legally linked to this public purpose under these rules." | That earmarking removes fungibility, automatically guarantees benefits, or eliminates deficit risk. |

The first public product should start with the visibility receipt. Statutory
lanes require role review and a much higher evidence bar.

## Findings

### TAXLANE-MODEL-01: Visibility is not dedication

- **Sources**:
  - `SRC-OMB-AP-13-FUNDS-FY2027`
  - `SRC-GAO-BUDGET-GLOSSARY-2005`
  - `docs/research/2026-06-21-budget-accounting-explainer.md`
- **Observation**: OMB and GAO distinguish general fund, special fund, trust
  fund, offsetting, and earmarked collections. A model can visually allocate
  tax payments across outlay categories, but legal dedication requires a legal
  rule.
- **Implication**: TAXLANE receipts must put `allocation_method` on every lane.
  The default method for ordinary income tax is `proportional_outlay_model` or
  `deficit_inclusive_model`, not `legal_dedication`.
- **Confidence**: High.

### TAXLANE-MODEL-02: Earmarked funds improve traceability but do not end
appropriation and override questions

- **Sources**:
  - `SRC-OMB-AP-13-FUNDS-FY2027`
  - `SRC-GAO-TRUST-DEDICATED-FUNDS-2020`
  - `SRC-CBO-TRUST-FUNDS-2020`
- **Observation**: Federal trust and dedicated funds are accounting mechanisms
  that link receipts and expenditures, but Congress can define availability,
  change program rules, and still face cash-flow gaps. CBO and GAO trust-fund
  material shows that dedicated funds can run deficits or declining balances.
- **Implication**: Every statutory lane needs fields for appropriation rule,
  shortfall rule, surplus/reserve rule, borrowing rule, and legislative override
  rule.
- **Confidence**: High.

### TAXLANE-MODEL-03: User fees and taxes answer different public-purpose
questions

- **Sources**:
  - `SRC-GAO-USER-FEE-DESIGN-2008`
  - `SRC-GAO-BUDGET-GLOSSARY-2005`
  - `docs/research/2026-06-21-budget-accounting-explainer.md`
- **Observation**: GAO's user-fee design guide treats user fees as charges that
  can make identifiable beneficiaries pay for above-general benefits. Taxes are
  compulsory public charges based on sovereign power and may fund broad public
  obligations.
- **Implication**: A public-purpose lane should not turn every service into a
  user fee. Lanes for courts, defense, public health, or general government may
  remain broad tax-funded public goods even when the receipt makes them visible.
- **Confidence**: High.

### TAXLANE-MODEL-04: Compliance burden is part of the policy cost

- **Sources**:
  - `reviews/2026-06-21-plan-role-review.md`
  - `SRC-IRS-F1040`
  - `SRC-IRS-P17`
- **Observation**: Splitting one income-tax label into several statutory taxes
  could add return lines, employer withholding rules, agency accounting, public
  reporting, and audit burden. Some visibility goals can be met with a receipt
  model instead of new taxpayer-facing calculations.
- **Implication**: TAXLANE should prefer visibility-only models for public
  education until a statutory lane proves that its legibility gain exceeds the
  taxpayer, employer, preparer, and administrator burden.
- **Confidence**: Medium-high. Burden estimates need future quantified source
  work.

### TAXLANE-MODEL-05: Deficit-inclusive lanes are more honest than
tax-only lanes

- **Sources**:
  - `SRC-OMB-HIST-1-1-FY2027`
  - `SRC-OMB-HIST-3-1-FY2027`
  - `docs/research/2026-06-21-current-system-explainer.md`
- **Observation**: Current outlays can exceed current receipts. A tax-only
  lane chart can make government look fully funded by current taxpayers when
  borrowing covers part of current spending.
- **Implication**: TAXLANE should include a `borrowed_share` or `deficit_gap`
  lane in visibility receipts before showing program allocations as complete.
- **Confidence**: High.

## Lane object model

Every lane record should include these fields before it appears in a public
receipt, model, or proposal:

| Field | Required for visibility receipt | Required for statutory lane | Meaning |
|---|---|---|---|
| `lane_id` | Yes | Yes | Stable machine-readable lane key. |
| `public_label` | Yes | Yes | Plain-language name shown to taxpayers. |
| `public_question` | Yes | Yes | The civic question the lane answers. |
| `omb_function_map` | Yes | Yes | OMB function/subfunction mapping. |
| `receipt_source` | Yes | Yes | Individual income tax, payroll tax, excise tax, fee, borrowing, or mixed. |
| `allocation_method` | Yes | Yes | Legal dedication, proportional outlay, deficit-inclusive, or proposed reform. |
| `spending_control` | Yes | Yes | Discretionary, mandatory, net interest, offsetting, trust-fund, mixed. |
| `legal_status` | Yes | Yes | Current-law status: general fund, dedicated, modeled, or proposed. |
| `appropriation_rule` | No | Yes | Whether spending needs annual appropriation, standing authority, or both. |
| `shortfall_rule` | No | Yes | What happens if receipts fall below target outlays. |
| `surplus_rule` | No | Yes | What happens if receipts exceed target outlays. |
| `reserve_rule` | No | Yes | Whether balances accumulate and how they may be used. |
| `borrowing_rule` | Yes | Yes | How borrowing and net interest are displayed or permitted. |
| `override_rule` | No | Yes | How Congress can alter, suspend, borrow from, or reallocate the lane. |
| `beneficiary_impact` | No | Yes | How beneficiaries are affected by shortfalls or rule changes. |
| `taxpayer_burden` | Yes | Yes | Expected taxpayer complexity. |
| `employer_burden` | Yes | Yes | Expected withholding/payroll/admin complexity. |
| `preparer_burden` | Yes | Yes | Expected tax-preparation complexity. |
| `agency_burden` | Yes | Yes | IRS, Treasury, OMB, and program-agency burden. |
| `audit_report` | No | Yes | Required public report or reconciliation artifact. |
| `source_ids` | Yes | Yes | Source ledger IDs used by the lane. |

## Candidate lane tiers

### Tier 1: Visibility receipt

Purpose: make the current system legible without changing law.

Required output:

- taxpayer income-tax liability or placeholder amount,
- proportional allocation across public-purpose lanes,
- separate payroll/social-insurance taxes when known,
- deficit gap or borrowed share,
- allocation-method labels,
- caveat that ordinary income tax is mostly general fund revenue.

This tier is the correct first public product.

### Tier 2: Budget presentation lane

Purpose: standardize public reporting around TAXLANE lane names and OMB
functions.

Possible output:

- annual lane table by OMB function/subfunction,
- current receipt sources by lane,
- mandatory/discretionary/net-interest labels,
- deficit-inclusive funding display,
- no taxpayer-level legal allocation claim.

This tier is useful before statutory reform because it tests whether the lane
taxonomy is understandable and auditable.

### Tier 3: Statutory program-linked tax lane

Purpose: create a legal link between a tax stream and a public purpose.

Minimum statutory design questions:

1. What is the tax base?
2. Is the rate fixed, formula-based, or annually reset?
3. Which receipt account, special fund, trust fund, or expenditure account gets
   the collections?
4. Is budget authority permanent, annual, limited, or subject to appropriation?
5. What happens in a shortfall?
6. What happens in a surplus?
7. Can balances be borrowed against, invested, swept, or reallocated?
8. Can Congress override the dedication through ordinary law?
9. What happens to beneficiaries if the lane is underfunded?
10. How much complexity does the lane add for taxpayers, employers, preparers,
    IRS, Treasury, OMB, and program agencies?

This tier should remain a prototype until role review approves a specific lane.

## First lane candidates

| Candidate lane | Recommended tier now | Why |
|---|---|---|
| Constitutional Government and Justice | Visibility receipt | Broad public good; statutory dedication may reduce flexibility without clear self-funding logic. |
| National Defense and Security | Visibility receipt | Large discretionary/general-fund area; useful for public visibility, risky as a rigid earmark. |
| Health and Care | Budget presentation lane | Mix of trust-fund, mandatory, discretionary, and general-fund flows needs clearer sublane accounting. |
| Retirement and Social Insurance | Budget presentation lane | Existing payroll/social-insurance dedicated structures already exist; ordinary income-tax allocation should stay separate. |
| Infrastructure, Transportation, and Places | Budget presentation lane | Existing excise/trust-fund style financing provides comparison material. |
| Debt Service and Past Commitments | Visibility receipt | Not a benefit lane; must be visible as cost of prior borrowing. |
| Borrowed Share / Deficit Gap | Visibility receipt | Required context, not a tax lane. |

## Role-review gates

| Role | Blocking question |
|---|---|
| Taxpayer Advocate | Can a non-expert distinguish actual tax liability from modeled allocation? |
| Budget Accountant | Does every lane state receipt source, fund group, spending measure, and allocation method? |
| Source Custodian | Are all lane mappings backed by source-ledger IDs and reproducible extraction rules? |
| Public Goods Steward | Is the lane a legitimate public obligation rather than a disguised private benefit or fee? |
| Program Beneficiary Reviewer | Does the lane protect continuity or honestly state shortfall harm? |
| Compliance Burden Reviewer | Does the lane add taxpayer, employer, preparer, or agency work, and is that work justified? |
| Fiscal Sustainability Reviewer | Does the lane show deficits, debt service, long-term commitments, and reserve behavior? |
| Reform Skeptic | Does the lane disclose fungibility, congressional override, and false-precision risk? |

## Adopt now

- Build the first public reading packet around Tier 1 visibility receipt, not a
  statutory lane proposal.
- Use the lane object model as the required schema for future lane work.
- Keep `Borrowed Share / Deficit Gap` visible and explicitly non-tax.
- Treat payroll/social-insurance taxes as separate receipt sources, not as
  ordinary income-tax lanes.

## Prototype next

- A sample visibility receipt with placeholder amounts and allocation labels.
- A lane crosswalk from OMB Table 3.1 functions to TAXLANE public labels.
- A statutory-lane worksheet for one low-risk candidate, likely infrastructure
  because existing dedicated transportation funding gives comparison material.

## Defer

- Actual statutory language.
- New tax rates, bracket changes, or taxpayer calculator behavior.
- Claims that program-linked lanes improve fiscal discipline without evidence.
- Any lane proposal that lacks shortfall, surplus, reserve, override, and burden
  fields.
