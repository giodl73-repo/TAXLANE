# Role Review: Public Thesis Packet

## Scope

This review applies the eight-role panel to:

- `data/derived/breadth_benchmark_matrix/public_thesis_packet.v1.draft.json`
- `docs/reading/public-thesis-packet.md`

## Decision

Approved as explanatory public-language design.

Not approved for statutory rates, effective rates, tax proposals, public rate
cards, savings estimates, waste findings, fraud findings, department cuts,
technology-savings claims, pilot selection, solver results, or balanced-budget
claims.

## Role findings

| Role | Result | Finding |
|---|---|---|
| T-1 Taxpayer Advocate | Pass with guardrail | Fairness is tied to burden, distribution, compliance, and service floors. |
| T-2 Budget Accountant | Pass with guardrail | Denominators, funds, offsets, deficit gap, and reconciliation boundaries remain explicit. |
| T-3 Source Custodian | Pass with P1 blocker | Future claims require custody, matched period, and evidence grade. |
| T-4 Public Goods Steward | Pass with guardrail | Public goods, transfers, financing costs, service commitments, and modernization costs remain distinct. |
| T-5 Program Beneficiary | Pass with P1 blocker | Lower-cost or lower-rate recognition remains blocked unless all required floors pass. |
| T-6 Compliance Burden | Pass with P1 blocker | Assigned-base rates remain blocked until taxpayer, employer, agency, filing, withholding, avoidance, and compliance effects are modeled. |
| T-7 Fiscal Sustainability | Pass with P1 blocker | Balanced-budget language remains blocked until endogenous interest, reserves, emergencies, macro feedback, interaction scoring, and the unrounded deficit gap reconcile. |
| T-8 Reform Skeptic | Pass with P1 blocker | The packet treats the thesis as auditable design, not proof of rates, savings, waste, or balance. |

## Required public-language cleanup

- Use "overspending risk" rather than unsupported "waste."
- Do not infer fraud from an improper-payment estimate, benchmark gap, or
  international comparison.
- Explain that technology is a transition path with implementation cost,
  training, cybersecurity, privacy, fallback operations, service risk, stress
  cases, measured productivity, and floors.
- Explain why some rates are blocked or not calculated.
- Explain that a balanced-budget claim remains blocked.

## P1 blockers

- Assigned-base models are missing.
- Behavior, incidence, distribution, administration, interaction scoring, and
  macro feedback are missing.
- Outcome floors are missing or false for future lower-cost scenarios.
- Pilot lane is not selected and the simulator has not run.
- Public rate cards have not passed role review.
- Fund, reserve, emergency, offset, endogenous-interest, and deficit-gap
  reconciliation is not complete.
