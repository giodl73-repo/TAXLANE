# Pulse 55: Quality Learning Center Official Context

## Result

Captured the Minnesota Department of Children, Youth, and Families provider
table hosted by the Minnesota House of Representatives. The official table
resolves the entity as `Quality Learning Center Inc`, license number `1087038`,
at 1411 Nicollet Avenue in Minneapolis.

The table reports CCAP payments of $1,730,115 for calendar year 2024 and
$2,150,964 for calendar year 2025. The House testimony describes the $1.9
million as 2025 funding, so that figure does not equal DCYF's CY2025 annual
value. The testimony follows a December 16 visit narrative but does not disclose
its data cutoff, whether the amount is year-to-date, or its calculation basis.
The official annual value therefore cannot be treated as corroboration or
counterevidence for the exact amount.

## Evidence Decision

The official table `supplies_context` to the existing claim-origin atom. It
does not populate the corroboration or counterevidence arrays, record an
official response, establish a legal or administrative status, or change any
attributed or substantive claim gate.

The table also reports $69,365 in CCAP overpayments assessed and $69,365
repaid. It does not disclose the assessment period, transaction lineage, or
basis. Those figures therefore cannot establish fraud, show that all provider
payments were improper, or support a recovery or savings claim.

## Custody

- Source ID: `SRC-MN-HOUSE-DCYF-CCAP-PROVIDER-DATA-2026-04-22`
- Raw path: `data/raw/minnesota-house/SRC-MN-HOUSE-DCYF-CCAP-PROVIDER-DATA-2026-04-22/2026-07-14/dcyf-ccap-provider-data.pdf`
- Bytes: `1277757`
- SHA-256: `E7068E1198D8DCE851907B60FC4A2A16FEDD5DE7A1D41AFCD2B02DCAABF3DEC1`

## Boundary

`Quality Learing Center` is the spelling in the testimony and public-facing
claim context. `Quality Learning Center Inc` is the official provider name.
License number `1087038` and the Nicollet Avenue address provide the entity
match; name similarity alone is not the join key.

## Trace

- Work package: `WP-TAX-073`
- Evidence: `EVID-TAX-073`
- Validation: `VAL-TAX-073`

## Next Bounded Action

Seek transaction-level CCAP payment ledgers, adjustments, the basis and cutoff
for the reported overpayment assessment, and license-closure records. Prepare
an external records-request route if useful, but do not submit a request
without explicit authorization.
