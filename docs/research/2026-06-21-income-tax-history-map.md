# Income-Tax History Map

## Decision supported

This note supports the first TAXLANE history record. It identifies the legal and
administrative events that must be represented before TAXLANE publishes a public
story about what the income tax was created to fund or how it became a durable
mass tax.

## Research question

Which primary-source milestones should anchor the first federal income-tax
history map, and how should those milestones constrain later public-purpose tax
lane claims?

## Scope

This is a history scaffold, not a full rate extraction. Later data work should
parse IRS Historical Table 23 and Statutes at Large text into year-by-year rate,
exemption, and bracket records.

## Findings

### TAXLANE-HIST-01: The first federal income tax was a wartime revenue
experiment, not the original baseline of peacetime federal finance

- **Sources**:
  - `SRC-SENATE-1861-REV-ACT`
  - `SRC-FRASER-1861-REV-ACT`
  - `SRC-ARCHIVES-16A`
- **Observation**: The 1861 income tax belongs in the Civil War revenue frame.
  The Senate history describes the act as a special-session war finance measure
  that taxed imports, imposed a direct land tax, and added a 3 percent tax on
  individual incomes above $800. The National Archives summary likewise treats
  Civil War fiscal needs as the prompt for the first American income tax.
- **Implication**: TAXLANE should not present the income tax as a timeless
  default funding instrument. The first history lane should mark it as
  `wartime-revenue-experiment`.
- **Confidence**: High for event classification; exact statutory language still
  needs page/section extraction from the Statutes at Large PDF.

### TAXLANE-HIST-02: Repeal matters because it shows the early income tax was
not yet a permanent civic financing standard

- **Sources**:
  - `SRC-ARCHIVES-16A`
  - `SRC-IRS-HISTORY-HIGHLIGHTS`
- **Observation**: The National Archives summary records that Congress repealed
  the Civil War income tax in 1872. That repeal is a distinct timeline event,
  not a footnote, because it separates the wartime tax experiment from the later
  constitutional and statutory design.
- **Implication**: TAXLANE should include an event class for `repeal-expiration`
  and should avoid implying continuous federal income-tax collection from 1861
  to the present.
- **Confidence**: High for event existence; future work should cite the repeal
  statute directly before extracting detailed repeal rules.

### TAXLANE-HIST-03: Pollock created a constitutional constraint that shaped the
Sixteenth Amendment

- **Sources**:
  - `SRC-LOC-POLLOCK-157-US-429`
  - `SRC-ARCHIVES-16A`
  - `SRC-CAPITOL-16A`
- **Observation**: The Pollock decision invalidated parts of the 1894 income tax
  as unapportioned direct taxes. National Archives and Capitol Visitor Center
  materials connect that constitutional constraint to the later amendment
  campaign.
- **Implication**: The history map must distinguish Congress's desire to tax
  income from Congress's constitutional authority to do so without
  apportionment. This affects TAXLANE language: "income tax authority" is not a
  single event.
- **Confidence**: High for the legal sequence; future work should separate the
  first Pollock decision from the rehearing record if the public reader needs
  doctrinal detail.

### TAXLANE-HIST-04: The Sixteenth Amendment authorized income taxes without
apportionment, but it did not itself allocate income-tax receipts to programs

- **Sources**:
  - `SRC-ARCHIVES-16A`
  - `SRC-CAPITOL-16A`
- **Observation**: The Sixteenth Amendment is an authority event: Congress may
  lay and collect taxes on incomes without apportionment among the states or
  regard to census enumeration. It is not, by itself, a program-linked funding
  formula.
- **Implication**: TAXLANE should classify this as
  `constitutional-authorization`, not as evidence that income-tax receipts are
  legally dedicated to any public-purpose lane.
- **Confidence**: High.

### TAXLANE-HIST-05: The 1913 Revenue Act reestablished the income tax as a
statutory instrument after constitutional authorization

- **Sources**:
  - `SRC-GOVINFO-1913-REV-ACT`
  - `SRC-FRASER-1913-UNDERWOOD`
  - `SRC-IRS-SOI-HT23`
- **Observation**: The 1913 statute is the starting point for the modern
  post-amendment income-tax record. It should be linked to the rate spine but
  kept separate from the amendment itself.
- **Implication**: The future `rates_timeline` needs at least two authority
  fields: constitutional authority and statutory rate/base authority.
- **Confidence**: High for event placement; exact bracket/rate extraction should
  come from IRS SOI Historical Table 23 and the statutory text.

### TAXLANE-HIST-06: World War II transformed the income tax into a mass
administrative system

