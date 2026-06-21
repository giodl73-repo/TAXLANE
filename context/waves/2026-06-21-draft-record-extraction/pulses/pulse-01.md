# Pulse 01: IRS Rates First Draft Rows

## Goal

Extract a small, source-anchored sample from IRS SOI Historical Table 23 into
draft `rates_timeline` JSONL records.

## Changes

- Add 1913-1918 lowest/highest regular-tax summary rows from IRS Table 23.
- Add extraction notes for the first draft rates slice.
- Mark this wave active in `PHASES.md`.

## Validation

- `git diff --check`
- Parse draft JSONL records as JSON.

## Status

Done.
