# Self-service artifact links

Expose build provenance beside the compiled WASM, source archive and licenses in the footer. The initial link-only PR 36 incorrectly assumed the JSON was already published. The follow-up fixes the builder to generate actual Git commit/dirty state, Rust and bindings versions, and source location. The existing browser evidence case now requests the file through the link and checks its schema and clean build.

Delivery repair from the TRACKER completion audit: footer link, generated provenance and an assertion in the existing browser case. Existing useful task, Rust/worker code, data, licenses, dependencies and authority boundaries are unchanged. The new artifact is build metadata, with no local absolute path or credentials.

Validation: git diff --check; existing hosted native/build/browser gates on PR and default branch; exact artifact/live file comparison; fresh UI link, anonymous file and keyboard/mobile checks before TRACKER snapshot. No new Rust build result is claimed from the initial audit.

Implementing-agent review: source custody distinguishes compiled WASM from Rust source; scope review admits no new domain or fiscal claim. This is a link repair, not a research or corridor promotion. Release receipts belong to TRACKER context/waves/2026-10-30-wasm-self-service-audit/.
