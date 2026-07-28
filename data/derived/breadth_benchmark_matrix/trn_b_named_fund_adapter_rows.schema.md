# TRN-B Named-Fund Adapter Rows Schema

Each adapter row preserves a named fund, fiscal year, source vintage, source
perimeter, CORE-H input, recomputed CORE-H output, source closing balance, and
reconciliation difference. Displayed OMB billions are converted to integer
tenths of billions without inferring additional precision.

All fourteen FY2025-FY2031 OMB rows are required for TRN-B-01 closure. When a
displayed change and displayed closing balance differ by one tenth of a billion,
the adapter must preserve the reported change difference while the explicit
rounding line reconciles the closing balance. TRN-B-01 completion does not
complete source bridges, Function 400 mapping, or TRN-B.
