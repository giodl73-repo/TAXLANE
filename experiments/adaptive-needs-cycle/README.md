# Adaptive needs cycle experiment

Build the first all-lane triage from the existing Wave D floor anchors and the
breadth benchmark matrix:

```powershell
python experiments/adaptive-needs-cycle/build_triage.py `
  --floor-readiness data/derived/breadth_benchmark_matrix/outcome_floor_wave_d_value_readiness.v1.draft.json `
  --breadth data/derived/breadth_benchmark_matrix/breadth_benchmark_matrix.v1.draft.jsonl `
  --output data/derived/breadth_benchmark_matrix/adaptive_needs_all_lane_triage.v1.draft.json
```

The output classifies problem and response types. It does not score pain,
rank lanes, allocate a profile reserve, admit savings, or authorize release.

Build the raw-custodied TRN screening denominator:

```powershell
python experiments/adaptive-needs-cycle/build_trn_screening_denominator.py `
  --hm20 data/raw/fhwa/SRC-FHWA-HIGHWAY-STATISTICS-2024-HM20/2026-07-28/hm20.html `
  --fi20 data/raw/fhwa/SRC-FHWA-HIGHWAY-STATISTICS-2024-FI20/2026-07-28/fi20.html `
  --output data/derived/breadth_benchmark_matrix/trn_rumble_strip_screening_denominator.v1.draft.json
```

The output is a functional-system screening ceiling, not eligible two-lane
mileage, target crashes, a candidate cost, or an outcome estimate.

Build the raw and calibrated HPMS two-way, two-lane screen:

```powershell
python experiments/adaptive-needs-cycle/build_trn_hpms_two_lane_screen.py `
  --metadata data/raw/fhwa/SRC-FHWA-HPMS-SPATIAL-SECTIONS-2024-TRN-SCREEN/2026-07-28/dataset-metadata.json `
  --functional-totals data/raw/fhwa/SRC-FHWA-HPMS-SPATIAL-SECTIONS-2024-TRN-SCREEN/2026-07-28/rural-screening-functional-system-totals.json `
  --two-lane-functional-totals data/raw/fhwa/SRC-FHWA-HPMS-SPATIAL-SECTIONS-2024-TRN-SCREEN/2026-07-28/rural-two-way-two-lane-functional-system-totals.json `
  --two-lane-state-totals data/raw/fhwa/SRC-FHWA-HPMS-SPATIAL-SECTIONS-2024-TRN-SCREEN/2026-07-28/rural-two-way-two-lane-state-totals.json `
  --official-screen data/derived/breadth_benchmark_matrix/trn_rumble_strip_screening_denominator.v1.draft.json `
  --output data/derived/breadth_benchmark_matrix/trn_hpms_two_way_two_lane_screen.v1.draft.json
```

The raw section-mile total and calibrated screen remain separate because FHWA
warns that spatial full-join aggregation may differ from official Highway
Statistics totals.

Audit suitability-field availability within that HPMS screen:

```powershell
python experiments/adaptive-needs-cycle/build_trn_suitability_availability.py `
  --metadata data/raw/fhwa/SRC-FHWA-HPMS-SPATIAL-SECTIONS-2024-TRN-SCREEN/2026-07-28/dataset-metadata.json `
  --hpms-screen data/derived/breadth_benchmark_matrix/trn_hpms_two_way_two_lane_screen.v1.draft.json `
  --field-completeness data/raw/fhwa/SRC-FHWA-HPMS-SPATIAL-SECTIONS-2024-TRN-SCREEN/2026-07-28/suitability-field-completeness.json `
  --shoulder-bands data/raw/fhwa/SRC-FHWA-HPMS-SPATIAL-SECTIONS-2024-TRN-SCREEN/2026-07-28/suitability-shoulder-bands.json `
  --iri-bands data/raw/fhwa/SRC-FHWA-HPMS-SPATIAL-SECTIONS-2024-TRN-SCREEN/2026-07-28/suitability-iri-bands.json `
  --by-functional-system data/raw/fhwa/SRC-FHWA-HPMS-SPATIAL-SECTIONS-2024-TRN-SCREEN/2026-07-28/suitability-by-functional-system.json `
  --output data/derived/breadth_benchmark_matrix/trn_hpms_rumble_suitability_availability.v1.draft.json
```

This audit reports missingness and transparent triage bands. A reported
four-foot shoulder is not verified usable width beyond a strip, IRI is not a
structural pavement test, and missing values are not treated as ineligible.

Build the first road-owner existing-treatment inventory pilot:

```powershell
python experiments/adaptive-needs-cycle/build_trn_penndot_treatment_inventory.py `
  --workbook data/raw/penndot/SRC-PENNDOT-STATEWIDE-RUMBLE-STRIPS-2024/2026-07-28/Statewide-Rumble-Strips-03-01-2024.xlsx `
  --landing-page data/raw/penndot/SRC-PENNDOT-STATEWIDE-RUMBLE-STRIPS-2024/2026-07-28/safety-infrastructure-improvement-programs.html `
  --hpms-state-totals data/raw/fhwa/SRC-FHWA-HPMS-SPATIAL-SECTIONS-2024-TRN-SCREEN/2026-07-28/rural-two-way-two-lane-state-totals.json `
  --output data/derived/breadth_benchmark_matrix/trn_penndot_existing_rumble_strip_inventory.v1.draft.json
