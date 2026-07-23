# Pulse 194: Income-security/family federal program perimeter bridge

## Goal

Close the narrow FY2025 federal account-perimeter source-custody step for the
income-security/family lane using already-local OMB Public Budget Database
outlays, without treating the result as complete source capture, benefit
package modeling, floor values, solver input, rates, or savings.

## Implemented

- Added `income_security_family_federal_program_perimeter_bridge.fy2025.v1.draft.json`.
- Added schema and public reader.
- Added Rust validation and a focused regression test.
- Linked the bridge from breadth-matrix and reading indexes.

## Boundary

This pulse closes only FY2025 federal account-perimeter source custody for OMB
function 600. State/local spending, CBO baseline and take-up context, child
poverty and income context, childcare and family-service context, food hardship
and nutrition context, international comparator lineage, federal/state/local
translation, benefit package modeling, floor values, pass/fail findings, solver
inputs, rates, savings, and balanced-budget claims remain blocked.

No external request was submitted and no agency or person was contacted.
