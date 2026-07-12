# Current Versus Benchmark Scoreboard

## Headline

TAXLANE now has a sourced current-state top line for every question in the
breadth matrix: **17 questions across 13 policy lanes, with zero open breadth
gaps**. Five questions (29.4%) have a matched benchmark; 12 (70.6%) have a
federal top line but still need a scope- and outcome-matched expected value.

This closes breadth, not the argument. The next phase is depth: explain the
largest observed differences, test plausible causes, attach outcomes, and only
then estimate any addressable opportunity.

Machine rows:
`data/derived/breadth_benchmark_matrix/breadth_benchmark_matrix.v1.draft.jsonl`.

## Comparable Top Lines

| Metric | Current | Benchmark | Reading |
|---|---:|---:|---|
| Total health spending | 17.2% GDP | 9.3% OECD average | Large observed cost gap; not a fraud or automatically recoverable-savings estimate. |
| Public pensions | ~7.3% GDP | ~8.1% peer norm | Spending is near the peer norm; financing-base and solvency questions remain separate. |
| National defense | ~3.0% GDP | 2.0–3.5% strategic band | Inside a policy band; the benchmark is strategic, not statistically expected spending. |
| Public family support | ~1.1% GDP | ~2.2% peer norm | Below the peer comparison on the matched paper basis. |
| All-government tax revenue | 26.0% GDP | 34.1% OECD average | Below the average; the average is context, not a mandatory target. |

## Federal Top Lines Without A Matched Expected Value

| Metric | Current | Why no single expected value yet |
|---|---:|---|
| Medicare plus health-care-services outlays | 27.38% of federal outlays | OECD total-health measures include different financing and government scopes. |
| Borrowed share | 25.31% of federal outlays | The appropriate path depends on the cycle, primary balance, debt stock, rates, and policy goals. |
| Gross Treasury interest | 17.34% of federal outlays | Gross and net interest, debt structure, and GDP paths must be kept separate. |
| Complete Veterans Benefits and Services | 5.38% of federal outlays | No outcome- and eligibility-matched comparison is attached; the earlier 4.98% subtotal covered only income security plus medical care. |
| Complete federal Transportation function | 2.07% of federal outlays | Federal-only spending omits major state/local capital and maintenance; 1.44% is ground transportation alone. |
| Complete federal Education, Training, Employment, and Social Services | 1.03% of federal outlays | Includes a −$35.005B higher-education net entry and excludes most state/local education spending; 0.98% is the school subfunction alone. |
| Disaster relief and insurance subfunction | 0.90% of federal outlays | Event incidence, exposure, mitigation, and supplemental funding vary sharply; do not relabel the broader parent function as disaster spending. |
| Justice Administration | 1.19% of federal outlays | State/local spending and matched safety, access, timeliness, corrections, and due-process outcomes are not yet attached. |
| Science, energy, environment, and natural resources | 2.18% of federal outlays | This is a disclosed composition of three OMB functions, not one official function or one benchmarkable outcome. |
| Agriculture | 0.68% of federal outlays | Program accounts and farm, acre, risk, productivity, and conservation denominators still need reconciliation. |
| International Affairs | 0.64% of federal outlays | The negative international-financial-program entry is net financing/accounting and must be bridged before component comparisons. |

## Breadth Closure And Depth Queue

All 17 matrix questions now meet at least Tier 2: a sourced current value with
its scope, period, unit, and interpretation boundary. No question remains a
Tier 3 breadth gap. The 12 Tier 2 questions are not “expected-value ready.”

The prioritized depth queue is:

1. **Health cost decomposition:** quantify price, utilization, administration,
   coverage, case mix, and outcomes behind the 17.2% versus 9.3% GDP comparison.
   The first diagnostic decomposition is now available in
   `docs/reading/health-cost-decomposition.md`; service-level scoring remains open.
2. **Fiscal-path scenarios:** connect borrowing, primary balance, net interest,
   debt stock, rates, and revenue bases under explicit current-policy and
   stabilization assumptions.
3. **Payment-integrity bridge:** separate reported improper payments into
   overpayments, underpayments, unknowns, documentation errors, confirmed
   fraud, recoveries, and preventable future loss by program.
4. **Benefit-and-outcome denominators:** deepen Social Security, family support,
   veterans, education, and justice with eligible population, service use,
   timeliness, accuracy, access, and outcome measures.
5. **Federalism and investment outcomes:** reconcile federal, state, and local
   scope for transportation, education, justice, disaster, agriculture, and
   environmental investment before peer ranking.
6. **Accounting bridges:** explain negative or net entries in higher education
   and international financial programs, and keep gross and net interest
   separate.

Do not add the displayed federal percentages into a “share of government
covered.” Some rows use complete OMB functions, others use subfunctions or
composed orientations, and total positive functions can exceed net federal
outlays because undistributed offsetting receipts reconcile the budget. The
defensible closure statistic is question coverage (17 of 17), not a summed
outlay percentage.

## Fraud And Savings Firewall

The current government-wide integrity top line is **$161.5 billion in FY2024
reported improper payments across covered programs**. It is not the full federal
payment universe, not a fraud estimate, and not an automatically recoverable
amount.

The matrix enforces:

```text
international efficiency gap != improper payments != fraud != recoverable savings
```

World comparisons can identify an efficiency question. Fraud requires evidence
of willful misrepresentation at the relevant program or transaction grain.
Recoverable savings requires a separate reviewed estimate that accounts for
recoverability, control cost, behavioral response, access, due process, and
outcome floors.

## Comparability Grades

| Grade | Meaning |
|---|---|
| A | Same concept, unit, and period on an authoritative harmonized source. |
| B | Useful comparison with a disclosed scope, year, or definition adjustment. |
| C | Directional only; do not headline without additional reconciliation. |
| Not scored | No matched benchmark is claimed. |

