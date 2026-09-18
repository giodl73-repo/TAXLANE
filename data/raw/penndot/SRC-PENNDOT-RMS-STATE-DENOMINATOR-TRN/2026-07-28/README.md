# PennDOT RMS complete state-denominator capture

Captured 2026-07-28 from PennDOT's public ArcGIS REST services with attributes
only and 2,000-feature pagination.

- `roadwaysegments-state-denominator-universe.json`: 58,733 records satisfying
  `JURIS='1' AND URBAN_RURAL='1' AND DIR_IND='B' AND FAC_TYPE='2' AND
  LANE_CNT=2`.
- `roadwayadmin-state-denominator-universe.json`: 38,394 administration
  intervals satisfying `JURIS='1' AND FHWA_FUNC_CLS IN ('3','4','5','6')`.
- `roadwayshoulder-state-denominator-universe.json`: 124,096 state-
  jurisdiction shoulder intervals satisfying `JURIS='1'`; SHA-256
  `9545a006af188e3e73fbd729703d967fa7a9ca8be593e22278212efc51cbc28c`.

The capture includes routes regardless of whether the rumble-strip workbook
contains a treatment. This removes the prior treatment-selection bias and
supports a complete state-owned screening denominator and a side-specific
shoulder-field availability audit. Reported paved width is not verified usable
clear width beyond a proposed strip and does not by itself establish design
eligibility, target crashes, or current cost.
