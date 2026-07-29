# taxlane-tools layout (ROUTE-aligned)

## Goal

`main.rs` is a **thin argv dispatcher**. Validators, builders, path constants,
and types live in domain modules.

TAXLANE is the portfolio **funding / investment-accounting** special case: the
same layout discipline as ROUTE, with domains named for fiscal rails and tracks
rather than highway tiers.

| Layer | Owns | Does not own |
|-------|------|----------------|
| `main.rs` | argv match → `run_*` | validators, path tables |
| `commands.rs` | `run_*` entrypoints (`ExitCode`) | deep validation bodies |
| `artifacts.rs` | path consts + `ARTIFACTS` ledger | business rules |
| `types.rs` | shared structs/enums + impls | I/O orchestration |
| `support/build.rs` | `build_*` writers/check builders | CLI argv |
| `support/check.rs` | `check_*` claim/intake checks | CLI argv |
| `support/util.rs` | `repo_root`, JSON helpers | domain policy |
| `support/misc.rs` | uncategorized helpers | new default home |
| `support/validate/*` | `validate_*` by track/prefix | CLI argv |

## Command contract

```text
taxlane-tools <area> <command> [--check]
```

`main` only matches argv and calls `run_*`. Prefer adding a new `run_*` +
`validate_*` / `build_*` in the right support module over growing `main`.

## Snapshot

- Split from a ~100k-line godfile into domain modules (ROUTE method).
- `support/validate/` buckets by first path token after `validate_`
  (`health`, `medicare`, `transportation`, `rev`, …); tiny buckets merge into
  `validate/misc`.
- Helper script: `tools/split_main.py` (one-shot; do not re-run on a split tree).

## Non-goals

- Clap migration in the same change
- Splitting `taxlane-core` in the same change
- Changing validator semantics or external claim posture
