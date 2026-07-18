# Global Country Comparison Coverage Contract

This artifact defines the intended multi-country comparison for all 15 Taxlane
lane IDs. It is a coverage and acquisition contract, not a dataset of observed
country values.

Each lane records its comparison mode, candidate countries, metrics, official
source families, and the boundary that must remain visible. The default panel
contains European, Asian, and other peers. A displayed result must contain at
least three comparator countries and, where a matched series exists, at least
one European and one Asian peer.

The contract deliberately permits lane-specific panels. Missing observations
must stay missing; Singapore and other supplemental countries are not imputed
or forced into OECD definitions.

All five public claim gates begin false. Country results may be promoted only
after official-source custody, checksum and metadata, matched accounting scope,
period and unit, missingness disclosure, and comparability review.

```text
comparison design != observed country result != efficiency != fraud != savings
```
