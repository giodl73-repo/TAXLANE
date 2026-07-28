# Income-security/family food hardship and nutrition capture gap schema

`income_security_family_food_hardship_nutrition_capture_gap.v1.draft.json`
records the still-open USDA food hardship and nutrition source-custody gate,
with partial ERS food-security and FNS SNAP raw custody now captured.

Required checks:

- `record_family` is `income_security_family_food_hardship_nutrition_capture_gap`.
- The record links the income-security/family source capture queue and closure
  queue.
- Candidate official USDA ERS and FNS source surfaces are named.
- Broader nutrition-program boundary raw artifact paths remain null until that
  custody exists.
- ERS household food-security report, report summary, statistical supplement,
  topic/data-product pages, and CPS Food Security Supplement documentation may
  be recorded with byte counts, SHA-256 hashes, retrieval date, and metadata
  path.
- FNS SNAP annual summary, monthly, persons, households, benefits, and
  FY1969-current ZIP raw files may be recorded with byte counts, SHA-256
  hashes, retrieval date, observed file structure, and metadata path.
- Food-security measures, material-hardship floors, benefit-package context,
  solver inputs, rates, savings, and balanced-budget claims remain blocked.
