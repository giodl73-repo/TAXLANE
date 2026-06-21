# Pulse 05: Dynamic Query Rules

## Goal

Define Treasury, USAspending, and other dynamic query snapshot rules before
those sources are used.

## Changes

- Add `data/metadata/dynamic-query-rules.md`.
- Mark dynamic Treasury and USAspending exports allowed only with query
  metadata, checksums, period locks, and review gates.
- Keep CBO budget data candidate-only until accessible source capture is
  verified.
- Mark the controlled source extraction wave complete.

## Validation

- `git diff --check`

## Status

Done.
