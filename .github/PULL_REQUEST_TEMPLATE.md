## Summary

- 

## Claim Surface

- [ ] Source custody
- [ ] Current-law description
- [ ] Modeled allocation
- [ ] Public reading packet
- [ ] Research paper or review
- [ ] Taxpayer-facing display copy
- [ ] Reform proposal
- [ ] Tooling or validation

## Guardrails

- [ ] Public numbers cite source IDs from `docs/sources/source-version-ledger.md`.
- [ ] Current law, model, legal dedication, civic illustration, and reform
      proposal language remains distinct.
- [ ] No personal tax, legal, accounting, investment, or national-security advice
      is introduced.
- [ ] No unsupported fraud, waste, abuse, or recoverable-savings claim is
      introduced.

## Validation

```powershell
git diff --check
```

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
```
