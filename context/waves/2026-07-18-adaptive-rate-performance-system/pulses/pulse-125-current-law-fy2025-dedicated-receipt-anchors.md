# Pulse 125 — FY2025 dedicated receipt anchors

## Result

Added a source-custodied FY2025 dedicated-receipt anchor packet for the named
fund paths needed before solver construction.

## Data populated

- OASI trust-fund off-budget receipts: `$1,097.382B`.
- DI off-budget receipts: `$186.354B`.
- OASDI receipt anchor sum: `$1,283.736B`.
- Medicare HI hospital-insurance receipts: `$395.350B`.
- Transportation trust-fund excise receipts: `$43.768B`.
- Airport-and-airway trust-fund excise receipts: `$23.118B`.

## Boundaries

- These are receipt anchors only, not complete trust-fund paths.
- OASI and DI are summed only as an OASDI receipt anchor; fund accounting remains
  separate until fund-balance sources are captured.
- Medicare HI remains separate from combined Medicare.
- Transportation and airport-and-airway rows remain source-labeled and are not a
  complete transportation trust-fund path.
- No external request was submitted and no agency or person was contacted.
- No solver input, target cost, rate, savings, waste, fraud, technology-savings,
  department-cut, or balanced-budget claim is made.

## Validation

The validator recomputes OASDI and covered-anchor sums, checks source hashes and
byte counts, preserves null blocked outputs, and enforces public warning phrases.
