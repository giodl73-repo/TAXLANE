# VERDICT candidate-admission bridge

Machine record:
`data/derived/breadth_benchmark_matrix/verdict_candidate_admission_bridge.v1.draft.json`.

## Purpose

VERDICT can summarize whether a lane repository has evidence for Value,
Effectiveness, Resilience, Deliverability, Iteration, Coverage and fair access,
and Trust. TAXLANE consumes that evidence only at the candidate boundary.

A repository-program score cannot change a target cost, allocation, savings
estimate, or rate. A current-system score cannot stand in for a candidate
effect. A candidate total cannot override a failed or unresolved applicable
floor.

## Reused interfaces

No new Rust interface is needed. A candidate assessment maps into the existing
CORE-M `CandidateGateReview` contract:

| VERDICT dimension | TAXLANE admission evidence |
|---|---|
| V — Value | Whole-system lifecycle cost, useful outcome denominator, implementation and transition cost, and a compatible comparison. |
| E — Effectiveness | Candidate-specific official effect evidence against the declared service outcome. |
| R — Resilience | Applicable adequacy, continuity, recovery, and stress floors. |
| D — Deliverability | Authority, financing role, workforce, procurement, capacity, schedule, and transition evidence. |
| I — Iteration | Observation cadence, named response authority, outcome review, rollback or retirement trigger, and successor handling. |
| C — Coverage and fair access | Access, distribution, equity, incidence, exclusion, delay, and burden evidence. |
| T — Trust | Exact source and candidate version, custody, assumptions, role review, correction path, and claim boundary. |

For a Lane 2.0 candidate all seven reviews are applicable. Each is encoded as
`RequiredReady` with an evidence path or `RequiredBlocked` with a successor
owner and blocking gate. A numeric VERDICT score is descriptive context; it is
not a CORE-M gate disposition.

## Fiscal admission sequence

```text
lane evidence -> candidate-scoped VERDICT assessment
    -> seven CORE-M gate reviews
    -> objective-compatible candidate dossier
    -> target-cost and fiscal floors
    -> baseline / modernization / stress paths
    -> aggregate accounting and endogenous interest
    -> distribution, behavior, macro, and role review
    -> rate publication gate
```

Value evidence can support a lower-cost review only when the candidate uses the
`LowerCostReform` objective and the existing productivity-savings, lifecycle,
implementation-cost, overlap, and stress requirements pass. A high value score
alone never creates savings.

Iteration evidence must identify the demonstrated loop. Analytical refresh is
useful evidence but is not operational response, outcome learning, or fiscal
rebalancing. TAXLANE's deterministic annual update contract supplies the fiscal
loop only after its inputs are admitted; it does not manufacture missing lane
outcomes.

## Current decision

The bridge is accepted as a reuse mapping. No solver run, target-cost change,
allocation change, savings admission, rate change, or public release is
authorized. Implementation remains in the existing CORE-M and adaptive-rate
interfaces; no VERDICT crate or duplicate TAXLANE scorer is warranted.

## Validation

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
git diff --check
```
