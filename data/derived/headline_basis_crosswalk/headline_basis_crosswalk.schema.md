# Headline Basis Crosswalk Schema

Each JSONL row defines one valid headline measure and the measures it cannot
replace. The record fixes period, unit, government scope, accounting scope,
source, and intended use.

`headline_use` is one of `canonical`, `supporting`, or `comparison_context`.
Every row remains `not_interchangeable`; explicit incompatibilities live in
`cannot_substitute_for`.

Canonical means canonical for the named question, not universally canonical.
For example, net interest is canonical for the additive federal budget lane,
while gross Treasury interest is the supporting exposure measure.

