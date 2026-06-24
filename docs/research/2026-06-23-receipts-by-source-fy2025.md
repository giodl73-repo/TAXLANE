# FY2025 Receipts by Source — the Revenue Side of the Lanes

## Decision supported

Closes the Source Custodian's P1 hold on the revenue base: extracts the actual
FY2025 receipts-by-source split from OMB Table 2.1 so the "all federal receipts
into lanes" model rests on real numbers, not a residual.

Data record:
`data/extracted/receipt_source/receipt_source.SRC-OMB-HIST-2-1-FY2027.2026-06-23.fy2025-source-split.jsonl`.

## What we collect (FY2025, sourced, reconciles exactly)

| Receipt source | FY2025 ($M) | Share | Allocation status |
|---|---:|---:|---|
| Individual income taxes | 2,656,044 | 50.72% | general receipt (general-fund engine) |
| Social insurance & retirement (payroll) | 1,748,294 | 33.39% | **dedicated** (OASDI, HI, unemployment, federal retirement) |
| Corporation income taxes | 452,089 | 8.63% | general receipt |
| Other (estate/gift, customs, misc) | 274,057 | 5.23% | general receipt |
| Excise taxes | 105,937 | 2.02% | mixed (some dedicated: highway, airport) |
| **Total receipts** | **5,236,421** | 100% | matches OMB Table 1.1 |

The five sources sum exactly to total receipts (5,236,421 $M).

## Which taxes fund which lanes

This is the spine of the all-receipts model:

- **Dedicated payroll ($1,748,294M)** funds the dedicated lanes: Social Security
  (OASDI), Medicare-HI, unemployment, federal retirement. This is why Social
  Security is carved out of the income-tax budget — it has its own ~$1.75T stream.
- **General-fund receipts ($3,488,127M = income + corporate + excise + other)**
  fund the general-fund lanes (defense, non-HI health, justice, veterans, income
  security, net interest, etc.).
- The general-fund lanes cost ~$5.43T (ex-Social Security) but general-fund
  receipts are only ~$3.49T — a ~$1.94T general-fund gap, which is essentially the
  whole federal deficit. **Under "spend what we collect," that gap is the target.**

## Key revenue facts for the rate debate

- The individual income tax is just over **half** of receipts; payroll is a
  third. So an "income-tax-only" reform touches ~51% of revenue.
- Closing the gap purely on the **income tax** would mean raising ~$1.77T on a
  ~$2.66T base — a ~67% increase. Spreading it across **all general-fund receipts**
  (income + corporate + excise + other = $3.49T) is a smaller proportional lift,
  which is why the panel and the all-receipts scope matter.
- Corporate income tax ($452B, 8.6%) and excise ($106B, 2.0%) are small relative
  to income and payroll — they cannot carry the gap alone either.

## Boundaries kept

- `allocation_status` distinguishes dedicated payroll from general receipts.
- No claim that any general receipt is legally tied to a specific lane.
- "Other" is an OMB roll-up (estate/gift, customs, misc); its sub-split needs
  Table 2.4/2.5 detail (deferred).

## Adopt now

- Use the five-source split as the revenue base for the all-receipts lane model.
- Treat the ~$1.94T general-fund gap as the balanced-budget target separate from
  the payroll-funded lanes.
