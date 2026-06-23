# Pulse 01: Work-Items JSONL

## Goal

Add a machine-readable accountability work-item artifact.

## Work

- Add a serializable `AccountabilityWorkItem` view in `taxlane-core`.
- Generate `data/derived/accountability_evidence/accountability-work-items.jsonl`.
- Validate work-item JSONL staleness during `income-tax-outlay validate`.
- Add review and VTRACE closeout for WP-TAX-013.

## Result

Done. The generated work items expose readiness, next action, demand question,
public-use blocker, and explicit public-claim allowance for future UI/API
handoff.
