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
python tools/build-pages.py --inputs ../taxlane-rust-inputs --cargo-home ../taxlane-cargo-home
npm ci
npx playwright install chromium
npm test
```

Rust 1.95.0 and the WASM target are declared in `rust-toolchain.toml`. The site
build writes its runtime and separate complete-source artifacts inside `dist/`,
uses verified owned offline Rust sources/home, enforces a 5 MB runtime budget, and never
rewrites canonical research artifacts. The workflow validates PRs and deploys
the tested artifact only from `main`. Local tests serve `/TAXLANE/` on port 8766.
For an existing browser binary, set `TAXLANE_BROWSER_PATH` when testing locally.
Use new output directories; `--target` can select an existing compiled cache or
an initially absent directory for an independent rebuild. Source and input/home
paths must not overlap. `--source-output` can select a new external source-artifact
directory. For an alternate build tree, set `TAXLANE_DIST` to its parent containing
`TAXLANE/`; the browser server always starts fresh. `TAXLANE_SITE_URL` selects an
existing HTTP(S) origin for live qualification, with no local server reuse.

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

## Transportation operating-cost equivalents

The optional `transit_share_bps` (0–10000) and `transit_area` (0–3) inputs default to zero, preserving old links. Rust multiplies only the positive transportation increment by that share, then divides dollar allocation by the selected FTA 2024 fixed-route bus average operating expense per vehicle revenue hour. It does not add a second budget outlay. Price year, source and nonforecast basis are included in the comparison result. Export/share retain the assumptions. Review and exact reference values: [acceptance](../reviews/2026-10-05-transit-cost-equivalents.md). This is ratio arithmetic owned by TAXLANE; domain service models remain with the jurisdiction-map owners.


## Retry without losing the scenario

Use Retry explorer after a baseline/WASM download, engine or calculation failure.
Loading has a fifteen-second deadline; calculation has ten seconds, distinct from
the input debounce. Retry retains the current lane/receipts/transit assumptions,
including changes newer than a loaded shared URL. Reset is still explicit.
Stale totals, cost equivalents and exports remain unavailable until a fresh Rust
result succeeds. Controls/evidence panels rebuild once and old worker callbacks
cannot restore results. No automatic loop or saved/uploaded personal data.
Full page reload still reads the URL rather than saving unshared session edits.
[Interface acceptance](../reviews/2026-10-30-budget-recovery.md) preserves all
existing ledger, integer-rounding, fungibility, source and authority boundaries.

## Inspect or save the complete comparison

Download comparison saves the exact Rust response as `taxlane-comparison.json`.
Inspect complete comparison JSON shows the same unrounded string, including
integer amounts in millions, fixed accounting rows and transit source/price-year/
nonforecast labels. Download scenario retains its original input-only format and
filename; comparison output is a separate artifact, not admitted fiscal evidence.

Stop explorer terminates the current worker and clears displayed/exportable
results. Retry explorer retains lane, receipts and transit assumptions and open
evidence panels; Reset remains explicit. Changed inputs and failures immediately
clear comparison JSON as well as totals. Only the current request may supply
results or a calculation rejection; late responses from retired workers and
obsolete requests cannot restore results or retire a newer comparison.
