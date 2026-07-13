# Pulse 03: VA PLTSS Source Review

## Result

Resolved the VA PLTSS discrepancy as an extraction error. The official Q4 2025
PaymentAccuracy scorecard and FY2024 annual workbook both report $218.30M in
projected FY2024 overpayments at 3.88%, using an October 2022 through September
2023 sample.

The prior probe used a URL that returned HTML instead of the scorecard PDF and
stored $2.502B / 15.54% values that do not appear in the official PLTSS source.
The corrected PDF is captured with metadata and checksum, and the affected
probe, program gate, depth card, scoreboard, and reading packet are corrected.

## Role gate

Pass for source-corrected program wording with the projected-estimate, scope,
rate, and sample-period labels. Fail for fraud, recoverability, prevented-loss,
or net-savings claims. Recovery activity remains a separate evidence class.

## Next bounded action

Deepen one remaining blocked payment-integrity link: match a program, period,
and definition across projected overpayments, identified debt, and collected
recovery, or document why the bridge cannot yet be made. Preserve veteran
access and due-process floors before scoring any control proposal.
