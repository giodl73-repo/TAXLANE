# Placeholder Receipt Placement Spec Role Review

## Scope

This review covers the static placement rules for the placeholder visibility
receipt display:

| Artifact | Role |
|---|---|
| `docs/design/placeholder-receipt-placement-spec.md` | Static placement, mobile, screenshot, and export rules. |
| `docs/design/README.md` | Design handoff index. |
| `docs/reading/placeholder-receipt-display-packet.md` | Required copy contract. |
| `reviews/2026-06-23-placeholder-display-packet-role-review.md` | Prior packet review that requested placement rules. |

## Decision

The placement spec is acceptable as a static design-review handoff.

This review does not approve a public taxpayer receipt, visual mockup,
interactive UI, calculator, taxpayer input flow, legal dedication claim, or
public release.

## Findings

| Role | Result |
|---|---|
| Taxpayer Advocate | Pass: the spec requires intro copy before the chart group and blocks tooltip-only caveats. |
| Budget Accountant | Pass: negative rows must travel with offset copy in screenshots and exports. |
| Source Custodian | Pass: the spec links back to the display packet and is included in generated manifest coverage. |
| Public Goods Steward | Pass: lane labels remain visibility categories, not statutory lane conversion. |
| Program Beneficiary | Pass: Social Security and Medicare rows require dedicated-financing copy in exports and screenshots. |
| Compliance Burden | Pass: calculator-style fields are explicitly blocked. |
| Fiscal Sustainability | Pass: lane-only display is blocked and financing context must remain adjacent or summarized. |
| Reform Skeptic | Pass: titles and alt text must preserve placeholder or modeled wording. |

## Required Guardrails

### P1: Keep lane chart exports context-complete

- **Roles**: Fiscal Sustainability Reviewer, Reform Skeptic, Taxpayer Advocate.
- **Evidence**: The placement spec blocks lane-only social cards and requires
  financing context for screenshots or exports that include the lane chart.
- **Risk**: A cropped image could imply current outlays are fully funded by
  ordinary income-tax receipts.
- **Decision**: Any future static mockup or export workflow must test for
  context-complete screenshots before public use.

### P1: Preserve caveats in narrow layouts

- **Roles**: Program Beneficiary Reviewer, Budget Accountant.
- **Evidence**: The mobile rules stack the intro, lane chart, financing context,
  and caveat copy in order.
- **Risk**: Narrow layouts could separate Medicare, Social Security, negative
  rows, or financing context from their caveats.
- **Decision**: Future mockups must treat caveat placement as a responsive
  requirement, not a desktop-only copy block.

### P2: Add mockup review before implementation

- **Roles**: Taxpayer Advocate, Compliance Burden Reviewer, Source Custodian.
- **Evidence**: This spec is prose only; no rendered layout has been reviewed.
- **Risk**: A future implementation could technically include all artifacts but
  make the warnings visually subordinate or easy to crop.
- **Decision**: Before implementing a public page, add a static mockup review
  that checks desktop, mobile, screenshot, and export states.

## Follow-Up

- A future mockup should remain static and scenario-based.
- Do not add taxpayer-specific inputs until the calculator and tax-advice
  boundary is reviewed.
