# Pulse 04: International + Historical Benchmark Research

## Goal

Supply the historical and international components of the fair-rate methodology
so each lane rate can be justified, not just computed — run ahead of Pulses 02-03
because the external benchmarks are the crux of "fair."

## Changes

- Added `docs/research/2026-06-23-international-historical-benchmark.md`:
  - Headline: US total tax-to-GDP 25.6% (2024) vs OECD avg 34.1% — the US is a
    low-tax country; the federal gap (outlays 23.1% vs receipts 17.3% of GDP) is
    as much a revenue shortfall as a spending excess.
  - Per-lane comparator table (government/compulsory spend % GDP) with a
    direction column for each lane's target cost.
  - Standout finding: US government/compulsory health spend 14.3% of GDP vs OECD
    7.1% — the strongest international evidence for the efficiency/waste case.
  - Statutory contribution-rate benchmarks (pensions, health) for the future
    rate-on-base framing.
  - Historical anchor: solvent post-war decades ran ~5% borrowed share vs ~25-32%
    today; the durable anchor is the solvency ratio, not 1950s functional shares.
- Updated `docs/sources/source-version-ledger.md`:
  - New "International and comparative benchmark sources" section (10 sources:
    OECD Revenue Statistics, Pensions, Health, SOCX, Gov-at-a-Glance, IMF, SIPRI,
    NATO, World Bank education, SSA).
  - New "US fiscal-ratio and projection sources" section (OMB Table 1.2, CBO).
  - Extraction-status table updated for CBO, international, and Table 1.2.

## Boundaries kept

- Comparators bound US rates; they do not set them.
- Scope caveats enforced (federal vs all-government; compulsory vs general health;
  year drift) to prevent false comparisons.
- No claim that matching a peer rate is automatically optimal for the US.

## Validation

- `git diff --check`

## Status

Done.
