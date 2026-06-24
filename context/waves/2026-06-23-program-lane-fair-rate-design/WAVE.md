# Wave: Program-Linked Lane Fair-Rate Design

## Goal

Move TAXLANE from visibility-only lane modeling to a **Tier 3 statutory
program-linked tax-rate proposal**: an explicit, defensible target rate for each
public-purpose lane, sized to fund the lane and close the borrowed-share gap,
and justified against US historical norms and international peer levels.

This is the deferred frontier. Both `public-purpose-lane-taxonomy` and
`program-linked-tax-model` listed rate recommendations under **Defer** pending
current-system and history sourcing. Most of that sourcing now exists
(OMB receipts/outlays, function/subfunction models, lane crosswalk), so the rate
work can begin under role review.

## Scope decision (owner-confirmed)

- **Fairness basis**: blend. Lead with a **solvency target** (fund the lane with
  near-zero borrowing), then justify each lane's level against **both** a US
  historical-norm era **and** international peer levels.
- **Tax scope**: **all federal receipts** re-cast into program-linked lanes
  (individual income, payroll/social-insurance, corporate, excise, customs,
  other), not income tax alone.
- **Objective**: close the FY2025 ~25% borrowed share of outlays; record the
  "legible per-lane rates deter waste/fraud" position as an **advocacy claim to
  be argued with evidence**, never as asserted fact.

## Anchoring baseline (FY2025, sourced)

| Measure | FY2025 ($M) | Source |
|---|---:|---|
| Total outlays | 7,011,105 | OMB Table 1.1 / 3.1 |
| Total receipts | 5,236,421 | OMB Table 1.1 |
| Deficit (borrowed) | 1,774,684 | OMB Table 1.1 |
| Borrowed share of outlays | 25.3% | derived model |
| Individual income tax receipts | 2,656,044 | OMB Table 2.1 |
| Income-tax coverage of outlays | 37.9% | derived model |

Per-lane FY2025 cost is reconciled from OMB Table 3.2 functions to the 16-lane
crosswalk (sums exactly to total outlays).

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Fair-rate methodology + FY2025 lane-cost baseline | done | Methodology note, sourced per-lane cost table, target-rate model schema, declared data gaps. |
| 04 | International + historical benchmark research | done | Cited benchmark note (OECD/SIPRI/NATO/World Bank/IMF/CBO); 12 comparator sources added to ledger; per-lane direction table; headline revenue-gap + health-efficiency finding. |
| 02 | Full FY2025 receipts-by-source extraction | done | Parsed OMB Table 2.1 FY2025: individual 2,656,044; payroll 1,748,294; corporate 452,089; other 274,057; excise 105,937 (reconciles to 5,236,421). Identified the ~$1.94T general-fund gap. |
| 05 | Quantified target-rate model (receipt-share) | done | Emitted `program_lane_rate_model` (17 lanes, shares sum 100%), first-cut rate note, solvency math, and per-lane dedicate/keep-general/efficiency recommendation. Statutory framing deferred. |
| 05c | Income-tax-as-budget allocation | done | Treated current income tax ($2.66T) as a fixed envelope; allocated across general-fund lanes (ex-Social Security); shows income tax covers 48.9% of them and the rest must be cut or collected, not borrowed. |
| 06 | Eight-role panel on the right rate | done | Ran all 8 `.roles` lenses; panel review with convergences (two-sided gap, low-tax country, health outlier, ring-fence payroll, protect social floor), divisions (cut/collect mix, pace), and two design corrections (receipt-side lanes; balance-rule guardrails). |
| 03 | Aggregate income base + illustrative statutory rates | done | Added IRS SOI TY2022 base (AGI 14.83T); income tax ≈14.2% of AGI; per-lane illustrative %-of-AGI rates; finding that income-tax-alone funding needs 36.6% of AGI (2.6x). |
| 05b | Quantify Health/Medicare efficiency target | planned | Cost the international-norm efficiency target for the health lanes. |
| 07 | Balance-rule guardrail spec | done | Specified the rule (accrual, per-fund, interest-senior, over-cycle reserve), six statutory lane fields, the E1-E10 evasion-to-guardrail map, and institutional triggers (independent estimator, actual-receipts sequester, override transparency). |

## Affected artifacts

- `docs/research/2026-06-23-program-lane-fair-rate-methodology.md` (new, pulse 01)
- `docs/sources/source-version-ledger.md` (pulses 02-04: OECD, GDP, IRS SOI AGI)
- `data/extracted/receipt_source/` (pulse 02)
- `data/derived/program_lane_rate_model/` (pulse 05, new family)
- `reviews/` (pulse 06)

## Success criteria

- Every proposed rate carries an `allocation_method` label and never claims
  current income-tax dollars are legally dedicated.
- Every lane rate is decomposable into solvency, historical, and international
  components, each cited.
- Deficit/borrowed share stays visible; no rate table implies the budget is
  balanced when it is not.
- Each statutory lane carries shortfall, surplus, reserve, override, borrowing,
  beneficiary-impact, and compliance-burden fields.
- The anti-waste claim is recorded as a claim with argument, not a finding.

## Non-goals

- No final statutory language or bill text.
- No individual taxpayer liability calculator.
- No claim that program-linked rates by themselves fix budget discipline.
- No partisan framing imported as fact.

## Validation

```powershell
git diff --check
# When data records change (pulses 02, 05):
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo test
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```
