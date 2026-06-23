# Accountability Performance Demand Response Rubric Core Metadata Review

## Scope

Reviewed the response rubric after moving class labels, meanings, and next
actions into core response-log class metadata.

## Findings

- Core response-log classes now expose human labels, rubric meanings, and
  next-action text.
- The rubric class list excludes `not-yet-received`, keeping the score table
  focused on actual reply classes.
- Rubric generation renders score meanings from the core metadata instead of
  hard-coded CLI Markdown rows.
- Existing no-finding and blocked-claim wording is preserved.

## Decision

Accepted as core-backed rubric metadata. It keeps human classification wording
aligned with the typed response-log contract.
