# EXPL-A claim ledger

This ledger controls the claims that may be reused by later explanation forms.
“Allowed” means supported for the stated Taxlane context, not universally true.

| ID | Class | Canonical claim | Evidence | Allowed shorthand | Prohibited drift |
|---|---|---|---|---|---|
| CLM-01 | model result | Fifteen canonical tracks have reviewed internal terminal dispositions. | `fifteen_track_terminal_disposition.v1.draft.json` | “The fifteen-track internal analysis is complete.” | “All federal policy questions are solved.” |
| CLM-02 | accounting result | Admitted FY2026 primary spending reduction is $0.000B. | terminal disposition; targeted spending decision | “No tested spending amount entered the package.” | “There is no waste” or “cuts are impossible.” |
| CLM-03 | model input | The frozen remaining FY2026 revenue target is $813.727B. | rate completion; PAY–NET–REV reconciliation | “Taxlane retained its $813.727B target.” | “The government officially needs exactly this tax increase.” |
| CLM-04 | recommendation | The preferred central schedule is 21/23/33/35/43/46/48%. | rate completion | “Taxlane's preferred central analytical schedule.” | “The new tax brackets” or “what everyone will pay.” |
| CLM-05 | uncertainty | The behavior-robust contingency is 22/24/34/36/44/47/49%. | rate completion | “The smallest tested schedule covering all three behavior cases without macro stress.” | “The guaranteed-safe schedule.” |
| CLM-06 | stress result | The +12.6-point schedule is the first tested schedule covering all nine deliberately adverse combined cases. | rate completion | “Internal severe-stress ceiling.” | “Forecast,” “recommendation,” or “required law.” |
| CLM-07 | evidence decision | HLT and DEF each retained six unresolved gates and admitted no FY2026 savings. | targeted spending decision | “The two targeted candidates failed the full admission gate.” | Counting $0.4B or $15B as package savings. |
| CLM-08 | accounting boundary | PAY is non-additive, OAS is separate, and NET is endogenous. | terminal disposition; reconciliation | Use the full three-part statement. | Adding PAY to program savings, mixing OAS payroll financing into ordinary-income revenue, or cutting NET directly. |
| CLM-09 | interpretation | Reviewed zero admission is a completed analytical result under current evidence. | terminal schema and review | “Zero is a result when evidence gates are not met.” | “The candidate was proven harmful” or “permanently rejected.” |
| CLM-10 | scope boundary | Taxlane is independent internal analysis with no external release or official request authorized. | explanation program; rate completion | “Repository-only work.” | Implying government, academic, institutional, or public endorsement. |
| CLM-11 | model boundary | $813.727B is Taxlane's frozen FY2026 ordinary-income rate-model target, not total receipts, total outlays, a trust-fund requirement, or an official federal target. | rate completion; PAY–NET–REV reconciliation | Use the complete boundary. | “The federal government needs exactly $813.727B.” |
| CLM-12 | limitation | The one-year result does not establish ten-year balance or long-run solvency, and compliance/transition burdens remain incomplete. | rate completion; terminal disposition | “One-year model result with stated limitations.” | “Balanced plan” or “implementation-ready tax system.” |
| CLM-13 | authority boundary | Taxlane cannot change statutes, appropriations, tax bases, or program rules. | explanation program | “Analytical recommendation only.” | “Taxlane sets” or “the policy will.” |

## Claim-label vocabulary

- **Sourced fact:** directly reported by a named, versioned source.
- **Derived accounting result:** recomputed from admitted, traceable inputs.
- **Model result:** output of Taxlane's stated model and assumptions.
- **Interpretation:** Taxlane's reasoned reading of the evidence.
- **Recommendation:** a normative choice made by Taxlane.
- **Uncertainty or stress:** a sensitivity case, not a forecast.
- **Blocked claim:** a statement Taxlane does not have authority or evidence to
  make.

Every downstream headline must carry one of these labels in metadata, nearby
copy, speaker notes, or an accessible disclosure.
