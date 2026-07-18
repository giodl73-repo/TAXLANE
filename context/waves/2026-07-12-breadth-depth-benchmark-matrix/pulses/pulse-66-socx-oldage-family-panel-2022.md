# Pulse 66: SOCX Old-Age And Family Panel, 2022

## Result

- Captures 42 OECD SOCX observations: seven countries, two programme groups,
  and total/cash/in-kind spending types.
- Adds matched 2022 public old-age/survivors and family-benefit spending as
  percent GDP across North America, Europe, Asia, and Australia.
- Keeps cash and services separate and retains small rounded component-to-total
  differences.
- Leaves Canada's blank old-age service observation missing rather than zero.
- Leaves Germany, Sweden, the Netherlands, Poland, and Singapore missing.
- Excludes Family Database tax breaks because that table is latest/mixed-year,
  not a matched 2022 composition.
- Keeps adequacy, outcome, efficiency, target, and savings gates blocked.

## Boundary

OECD `TP01` combines public old-age and survivors spending. It is not United
States Social Security alone and does not capture every mandatory occupational
or private pension pillar. Family spending covers benefits directed to families
and excludes broader health and housing support. Spending differences do not
establish replacement-rate adequacy, poverty reduction, childcare access, or
causal efficiency.

## Next Gate

Attach matched old-age poverty and pension replacement-rate observations, then
child-poverty or childcare-participation outcomes where definitions and years
align. Keep those outcome measures separate from spending.

```text
public social spending != complete system != outcome != savings
```
