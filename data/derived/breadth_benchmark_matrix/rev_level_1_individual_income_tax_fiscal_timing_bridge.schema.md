# REV Level-1 individual-income tax/fiscal timing bridge schema

This record completes `REV-L1-03` as a bounded reblock, not a numeric bridge.
It must name both tax-year return endpoints and the fiscal-year receipt endpoint,
enumerate every required collection/refund/cohort component, and keep all
unreconciled weights and matched-base outputs null.

Completion of the diagnostic may open `REV-L1-04`; it may not open candidate
selection, a solver run, a rate calculation, or a public rate card.
