# Placeholder Receipt Mockup Checklist Role Review

## Scope

This review covers the static checklist for reviewing any future placeholder
receipt mockup:

| Artifact | Role |
|---|---|
| `docs/design/placeholder-receipt-mockup-review-checklist.md` | Pre-implementation checklist for static mockups. |
| `docs/design/placeholder-receipt-placement-spec.md` | Placement rules that the checklist operationalizes. |
| `reviews/2026-06-23-placeholder-placement-spec-role-review.md` | Prior review requiring mockup review before implementation. |

## Decision

The mockup review checklist is acceptable as a pre-implementation review gate.

This review does not approve a static mockup, public taxpayer receipt,
interactive UI, calculator, taxpayer input flow, legal dedication claim, or
public release.

## Findings

| Role | Result |
|---|---|
| Taxpayer Advocate | Pass: the checklist requires visible modeled/placeholder language and blocks confusing liability-style fields. |
| Budget Accountant | Pass: offset caveats and negative-row treatment are explicit acceptance checks. |
| Source Custodian | Pass: required inputs point to canonical scenario, chart specs, display packet, and placement rules. |
| Public Goods Steward | Pass: the checklist does not turn public-purpose lanes into statutory proposals. |
| Program Beneficiary | Pass: Social Security and Medicare dedicated-financing caveats are required. |
| Compliance Burden | Pass: taxpayer data entry and filing-style controls are blocking findings. |
| Fiscal Sustainability | Pass: financing context must appear before share/export boundaries. |
| Reform Skeptic | Pass: misleading "where your taxes went" wording is blocked without modeled-allocation context. |

## Required Guardrails

### P1: Keep acceptance binary for calculator-shaped controls

- **Roles**: Compliance Burden Reviewer, Taxpayer Advocate.
- **Evidence**: The checklist treats taxpayer input fields, filing status,
  income, withholding, refund, credit, or liability labels as blocking findings.
- **Risk**: A future reviewer could accept a mockup that looks like a tax tool
  while claiming it is only a static education artifact.
- **Decision**: Any calculator-shaped control fails the checklist until a
  separate calculator and tax-advice boundary review exists.

### P1: Require context-complete exports

- **Roles**: Fiscal Sustainability Reviewer, Reform Skeptic.
- **Evidence**: Screenshot completeness and financing-context visibility are
  explicit acceptance checks.
- **Risk**: Lane-only exports can overstate income-tax coverage and hide
  borrowing.
- **Decision**: A mockup review must check at least one share/export state if
  the proposed surface supports sharing or screenshots.

### P2: Record reviewed artifact versions

- **Roles**: Source Custodian, Budget Accountant.
- **Evidence**: The checklist names required inputs but does not itself pin
  review-time hashes.
- **Risk**: A future mockup review could be separated from the exact scenario,
  chart, and copy versions it evaluated.
- **Decision**: Future mockup reviews should cite the manifest snapshot or child
  commit used for review.

## Follow-Up

- The next UI-facing step may be a static mockup, but only as a scenario-based
  artifact reviewed against this checklist.
- Public release remains blocked until a future mockup review explicitly keeps
  desktop, mobile, screenshot, and export states context-complete.
