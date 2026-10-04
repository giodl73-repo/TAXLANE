# Browser budget explorer

Status: native/WASM/site build, browser checks, visual inspection, and final
code review passed; hosted publication pending. No new admitted research
scenario results claimed.

Build a small interactive GitHub Pages front door using TAXLANE's Rust accounting
core compiled to WebAssembly. Readers adjust spending assumptions and compare
the resulting outlays, financing gap, and fiscal deltas against a sourced,
dated baseline. Keep negative accounting offsets and net interest distinct.

The sandbox does not change admitted savings, research closure records, or the
preferred analytical rate schedule. Existing tax-response runs are Python model
outputs; a browser adapter must preserve their scope and cannot imply a new
microsimulation or a service-outcome model that has not been implemented.

Validate source/corpus reconciliation, input limits, accounting identities,
native/WASM agreement, browser controls and reset/share behavior, mobile layout,
failure states, bundle size, code review, CI, and live deployment. Follow the
portfolio sequence and licensing boundary in TRACKER's interactive Pages wave.

Evidence: 152 existing core tests plus 3 adapter tests passed; scoped fmt and
strict Clippy passed; full site 180,975 bytes; three real browser tests passed
again after the mobile sticky-summary correction (6.2 seconds). Pinned local
Playwright 1.58.2 used Chromium revision 1243. Final bounded built-in Codex
review was clean. Hosted browser installation and deployment remain release gates.
