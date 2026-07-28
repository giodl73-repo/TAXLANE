# Receipt-base/rate-bridge readiness rollup schema

`receipt_base_rate_bridge_readiness_rollup.v1.draft.json` consolidates
available receipt-base and rate-bridge context while preserving the no-rate
boundary.

Required checks:

- The record links assigned-base gap, official source capture, reconciliation
  gap, OMB/CBO revenue overlap, IRS individual and corporate context, Social
  Security OASDI boundary, Medicare HI perimeter evidence, transportation
  receipt progress, and rate-publication readiness artifacts.
- Exactly six context rows are present.
- Context rows may expose source context only and must keep assigned-base,
  rate-publication, and solver readiness false.
- Readiness counts for assigned bases, legal/economic bases, incidence,
  administration, solver yields, solver readiness, and public rates remain zero.
- Matched receipt bases, solver inputs, statutory/effective rates, public rate
  cards, savings, technology-savings, and balanced-budget claims remain blocked.
