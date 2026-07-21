# Lane floor source work queue

Machine record:
`data/derived/breadth_benchmark_matrix/lane_floor_source_work_queue.v1.draft.json`

Pulse 177 turns the lane floor readiness rollup into the next practical work:
what official source families need to be captured before thresholds and observed
floor values can be populated.

This queue covers all fifteen analytical lanes. It does not choose thresholds
and it does not populate values.

Rules:

- official sources only;
- use existing captured sources when available;
- no new external download in this pulse;
- no FOIA request, records request, form, email, phone call, or agency/person contact;
- threshold selection requires stronger-model review;
- missing values remain null;
- blocked gates remain false.

Every work item keeps threshold values, baseline values, policy values, stress
values, pass/fail findings, solver readiness, target costs, federal effects,
gross savings, net savings, rates, and public rate cards blocked.

This is an official-source work queue, not threshold selection, not observed floor values, not pass/fail findings, not target-cost selection, not a federal score, not gross savings, not net savings, not solver input, not rate calculation, not a public rate card, not a technology-savings claim, and not a balanced-budget claim.
