# Program-Linked Lane Fair-Rate Methodology (FY2025 Baseline)

## Decision supported

This note opens TAXLANE's Tier 3 work: proposing an explicit, defensible target
tax rate for each public-purpose lane. It defines what a "fair rate" means, sets
the FY2025 cost baseline each rate must answer to, and specifies the model schema
the later quantified pulses will fill. It does not yet publish final rates; it
makes the rate question auditable.

It builds directly on:

- `docs/research/2026-06-21-public-purpose-lane-taxonomy.md` (the lanes)
- `docs/research/2026-06-21-program-linked-tax-model.md` (the lane object model and
  three model tiers)
- `data/derived/lane_crosswalk/` (lane to OMB function mapping)
- `data/derived/income_tax_outlay_model/` (decade coverage and borrowed-share history)

## Research question

What target tax rate should fund each public-purpose lane if the federal revenue
system were re-cast into program-linked lanes, such that (a) lanes are funded
with near-zero borrowing, and (b) each lane's level is defensible against US
historical norms and international peer levels?

## Reform thesis and advocacy claim

TAXLANE's standing advocacy position is taxpayer legibility. This wave extends it
to a reform proposal:

> If each public purpose carried a visible, named rate sized to its actual cost,
> citizens could debate the size of each lane directly, and the gap between what
> we spend and what we raise would stop being hidden in borrowing.

The stronger political claim the owner stated — that legible per-lane rates help
**push out waste and fraud and return to "regular" (non-borrowed) spending** — is
recorded here as an **advocacy claim to be argued with evidence**, consistent with
the product rule "do not claim a program-linked tax fixes budget discipline by
itself." The mechanism to argue (not assert) is: a named lane with a published
rate, target outlay, and annual reconciliation makes overspending and unfunded
growth visible per purpose, which lowers the cost of public oversight. Evidence
for or against this belongs in the role review and the international comparison.

## Scope (owner-confirmed)

- **Fairness basis**: blend — solvency target first, then historical and
  international justification per lane.
- **Tax scope**: all federal receipts re-cast into lanes (individual income,
  payroll/social-insurance, corporate, excise, customs, other).
- **Objective**: close the FY2025 borrowed share (~25% of outlays) toward the
  ~5% range that prevailed in solvent post-war decades.

## Sources

Used now (sourced, in repo):

- `SRC-OMB-HIST-1-1-FY2027` — total receipts, outlays, deficit spine.
- `SRC-OMB-HIST-2-1-FY2027` — receipts by source (individual income tax amount used).
- `SRC-OMB-HIST-3-1-FY2027` / `SRC-OMB-HIST-3-2-FY2027` — outlays by function/subfunction.
- `data/derived/income_tax_outlay_model/decade-summary.md` — historical coverage and borrowed share by decade.

Pending (declared data gaps below; added to the ledger in later pulses):

- Full FY2025 receipts-by-source split (corporate, social insurance, excise,
  customs, other).
- Aggregate AGI / taxable income base (IRS SOI) for statutory rate-on-base framing.
- GDP series (OMB Table 1.2 / BEA) for percent-of-GDP comparison.
- International benchmarks (OECD Revenue Statistics; peer social-contribution and
  function spending levels).

## FY2025 solvency baseline (sourced)

All amounts in millions of nominal dollars, FY2025 actual.

| Measure | FY2025 ($M) |
|---|---:|
| Total outlays | 7,011,105 |
| Total receipts | 5,236,421 |
| Deficit (borrowed) | 1,774,684 |
| Borrowed share of outlays | 25.31% |
| Individual income tax receipts | 2,656,044 |
| Income-tax coverage of outlays | 37.88% |
| Non-income-tax receipts (residual) | 2,580,377 |

The residual non-income-tax receipts figure is derived as total minus individual
income tax. Its breakdown by source is a Pulse 02 extraction, not yet sourced
per category.

## Per-lane FY2025 cost (sourced)

Reconciled from OMB Table 3.2 functions to the 16-lane crosswalk. The lane costs
plus the two negative offset lanes sum exactly to total outlays (7,011,105).

