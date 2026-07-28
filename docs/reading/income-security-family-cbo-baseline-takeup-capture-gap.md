# Income-security/family CBO baseline/take-up capture gap

Machine record:
`data/derived/breadth_benchmark_matrix/income_security_family_cbo_baseline_takeup_capture_gap.v1.draft.json`

Pulse 195 records why the CBO baseline/take-up gate cannot be closed yet.

The target source family is CBO selected-program baseline material for
income-security/family. The first candidate is CBO's February 2026 SNAP baseline
PDF:

`https://www.cbo.gov/system/files/2026-01/51312-2026-02-snap.pdf`

Automated capture from `cbo.gov` returned JavaScript challenge HTML, not the
PDF. A second request with a browser-like user agent returned a 770-byte
challenge response, which was deleted instead of committed as raw source
custody.

The official CBO selected-program page is browser-visible, and the February
2026 SNAP PDF is browser-readable as `SNAP Baseline--02-2026-rev`. Browser text
shows a 3-page PDF with FY2026-FY2036 baseline fields for estimated outlays,
budget authority, average monthly participation, average monthly benefit per
participant, Thrifty Food Plan estimated change, fiscal-year unemployment rate,
nutrition assistance for Puerto Rico and American Samoa, and employment and
training budget authority. This is browser-visible context only: direct
command-line attempts on 2026-07-24 to capture the official PDF and spreadsheet
URLs returned HTTP 403, so no local raw PDF/spreadsheet custody, byte count,
SHA-256, or extracted baseline values are claimed.

The official CBO open-data repository was reachable at
`https://github.com/US-CBO/cbo-data`, with observed HEAD
`284a95665f9f2f74ed1f482feb629b43fce323da`, but its catalog did not expose this
selected-program SNAP baseline as a machine-readable CSV.

## Next Manual Gate

Before this component can close, a reviewer needs to manually capture the CBO
SNAP baseline PDF, record byte count, SHA-256, metadata path, retrieval date,
and page URL, then extract baseline outlays, participation or take-up context,
average benefit basis, fiscal-year coverage, and CBO caveats.

That would close only a SNAP component. The broader CBO baseline/take-up gate
still also needs the remaining income-security/family components.

This gap record is not raw CBO source custody, not CBO baseline values, not
take-up context, not a benefit package model, not a take-up model, not floor
values, not federal/state/local translation, not solver input, not rate
calculation, not a public rate card, not gross savings, not net savings, not a
department-cut instruction, not a technology-savings claim, and not a
balanced-budget claim.

Short validator phrases: not take-up context; not a balanced-budget claim.
