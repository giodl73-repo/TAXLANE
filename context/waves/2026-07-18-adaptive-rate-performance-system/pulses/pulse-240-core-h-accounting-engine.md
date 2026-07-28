# Pulse 240 — CORE-H Accounting Engine

Implemented checked-integer Rust interfaces for named-fund flows, reserves,
federal deficit/debt rollforward, and endogenous interest. Arithmetic overflow
and reserve overdraw return errors; interest computation returns its exact
numerator remainder rather than silently rounding.
