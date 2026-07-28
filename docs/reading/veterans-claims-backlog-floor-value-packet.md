# Veterans Claims Backlog Floor Value Packet

Machine record:
`data/derived/breadth_benchmark_matrix/veterans_claims_backlog_floor_value_packet.v1.draft.json`

This is a Wave D floor-value packet for the Veterans lane. It converts the VA
claims backlog probe already attached in the Veterans depth card into a draft
threshold rationale and baseline value, but it does not pass or fail any policy
scenario.

Draft threshold rule: a lower-cost Veterans scenario cannot pass the
claims-timeliness floor if reviewed policy and stress evidence show the
compensation and pension rating bundle claims backlog rising above the
2026-06-30 VA probe baseline.

Selected baseline and threshold:

| Field | Value |
| --- | ---: |
| Claims backlog threshold ceiling | 68,207 claims |
| Claims backlog baseline | 68,207 claims |
| Supporting claim-based quality probe | 84.91 percent |

The backlog and quality values are review signals only. They are not a complete
claims-timeliness series, not appeals evidence, not per-veteran cost evidence,
not a service-package path, not a waste finding, not a fraud finding, and not
savings evidence.

This is not role-reviewed final threshold selection, not policy values, not
stress values, not pass/fail evidence, not lower-cost scenario admissibility,
not solver input, not rate calculation, not gross savings, not net savings, and
not a balanced-budget claim.

Compact validator phrase: draft no-regression Veterans claims-backlog floor
threshold.
Compact validator phrase: not a complete claims-timeliness series.
Compact validator phrase: policy and stress values remain null.
Compact validator phrase: not a balanced-budget claim.
