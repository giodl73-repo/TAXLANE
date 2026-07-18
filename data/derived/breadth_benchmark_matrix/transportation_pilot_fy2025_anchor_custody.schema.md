# Transportation Pilot FY2025 Anchor Custody Schema

Canonical artifact:
`transportation_pilot_fy2025_anchor_custody.v1.draft.json`.

Required top-level fields:

- `record_id = transportation-pilot-fy2025-anchor-custody:v1`
- `record_family = transportation_pilot_fy2025_anchor_custody`
- `version`
- `status`
- `pulse = 95`
- governing paths
- `source_custody`
- `scope`
- `fy2025_anchor_reconciliation`
- `still_missing_for_baseline_path`
- `output_placeholders`
- `claim_booleans`
- `non_claim_boundary`

Validation rules:

- The raw OMB Table 3.2 file must already exist locally.
- The recorded byte count and SHA-256 must match the local raw file.
- No new external request may be marked submitted.
- Custody may close only for the FY2025 transportation anchor, not the full
  baseline path.
- FY2025 transportation components must sum to 145,320 million and match the
  parent transportation row and transportation depth card.
- Current-law reform delta must be zero.
- Federal/state/local translation and trust-fund reconciliation remain `null`.
- Full baseline-path blockers remain listed.
- Every output placeholder remains `null`.
- Only `fy2025_anchor_custody_published` may be true; baseline, floor,
  modernization, stress, simulator, target-cost, rate, public-card, savings,
  waste, fraud, department-cut, technology-savings, and balanced-budget claims
  remain false.
