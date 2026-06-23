# Accountability Evidence Role Review

## Scope

This review covers the first accountability and fraud/waste evidence boundary:

| Artifact | Role |
|---|---|
| `docs/data/accountability-evidence-schema.md` | Schema for evidence-backed accountability records. |
| `docs/research/2026-06-23-accountability-evidence-boundary.md` | Boundary note for public performance and fraud/waste language. |
| `docs/data/dictionary.md` | Record family index and field contract. |
| `docs/vtrace/WORK_PACKAGES.md` | VTRACE work package for accountability evidence modeling. |

## Decision

The accountability evidence model is acceptable as a pre-claim schema and
research boundary.

This review does not approve public fraud allegations, named vendor or recipient
claims, taxpayer-facing accusations, automated fraud detection, or final
performance scoring.

## Findings

| Role | Result |
|---|---|
| Source Custodian | Pass: records require source IDs, observed date, comparison basis, and review status. |
| Reform Skeptic | Pass: public wording rules block accusation-style claims without official findings or adjudication. |
| Budget Accountant | Pass: lane and program evidence remains separate from modeled tax allocation. |
| Taxpayer Advocate | Pass with caution: the model supports performance demand, but public summaries must stay plain and not overstate certainty. |
| Compliance Burden | Pass: no taxpayer input or filing workflow is introduced. |
| Public Goods Steward | Pass: accountability is tied to public-purpose lanes without implying current legal dedication. |

## Required Guardrails

### P1: Do not infer fraud from variance alone

- **Roles**: Source Custodian, Reform Skeptic.
- **Evidence**: The schema separates `anomaly_class` from
  `allegation_status`.
- **Risk**: A variance or outlier could be misread as proof of fraud.
- **Decision**: Public fraud wording requires `official_finding` or
  `adjudicated` allegation status.

### P1: Keep source custody before named claims

- **Roles**: Source Custodian, Taxpayer Advocate.
- **Evidence**: Named vendor, recipient, award, or person-level concerns are
  explicitly gated by role review.
- **Risk**: TAXLANE could harm credibility or due process by publishing
  unsupported named accusations.
- **Decision**: Named concerns remain blocked until source custody and role
  review approve the exact wording.

### P2: Validate shape before scoring

- **Roles**: Budget Accountant, Reform Skeptic.
- **Evidence**: Rust validation is deferred until the field model is stable.
- **Risk**: A score could imply certainty before the evidence model is proven.
- **Decision**: The first Rust support should validate record shape and required
  fields, not assign fraud scores.

## Follow-Up

- Select one low-risk source family for example accountability records.
- Add Rust domain types only after this schema is stable enough for validation.
