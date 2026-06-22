# Wave: Income-Tax Outlay Artifact Manifest

## Goal

Create an auditable manifest for the income-tax outlay model artifact chain.

## Thesis

The model now has canonical JSONL, derived summaries, CSV exports, chart specs,
and reader docs. A manifest should make the chain discoverable and reproducible
by recording artifact roles, row counts where applicable, and checksums.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Artifact manifest | done | Added a manifest generator and generated manifest for model artifacts. |

## Success Criteria

- Manifest lists canonical JSONL, derived JSONL, CSV exports, chart specs, and
  reader/research notes.
- Manifest records SHA-256 checksums for tracked artifacts.
- Row counts are recorded for JSONL and CSV data files.
- The manifest states that JSONL files remain canonical and CSV/chart files are
  derived views.

## Non-Goals

- Do not modify model values.
- Do not add new charts.
- Do not make new public claims.

## Validation

Run:

```powershell
python data/derived/income_tax_outlay_model/build_manifest.py --check
git diff --check
```
