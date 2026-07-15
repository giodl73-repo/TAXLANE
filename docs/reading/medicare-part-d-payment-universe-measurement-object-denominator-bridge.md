# Medicare Part D Payment-Universe Measurement-Object and Denominator Bridge

Machine record:
`data/derived/breadth_benchmark_matrix/medicare_part_d_payment_universe_measurement_object_denominator_bridge.fy2024.v1.draft.json`.

## Narrow component closure

The CY2022 submission guide establishes the reviewed object. At printed page 1/
PDF file page 4, CMS describes validation of CY2022 prescription drug event
(PDE) data. Printed page 2/PDF file page 5 identifies a sample of PDE records;
printed page 23/PDF file page 26 ties those records to the sponsor contract; and
printed page 36/PDF file page 39 anchors the frame to the June 29, 2023 cutoff
and final reconciliation PDE.

The FY2024 findings close the cost-basis portion at printed and file page 2.
CMS compares corrected GDC with PDE GDC and records the difference as a partial
GDC error. Footnote 3 defines GDC as ingredient cost paid, dispensing fee paid,
sales tax, and vaccine administration fee. It also says GDC is conceptually the
combined plan and beneficiary liability, while government subsidizes portions
under statutory rules. Printed and file page 3 confirms the comparison in the
underpayment direction.

## Published denominator identification

The FY2024 findings at printed and file page 1 publish a **$96.52 billion Part D
Denominator**, a rounded $3.58 billion gross improper-payment numerator, and a
3.70% rate. PaymentAccuracy FY2024 annual source row 828 reports the exact
outlays value as **$96,521.39 million**, with $3,575.09 million improper,
$3,052.65 million overpaid, and $522.44 million underpaid. The outlays value is
$96.52139 billion and rounds to the CMS denominator.

That numeric and label bridge closes identification of the published
denominator. It does not disclose its component algebra and does not make GDC
equivalent to federal outlays.

## Residual payment-universe gaps

The full field still lacks a same-period authoritative definition and
enumeration of included and excluded streams. The unresolved taxonomy includes
the explicit treatment of direct subsidy or capitation, reinsurance, low-income
cost-sharing subsidy, risk sharing and reconciliation, premiums, beneficiary
cost sharing, manufacturer discounts, direct and indirect remuneration, and
employer subsidy. These are questions requiring source resolution, not claims
that each category belongs in the denominator.

Also unresolved are the bridge from GDC and plan/beneficiary liability to
federal payments and outlays; whether the denominator is federal outlays, GDC,
or a modeled or simulated measure and its period and settlement basis; negative
adjustments, reversals, post-reconciliation changes, duplicate, rejected, and
deleted PDEs; covered plan, contract, and beneficiary populations; benefit
phases, subsidy components, and overlap controls; gross/net, recovery, offset,
and rounding treatment; and the connection among the PDE sample, beneficiary
simulation, denominator categories, and weights.

## Guardrails and status

A PDE record must not be treated as proof of the underlying individual pharmacy
claim. The PDE/GDC review object is not the whole payment universe. GDC combines
plan and beneficiary liability and must not be labeled federal outlay;
government subsidizes only
portions. Plan or beneficiary liabilities cannot be added to outlays without an
authoritative accounting bridge and double-count controls. Neither the
$96.52 billion label nor its row-828 reconciliation proves composition or GDC
equivalence, and sponsor-contract reconciliation PDEs do not prove one-to-one
coverage of every payment stream.

One narrow component closes internally, but the full `payment universe` field
stays open. Medicare Part D remains three fields closed and five open. Every
public-claim, field-closure, scoring, fraud, waste, debt, collectibility,
recovery, and savings gate remains false.
