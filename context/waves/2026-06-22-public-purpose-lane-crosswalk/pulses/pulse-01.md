# Pulse 01: Draft OMB Lane Crosswalk

## Goal

Add the first draft lane crosswalk from OMB Historical Table 3.1 and 3.2
categories to TAXLANE public-purpose lane IDs.

## Changes

- Add `data/derived/lane_crosswalk/` with a draft JSONL crosswalk and README.
- Add a research note explaining the function-aligned mapping rule.
- Update data documentation so lane crosswalks are treated as mapping inputs,
  not receipt allocation proof.
- Update wave status.

## Validation

- `git diff --check`

## Status

Done.