- **Sources**:
  - `SRC-IRS-HISTORY-HIGHLIGHTS`
  - `SRC-DOL-1942-REV-ACT`
  - `SRC-SENATE-1943-CURRENT-PAYMENT`
- **Observation**: IRS history records that the Revenue Act of 1942 sharply
  increased taxes and the number of Americans subject to income tax, and that
  the Current Tax Payment Act of 1943 required employers to withhold wage taxes.
  The Department of Labor summary describes the wartime expansion from a small
  payer base toward a broad worker base.
- **Implication**: TAXLANE should treat 1942 and 1943 as administrative-scale
  events, not only rate events. A public reader should learn when income tax
  became something ordinary wage earners experienced through withholding.
- **Confidence**: Medium-high. The IRS and DOL summaries are official, but the
  1942 and 1943 statutes should be added as direct statutory sources before
  publishing a detailed legal timeline.

### TAXLANE-HIST-07: The modern code is a consolidation and reform surface, not
the origin of income-tax authority

- **Sources**:
  - `SRC-GOVINFO-1954-IRC`
  - `SRC-GOVINFO-1986-TRA`
  - `SRC-USCODE-T26`
  - `SRC-IRS-CODE-GUIDANCE`
- **Observation**: The 1954 Internal Revenue Code and 1986 Tax Reform Act are
  modern-code structure and reform events. IRS guidance points readers to Title
  26 as the current public code surface.
- **Implication**: TAXLANE should separate "why Congress can tax income" from
  "where current income-tax law is organized." This prevents public explanations
  from overloading the Internal Revenue Code as if it were the constitutional
  source.
- **Confidence**: High for event class; future work should add a current-law
  explainer in Pulse 11.

## Timeline scaffold

| Year | Event class | Event | TAXLANE use |
|---:|---|---|---|
| 1861 | `wartime-revenue-experiment` | Revenue Act of 1861 creates a Civil War income-tax measure. | Show origin as war finance, not permanent peacetime baseline. |
| 1872 | `repeal-expiration` | Civil War income tax is repealed. | Break continuity before the 1894 and 1913 story. |
| 1894 | `statutory-reauthorization-attempt` | Wilson-Gorman era income tax is enacted. | Mark as pre-amendment attempt subject to constitutional challenge. |
| 1895 | `constitutional-constraint` | Pollock invalidates key parts of the income tax as unapportioned direct taxes. | Explain why an amendment became politically and legally important. |
| 1909 | `constitutional-proposal` | Congress proposes the Sixteenth Amendment. | Separate proposal from ratification and statute. |
| 1913 | `constitutional-authorization` | Sixteenth Amendment is ratified. | Establish income-tax authority without program dedication. |
| 1913 | `statutory-reauthorization` | Revenue Act of 1913 reestablishes the federal income tax. | Start the post-amendment rate/base timeline. |
| 1942 | `mass-tax-expansion` | Revenue Act of 1942 broadens the income-tax base during World War II. | Explain the move from elite tax to broad wage-earner tax. |
| 1943 | `withholding-administration` | Current Tax Payment Act requires employer withholding. | Explain the pay-as-you-go administrative model. |
| 1954 | `code-consolidation` | Internal Revenue Code of 1954 reorganizes federal tax law. | Connect history to current-code structure. |
| 1986 | `code-reform` | Tax Reform Act of 1986 reforms the Internal Revenue Code. | Anchor modern reform baseline and current Title 26 framing. |

## Design rules for later TAXLANE outputs

1. Historical origin claims must carry an event class.
2. Rate claims must cite the statute or IRS historical table separately from the
   constitutional authority claim.
3. "What it was supposed to fund" must distinguish wartime revenue, general
   revenue, and legally dedicated program receipts.
4. Withholding should be treated as collection administration, not as a separate
   public-purpose tax.
5. Modern-code citations should point to Title 26 or statute sections, while
   public-purpose allocation claims should point to budget and appropriations
   sources.

## Adopt now

- Use the timeline scaffold as the default order for the public history packet.
- Add `event_class`, `authority_source_id`, `rate_source_id`, and
  `allocation_status` fields to the future history/rates data model.
- Treat 1942 and 1943 as required public-reader events because they explain why
  income tax became a mass taxpayer experience.

## Prototype behind a boundary

- A visual "authority ladder" that separates Constitution, statute, code,
  budget receipts, appropriations, and taxpayer receipts.
- A paired timeline with top marginal rates from IRS SOI Historical Table 23.

## Defer

- Do not publish exact 1861, 1894, 1913, 1942, or 1986 rate schedules until the
  source text and IRS table are extracted into a checked data table.
- Do not choose a final reform narrative from this history note alone.
