# Annual 100-Point Budget Ballot

This experiment simulates every voter allocating exactly 100 budget points
across TAXLANE's positive FY2025 program lanes. A computer interface would
prevent submission until the allocation equals 100.

Process:

1. Each synthetic voter receives the current lane shares and target context.
2. The voter allocates exactly 100 points across all displayed lanes.
3. Individual ballots are averaged within each state and DC.
4. State averages are combined using 2024-2028 Electoral College weights.
5. The result is compared with House-apportionment and equal-state weighting.

Negative accounting offsets are excluded from sliders and remain in the fiscal
reconciliation layer. This first run is a synthetic institutional simulation,
not measured public opinion or a forecast of an election.

Run:

```powershell
python experiments/annual-budget-ballot/simulate.py
```

Inputs: `config.v1.json` and the canonical FY2025 program-lane model.

Outputs: `outputs/synthetic-run.v1.json` and `outputs/synthetic-run.v1.md`.

The V2 diversity stress test uses 13 personalities, contrarian and single-issue
ballots, weaker current-budget anchoring, wider personal variation, and 30
uncertainty runs. Run `python experiments/annual-budget-ballot/simulate_v2.py`;
outputs are `outputs/diverse-run.v2.json` and `outputs/diverse-run.v2.md`.

The Electoral College allocations are the National Archives' 2024/2028
allocations based on the 2020 Census. The state opinion profiles and archetype
weights are synthetic assumptions, not survey estimates or partisan labels.