| Lane | Functions | FY2025 cost ($M) | Share of outlays | Funded today by |
|---|---|---:|---:|---|
| Social Security | 650 | 1,580,673 | 22.55% | Dedicated payroll (OASDI) |
| Medicare | 570 | 996,718 | 14.22% | Payroll (HI) + premiums + general |
| Health (non-Medicare) | 550 | 978,511 | 13.96% | General fund |
| Net interest | 900 | 970,065 | 13.84% | Financing cost (general) |
| National Defense | 050 | 916,140 | 13.07% | General fund |
| Income Security | 600 | 701,609 | 10.01% | General fund / mixed |
| Veterans | 700 | 377,163 | 5.38% | General fund |
| Transportation | 400 | 145,320 | 2.07% | Excise/trust + general |
| Justice + General Government | 750, 800 | 123,692 | 1.76% | General fund |
| Nat. Resources/Energy/Environment | 270, 300 | 110,599 | 1.58% | Mixed |
| Community/Regional Development | 450 | 82,374 | 1.17% | General fund |
| Education/Training/Employment | 500 | 72,042 | 1.03% | General fund |
| Agriculture | 350 | 47,447 | 0.68% | Mixed |
| International Affairs | 150 | 45,171 | 0.64% | General fund |
| Science/Space | 250 | 41,966 | 0.60% | General fund |
| Commerce/Housing Credit | 370 | -28,131 | -0.40% | Offset-heavy |
| Undistributed offsetting receipts | 950 | -150,254 | -2.14% | Offsets |

Reading note: "Share of outlays" is the share of each spending dollar, the
natural receipt-view denominator. It is not a legal dedication of any receipt
source. Net interest and the two negative lanes are financing/accounting context,
not program services.

## What "fair rate" means: three benchmarks

A lane's recommended rate is built from three cited components, then reconciled.

### 1. Solvency component (cost-coverage)

The first anchor is arithmetic: what rate funds the lane's target outlay with
near-zero borrowing? Two solvency facts frame every lane:

- Aggregate: to cover FY2025 outlays from receipts, total federal receipts would
  need to rise from 5,236,421 to ~7,011,105, about +33.9%, holding outlays fixed.
- Per lane: each lane's "share of outlays" above is the share of that funded
  total the lane consumes.

Solvency can be met by raising rates, lowering target outlays, or both. This note
does not assume spending is fixed; lane target outlays are an input the historical
and international benchmarks can move.

### 2. Historical component

Anchor each lane against the era when the budget was near balance. From the
derived decade summary:

| Decade | Income-tax coverage of outlays | Borrowed share | Defense share | Human-resources share |
|---|---:|---:|---:|---:|
| 1950s | 42.2% | 4.5% | 59.5% | 22.8% |
| 1960s | 42.0% | 4.6% | 46.4% | 31.9% |
| 1990s | 41.4% | 10.3% | 18.5% | 59.3% |
| 2020s | 34.3% | 31.7% | 12.4% | 69.3% |

The historical lesson is not "restore 1950s shares" (the mission mix changed
permanently from defense toward human resources); it is that solvent budgets ran
a ~5% borrowed share, and total receipts covered outlays far more fully. The
historical benchmark for each lane is its share in a chosen reference window plus
the receipts-to-outlays ratio of that window. Reference-window receipts as % GDP
is a Pulse 03 data gap.

### 3. International component

Anchor each lane against what peer countries effectively levy and spend for the
same public purpose: total tax-to-GDP, social-contribution rates for pensions and
health, and function spending as % GDP. These require OECD and peer-country
sources not yet in the ledger (Pulse 04). The international component answers
"is this lane's cost itself high or low among comparable countries?" before a
rate is called fair.

### Reconciliation rule

`recommended_rate = solvency_rate adjusted toward the historical and international
anchors`, with the direction and size of adjustment recorded per lane and cited.
Where the three disagree, the note records the disagreement rather than averaging
silently.

## Target-rate model schema (to be filled in Pulse 05)

Extends the Tier 3 lane object model in `program-linked-tax-model.md` with rate
fields. Proposed record family: `program_lane_rate_model`.

