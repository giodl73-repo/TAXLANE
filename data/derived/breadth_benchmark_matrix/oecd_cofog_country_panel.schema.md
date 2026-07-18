# OECD COFOG Country Panel

The machine record contains one row for each of 11 countries and ten top-level
COFOG divisions in calendar 2022. `spending_percent_gdp` is derived from two
same-source OECD observations in current-price national-currency millions.
Currency therefore cancels; no exchange rate or PPP conversion is used.

An observation is either `observed` or `missing_not_imputed`. Canada has GDP
but no bounded 2022 Table 11 function observations. The United States has no
GF05 environmental-protection observation. Those eleven cells remain null.

COFOG is an all-government functional spending classification. It does not by
itself establish service quality, outcomes, causal efficiency, fraud, or
savings. GF04 is broader than transportation or agriculture, GF10 is broader
than pensions or family support, and GF01 is broader than interest or foreign
affairs.
