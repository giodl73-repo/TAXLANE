# Pulse 43: Medicare Part D Payment-Universe Measurement-Object and Denominator Component Closure

## Objective

Determine whether same-period sources identify the Part D reviewed record,
its discrepancy cost basis, and the published denominator without treating that
narrow bridge as a complete enumeration of the Part D payment universe.

## Evidence

The CY2022 guide identifies sampled PDE records tied to sponsor contracts and
the final reconciliation PDE frame.

The FY2024 findings define a prescription discrepancy as a partial GDC error
based on corrected-versus-PDE GDC. GDC consists of ingredient cost paid,
dispensing fee paid, sales tax, and vaccine administration fee. CMS expressly
describes it as combined plan and beneficiary liability, of which government
subsidizes portions under statutory rules.

The findings publish a $96.52 billion `Part D Denominator`. PaymentAccuracy
FY2024 annual row 828 reports $96,521.39 million in outlays, which is
$96.52139 billion and rounds to the findings value.

## Decision

Close one internal component: the measurement object is the sampled PDE record
in the reconciliation frame; discrepancies use corrected-versus-PDE
partial GDC; and the published denominator is identified as $96.52 billion,
reconciling by rounding to row 828's $96,521.39 million outlays value.

Keep the full `payment universe` field open. The numeric bridge does not define
the denominator algebra, enumerate its streams, or establish that PDE-level GDC
equals federal outlays.

## Residuals and guardrails

Nine residuals remain:

1. formal definition and algebra for the $96.52 billion denominator;
2. complete included and excluded payment-stream taxonomy;
3. the relationship among PDE-level GDC, plan liability, beneficiary liability,
   government subsidies, federal payments, and reported outlays;
4. whether the denominator is federal outlays, GDC, or a modeled or simulated
   measure, including its period and settlement basis;
5. treatment of negative adjustments, reversals, post-reconciliation changes,
   duplicate records, rejected PDEs, and deleted PDEs;
6. included and excluded plan types, contracts, and beneficiary populations;
7. payment phases and subsidy components, with overlap and double-count controls;
8. gross/net, recovery, offset, and rounding rules; and
9. linkage among the PDE sample, beneficiary simulation, denominator categories,
   and statistical weights.

Direct subsidy or capitation, reinsurance, low-income cost-sharing subsidy, risk
sharing, premiums, beneficiary cost sharing, manufacturer discounts, direct and
indirect remuneration, and employer subsidy remain examples requiring explicit
source treatment, not asserted denominator members.

A PDE record does not itself prove the underlying individual pharmacy claim;
the PDE/GDC object is not the full payment universe; and GDC is combined plan
and beneficiary liability, not federal outlay. Government subsidizes only
portions. Do not add plan or
beneficiary liabilities to outlays without an accounting bridge and
double-count controls. Do not infer one-to-one PDE coverage of all denominator
streams.

This pulse adds one component closure and zero full-field closures. Part D
remains three fields closed and five open. Every public, field-closure, scoring,
fraud, waste, debt, collectibility, recovery, and savings gate remains false.
