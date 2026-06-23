# TAXLANE Evidence Ledger

| Evidence ID | Type | Source / Command | Expected Result | Actual Result | Status |
|---|---|---|---|---|---|
| EVID-TAX-001 | command/review | `cargo run -p taxlane-tools -- income-tax-outlay validate`; `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`; receipt/display role reviews | Current receipt/display artifacts validate and are reviewed as static, modeled, non-calculator handoffs. | VTRACE adoption package records the trace; validation commands are required in this slice. | passed |
| EVID-TAX-002 | schema/review | `docs/data/accountability-evidence-schema.md`; `docs/research/2026-06-23-accountability-evidence-boundary.md`; `reviews/2026-06-23-accountability-evidence-role-review.md` | Evidence-backed accountability records exist without unsupported allegations. | Schema and role review define evidence, anomaly, allegation, and due-process boundaries. | passed |
| EVID-TAX-003 | command/review | `cargo test`; `crates/taxlane-core/src/lib.rs`; `reviews/2026-06-23-rust-core-crate-architecture-review.md` | Rust domain crate supports reusable validation/accountability models. | `taxlane-core` owns artifact metadata and accountability evidence boundary validation; CLI uses it during manifest validation. | passed |
