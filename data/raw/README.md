# Raw Source Artifacts

This directory holds publisher files or exact query exports after custody
approval.

## Layout

Use source-family directories:

```text
data/raw/{publisher}/{source-id}/{observed-date}/
```

Example:

```text
data/raw/omb/SRC-OMB-HIST-2-1-FY2027/2026-06-21/hist02z1_fy2027.xlsx
```

## Rules

1. Do not commit a raw artifact without a metadata record in `data/metadata/`.
2. Preserve the publisher filename unless it is ambiguous or unsafe.
3. Do not overwrite raw artifacts.
4. Do not edit raw artifacts after capture.
5. Do not place extracted CSV, JSONL, notes, or public explanations here.

Dynamic query exports are deferred until query snapshot rules are written.
