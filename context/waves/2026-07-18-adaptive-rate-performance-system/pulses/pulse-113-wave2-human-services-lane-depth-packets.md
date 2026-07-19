# Pulse 113 — Wave 2 human-services lane-depth packets

Pulse 113 executes the second scaled-agent lane-depth wave as an integrated
scaffold for:

- Income security and family
- Education and workforce
- Veterans

Artifacts:

- `data/derived/breadth_benchmark_matrix/wave2_human_services_lane_depth_packets.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/wave2_human_services_lane_depth_packets.schema.md`
- `docs/reading/wave2-human-services-lane-depth-packets.md`

Boundary:

- These are explainability scaffolds, not final lane-depth completion.
- Missing values remain null.
- Blocked statuses remain false.
- Human-service review signals do not become savings estimates, waste findings,
  fraud findings, benefit-cut instructions, or technology-savings claims.
- Solver, rate, target-cost, savings, waste, fraud, department-cut,
  technology-savings, and balanced-budget claims remain blocked.

Validation:

- Rust validator requires the exact three Wave 2 lanes.
- Rust validator checks per-lane null/false claim gates.
- Rust validator checks public warning phrases.
