# Wave: Income-Tax Subfunction Model

## Goal

Create the first derived model that estimates ordinary individual income-tax
receipts across OMB Table 3.2 subfunctions by proportional outlay share.

## Thesis

The full Table 3.2 extraction provides a reconciled subfunction outlay spine.
The next useful derived view is an annual subfunction-level visibility model
that preserves the same caveat as the broad model: this is modeled allocation,
not legal dedication or program tracing.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Annual subfunction allocation model | done | Added Rust generation for FY1962-FY2025 individual income-tax modeled allocation rows by Table 3.2 subfunction. |

## Success Criteria

- `cargo run -p taxlane-tools -- income-tax-outlay subfunction-model` writes
  draft JSONL, profile, and README outputs.
- `cargo run -p taxlane-tools -- income-tax-outlay subfunction-model --check`
  detects stale outputs.
- Modeled rows sum back to individual income-tax receipts for each fiscal year.
- The model excludes TQ, FY2026-FY2031 estimates, and Table 3.2 `N/A` cells.
- Public wording stays explicitly modeled and non-legal.

## Non-Goals

- Do not create public lane crosswalks.
- Do not infer legal dedication.
- Do not include Table 3.2 parent totals as allocation rows.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay subfunction-model --check
cargo run -p taxlane-tools -- outlay-function table-3-2 --check
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
