# Income-security/family food hardship and nutrition capture gap

This packet explains
`data/derived/breadth_benchmark_matrix/income_security_family_food_hardship_nutrition_capture_gap.v1.draft.json`.

Pulse 199 records the next unresolved USDA source gate for the
income-security/family lane. Candidate official surfaces include USDA ERS food
security publications and topic pages, plus USDA FNS SNAP participation, cost,
and benefit data tables.

The gap remains open, but the ERS food-security and FNS SNAP portions now have
local raw custody. On 2026-07-24 the repo captured the official ERS household
food-security report, report summary, statistical supplement, topic page, data
product page, and 2024 CPS Food Security Supplement documentation under
`data/raw/usda/SRC-USDA-ERS-HOUSEHOLD-FOOD-SECURITY-2024/2026-07-24/`. The ERS
metadata packet is
`data/metadata/SRC-USDA-ERS-HOUSEHOLD-FOOD-SECURITY-2024.2026-07-24.metadata.md`.
The primary ERS report is `err-358-household-food-security-2024.pdf`, 1017042
bytes, SHA-256
`dfe19c73cd5fbaa08a2dec52768690c968892150153806fec83038d3dac0adf7`.

The live official FNS page also exposed current resource-file links, and the
repo captured the annual summary PDF/workbook, FY1969-current ZIP, monthly
workbook, persons workbook, households workbook, and benefits workbook under
`data/raw/usda/SRC-USDA-FNS-SNAP-PARTICIPATION-COST-DATA/2026-07-24/`. The FNS
metadata packet is
`data/metadata/SRC-USDA-FNS-SNAP-PARTICIPATION-COST-DATA.2026-07-24.metadata.md`.

The primary annual summary workbook is
`snap-annualsummary-7.xlsx`, 24215 bytes, SHA-256
`53c101e4f23c12d04c65ed304919b5f5ed18c560f9ea81acb9191cf8a54254e3`, with one
observed worksheet dimension, `A1:F200`. Supporting captured FNS files include
the annual summary PDF, FY1969-current ZIP, monthly workbook, persons workbook,
households workbook, and benefits workbook.

Before values can be populated, a reviewer still needs broader nutrition
assistance boundary context, source-year and household/child perimeter review,
benefit/allotment basis review, and program boundary caveats.

This capture gap has partial ERS food-security and FNS SNAP raw custody, but it
is not complete USDA raw source custody, not a complete nutrition-program
boundary, not material-hardship floor values, not food-security floor values,
not benefit-package context, not a benefit package model, not a take-up model,
not pass/fail findings, not target-cost selection, not gross savings, not net
savings, not solver input, not rate calculation, not a public rate card, not a
department-cut instruction, not a technology-savings claim, and not a
balanced-budget claim.

Short validator phrases: food-hardship/nutrition source-capture gap with
partial ERS food-security and FNS SNAP raw custody; not complete USDA raw
source custody; not solver input; not a balanced-budget claim.

Exact validator phrases: USDA ERS food security publications; USDA FNS SNAP
participation, cost, and benefit data tables.

Exact validator phrase: USDA FNS SNAP participation, cost, and benefit data tables.
