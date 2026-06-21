---
name: income-tax-pulse
description: Execute the next INCOME-TAX wave pulse with docs, research, validation, and commit-ready updates.
allowed-tools:
  - Read
  - Write
  - Glob
  - Grep
  - Bash
---

# INCOME-TAX Pulse

Use this skill for INCOME-TAX development pulses.

## Workflow

1. Read `context/waves/PHASES.md`.
2. Read the active wave `WAVE.md`.
3. Read the target pulse under `pulses/`.
4. Implement the smallest complete slice.
5. Keep sourced explanation separate from reform preference.
6. Update docs and wave/pulse status.
7. Run validation commands and `git diff --check`.
