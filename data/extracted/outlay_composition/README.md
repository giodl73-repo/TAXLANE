# Outlay Composition Extraction

This directory holds draft `outlay_composition` records extracted from OMB
Historical Table 6.1 (Composition of Outlays).

## Source

| Source ID | Raw artifact | Use |
|---|---|---|
| `SRC-OMB-HIST-6-1-FY2027` | `data/raw/omb/SRC-OMB-HIST-6-1-FY2027/2026-06-24/hist06z1_fy2027.xlsx` | National-defense outlays as a percentage of GDP, function-050 basis. |

## Current Output

The national-defense %-of-GDP series for all actual fiscal years, 1940-2025
(FY2026-FY2031 estimates excluded):

```text
outlay_composition.SRC-OMB-HIST-6-1-FY2027.2026-06-24.national-defense-gdp.draft.jsonl
table-6-1-national-defense-gdp-profile.md
```

Each JSONL row carries `record_family: outlay_composition`, `function_code: "050"`,
`measure: "percent_of_gdp"`, the `percent` value, the source row reference, and a
`status: draft-extracted` label. The figures are the OMB budget-function (050) basis,
**not** the SIPRI/NATO defense definition; the two series are never merged.

## Regeneration

```powershell
cargo run -p taxlane-tools -- outlay-composition table-6-1-national-defense
cargo run -p taxlane-tools -- outlay-composition table-6-1-national-defense --check
```

The `--check` mode regenerates the artifacts in memory and fails if they differ from the
committed files. No public lane allocation should use these draft rows; they back the
historical defense-share figures in the T3 research paper
(`research/publications/defense-tax-in-allied-perspective/`).
