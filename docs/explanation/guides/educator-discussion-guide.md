# Educator discussion guide: understanding the Taxlane result

## Use and boundary

This repository-contained guide supports civic discussion of evidence,
accounting, uncertainty, and public purpose. It does not ask participants to
endorse Taxlane's rate recommendation and does not provide tax advice. The
`.roles` reviewers are AI-simulated lenses, not external experts.

## Learning objectives

Participants should be able to:

1. distinguish a headline estimate from an admitted fiscal effect;
2. explain why reviewed zero admission can be a completed result;
3. separate general revenue, dedicated finance, borrowing, payment integrity,
   and net interest;
4. distinguish marginal bracket rates from effective rates and liability;
5. compare the central, contingency, and stress tiers without treating any as
   law or an official score;
6. identify evidence that could reopen a track.

## Suggested 60-minute sequence

| Minutes | Activity | Source |
|---:|---|---|
| 0–8 | Read the one-page result and list every claim label participants notice. | `one-page-result.md` |
| 8–18 | Work through the marginal-rate example; contrast bracket and effective rate. | citizen guide §5; glossary |
| 18–30 | Use HLT and DEF to map headline context through an evidence gate. | citizen guide §3; targeted decision |
| 30–42 | Assign small groups OAS, PAY, REV, and NET; ask why their amounts cannot be added. | fifteen-track guide |
| 42–52 | Compare the three rate tiers and identify assumptions, not preferences. | citizen guide §6 |
| 52–60 | Ask what evidence would change the result and what claims remain prohibited. | terminal triggers; FAQ |

## Discussion prompts

- What is gained and lost by requiring every service floor to pass before
  savings enter a model?
- When does caution protect beneficiaries, and when might it preserve a poor
  status quo?
- Why can payment-error measurement differ from cash savings?
- What does general-revenue flexibility accomplish that dedicated lanes cannot?
- Why might a clear taxpayer receipt still leave the tax return unchanged?
- Which assumptions in the central result are empirical, and which are
  normative choices?
- Who has legal authority to change a rate, base, appropriation, eligibility
  rule, or implementation plan—and what can Taxlane itself not change?
- How should uncertainty be shown without presenting three equal
  recommendations?

## Misconception checks

| Misconception | Correction |
|---|---|
| “The highest bracket rate applies to all income.” | A marginal rate applies only within its bracket. |
| “Zero admission means no inefficiency exists.” | It means no tested amount passed the stated gates. |
| “A published estimate is automatically available to finance another policy.” | Scope, current law, implementation, interactions, and outcomes must match. |
| “Trust-fund receipts and general income taxes are interchangeable labels.” | They have different legal and accounting rails even though federal finance remains interconnected. |
| “Lower debt-service cost can be declared directly.” | Taxlane derives NET from admitted upstream borrowing paths. |
| “AI-simulated review means experts endorsed the result.” | The roles are internal structured review lenses only. |

## Evidence exercise

Choose one track. Identify its public purpose, disposition, admitted effect,
evidence source, and reopening trigger. Then write one supported sentence and
one tempting but prohibited sentence. Compare both with the EXPL-A claim ledger.

## Assessment rubric

- **Complete:** preserves year/unit, model/law, marginal/effective,
  additive/non-additive, and internal/official distinctions.
- **Developing:** reaches the main conclusion but drops one boundary.
- **Needs revision:** converts headline context into savings, calls the schedule
  law or personal liability, or implies external endorsement.

## Facilitator caution

Do not solicit personal income or tax information. Use hypothetical examples
without dollar thresholds. Present value disagreements as discussion questions,
not as evidence failures, while keeping factual and accounting errors distinct.

Evidence routes: `fifteen_track_terminal_disposition.v1.draft.json`,
`targeted_spending_rate_decision.v1.draft.json`, and
`rev_internal_rate_analysis_completion.v1.draft.json` under
`data/derived/breadth_benchmark_matrix/`. The rate experiment used
Tax-Calculator 6.5.1, bundled CPS tax-unit data, and tax year 2026.
