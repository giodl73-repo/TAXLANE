# Pulse 02: Apply the Fixes

## Goal
Fix the defects the pressure-test found.

## Changes
- program-lane-system.md: lane table now nets to 100c (offset line + denominator stated) with an obligation-type column (public good / transfer / financing); scope labels on 25.6% (all-government) and 14.3% (government/compulsory incl. private); "core finding" demoted to "arithmetically closable, politically a choice"; corporate->cluster reframed as volatility-matching with rigidity cost named; funders relabeled "general revenue".
- 2026-06-24-rate-adjustment-operating-model.md: added Hardening Rules (over-the-cycle reserve before sequester; net interest senior/sequester-exempt + debt-service-growth trigger; coverage floor + outcome verification on efficiency moves; receipt firewall + tax-year-boundary withholding; estimator anti-capture + capped payback closing E11; multi-year horizon for structural drivers); added distributional_impact, compliance_impact, contestation fields to lane_rate_change.
- program-lane-rate-cards.md: AGI base labeled TY2022 cross-year; defense %GDP basis reconciled (3.0% OMB vs 3.2-3.4% SIPRI/NATO); Hague 3.5% core.
- rate-change-worked-examples.md: honesty caveats (efficiency not auto-realized; SS is a multi-year path; defense is the honest one).

## Validation
- `git diff --check`.

## Status
Done.
