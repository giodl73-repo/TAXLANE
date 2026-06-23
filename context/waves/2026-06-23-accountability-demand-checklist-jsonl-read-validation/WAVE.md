# Accountability Demand Checklist JSONL Read Validation Wave

## Goal

Validate generated performance demand checklist JSONL through a typed core
record before downstream UI/API use.

## Scope

- Add an owned checklist record type to `taxlane-core`.
- Validate generated JSONL by deserializing into the core record type.
- Compare generated JSONL rows against core-derived expected records.
- Close VTRACE work package `WP-TAX-022`.

## Status

Complete.
