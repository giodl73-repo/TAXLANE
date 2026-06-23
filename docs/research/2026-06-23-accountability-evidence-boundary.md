# Accountability Evidence Boundary

## Purpose

TAXLANE should help people demand performance on public money and investigate
waste, fraud, and abuse. That requires an evidence boundary before public
claims.

## Boundary

The first accountability model is not a fraud accusation model. It is a way to
record:

- what public-purpose lane or program is being reviewed,
- which source supports the observation,
- what comparison basis is being used,
- what kind of anomaly or performance signal appears,
- whether the signal is merely a question, an official finding, or adjudicated,
- and which review status applies.

## Why This Matters

Taxpayers need better visibility into whether public money reaches the purpose
used to justify it. But accountability work can become misleading if a data
quality gap, variance, or outlier is presented as fraud before review.

TAXLANE should therefore separate:

| Class | Meaning |
|---|---|
| Performance gap | The source suggests outcomes or delivery fell short of a named target. |
| Waste signal | Spending may be inefficient, duplicative, or unsupported by the named comparison basis. |
| Abuse signal | Use may conflict with program rules or public-purpose expectations. |
| Fraud signal | Evidence may warrant fraud review, but public fraud wording requires official finding or adjudication. |
| Official finding | Inspector general, GAO, court, agency, or other authorized source made a finding. |

## Next Safe Step

The next safe implementation step is to keep this model as documentation until
source custody and example records are selected. Rust checks should come after
the domain model fields stabilize and should validate record shape, not infer
fraud by itself.
