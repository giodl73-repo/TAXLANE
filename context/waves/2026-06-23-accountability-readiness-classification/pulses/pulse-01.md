# Pulse 01: Public-Claim Readiness States

## Goal

Add typed readiness classification for accountability evidence records.

## Changes

- Add `PublicClaimReadiness` to `taxlane-core`.
- Classify records as `EvidenceOnly`, `NeedsRoleReview`, or `PublicClaimEligible`.
- Add tests for source-reviewed and role-reviewed official-finding paths.
- Update schema notes, dataset notes, review, and VTRACE rows.

## Validation

- `cargo test`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`
- `git diff --check`

## Status

Done.
