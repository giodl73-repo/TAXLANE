# Wave 5 fiscal-control overlay depth packets

Machine record:
`data/derived/breadth_benchmark_matrix/wave5_fiscal_control_overlay_depth_packets.v1.draft.json`

Wave 5 covers the fiscal-control overlays: revenue-solvency, payment integrity,
and net interest.

Revenue-solvency is not a program-spending lane and is not additive to lane
costs. It is blocked from rates until matched receipt bases, legal perimeter,
economic perimeter, baseline amounts, elasticity, avoidance and compliance,
employer taxpayer and agency burden, distribution by income, tax interactions,
current-law yield, and reform yield are modeled. Statutory rates cannot be
published before matched receipt bases, behavior, incidence, distribution, and
administration are modeled. A value calculated after subtracting dedicated
receipts is not share of every tax dollar.

Payment integrity is a non-additive overlay. Improper-payment estimates are not
savings. No payment-integrity savings credit is allowed without causal
prevention or same-cohort collection lineage. Never infer fraud from an
improper-payment estimate. The operating boundary remains: improper payments !=
overpayments != confirmed fraud != recoverable dollars != collected recoveries
!= net savings.

Net interest is endogenous. Net interest cannot be cut directly. After any
primary-balance change, the solver must recompute deficit, debt, maturity-bucket
debt stock, and subsequent net interest before any solver output is eligible.
The net-interest artifact is a formula contract only; it is not a net-interest
path.

Trust funds remain separate. General-fund transfers must be explicit. Missing
values remain null and blocked gates remain false.

Technology changes are transition paths, not automatic savings. Technology may
support tax administration, payment review, claims timeliness, forecasting, or
debt-risk monitoring only after implementation cost, access, false-positive,
appeal, taxpayer burden, agency burden, incidence, distribution, and fiscal
formula effects are modeled.

This is not a solver run, not target-cost selection, not rate calculation, not a
public rate card, not a tax proposal, not a savings estimate, not a waste
finding, not a fraud finding, not a department-cut instruction, not a
technology-savings claim, and not a balanced-budget claim.
