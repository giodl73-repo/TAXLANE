# Pulse 03: Concept Source Review

## Goal

Verify the first AP13 concept records against the captured source text and
promote them only where the anchors support the summaries.

## Changes

- Rename the AP13 concept JSONL from draft to source-reviewed.
- Promote reviewed rows to `source-reviewed`.
- Add a source-review note with checksum, reviewed anchors, and limits.
- Mark this pulse complete in the wave.

## Validation

- Parse every source-reviewed JSONL record.
- `git diff --check`

## Status

Done.
