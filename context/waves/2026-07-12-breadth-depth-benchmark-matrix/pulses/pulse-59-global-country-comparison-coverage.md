# Pulse 59: Global Country Comparison Coverage

## Goal

Define a complete, non-naive multi-country comparison design for every Taxlane
lane before collecting and publishing country values.

## Result

- Covers all 15 unique breadth-matrix lane IDs.
- Establishes a default European panel of Germany, France, the United Kingdom,
  Sweden, the Netherlands, and Poland.
- Establishes Japan and Korea as the core definition-compatible Asian peers,
  with Singapore allowed only where an official matched series exists.
- Adds Canada and Australia as additional institutional peers.
- Requires at least three comparator countries and European plus Asian coverage
  whenever a matched official series supports it.
- Separates spending, service/output, and outcome metrics.
- Uses structured country cases rather than fake numerical league tables for
  payment integrity and veterans.
- Keeps every country-value, ranking, efficiency, fraud, and savings gate false
  until official source bytes, metadata, checksums, matched scope, missingness,
  and comparability review are complete.
- Corrects the scoreboard's stale 13-lane headline to the 15 unique lane IDs in
  the canonical 17-row matrix.

## Artifacts

- `data/derived/breadth_benchmark_matrix/global_country_comparison_coverage.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/global_country_comparison_coverage.schema.md`
- `docs/reading/global-country-comparison-coverage.md`

## Next Gate

Capture the OECD annual government-expenditure-by-function structure and a
bounded common-year country extract, then add source metadata, checksums,
missingness, and scope review before exposing any country value.

```text
comparison design != observed country result != efficiency != fraud != savings
```
