# Pulse 108 — Current-law path inventory

Pulse 108 adds
`data/derived/breadth_benchmark_matrix/current_law_path_inventory.v1.draft.json`.

The inventory names the official annual current-law paths still required before
the deterministic solver can assemble a baseline year plus ten-year horizon.

No annual values are added. Every row remains not ready with `value: null`.
Official source custody, trust-fund separation, Medicare HI separation, no
interpolation without an explicit model, and all solver/rate/public-claim
blocks remain enforced.
