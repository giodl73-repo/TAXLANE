# TAXLANE Data Custody

This directory is for source artifacts and machine-readable records. It is
separate from `docs/data/`, which defines schemas and custody rules in prose.

No file in this directory is public-facing explanation by itself. Public claims
must be written in `docs/` and cite source IDs, schema files, and review status.

## Showcase Custody Path

For the current showable source-custody/readiness state, start with
`docs/reading/taxlane-showcase-readiness-summary.md` and the machine record
`data/derived/breadth_benchmark_matrix/taxlane_showcase_readiness_summary.v1.draft.json`.
Those artifacts explain which evidence trails are demo-ready and which solver,
rate, savings, public-card, department-cut, technology-savings, and balanced-
budget outputs remain blocked.

## Directory Map

| Directory | Purpose | Commit rule |
|---|---|---|
| `raw/` | Publisher files or exact query exports. | Allowed only with sidecar metadata and checksum. |
| `metadata/` | Fetch records, checksums, source observations, and query parameters. | Required for every raw artifact or dynamic query capture. |
| `extracted/` | Draft records extracted from raw source artifacts into schema-backed formats. | Allowed only when source IDs and schema version are recorded. |
| `derived/` | Modeled or transformed records built from extracted records. | Blocked until inputs are reviewed and derivation method is documented. |

## Status Levels

Use these statuses in metadata and extracted records:

- `captured`: Source artifact or query result is stored with metadata.
- `draft-extracted`: Records have been parsed but not reconciled.
- `source-reviewed`: Source identity, version, and row anchors have been checked.
- `budget-reviewed`: Budget semantics and reconciliation checks have been run.
- `reviewed`: Source and budget reviews are complete.
- `superseded`: A newer source version replaces this one.
- `retired`: The file or record should no longer be used.

## First-Use Rules

1. Confirm the source appears in `docs/sources/source-version-ledger.md`.
2. Confirm the relevant schema exists in `docs/data/`.
3. Capture metadata before extracting rows.
4. Preserve publisher filenames when possible.
5. Record checksums for raw artifacts.
6. Keep dynamic query outputs out of `raw/` until query snapshot rules exist.
7. Treat every extracted file as draft until role review is recorded.

## Validation

For documentation-only changes, run:

```powershell
git diff --check
```

For data, derived records, source links, chart specs, manifests, or public
packets, run the full guardrail:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
```
