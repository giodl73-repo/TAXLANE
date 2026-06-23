# Accountability Core Workflow Review

## Scope

This review covers the Rust crate boundary for accountability workflow helpers:

| Artifact | Role |
|---|---|
| `crates/taxlane-core/src/lib.rs` | Core accountability record workflow helpers and tests. |
| `tools/taxlane/src/main.rs` | Report generation that calls the core helpers. |

## Decision

Accept the workflow helpers in `taxlane-core`.

The core crate now owns next-action, demand-question, and public-use-blocker
wording for accountability evidence records. The CLI remains responsible for
file IO and Markdown generation.

## Role Findings

| Role | Finding |
|---|---|
| Maintainer | Pass: shared workflow rules now live beside readiness classification and are covered by unit tests. |
| Reform Skeptic | Pass: helper wording preserves non-allegation and non-score boundaries. |
| Taxpayer Advocate | Pass: demand questions remain plain enough for public accountability use. |
| Source Custodian | Pass: helper wording still assumes records are validated and source-custodied before reporting. |

## Guardrails

- Do not move file IO or report rendering into `taxlane-core`.
- Do not add fraud scoring helpers without source, review, and due-process requirements.
- Keep future UI/API surfaces calling the core helpers instead of duplicating wording rules.
