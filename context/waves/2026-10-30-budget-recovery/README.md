# Scenario-preserving budget explorer recovery

The previous coordinator created its worker only once, required reload after
failure, and bounded neither initialization nor calculations. Explicit Retry now
recreates the worker with independent loading/calculation/debounce timers. Keep
current assumptions rather than reloading a stale shared URL, rebuild controls
without duplicates, retain open evidence panels, clear stale totals/transit and
disable exports until new Rust output. Guard constructor/render failures and old
workers/revisions. Reset remains explicit. No worker/Rust/ledger/source/rule,
license, dependency or authority changes. Interface review accepted in
reviews/2026-10-30-budget-recovery.md before implementation; no domain promotion.

Gates: existing scoped native/strict checks, actual WASM build, original tests and
eight recovery cases, hosted PR/default, exact artifact and live navigation.
The former baseline-failure test now requires visible Retry instead of reload;
its readable research route and disabled-reset assertions remain.

Review also reproduced malformed null baseline rows breaking failure cleanup
before Retry could appear. Check rendering shape before assigning the baseline;
accounting validation remains in Rust. A dedicated malformed-row test fails on
the initial implementation and requires clean in-page recovery after the fix.
Local scoped native checks pass all 156 tests, plus fmt/strict adapter Clippy.
