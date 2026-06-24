# Pulse 07: Balance-Rule Guardrail Spec

## Goal

Answer the panel's biggest blocking objection (Reform Skeptic P1: a balance rule
and lane rates are transparency, not control) by specifying the guardrails that
make "spend only what we collect" enforceable rather than theater.

## Changes

- Added `docs/research/2026-06-23-balance-rule-guardrail-spec.md`:
  - Precise rule statement with four loophole-closing requirements (accrual +
    multi-year true-up; per-fund balance; net interest senior; over-the-cycle
    reserve).
  - Six required statutory lane fields (appropriation/shortfall/surplus/reserve/
    override rules + deficit_gap display).
  - Evasion-to-guardrail map covering all ten Reform Skeptic evasions
    (E1-E10).
  - Institutional triggers: independent estimator, automatic actual-receipts
    sequester, override transparency.

## Boundaries kept

- Reform-design, not statutory text; subject to role review before advocacy.
- Does not claim lane rates alone improve discipline; control lives in the rules.
- Always-visible `deficit_gap` line and actual-receipts sequester treated as
  non-negotiable.

## Validation

- `git diff --check`.

## Status

Done.
