# Accountability Performance Demand Letter Review

## Scope

Reviewed `performance-demand-letter.md`, its Rust generator, and VTRACE rows.

## Findings

- The template asks for source records, reviewed performance evidence,
  role-approved wording, and the public-claim basis.
- The template preserves the modeled-not-legal income-tax visibility boundary.
- The template explicitly says it is not an allegation, legal demand, fraud
  finding, waste finding, abuse finding, or performance scorecard.
- Rust validation checks the generated Markdown for staleness and requires
  README discoverability.

## Decision

Accepted as a public-safe request template. It gives readers an actionable ask
without relaxing claim gates or creating findings.
