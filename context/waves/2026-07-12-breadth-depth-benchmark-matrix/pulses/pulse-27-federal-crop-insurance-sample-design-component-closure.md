# Pulse 27: Federal Crop Insurance Sample-Design Component Closure

## Result

Captured checksum-verified official custody of USDA OIG audit report
`50024-0016-11`. The independent FY2024 PIIA performance audit says auditors
reviewed USDA samples and underlying sampling methodologies for susceptible
programs, identifies FCIC in the tested Phase 2 and high-priority populations,
and reports FCIC compliant with all six PIIA criteria.

Combined with the existing official disclosures—326 policies from reinsurance
year 2022, AIP-aware selection, high/medium/low AIP tiers, and a statistically
valid designation—this closes only the disclosed sampling-governance/design
component internally.

## Decision Gate

Pass for narrow internal component closure covering the published count,
period, AIP-aware selection, tier labels, statistical-validity designation,
and independent audit review.

Fail for full `sample design` closure. Frame construction, tier allocation,
selection probabilities, randomization, replacement, nonresponse, weights,
the estimator, and variance method remain unpublished. Estimation method,
exclusion rules, and recoverable-savings basis also remain open.

Fail for program scoring or public claims about fraud, waste, debt,
collectibility, recovery, prevention, or savings.

## Governance Boundary

Other Matters item 3 on printed page 23 encourages more detailed OCFO review
of Sampling and Estimation Methodology Plans. PIIA compliance and independent
audit review therefore establish a governance control, not public
reproducibility of FCIC's full sampling method.

## Custody

- Source: `SRC-USDA-OIG-PIIA-COMPLIANCE-FY2024`
- Official URL: `https://usdaoig.oversight.gov/sites/default/files/reports/2025-09/50024-0016-11_FR_508.pdf`
- Raw path: `data/raw/usda/SRC-USDA-OIG-PIIA-COMPLIANCE-FY2024/2026-07-13/50024-0016-11_FR_508.pdf`
- Bytes: `5619427`
- SHA-256: `A3CEBE04D34D926737995EE9B176F5D7F43EFF0E60DC20574DDB8D4FA7B5C60F`
- Evidence: printed pages 3-5 and 10 (PDF file pages 7-9 and 14; zero-based viewer indices 6-8 and 13), plus printed page 23 (PDF file page 27; zero-based viewer index 26)

## Integration Status

Standalone custody, metadata, bridge, reader, and pulse are complete. Shared
methodology chain, cards, ledger, READMEs, Rust validator, and manifest are
intentionally unchanged.

## Next Bounded Action

Integrate the narrow component decision without changing the program's four
closed / four open methodology-field state or any claim firewall.
