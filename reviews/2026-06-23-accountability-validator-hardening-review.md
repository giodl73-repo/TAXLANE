# Accountability Validator Hardening Review

## Scope

This review covers the Rust guardrails added to `taxlane-core` accountability
evidence validation.

## Decision

Accept the validator hardening as the next accountability safety layer.

The validator now blocks draft possible-fraud, possible-waste, and
possible-abuse signals, and blocks public accusation-style wording unless the
record has `official_finding` or `adjudicated` allegation status.

## Findings

| Role | Result |
|---|---|
| Reform Skeptic | Pass: public accusation wording cannot pass as an unreviewed evidence signal. |
| Source Custodian | Pass: possible-misconduct signals must move beyond draft review status. |
| Maintainer | Pass: behavior is covered by unit tests in `taxlane-core`. |
| Taxpayer Advocate | Pass: reviewed, non-accusatory signals remain allowed for performance-demand workflows. |

## Guardrails

- The phrase list is a safety floor, not a complete natural-language classifier.
- Role review remains required before publishing public claims.
- Official findings and adjudications still need source custody and exact source references.
