# Placeholder Receipt Display Packet Role Review

## Scope

This review covers the static packet that pairs the placeholder visibility
receipt chart specs with required explanatory copy:

| Artifact | Role |
|---|---|
| `docs/reading/placeholder-receipt-display-packet.md` | Static display handoff and required copy. |
| `docs/charts/taxpayer-receipt-model/placeholder-lane-bars.vl.json` | Placeholder lane chart spec. |
| `docs/charts/taxpayer-receipt-model/placeholder-financing-context.vl.json` | Companion financing context chart spec. |
| `reviews/2026-06-22-placeholder-display-spec-role-review.md` | Prior review for chart-spec use. |

## Decision

The static display packet is acceptable as a design-review handoff.

This review does not approve a public taxpayer receipt, interactive display,
taxpayer input flow, calculator, legal dedication claim, program-level tracing
claim, statutory tax lane, or final visual design.

## Findings

| Role | Result |
|---|---|
| Taxpayer Advocate | Pass with caution: the required intro copy is clear, but a future public surface still needs visual placement review so caveats are not hidden below the fold. |
| Budget Accountant | Pass: negative rows are described as offsets or netting, not negative service payments. |
| Source Custodian | Pass: the packet points to the canonical receipt JSON and prior chart-spec review. |
| Public Goods Steward | Pass: lane labels remain civic visibility categories rather than statutory conversion proposals. |
| Program Beneficiary | Pass: Social Security and Medicare dedicated-financing caveats are required near the affected rows. |
| Compliance Burden | Pass: the packet blocks taxpayer input fields and personalized receipt wording. |
| Fiscal Sustainability | Pass: the lane chart is explicitly blocked from standalone use without borrowed-share and income-tax-coverage context. |
| Reform Skeptic | Pass: the packet preserves modeled-not-legal wording and blocks legal funding claims. |

## Required Guardrails

### P1: Keep the required copy attached to the chart pair

- **Roles**: Taxpayer Advocate, Reform Skeptic, Budget Accountant.
- **Evidence**: The packet relies on nearby intro, offset, dedicated-financing,
  and financing-context copy to prevent overclaiming.
- **Risk**: A future mockup could show the charts while omitting the caveats
  that make the display acceptable.
- **Decision**: Future display work must treat the required copy as part of the
  artifact, not optional prose.

### P1: Do not convert the handoff into a calculator

- **Roles**: Compliance Burden Reviewer, Taxpayer Advocate.
- **Evidence**: The packet uses a placeholder `$1,000` ordinary income-tax
  amount and blocks taxpayer inputs.
- **Risk**: Input fields would turn the artifact into a tax-advice or taxpayer
  liability surface before the repo has reviewed that boundary.
- **Decision**: Keep future work static until a separate calculator and
  tax-advice review exists.

### P2: Review visual placement before public release

- **Roles**: Taxpayer Advocate, Fiscal Sustainability Reviewer, Program
  Beneficiary Reviewer.
- **Evidence**: The packet defines copy, but not viewport placement,
  responsive behavior, or screenshot-safe layout.
- **Risk**: Public screenshots could crop out financing context or caveats even
  when the markdown packet is complete.
- **Decision**: Before public release, add a static mockup or placement spec
  that keeps the lane chart, financing context, and required caveats visible
  together.

### P2: Keep manifest coverage current

- **Roles**: Source Custodian, Budget Accountant.
- **Evidence**: The packet and review are part of the artifact chain used by
  chart and receipt handoffs.
- **Risk**: A later copy change could bypass generated manifest review.
- **Decision**: Include this review in generated manifest coverage and rerun
  validation after packet or display-rule changes.

## Follow-Up

- A future static mockup may be useful, but only if it keeps the display
  non-interactive and preserves all required caveats.
- Do not add taxpayer-specific inputs until the calculator boundary is reviewed.
