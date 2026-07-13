# Pulse 22: Pell Current-Entrant Persistence Baseline

## Result

Captured the exact official NCES DataLab retrieval response for BPS:20/22
query `396385`, retrieval code `zclxfu`, and froze a standalone descriptive
baseline crossing entry-year `PELL20` receipt amount with the five-category
`PROUT3_NEW` three-year attainment and persistence outcome. The custody object
preserves the saved specification, weighted percentage estimates, BRR standard
errors, relative standard errors, weighted counts, confidence bounds, filters,
suppression state, and retrieval identity.

The cross-tab closes the narrow source-capture gate from Pulse 21. It does not
close causal, mature-outcome, cost, or fiscal gates.

## Decision gate

Pass for an official national survey-weighted descriptive comparison of
students with positive versus zero recorded Pell amount in academic year
2019-20. Fail for Pell eligibility, a Pell program effect, a randomized or
adjusted counterfactual, mature completion, permanent dropout, incremental
program cost, compatible outlay allocation, fiscal return, fraud, improper
payment, recovery, or savings.

The `PROUT3_NEW` result has five categories. It combines all enrolled students
without a degree and must not be presented as the six-category Table A-1
distribution that separates 4-year and less-than-4-year enrollment.

## Custody and reproducibility

- Source ID: `SRC-NCES-DATALAB-BPS20-22-PELL-PERSISTENCE-2026`
- Dataset ID: `168`
- Collection period: `2020/2022`
- Weight: `WTA000`
- Variance method: `BRR`
- Universe: `All respondents.`
- Filters: none
- Suppressions: none
- Query ID: `396385`
- Retrieval code: `zclxfu`
- Public URL: <https://nces.ed.gov/datalab/powerstats/table/zclxfu>
- API URL: <https://nces.ed.gov/datalab/api/v1/workspace/retrieve/zclxfu>
- Raw bytes: `8023`
- SHA-256: `AEDC7781DDC8DA4A9F59942E16B398F58CFFB20128CC6FE44CF24D6F04795DC5`

## Integration status

Shared integration complete. The standalone baseline remains the
canonical home for the full DataLab result arrays and now links through the
Pulse 21 BPS bridge, education depth card, higher-education account bridge,
FY2024 FSA access baseline, experimental-Pell evidence, and B&B
bachelor-completer baseline. The education breadth row, public scoreboard,
readers, WAVE, source ledger, Rust catalog and validator, and generated manifest
carry the same descriptive boundary. The validator also reconciles the derived
arrays against the nested specification and result in the raw NCES response.

## Next bounded action

Capture and review BPS:20/25 before treating these outcomes as mature. Run and
preserve a covariance-aware group-difference test before making significance
claims; separate cell standard errors are insufficient. Retain the
receipt-not-eligibility, five-versus-six-category, observational,
pandemic-era, noncausal, cost, fiscal, fraud, and savings boundaries.
