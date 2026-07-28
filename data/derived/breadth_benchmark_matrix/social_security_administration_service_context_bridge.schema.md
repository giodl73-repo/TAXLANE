# Social Security administration service context bridge schema

`social_security_administration_service_context_bridge.v1.draft.json` records
browser-visible SSA performance context for the Social Security administration
and transition-capacity work item.

Required contract:

- `record_family` is `social_security_administration_service_context_bridge`.
- The record links the Social Security source-capture rollup and Wave D
  readiness artifact.
- Source custody must record the official SSA URL, retrieval date, displayed
  last-updated date, command-line access boundary, and browser-visible status.
- `context_values` may carry service-channel and processing-time context only.
- `blocked_outputs` remain null.
- Raw-byte custody, floor values, pass/fail findings, solver inputs, rates,
  savings, technology-savings, department-cut, and balanced-budget claims remain
  false.
