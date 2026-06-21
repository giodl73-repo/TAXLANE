# Wave: Controlled Source Extraction

## Goal

Move TAXLANE from source dictionaries to controlled, reviewable source
extraction without mixing raw artifacts, extracted draft records, and public
explainers.

## Thesis

TAXLANE can only publish credible taxpayer-facing allocation models if its
source artifacts and extracted records are reproducible. The next step is to
define where raw files live, how fetch metadata is recorded, and how draft
records move from raw source to reviewed data.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Data custody scaffold | done | Created top-level data custody structure, raw/extracted/derived boundaries, and metadata requirements. |
| 02 | IRS rates first-source capture | done | Captured IRS SOI Table 23 raw artifact, sidecar metadata, checksum, and draft extraction workspace. |
| 03 | OMB receipts first-source capture | done | Captured OMB receipt/fund raw artifacts, metadata, checksums, and draft extraction workspaces. |
| 04 | OMB outlays first-source capture | pending | Capture OMB outlay spreadsheet metadata and prepare outlay/lane draft extraction. |
| 05 | Dynamic query rules | pending | Define Treasury, USAspending, and other dynamic query snapshot rules before use. |

## Success criteria

- Raw, extracted, and derived data have separate repo locations and rules.
- Every committed source artifact has sidecar metadata and checksum rules.
- Extracted draft records point to schema files and source IDs.
- Dynamic query exports remain blocked until query snapshot rules exist.
- Validation commands pass.

## Non-goals

- Do not publish taxpayer allocation outputs in this wave.
- Do not treat draft extracted records as reviewed data.
- Do not use dynamic query sources before query custody rules are written.
- Do not replace source-ledger IDs with filenames.

## Validation

Run:

```powershell
git diff --check
```
