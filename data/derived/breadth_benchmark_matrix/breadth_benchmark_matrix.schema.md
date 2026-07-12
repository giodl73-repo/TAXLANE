# Breadth Benchmark Matrix Schema

One JSON object per line. Each row represents one fiscal metric or an explicit
coverage gap.

| Field group | Purpose |
|---|---|
| Identity | `record_id`, `record_family`, `lane_id`, and `metric_label` identify the comparison. |
| Breadth/depth | `depth_tier` and `coverage_status` distinguish full comparisons, topline cards, and missing coverage. |
| Current | `current_value`, `current_unit`, `current_period`, and `current_basis` define "now." |
| Benchmark | `benchmark_low`, `benchmark_high`, `benchmark_unit`, `benchmark_period`, and `benchmark_type` define the comparison without treating it automatically as a target. |
| Interpretation | `gap_direction`, `comparability_grade`, and `efficiency_gap_status` state what the comparison can support. |
| Integrity | Improper-payment, fraud, and recoverable-savings fields remain separate. |
| Evidence | `source_ids` and `next_depth_need` preserve provenance and the next missing layer. |

## Hard boundaries

- A peer difference is an observed comparison, not a causal waste estimate.
- An improper-payment estimate is not a fraud estimate.
- Neither a peer gap nor an improper-payment estimate is automatically
  recoverable savings.
- Coverage gaps publish no invented current value or benchmark.
- Current and benchmark units must match before a numeric gap is interpreted.

