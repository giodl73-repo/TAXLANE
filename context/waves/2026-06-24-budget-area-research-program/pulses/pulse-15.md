# Pulse 15: OMB Table 6.1 Defense-%-GDP Extraction (Data-Layer Wave)

Closes the loop the T3 paper implied: its §2 defense-%-GDP history is now a
custody-backed, regenerable extracted record, not just an inline citation.

## Changes
- **Raw custody:** captured `hist06z1_fy2027.xlsx` to
  `data/raw/omb/SRC-OMB-HIST-6-1-FY2027/2026-06-24/` with a `data/metadata/` record
  (SHA-256, source URL, coverage) per the raw-capture rule.
- **Rust CLI command:** added `taxlane-tools outlay-composition table-6-1-national-defense
  [--check]` (mirrors the Table 3.2 national-defense extraction): finds the "As
  percentages of GDP" section, extracts the national-defense (function-050) row across
  **actual years 1940-2025 (86 records)**, anchors 1953≈13.8 and FY2025≈3.0, and emits a
  validated JSONL + profile under `data/extracted/outlay_composition/`.
- **Validation:** `--check` reproduces the committed artifacts byte-for-byte;
  `cargo test --workspace` stays green (24/24); the existing Table 3.2 extraction is
  unaffected. New evidence row `EVID-TAX-070`.
- **Paper repoint:** T3 §2 now cites the derived record
  (`table-6-1-national-defense-gdp-profile.md`) and the regenerating command; PDF rebuilt.

## Status
Done. The T3 history figures (13.8% 1953, 6.0% FY1986, 2.9% 2000, 3.0% FY2025) are now
backed by an in-repo, regenerable extraction. No paper acceptance affected.
