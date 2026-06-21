# Lane Crosswalk Extraction

This directory is reserved for draft `lane_crosswalk` records that map OMB
functions, subfunctions, and program context to TAXLANE public-purpose lanes.

## Initial Inputs

Lane crosswalk records should be built from reviewed or source-reviewed
`outlay_function` records first. Program and agency records can explain lanes,
but they should not replace the OMB function/subfunction spine.

## Planned Output

Use versioned JSONL records:

```text
lane_crosswalk.omb-fy2027-v1.draft.jsonl
```

Rows must follow `docs/data/outlays-lanes-schema.md`.

## Review Gates

1. Include explicit function and subfunction inclusions.
2. Include explicit exclusions.
3. Include allowed allocation methods.
4. Mark deficit context requirements.
5. Keep net interest and borrowed-share context visible.
6. Do not publish taxpayer-facing allocations from draft crosswalk records.
