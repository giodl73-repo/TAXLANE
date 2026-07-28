# Health Medicare Provider Adequacy Margin Floor Value Packet

Machine record:
`data/derived/breadth_benchmark_matrix/health_medicare_provider_adequacy_margin_floor_value_packet.v1.draft.json`

Source context:
`data/derived/breadth_benchmark_matrix/health_target_admissibility.v1.draft.json`

This is a Wave D floor-value packet for the health/Medicare lane. It converts
the health target-admissibility hospital adequacy context into a draft provider
adequacy margin threshold rationale and baseline value, but it does not pass or
fail any policy scenario.

Draft threshold rule: a lower-cost health payment scenario cannot pass this
provider-adequacy floor if reviewed policy and stress evidence push relatively
efficient hospitals below the FY2024 median FFS Medicare margin baseline.

Selected baseline and threshold:

| Field | Value |
| --- | ---: |
| Provider adequacy margin threshold floor | -1.0 percent |
| Relatively efficient hospital median FFS Medicare margin | -1.0 percent |
| Aggregate FFS Medicare hospital margin | -12.1 percent |
| Efficient hospital projected 2026 median FFS Medicare margin | 1.0 percent |
| All-payer operating margin | 6.5 percent |
| Commercial hospital price reference | 253 percent of Medicare |

The access context is `good_overall`; the quality context is `mixed`.

This baseline is provider-adequacy context only. It is not a universal
Medicare-relative target, not access-floor passage, not quality-floor passage,
not rural or safety-net capacity evidence, not policy values, not stress
values, not pass/fail evidence, not a federal policy score, not gross savings,
not net savings, not solver input, not rate calculation, and not a
balanced-budget claim.

Compact validator phrase: draft no-regression health/Medicare provider-adequacy margin floor threshold.
Compact validator phrase: not a universal Medicare-relative target.
Compact validator phrase: policy and stress values remain null.
Compact validator phrase: not a balanced-budget claim.
