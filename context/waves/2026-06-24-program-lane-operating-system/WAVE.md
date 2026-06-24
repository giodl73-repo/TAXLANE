# Wave: Program-Lane Operating System

## Goal

Make the fair-rate system **functional**: give each lane a public argument for its
rate, a defined mechanism for moving the rate up or down, and a mandatory
explanation for every change — so the system can actually operate, not just sit
as a static proposal.

Builds on the completed `2026-06-23-program-lane-fair-rate-design` wave (lanes,
rates, benchmarks, panel, guardrails, full allocation).

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Per-lane rate-rationale cards | done | A "rate card" per lane: the rate, who funds it, and the four-anchor argument (cost, solvency, history, peers) plus the panel direction. |
| 02 | Rate-adjustment operating model | done | How a lane rate moves up or down: triggers, decision authority, the rate lifecycle, and a mandatory public-explanation template. |
| 03 | Worked rate-change examples | done | Three end-to-end `lane_rate_change` records: defense up +$146B (Hague), health down -$395B (efficiency), Social Security cap lift on a $297B shortfall. |
| 04 | Public presentation packet + chart | planned | Assemble the cards + operating model into a citizen-facing reading packet with a chart spec. |

## Design rules

- Every rate card carries the `allocation_method` label and the deficit context.
- Every rate move requires a published explanation: old rate, new rate, trigger,
  source, decision authority, and effect on the borrowed-share line.
- Keep descriptive (current law) separate from proposed reform.
- Argue rates from the four anchors and the panel; never assert a single "correct"
  rate as fact.

## Non-goals

- No statutory bill text or live calculator.
- No claim a rate is legally dedicated unless it already is (payroll, trust excise).

## Validation

```powershell
git diff --check
```
