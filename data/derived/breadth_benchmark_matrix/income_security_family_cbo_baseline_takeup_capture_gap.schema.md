# Income-security/family CBO baseline/take-up capture gap schema

`income_security_family_cbo_baseline_takeup_capture_gap.v1.draft.json` records a
blocked CBO selected-program capture attempt for income-security/family
baseline and take-up context.

Required shape:

- `record_family` is `income_security_family_cbo_baseline_takeup_capture_gap`.
- `source_discovery` names official CBO selected-program and open-data sources.
- `capture_attempts` records failed automated capture without treating challenge
  HTML as raw source custody.
- `next_manual_capture_requirements` states the exact manual custody fields that
  must be recorded before any CBO values may be populated.
- `readiness_status` keeps CBO SNAP custody, baseline values, take-up context,
  CBO/take-up gate readiness, source-capture completion, and solver readiness
  false.
- `blocked_outputs` must stay null.
- Only `claim_booleans.cbo_capture_gap_published` may be true.

This record is not CBO source custody, not CBO baseline values, not take-up
context, not a benefit package model, not a take-up model, not floor values, not
federal/state/local translation, not solver input, not rates, not savings, and
not balanced-budget readiness.
