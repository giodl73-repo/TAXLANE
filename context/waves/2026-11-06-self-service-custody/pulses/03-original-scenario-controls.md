# Pulse 03 — original native/WASM response controls

Add a test-only native JSON-lines example mirroring the existing `compare_json`
parse/compare/serialize closure without constructing native `JsValue` errors.
Production Rust, manifests, lockfile, baseline and original browser implementation
remain unchanged. Keep raw baseline/scenario JSON as strings through Node so
values beyond JavaScript's exact integer range survive.

The frozen original baseline generates5,615 portable controls: all integer
percentage settings for fourteen adjustable lanes and receipts, combined lane
settings, transit areas/shares/increments, deterministic mixed scenarios,
sub-percent rounding, synthetic i64 boundaries, fixed rows, fiscal-year typing,
escaped text, malformed JSON and invalid typed inputs. These are software
fixtures, not policy forecasts or admitted fiscal evidence.

All5,525 successful responses and90 rejection messages match as complete strings
between native release Rust and the actual original WASM adapter. Both raw
response streams also have identical byte length and SHA256:13,083,626 bytes,
`b5be2b606ae39231dbe512d94028a5ce969e6a8ba7a81f37adf5ca213c92f615`.
The verifier independently checks every successful result's integer scaling,
lane order, sums, fixed interest, receipts, deficits and transit arithmetic and
source/nonforecast labels. Only independent floating arithmetic uses tolerance;
native/WASM strings are never normalized, rounded or compared by selected fields.
Five negative controls reject mismatched responses, matching-but-wrong accounting,
missing records, empty matrices and incorrect success/rejection expectations.

Two additional oversized `usize` inputs are separate observations: native64
rejects an unknown transit area; WASM32 rejects integer overflow while parsing.
Both reject, and their original target-specific messages are retained. They are
excluded from portable parity rather than changing original behavior.

Before execution, two v1 malformed fixtures were found to mutate hardcoded row14
(science/space), not net interest. Corrected v2/v3 generators select by lane ID.
The original generated v1 controls remain in local qualification evidence; no
production correction was needed. The reusable v3 runner reproduces v2's input
and output hashes. Local formatting and strict all-target/all-feature adapter
Clippy pass. Predecessor hosted run42215526725 passed at5adb3998; the new workflow
adds the matrix and retains its raw evidence as an artifact.

## Reproduce

With declared Rust/Node/Python/bindgen prerequisites and the original site built:

```sh
python tools/build-pages.py
python tools/check-scenario-controls.py dist/TAXLANE new-scenario-controls
```

The destination must be new. Cargo uses the caller's configured compiled target;
portfolio laptop runs should keep the documented shared target. Each baseline
and scenario is passed as raw JSON to both adapters. A receipt records complete
input/output hashes, counts and scope. Gzipped original local streams/commands
and negative-control outcomes are under `validation/pulse03/`.

## Still open

This is original-adapter parity using local installed tools/cache and an existing
site build. It is not a live-worker test, final artifact proof, independent owned
source/empty-target replay or source release. Full-source receiver/publication,
raw result inspection/export, request/recovery qualification, hosted final
artifacts, anonymous release/live readback and portfolio pointer promotion remain
open. Existing research/source/admission holds and split licenses are unchanged.

## Reader integration checkpoint

Worker messages now carry the exact `compare_json` return string; the interface
parses that string for presentation and exposes it unchanged for optional
inspection and comparison download. Original scenario export remains input-only
with the same filename/schema. Input changes, errors and Stop clear raw output
and disable its download immediately. Stop terminates the worker; Retry retains
current assumptions/evidence. Calculation rejections include a request ID so an
obsolete rejection cannot retire newer output. Initialization failures remain
fatal to their current worker generation. Production Rust remains unchanged.

Add actual-worker browser cases for exact inspection/export bytes, retained input
export, unrounded transit values, mobile overflow, keyboard Stop/Retry, retired
worker callbacks and stale/current rejection correlation. Validation outcome is
recorded separately after executing the full existing and new browser suite.
