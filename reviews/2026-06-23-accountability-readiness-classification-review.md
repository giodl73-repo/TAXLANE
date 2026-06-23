# Accountability Readiness Classification Review

## Scope

This review covers `taxlane-core` public-claim readiness classification for
accountability evidence records.

## Decision

Accept the typed readiness states:

| State | Meaning |
|---|---|
| `EvidenceOnly` | The record can support internal evidence review but is not ready for public claims. |
| `NeedsRoleReview` | The record has source or accountability review and needs role review before public use. |
| `PublicClaimEligible` | The record has role review plus official finding or adjudicated status. |

## Findings

| Role | Result |
|---|---|
| Reform Skeptic | Pass: public eligibility requires official/adjudicated status and role review. |
| Source Custodian | Pass: source-reviewed records are clearly not public-claim eligible yet. |
| Maintainer | Pass: readiness logic lives in `taxlane-core` and is covered by unit tests. |
| Taxpayer Advocate | Pass: future UI/reporting code can avoid flattening all records into public accusations. |

## Guardrails

- `PublicClaimEligible` is necessary but not sufficient for publication; exact wording still needs role review.
- `EvidenceOnly` and `NeedsRoleReview` records must not be presented as fraud, waste, abuse, or performance findings.
