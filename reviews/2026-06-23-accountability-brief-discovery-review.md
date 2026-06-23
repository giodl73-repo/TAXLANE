# Accountability Brief Discovery Review

## Scope

This review covers discovery links for the reader-facing accountability public
brief:

| Artifact | Role |
|---|---|
| `README.md` | Top-level reader entry point. |
| `docs/reading/README.md` | Reading packet index and reading order. |
| `tools/taxlane/src/main.rs` | Discovery validation. |

## Decision

Accept the discovery update.

The accountability public brief is now linked from the top-level README and
the reading packet index. Validation checks those links so the brief does not
become an orphan artifact.

## Role Findings

| Role | Finding |
|---|---|
| Taxpayer Advocate | Pass: readers can find the current accountability handoff from normal entry points. |
| Reform Skeptic | Pass: the index guardrail repeats that accountability packets are question surfaces, not findings. |
| Maintainer | Pass: validation checks README and reading-index links. |

## Guardrails

- Keep the public brief discoverable before adding any UI surface.
- Keep the reading-index guardrail explicit: draft accountability records are
  not fraud, waste, abuse, or performance findings.
