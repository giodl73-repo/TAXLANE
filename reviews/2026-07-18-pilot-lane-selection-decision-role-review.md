# Role Review: Pilot Lane Selection Decision

## Scope

This review applies the eight-role panel to:

- `data/derived/breadth_benchmark_matrix/pilot_lane_selection_decision.v1.draft.json`
- `docs/reading/pilot-lane-selection-decision.md`

## Decision

Approved to select transportation asset maintenance and safety as the first
pilot for scaffold work only.

Not approved for simulator execution, target costs, rates, public rate cards,
tax proposals, savings estimates, waste findings, fraud findings, department
cuts, technology-savings claims, outcome-floor thresholds, solver results, or
balanced-budget claims.

## Role findings

| Role | Result | Finding |
|---|---|---|
| T-1 Taxpayer Advocate | Pass with guardrail | Selection does not publish a rate, burden, or taxpayer allocation and keeps fairness analysis blocked. |
| T-2 Budget Accountant | Pass with guardrail | Transportation trust-fund, general-fund, state/local, offset, and reconciliation boundaries remain explicit. |
| T-3 Source Custodian | Pass with P1 blocker | Future simulator inputs require official source custody, matched period, byte count, SHA-256, and metadata. No new source is captured here. |
| T-4 Public Goods Steward | Pass with guardrail | Maintenance, safety, access, resilience, project delivery, and public-good service floors remain separate from rate claims. |
| T-5 Program Beneficiary | Pass with P1 blocker | No lower-cost recognition is allowed until access, quality/safety, equity, adequacy/resilience, delivery feasibility, and lane-specific floors pass. |
| T-6 Compliance Burden | Pass with guardrail | Selection does not create a filing, withholding, fee, user-charge, employer, taxpayer, or agency compliance burden. |
| T-7 Fiscal Sustainability | Pass with P1 blocker | Selection does not alter fund balances, reserves, debt, net interest, deficit gap, or balanced-budget status. |
| T-8 Reform Skeptic | Pass with guardrail | The selection is narrow and falsifiable; it avoids proof rhetoric, savings language, waste language, and technology-cut shortcuts. |

## Stop conditions preserved

- No normative target is chosen.
- No tax distribution is chosen.
- No outcome-floor threshold is chosen.
- No causal evidence is interpreted.
- No fiscal identity conflict is resolved.
- No public savings, efficiency, balanced-budget, or statutory-rate claim is
  made.

## P1 blockers before simulator execution

- Official source custody.
- Current-law baseline path.
- Floor indicator contract and later floor thresholds.
- Modernization and stress path contracts.
- Transition costs and measured productivity.
- Federal/state/local translation.
- Fund, reserve, emergency, offset, debt, and net-interest reconciliation.
