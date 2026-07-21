# Health floor source capture status schema

`health_floor_source_capture_status.v1.draft.json` records the first
health/Medicare source-capture status after the lane floor source work queue.

Required invariants:

- `record_family` is `health_floor_source_capture_status`.
- `pulse` is `178`.
- The record links the floor source queue, floor readiness rollup, health floor
  definition packet, FY2025 ledger custody, dedicated receipt anchors, and PHI
  sensitivity boundary.
- Existing OMB fiscal sources may be marked custody-ready only for FY2025 fiscal
  context.
- Floor-indicator, threshold, baseline-floor, policy-floor, and stress-floor
  custody remain incomplete.
- Source candidates still needed keep raw path, byte count, and SHA-256 null.
- Every floor value remains null and every floor passage flag remains false.
- Blocked outputs remain null.
- Only publication and partial fiscal-custody booleans may be true.
