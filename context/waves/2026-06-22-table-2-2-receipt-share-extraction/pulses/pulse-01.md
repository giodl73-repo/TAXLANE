# Pulse 01: Table 2.2 Receipt Shares

## Goal

Add draft receipt-source share-of-total records from OMB Historical Table 2.2.

## Changes

- Add Rust `receipt-source table-2-2` subcommand.
- Emit Table 2.2 draft JSONL records for fiscal years 1934-2031.
- Add `table-2-2-profile.md`.
- Update receipt-source README current outputs.

## Validation

- `cargo run -p taxlane-tools -- receipt-source table-2-2 --check`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
