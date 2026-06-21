# Source Metadata

This directory holds custody records for raw artifacts and query captures.

## Metadata File Naming

Use:

```text
{source-id}.{observed-date}.metadata.md
```

Example:

```text
SRC-OMB-HIST-2-1-FY2027.2026-06-21.metadata.md
```

## Required Fields

Every metadata record must include:

| Field | Meaning |
|---|---|
| `source_id` | Source ID from `docs/sources/source-version-ledger.md`. |
| `publisher` | Official publisher or steward. |
| `source_url` | Canonical URL used for capture. |
| `observed_date` | Date TAXLANE observed or fetched the source. |
| `capture_method` | Browser download, scripted fetch, manual copy, API query, or other. |
| `raw_path` | Repo path to the raw artifact, if committed. |
| `checksum_sha256` | SHA-256 checksum for raw artifact or query body. |
| `schema_file` | Schema governing extraction from this source. |
| `coverage` | Tax years, fiscal years, legal dates, or query period covered. |
| `status` | Custody status. |
| `notes` | Manual steps, caveats, or blockers. |

## Dynamic Query Additions

Dynamic query records must also include:

- Query URL or endpoint.
- Query parameters.
- Run timestamp and timezone.
- Pagination or result-limit behavior.
- Whether results can change after capture.

Dynamic query exports remain blocked until `context/waves` records a query
snapshot pulse.

The first query snapshot rules are defined in `dynamic-query-rules.md`.
