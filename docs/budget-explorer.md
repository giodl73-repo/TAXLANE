# Interactive budget explorer

The Pages front door is a hypothetical accounting sandbox, separate from the
repository-only explanation site and admitted research conclusions. Adjust 14
positive program lanes and a receipts assumption against the FY2025 17-row ledger;
net interest and the two negative accounting offsets stay fixed. It shows funding
changes and the annual financing gap, not a model of service outcomes or future
debt stocks. Receipt assumptions are not tax rates. FY2025 budget arithmetic does
not recalibrate the separate FY2026 Tax-Calculator results.

`taxlane-scenario` calls `taxlane-core::reconcile_federal_year` in native tools and
WASM. The worker owns computation; JavaScript renders controls/results. Inputs
are integer basis points limited to -100%..=+100%; displayed scenario amounts round
to the nearest million dollars. The baseline builder preserves source IDs and
rejects inconsistent fiscal years/accounting context; the Rust validator requires
all lanes to reconcile exactly before the site can be published.

```powershell
cargo test --locked -p taxlane-core -p taxlane-scenario
cargo fmt -p taxlane-scenario --check
cargo clippy --locked --no-deps -p taxlane-scenario --all-targets --all-features -- -D warnings
cargo install wasm-bindgen-cli --version 0.2.127 --locked
python tools/build-pages.py
npm ci
npx playwright install chromium
npm test
```

Rust 1.95.0 and the WASM target are declared in `rust-toolchain.toml`. The site
build writes only inside `dist/`, enforces a 5 MB delivery budget, and never
rewrites canonical research artifacts. The workflow validates PRs and deploys
the tested artifact only from `main`. Local tests serve `/TAXLANE/` on port 8766.
For an existing browser binary, set `TAXLANE_BROWSER_PATH` when testing locally.

Share links hold percent changes, which replay against the named FY2025 baseline.
Download JSON includes the baseline version and year. Source/WASM downloads and
the scoped content/software licenses are linked from the interface. There is no
server-side storage, analytics, or account requirement.

Local evidence: 152 existing core tests and 3 new scenario tests passed; scoped
strict Clippy passed. Existing core lint warnings are outside the new adapter's
strict check and were not edited. The complete WASM/site build is 180,975 bytes.
Three browser checks passed with pinned Playwright 1.58.2 and Chromium revision
1243: actual WASM accounting, keyboard controls, share/reload/reset/download,
mobile overflow, malformed input, and failed baseline download. Code review,
hosted installation, and deployment evidence are recorded in the local wave.
