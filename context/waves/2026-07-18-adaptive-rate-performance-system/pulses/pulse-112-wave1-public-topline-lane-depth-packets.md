# Pulse 112 — Wave 1 public-topline lane-depth packets

Pulse 112 executes the first scaled-agent lane-depth wave as an integrated
scaffold for:

- Health and Medicare
- Social Security
- National defense

Artifacts:

- `data/derived/breadth_benchmark_matrix/wave1_public_topline_lane_depth_packets.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/wave1_public_topline_lane_depth_packets.schema.md`
- `docs/reading/wave1-public-topline-lane-depth-packets.md`

Boundary:

- These are explainability scaffolds, not final lane-depth completion.
- Missing values remain null.
- Blocked statuses remain false.
- Health private-insurance sensitivity remains non-federal and non-target.
- Social Security remains a separate OASDI trust-fund lane.
- Defense review signals do not become automatic savings, waste findings, fraud
  findings, or cut instructions.
- Solver, rate, target-cost, savings, waste, fraud, department-cut,
  technology-savings, and balanced-budget claims remain blocked.

Validation:

- Rust validator requires the exact three Wave 1 lanes.
- Rust validator checks per-lane null/false claim gates.
- Rust validator checks public warning phrases.
