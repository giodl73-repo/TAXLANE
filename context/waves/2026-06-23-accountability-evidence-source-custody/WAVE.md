# Wave: Accountability Evidence Source Custody

## Goal

Add the first source-custody accountability evidence record and validate it through Rust before public use.

## Thesis

TAXLANE can only demand performance and investigate waste or fraud credibly if accountability records are validated against source custody before any public claim. The first record should prove the pipeline and explicitly avoid alleging misconduct or scoring performance.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Source-custody evidence record | done | Added draft accountability evidence record, dataset method note, source-ledger validation, role review, manifest coverage, and VTRACE closeout for WP-TAX-004. |

## Success Criteria

- Draft record parses through `taxlane-core`.
- Every source ID appears in `docs/sources/source-version-ledger.md`.
- Record states it is not an allegation, fraud signal, waste signal, or performance score.
- TAXLANE and VTRACE validation pass.

## Non-Goals

- Do not add scoring.
- Do not publish public accusations.
- Do not add named vendor, recipient, award, or person-level records.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo test
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
git diff --check
```
