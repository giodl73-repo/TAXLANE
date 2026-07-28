# Taxpayer profile scenarios

This record supplies a repository-local current-versus-proposed explanation for
Tax Year 2026. It uses the current-law schedules, Taxlane schedules, and filing
status breakpoints already preserved by the scorer-ready specification.

The profiles take **taxable ordinary income** as an input. They do not infer
gross income, deductions, filing eligibility, credits, payroll tax, capital
gains, withholding, refunds, incidence, or complete household liability. When
all seven ordinary rates rise uniformly and thresholds remain unchanged, the
narrow difference equals taxable ordinary income multiplied by the uplift.

## Required boundaries

- Current law, Taxlane recommendation, contingency, and stress must remain
  separately labeled.
- Peer-aligned defense, health, and social-protection scenarios carry null
  savings and null rates until a same-basis FY2026 crosswalk, candidate design,
  service floors, transition, incidence, and rate rerun exist.
- A country spending share is descriptive composition, not evidence that its
  level is efficient, fair, transferable, or available as U.S. savings.
- The record is not enacted law, an official score, a complete tax return,
  personal advice, or public-release authority.

## Reproduction

For each profile and scenario, apply the seven marginal rates to the taxable
ordinary income within the six sourced breakpoints. Sum the bracket amounts.
The stated Taxlane difference must also equal `taxable_ordinary_income × 0.11`.
