# Pulse 01: Core Workflow Helpers

## Goal

Move accountability workflow wording from the CLI into `taxlane-core`.

## Work

- Add core methods for next action, demand question, and public-use blocker.
- Add unit tests for role-review and missing-evidence paths.
- Update report generation to call the core methods.
- Add review and VTRACE closeout for WP-TAX-012.

## Result

Done. The CLI still owns repository IO and Markdown rendering, while
`taxlane-core` owns reusable accountability workflow policy.
