# Pulse 45: Medicare Part D Published Uncertainty-Output Component Closure

## Objective

Record the same-period Part D confidence-interval and annual margin-of-error
outputs without inferring their undisclosed construction or treating statistical
uncertainty as recoverable-dollar uncertainty.

## Evidence

The FY2024 findings publish a rounded $3.58 billion gross improper-payment
estimate at 3.70%. Table 1 reports a 95% dollar confidence interval of $3.19
billion to $4.01 billion and rate bounds of 3.31% to 4.15%.

PaymentAccuracy annual row 828 reports $3,575.09 million improper payments, a
0.037039355 rate, the confidence label `95% to <100%`, and a margin-of-error
value of 0.42.

## Decision

Close one internal component: the published 95% confidence interval and annual
margin-of-error output are identified at publisher-reported precision. Keep the
full `estimation method` field open.

The annual row does not disclose a unit or formula for 0.42 or its relationship
to the findings intervals. Do not infer units and do not force reconciliation.
The findings do not publish a separate interval for the net estimate.

## Residuals and guardrails

Still missing are official APR binary custody; estimator formula; sample weights,
strata, projection, and aggregation; PDE-to-beneficiary linkage; benefit-
parameter simulation; record and exclusion treatment; variance and finite-
population methods; confidence-limit construction; margin-of-error definition
and formula; unrounded values and rounding rules; and same-period beneficiary-
sample and extrapolation confirmation.

The confidence interval is statistical uncertainty around the gross result, not
a debt, collectibility, recovery, prevention, or savings range. Rounded outputs
cannot reconstruct estimator mechanics, and OMB-compliant or statistically
valid language remains an attestation rather than a reproducible method.

This pulse adds one component closure and zero full-field closures. Part D
remains three fields closed and five open. Every public, field-closure, scoring,
fraud, waste, debt, collectibility, recovery, prevention, and savings gate
remains false.
