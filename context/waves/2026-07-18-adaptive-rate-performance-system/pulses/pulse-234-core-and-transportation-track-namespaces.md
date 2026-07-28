# Pulse 234 — CORE and transportation track namespaces

The corpus now separates extensible shared work from repeatable lane work:

- `CORE-*` owns shared evidence, fiscal accounting, solver, and publication
  infrastructure;
- `TRN-A` through `TRN-F` owns the transportation implementation recipe.

`CORE-G` is required to start `TRN-A`. `CORE-H` is not a start blocker for
`TRN-A`; it becomes mandatory before `TRN-B`. Later CORE waves can be added
without renaming the transportation sequence.

This naming decision does not itself complete CORE-G or start TRN-A.