```

The pilot audits PennDOT's treatment intervals and constructs a two-way,
two-lane analytical footprint. It does not subtract that footprint from HPMS:
an RMS/LRS join is still required to align geography, functional system, road
ownership, pavement, geometry, and crashes.

Capture and join the public RMS layers for the workbook's county-route universe:

```powershell
python experiments/adaptive-needs-cycle/capture_penndot_rms_join_layers.py `
  --workbook data/raw/penndot/SRC-PENNDOT-STATEWIDE-RUMBLE-STRIPS-2024/2026-07-28/Statewide-Rumble-Strips-03-01-2024.xlsx `
  --output-dir data/raw/penndot/SRC-PENNDOT-RMS-OPEN-DATA-TRN-JOIN/2026-07-28

python experiments/adaptive-needs-cycle/build_trn_penndot_rms_join.py `
  --workbook data/raw/penndot/SRC-PENNDOT-STATEWIDE-RUMBLE-STRIPS-2024/2026-07-28/Statewide-Rumble-Strips-03-01-2024.xlsx `
  --segments data/raw/penndot/SRC-PENNDOT-RMS-OPEN-DATA-TRN-JOIN/2026-07-28/roadwaysegments-workbook-route-universe.json `
  --admin data/raw/penndot/SRC-PENNDOT-RMS-OPEN-DATA-TRN-JOIN/2026-07-28/roadwayadmin-workbook-route-universe.json `
  --shoulder data/raw/penndot/SRC-PENNDOT-RMS-OPEN-DATA-TRN-JOIN/2026-07-28/roadwayshoulder-workbook-route-universe.json `
  --hpms-state-totals data/raw/fhwa/SRC-FHWA-HPMS-SPATIAL-SECTIONS-2024-TRN-SCREEN/2026-07-28/rural-two-way-two-lane-state-totals.json `
  --output data/derived/breadth_benchmark_matrix/trn_penndot_rms_existing_treatment_join.v1.draft.json
```

The join can establish a comparable state-owned treated footprint. Because its
capture universe is selected by treatment presence, a complete state-owned RMS
denominator remains necessary before calculating untreated miles.

Capture and build the complete state-owned denominator:

```powershell
python experiments/adaptive-needs-cycle/capture_penndot_rms_state_denominator.py `
  --output-dir data/raw/penndot/SRC-PENNDOT-RMS-STATE-DENOMINATOR-TRN/2026-07-28

python experiments/adaptive-needs-cycle/build_trn_penndot_state_untreated_screen.py `
  --workbook data/raw/penndot/SRC-PENNDOT-STATEWIDE-RUMBLE-STRIPS-2024/2026-07-28/Statewide-Rumble-Strips-03-01-2024.xlsx `
  --segments data/raw/penndot/SRC-PENNDOT-RMS-STATE-DENOMINATOR-TRN/2026-07-28/roadwaysegments-state-denominator-universe.json `
  --admin data/raw/penndot/SRC-PENNDOT-RMS-STATE-DENOMINATOR-TRN/2026-07-28/roadwayadmin-state-denominator-universe.json `
  --shoulder data/raw/penndot/SRC-PENNDOT-RMS-STATE-DENOMINATOR-TRN/2026-07-28/roadwayshoulder-state-denominator-universe.json `
  --prior-join data/derived/breadth_benchmark_matrix/trn_penndot_rms_existing_treatment_join.v1.draft.json `
  --hpms-state-totals data/raw/fhwa/SRC-FHWA-HPMS-SPATIAL-SECTIONS-2024-TRN-SCREEN/2026-07-28/rural-two-way-two-lane-state-totals.json `
  --hpms-screen data/derived/breadth_benchmark_matrix/trn_hpms_two_way_two_lane_screen.v1.draft.json `
  --output data/derived/breadth_benchmark_matrix/trn_penndot_state_owned_untreated_screen.v1.draft.json
```

This same-basis subtraction produces untreated state-owned screening mileage
and audits complete interval-level shoulder records. Reported current paved
width remains upstream of usable-width, pavement, user, noise, crash, and
current-cost eligibility filters.

Build the first VET, TRN, and ISF pain-aligned dossiers:

```powershell
python experiments/adaptive-needs-cycle/build_initial_dossiers.py `
  --output data/derived/breadth_benchmark_matrix/adaptive_initial_intervention_dossiers.v1.draft.json
```

The generated dossiers select bounded analytical candidates and record reviewed
official web evidence with an explicit non-custodied status. Historical effects
and base budgets advance only the gates they actually support; they are not
converted into current scores or allocations.

The candidate-planning inputs also preserve the next calculation boundary:
VET has a second timeliness baseline but no incremental effect, TRN has a
historical unit cost but no eligible national mileage, and ISF has a TY2026
current-law comparator but no incremental proposal score.

Audit the four reserves against the scoring contract:

```powershell
python experiments/adaptive-needs-cycle/allocate_reserves.py `
  --contract data/derived/breadth_benchmark_matrix/adaptive_needs_scoring_allocation_contract.v1.draft.json `
  --triage data/derived/breadth_benchmark_matrix/adaptive_needs_all_lane_triage.v1.draft.json `
  --profiles data/derived/breadth_benchmark_matrix/peer_informed_savings_and_reinvestment_profiles.ty2026.v1.draft.json `
  --dossiers data/derived/breadth_benchmark_matrix/adaptive_initial_intervention_dossiers.v1.draft.json `
  --output data/derived/breadth_benchmark_matrix/adaptive_profile_reserve_allocation_readiness.v1.draft.json
```

When one or more intervention rows eventually clear every gate, the allocator
computes the published weighted score, applies evidence and delivery discounts,
holds ten percent for contingency, applies candidate and single-lane caps, and
leaves any remainder in reserve.