| Field | Meaning |
|---|---|
| `lane_id` | Stable lane key from the crosswalk. |
| `public_label` | Plain-language lane name. |
| `current_cost_amount` | FY2025 outlay for the lane ($M), sourced. |
| `current_cost_share` | Lane share of total outlays. |
| `current_financing` | How the lane is funded today (general, payroll, mixed, offset). |
| `target_cost_amount` | Proposed target outlay; defaults to current cost unless a benchmark moves it. |
| `target_cost_basis` | `hold-current`, `historical-norm`, or `international-norm`, with citation. |
| `assigned_receipt_base` | Tax base assigned to the lane (income, payroll, corporate, excise, customs, mixed). |
| `solvency_rate` | Rate/share that funds the target with near-zero borrowing. |
| `historical_anchor` | Reference-window comparator and value, cited. |
| `international_anchor` | Peer-country comparator and value, cited. |
| `recommended_rate` | Reconciled target rate. |
| `rate_framing` | `receipt-share` or `statutory-rate-on-base`. |
| `allocation_method` | Always `proposed_reform` at this tier; never `legal_dedication`. |
| `spending_control` | Discretionary, mandatory, net-interest, offsetting, trust-fund, mixed. |
| `shortfall_rule` / `surplus_rule` / `reserve_rule` / `borrowing_rule` / `override_rule` | Statutory-lane rules. |
| `beneficiary_impact` | Effect of shortfall or rule change on beneficiaries. |
| `taxpayer_burden` / `employer_burden` / `preparer_burden` / `agency_burden` | Compliance cost. |
| `deficit_context_note` | Required visible borrowed-share context. |
| `source_ids` | Ledger IDs backing cost, historical, and international values. |
| `status` | `draft`, `reviewed`, `superseded`. |

## Two rate framings

The owner asked for "rates to pay" each lane. Two framings will both be modeled:

1. **Receipt-share framing**: each lane shown as a share of a household's total
   federal tax bill (= lane share of a funded outlay total). Populatable now from
   the cost table; this is the legible-receipt view.
2. **Statutory-rate-on-base framing**: each lane shown as a percentage rate on a
   defined base (e.g., a "Defense lane: X% of taxable income", a "Retirement lane:
   Y% payroll"). Requires the aggregate AGI/payroll base (Pulse 03) before a real
   percentage can be stated. Until then this framing stays illustrative.

## Honesty guardrails (what this is not)

- Not a claim that current income-tax or payroll dollars are legally dedicated to
  these lanes. Allocation method at this tier is always `proposed_reform`.
- Not a balanced-budget assertion. Every view shows the borrowed share until
  receipts actually cover outlays.
- Not a fungibility denial. Earmarking improves traceability but does not stop
  Congress reallocating, and dedicated funds can still run shortfalls.
- Not advice. No individual liability is computed.
- Not a fraud finding. The anti-waste position is an argued claim, not an
  allegation against any program or person.

## Declared data gaps (drive Pulses 02-04)

1. FY2025 receipts-by-source split — needed for the all-receipts revenue base.
2. Aggregate AGI / taxable income and payroll base — needed for statutory rates.
3. GDP series — needed for percent-of-GDP historical and international comparison.
4. OECD and peer-country benchmarks — needed for the international fairness
   component; not yet in the source ledger.

No rate may be published as "fair" until at least the solvency component
(available now) and one of {historical, international} (Pulses 03-04) are sourced
for that lane.

## Role-review gates (carried from the lane object model)

| Role | Blocking question for the rate proposal |
|---|---|
| Taxpayer Advocate | Can a non-expert tell a proposed rate from current law? |
| Budget Accountant | Does each rate state cost, receipt base, and allocation method? |
| Source Custodian | Are cost, historical, and international values each ledger-sourced? |
| Public Goods Steward | Is the lane a legitimate public obligation, not a disguised fee? |
| Program Beneficiary Reviewer | Does the rate protect continuity or state shortfall harm? |
| Compliance Burden Reviewer | Is the added taxpayer/employer/agency work justified? |
| Fiscal Sustainability Reviewer | Does the rate keep deficit and debt service visible? |
| Reform Skeptic | Does the rate disclose fungibility, override risk, and false precision? |

## Adopt now

- Use the sourced per-lane FY2025 cost table as the cost denominator for all rate
  work.
- Use the three-benchmark structure (solvency, historical, international) as the
  required decomposition of every recommended rate.
- Keep payroll/social-insurance lanes (Social Security, Medicare HI) labeled as
  already-dedicated financing, distinct from general-fund lanes, even in the
  all-receipts model.

## Prototype next

- Pulse 02: extract full FY2025 receipts-by-source split.
- Pulse 04: run the cited international/historical benchmark research pass.
- Pulse 05: emit the first `program_lane_rate_model` record with receipt-share
  rates populated and statutory-rate framing flagged pending the base.

## Defer

- Final statutory rates and bill language.
- Any "fair" label on a lane lacking at least solvency plus one external benchmark.
- Claims that lane rates improve discipline or reduce fraud without argued evidence.
