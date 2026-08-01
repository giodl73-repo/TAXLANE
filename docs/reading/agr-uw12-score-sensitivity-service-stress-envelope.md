# AGR UW12 sensitivity and service-stress envelope

## What the executable model proves

The `taxlane-agr-insurance` crate turns the legislative design into exact,
repeatable arithmetic. It applies the positive difference between an assumed
baseline return and the 12-percent test target to retained premium, then applies
participation, service-floor pauses, the three-year phase-in, federal
administration cost, and stabilization cost.

For every $1 billion of normalized annual retained premium, the historical
16.8-percent context produces $420 million of ten-year gross reduction and
$399 million after a five-percent administration allowance. A stressed case
with 90-percent participation, 20-percent paused market share, five-percent
administration, and ten-percent stabilization produces $257.04 million net.

Those are unit sensitivities, not federal savings estimates. The current
retained-premium base and current return are not public, the reinsurance-to-
fiscal-year mapping is simplified, and the behavioral assumptions are not
observations. The envelope therefore cannot be scaled from aggregate program
compensation or passed to the fiscal solver.

## The important negative cases

The current-law null returns zero. A full service-floor pause returns zero. A
10.2-percent assumed return—below the 12-percent target—also returns zero. The
model cannot manufacture a cut merely because a policy target exists.

## Portfolio consequence

AGR has reached its current public analytical frontier and moves to named-
trigger monitoring with zero admission. It reopens when current insurer market,
service, and return data, a matching score, or implementation evidence become
available. The active evidence lane moves to SEE's energy and weatherization
candidate.
