# Contributing To TAXLANE

TAXLANE welcomes outside use and review, but contribution quality depends on
claim discipline. This repo is public-policy research and civic design, not tax,
legal, accounting, investment, or national-security advice.

## Contribution Paths

- Source custody: add or improve source-ledger entries before downstream claims.
- Reading packets: make public packets clearer while preserving claim labels.
- Research review: inspect a paper, source note, or review synthesis for
  comparability, denominator, and proposal/current-law boundaries.
- Data validation: reproduce a derived artifact and run the validator.
- Display review: check whether taxpayer-facing receipt copy labels payer base,
  beneficiary base, financing relationship, denominator, and allocation status.

## Readiness First

Before changing public-facing docs, start with the
[Taxlane showcase readiness summary](docs/reading/taxlane-showcase-readiness-summary.md).
It is the current statement of what the repo can show and what remains blocked:
solver inputs, rates, savings, public rate cards, department-cut instructions,
technology-savings claims, and balanced-budget claims.

## Claim Rules

- Label every public allocation claim as current law, legal dedication,
  proportional allocation, modeled allocation, civic illustration, or reform
  proposal.
- Keep income-tax receipts, payroll taxes, borrowing, fees, trust funds, and
  outlays separate unless a cited accounting source supports the join.
- Do not convert an improper-payment amount into fraud, waste, abuse, or
  recoverable savings without source-backed proof for that specific quantity.
- Preserve year basis, fiscal/tax/calendar-year basis, government level, payer
  base, beneficiary base, and denominator.
- Treat rate recommendations as proposals and value judgments.
- Disclose that repo review panels are AI-simulated review lenses, not external
  endorsements.

## Before A Pull Request

Run the lightweight docs check:

```powershell
git diff --check
```

Run the full artifact guardrail when touching data, derived records, charts,
source links, or public packets:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
```

Rebuild paper PDFs only when paper markdown changes:

```powershell
pwsh docs/papers/build.ps1
```

## Pull Request Checklist

- The change states whether it affects current-law description, modeled
  allocation, source custody, research interpretation, display copy, or reform
  proposal.
- New public numbers cite a source ID from `docs/sources/source-version-ledger.md`.
- Any changed taxpayer-facing text preserves the claim labels.
- Validation commands and results are listed in the pull request.
- No personal tax advice or unsupported savings/fraud claim is introduced.
