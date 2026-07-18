# Pulse 68: Age Relative-Income Poverty Panel

## Result

- Captures actual OECD Income Distribution Database observations rather than
  relabeling a “2022 or latest available” publication snapshot as common-year.
- Adds older-person poverty for all 11 core OECD peers with explicit 2020-2024
  country years and the Netherlands provisional flag.
- Adds child poverty for ten peers in 2021 plus Australia's 2020 fallback.
- Leaves Singapore absent and never imputes a value.
- Defines poverty as income below 50 percent of each country's own median,
  not an absolute or common international living-standard threshold.
- Keeps spending, modeled replacement, causality, efficiency, target, and
  savings gates blocked.

## Next Gate

Move to another uncovered specialist lane. Childcare participation is not used
because current official coverage would mix 2023 observations with United
Kingdom 2018 and United States 2011 values.

```text
country-relative poverty != common absolute threshold != policy causality
```
