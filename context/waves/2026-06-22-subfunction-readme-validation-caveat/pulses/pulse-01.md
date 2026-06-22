# Pulse 01: Subfunction README Caveats

## Goal

Make the generated subfunction model README carry validation and decade caveats.

## Changes

- Add a decade rollup caveat to the Rust README template.
- Add the aggregate validation command to the generated README.
- Regenerate the README and manifest.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
