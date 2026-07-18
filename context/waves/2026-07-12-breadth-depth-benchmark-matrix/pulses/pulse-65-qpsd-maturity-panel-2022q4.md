# Pulse 65: QPSD Maturity Panel, 2022 Q4

## Result

- Rejects the OECD Economic Outlook `RFSH` candidate as a harmonized maturity
  measure because its official metadata does not define the index scale,
  government perimeter, instrument coverage, or “coming period” horizon.
- Captures four explicit maturity components from the joint World Bank–IMF
  Quarterly Public Sector Debt database instead.
- Covers short-term debt by original maturity for ten peers and
  long-term-original debt due within one year for six peers.
- Calculates a combined due-within-one-year stock only for those six countries.
- Preserves Canada as source-null, Singapore as absent, and Japan's reported
  zero remaining-maturity component as a genuine zero.
- Keeps ranking, target, refinancing-projection, and savings gates blocked.

## Boundary

Short-term original maturity is not debt maturing in the next year. France, the
Netherlands, Poland, and the United States have an observed short-term stock but
no observed long-term-original component due within one year, so their combined
field remains null. QPSD is a voluntary, as-available stock snapshot, not gross
financing needs or an average maturity measure.

## Next Gate

Move to the social-security and family-support specialist panel while retaining
debt maturity as partial context. A fuller maturity comparison requires direct
residual-maturity coverage from national debt-management sources or improved
international reporting.

```text
original maturity != remaining maturity != projected refinancing
```
