# A citizen's guide to the Taxlane result

## 1. The question

Federal budget discussions often begin with a promised saving and then subtract
it immediately from the amount that must be financed. Taxlane reverses that
order. It asks whether a proposed spending change has enough evidence to enter
the accounting at all.

Taxlane separated the federal fiscal picture into fifteen tracks because health
care, transportation, Social Security, defense, disaster response, courts,
education, and interest on debt do not share one financing method or one test of
success. The tracks make public purposes visible without pretending that every
tax dollar is legally tagged to a program.

The categories are not one moral ranking. Public goods can benefit broad
communities; transfers support eligible people; administration and enforcement
operate the legal system; debt service honors past financing commitments. Each
raises different questions about beneficiaries, continuity, and funding.

## 2. The short result

All fifteen tracks reached an internal terminal disposition. Ten closed at
reviewed zero admission. The other five retained special typed or accounting
roles: transportation's conditional cost note, Social Security's dedicated
solvency rail, payment integrity's non-additive overlay, revenue's internal
planning rate card, and net interest's endogenous result.

No tested candidate supplied an admissible FY2026 primary spending reduction.
The remaining model revenue target therefore stayed $813.727 billion.

## 3. Why “zero” is a result

A candidate can have a published headline estimate and still fail Taxlane's
decision rule. Before counting savings, Taxlane asks:

1. Is the source in custody and correctly scoped?
2. Does the estimate match current law and the relevant year?
3. Is there an executable implementation path?
4. Are outlays, receipts, trust funds, and other financing rails separated?
5. Are interactions and double counting resolved?
6. Are distribution and compliance burdens visible?
7. Do access, quality, safety, service-continuity, and other applicable outcome
   floors pass?

The targeted health and defense candidates illustrate the rule. Their headline
FY2026 contexts were $0.4 billion and $15.0 billion, respectively, but each
retained six unresolved gates. Taxlane admitted $0 from both. The headline
amounts remain context; they do not become package savings.

That is not a declaration that the candidates are bad, that their beneficiaries
are beyond review, or that current programs are optimal. It is a refusal to
spend an uncertain saving on paper before its consequences and implementation
are understood.

## 4. The accounting rails

The $813.727 billion figure is Taxlane's frozen FY2026 ordinary-income rate-
model target after admitted package effects. It is not total federal spending,
total receipts, an official deficit estimate, or one taxpayer's allocation.

- **Social Security (OAS):** dedicated payroll-tax and trust-fund questions stay
  separate from the ordinary-income model.
- **Payment integrity (PAY):** measurement and control opportunities are not
  additive savings until a program owner has a realized, scored effect.
- **Net interest (NET):** interest changes are recomputed from admitted upstream
  borrowing paths; interest is not cut directly by assumption.
- **Revenue (REV):** receives the remaining target after those boundaries are
  respected.

This is why simply adding every promising number would produce a misleading
result.

## 5. What the rates mean

Taxlane tested uniform increases to a model schedule. Its preferred central
schedule is 21/23/33/35/43/46/48 percent.

The experiment used Tax-Calculator 6.5.1 with its bundled CPS tax-unit file for
tax year 2026. “Preferred” reflects Taxlane's chosen objective and decision
rule; it is a recommendation, not a value-free fact dictated by software.

These are marginal bracket rates. Imagine a hypothetical threshold: a higher
rate above it would apply only to taxable income above the threshold, not to all
income. An effective rate instead compares total tax with a broader income
amount. Taxlane's schedule alone cannot tell anyone what they would owe.

The model also does not supply the full legal design—thresholds, filing status,
deductions, credits, withholding, refunds, payroll taxes, transition rules, and
many administrative details would matter.

## 6. Why there are three tiers

The central schedule is a recommendation under the selected behavioral case,
not a guarantee.

| Tier | Schedule | Meaning |
|---|---|---|
| Preferred central | 21/23/33/35/43/46/48 | smallest tested one-decimal uplift clearing Taxlane's central case after its administration ceiling |
| Behavior-robust contingency | 22/24/34/36/44/47/49 | smallest tested schedule covering all three taxable-income response cases without added macro stress |
| Severe stress ceiling | 22.6/24.6/34.6/36.6/44.6/47.6/49.6 | first tested schedule covering all nine deliberately adverse internal combined cases; not the baseline recommendation or a forecast |

Keeping the tiers separate prevents uncertainty from being hidden inside one
apparently precise answer.

## 7. What could change the result?

Taxlane is complete for the evidence it currently holds, not frozen forever.
A track reopens when its recorded trigger occurs—for example, a current-law
score closes missing access and distribution evidence, an enacted program
changes implementation, an owner-attributed payment-integrity effect becomes
available, or an admitted upstream debt path requires interest recomputation.

A new fiscal target, model version, behavioral assumption, or policy objective
could also reopen the revenue analysis.

## 8. Public purpose and limits

Taxes can finance broad public goods, transfers, administration, enforcement,
and debt service. Fees, premiums, fines, payroll contributions, general revenue,
and borrowing are not interchangeable. Taxlane's goal is legibility: make the
purpose, financing relationship, and claim status visible while preserving the
government's real budget process and money's fungibility.

The analysis remains one-year and internal. It does not establish ten-year
balance, total deficits/debt, or long-run solvency. Federal borrowing remains a
separate part of the wider fiscal picture. Taxpayer, employer, preparer, avoidance,
enforcement, and transition burdens remain incomplete. Taxlane cannot change
law, appropriations, tax bases, eligibility, or implementation.

## 9. How to audit it

- Read the track endpoint:
  `docs/reading/fifteen-track-terminal-disposition.md`.
- Read the rate result:
  `docs/reading/rev-internal-rate-analysis-completion.md`.
- Inspect the controlled claims and numbers:
  `docs/explanation/foundation/claim-ledger.md` and `number-ledger.md`.
- Run the repository validator:
  `cargo run -p taxlane-tools -- income-tax-outlay validate`.

> This guide explains a Taxlane internal model result and recommendation. It is
> not enacted law, an official score, personal tax advice, formal balance
> certification, institutional endorsement, or authorization for external
> release.

Machine evidence: `fifteen_track_terminal_disposition.v1.draft.json`,
`targeted_spending_rate_decision.v1.draft.json`, and
`rev_internal_rate_analysis_completion.v1.draft.json` under
`data/derived/breadth_benchmark_matrix/`.
