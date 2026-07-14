# Pulse 26: Federal Crop Insurance Payment-Universe Closure

## Result

Captured checksum-verified official custody of USDA OIG audit report
`05403-0001-11`. The embedded FCIC/RMA FY2024 report says the improper-payment
rate considers all categories of payments and enumerates premium subsidy,
Administrative and Operating expense, and indemnities. It also identifies
high, medium, and low AIP payment tiers and an annual statistically valid
sample. Existing COM-23-001 adds that all applicable policyholder and AIP
documentation for the 326 sampled policies is submitted through CARS.

## Decision Gate

Pass for internal closure of `payment universe`. FCIC now has four internally
closed methodology fields and four open fields: sample design, estimation
method, exclusion rules, and recoverable-savings basis.

Fail for sample-design closure because tier allocation, frame construction,
selection probabilities, and randomization details remain unpublished. Fail
for estimation-method and exclusion-rule closure because weights, projection,
variance, and policy-level or review exclusions remain unpublished. Fail for
program scoring or public claims about fraud, debt, collectibility,
collections, recovery, prevention, or savings.

## Unaudited And Excluded Evidence

Printed pages 60-61 are explicitly labeled Other Information (Unaudited), and
printed page 18 is in Management's Discussion and Analysis (Unaudited). The
FY2024 payment-integrity table appears to print `$579.93M` as overpayments,
which conflicts with the authoritative annual workbook value of `$573.93M`.
The apparent typo is excluded and must not be used for reconciliation, scoring,
or claims.

## Custody

- Source: `SRC-USDA-OIG-FCIC-RMA-FS-FY2024`
- Official URL: `https://usdaoig.oversight.gov/sites/default/files/reports/2024-11/05403-0001-11_FR_508.pdf`
- Raw path: `data/raw/usda/SRC-USDA-OIG-FCIC-RMA-FS-FY2024/2026-07-13/05403-0001-11_FR_508.pdf`
- Bytes: `7242677`
- SHA-256: `0797BD2CCB1027B568BCE3B640849E89F30A235F528B1B1A2B249D525695ED32`
- Evidence: printed page 18 (PDF file page 29; zero-based viewer index 28) and printed pages 60-61 (PDF file pages 71-72; zero-based viewer indices 70-71)

## Integration Status

Standalone custody, bridge, reader, and pulse are complete. Shared methodology
chain, cards, breadth rows, scoreboard, WAVE, source ledger, READMEs, validator,
and manifest integration are intentionally delegated to the parent task.

## Next Bounded Action

Integrate the fourth field closure through the shared methodology chain while
preserving the four remaining open fields and every claim firewall.
