# Panel Review: Operating System (Pressure-Test)

## What was reviewed

The program-lane operating system before any public use: the rate-adjustment
operating model, the citizen explainer (`program-lane-system.md`), the rate cards,
and the worked examples. All eight `.roles` lenses read the actual artifacts and
returned severity-tagged findings.

## Findings by role

| Role | Decision | Top finding |
|---|---|---|
| Taxpayer Advocate | Allocation claim unclear | "Funded by: income tax" on general-revenue lanes overstates tracing; relabel "general revenue". |
| Budget Accountant | Needs allocation label | **Lane shares sum to 103¢, not 100¢** (offset lanes dropped); defense ¢-of-dollar conflated with %-of-GDP. |
| Source Custodian | Needs primary source | **Federal vs all-government scope mix** (25.6% all-gov used as federal; health 14.3% includes compulsory private); no inline source IDs; TY2022 base on FY2025 lanes unlabeled. |
| Public Goods Steward | Funding-mode tradeoff missing | Public-good vs transfer vs financing not surfaced (~⅓ of the dollar is transfers); corporate→cluster conflates benefit vs volatility rationale. |
| Program Beneficiary | Shortfall rule missing | **"Coverage protected" is an adjective, not a rule**; no coverage floor, no efficiency verification, no distributional field. |
| Compliance Burden | Employer-admin impact unclear | **Missing firewall**: "a lane is a line on the receipt, never a separately-filed/withheld tax"; payroll rate changes need a tax-year boundary. |
| Fiscal Sustainability | Debt-service treatment missing | **Operating model dropped the spec's over-the-cycle reserve and net-interest seniority** → implies pro-cyclical austerity; SS shown as a one-year fix. |
| Reform Skeptic | Fungibility caveat missing | **E11 rolling-payback / suspendable sequester**; independent estimator asserted not defended; "you can balance the budget" overclaims feasibility. |

## Consolidated fix list (applied in Pulse 02)

**Arithmetic / sourcing (hard errors):**
1. Show the offset line so lane shares net to 100¢; state the denominator (share of $7.01T outlays).
2. Label every ratio by scope: federal (OMB ~17.3% receipts / 23.1% outlays of GDP) vs all-government (OECD 25.6%); health 14.3% = government/compulsory incl. private.
3. Make the defense baseline consistent (3.0% of GDP, FY2025 function 050) and the Hague target consistent (3.5% core).
4. Label the TY2022 income base / FY2025 lane mapping as a cross-year illustration.

**Operating-model hardening (design gaps):**
5. Inherit the spec: add the over-the-cycle reserve path (shortfalls draw the reserve before firing rate/benefit sequesters) and net-interest seniority (first claim, sequester-exempt) into the operating model.
6. Make "coverage protected" a binding rule: a coverage/enrollment floor plus independent verification that an efficiency gain held outcomes constant.
7. Add the receipt firewall: a lane never acquires its own filing/base/withholding; rate changes affecting payroll take effect only at a tax-year boundary with notice.
8. Harden the two load-bearing nodes: specify the independent estimator (anti-capture) and cap payback deferrals (close E11).
9. Add `distributional_impact` and `compliance_impact` fields to the `lane_rate_change` record.

**Framing (overclaim):**
10. Demote "core finding" — the gap is *arithmetically closable*; closing it is a political choice. Relabel section 4 as modeled attribution. Surface obligation type (public good / transfer / financing) and make the corporate→cluster rationale an argued choice naming dedication's cost (rigidity).

## What the panel affirmed

- The borrowed-share line is recomputed on every change and never hidden.
- Per-fund balancing and legal-vs-modeled dedication are handled correctly.
- The guardrail spec's evasion map and explicit non-claims are strong.
- The Defense worked example ("named, not borrowed") and the Social Security
  cap example (protect benefit, name incidence) are the model done right.
