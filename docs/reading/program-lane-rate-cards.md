# Program-Lane Rate Cards — Why Each Rate Makes Sense

**What this is:** one card per public-purpose lane. Each card shows the rate (as a
share of the federal tax dollar), who pays for it, the argument for that level, and
how the rate would move up or down. These are **reform proposals**, not current
law, and they describe a *modeled* allocation — not a legal tracing of your
specific dollars. The borrowed share is shown, never hidden.

Data: `data/derived/program_lane_rate_model/lane_rate_cards.v1.draft.jsonl`.

---

## How to read a card

- **Rate** = share of a fully-funded federal tax dollar (Social Security 22.6¢,
  Defense 13.1¢, and so on), i.e. cents per dollar of FY2025 outlays. For
  income-tax-funded lanes this also maps to a percent of income (the whole income
  tax is ~14.2% of AGI on a TY2022 base — a cross-year illustration, not a
  filing-year rate).
- **Funded by** = which tax actually pays. Payroll and trust-fund excise are
  *already* legally dedicated; income and corporate are general revenue.
- **Why this rate** = the argument from four anchors — what it costs, what
  solvency requires, what US history shows, and what peer countries spend.
- **How it moves** = the triggers that would raise or lower it.

---

## The big lanes (≈90% of the budget)

### Social Security — 22.6¢
**Funded by:** its own OASDI payroll tax (12.4%, capped at $184,500).
**Why this rate:** It is a paid-in, earned benefit funded by its own dedicated
tax — the one lane where dedication already works. Public pension spending (7.3%
of GDP) is near the OECD norm (8.1%), and the 12.4% rate is *below* peers (Germany
18.6%, Japan 18.3%). But the US uniquely **caps** the taxed wage base where peers
often don't, so the fair lever is the **cap, not the rate**.
**Moves up** on a trust-fund shortfall (FY2025 already ran a ~$297B gap covered by
drawdown). **Moves down** only on sustained surplus or benefit-formula reform.

### Medicare — 14.2¢
**Funded by:** HI payroll (2.9%) + premiums + general revenue.
**Why this rate:** Only Part A is payroll-funded; most of Medicare is general
revenue. The US pays roughly **twice** the OECD rate for health overall, so the
honest argument is to **bend delivery cost**, not raise the rate to feed a system
that already costs 2x peers. **Moves up** with an aging population and provider
inflation; **moves down** with delivery and drug-price efficiency.

### Health, non-Medicare — 14.0¢
**Funded by:** general revenue (income tax).
**Why this rate:** The clearest efficiency case in the entire budget. US
government/compulsory health is **14.3% of GDP vs the OECD's 7.1%** — about double,
for narrower coverage. Fund the coverage; drive the **per-unit price** toward peer
levels. A 20% efficiency gain across the federal health lanes frees ~$395B.
**Moves up** with coverage expansion; **moves down** as price-per-outcome falls.

### Debt Service / Net Interest — 13.8¢
**Funded by:** general revenue (it is a financing cost, not a program).
**Why this rate:** This is the bill for *past* borrowing — already larger than
defense. It is **senior and uncuttable**, and it is the single best argument for
balancing now: every year of deficit makes this lane bigger and crowds out
everything else. **Moves up** with deficits and interest rates; **moves down** only
by running surpluses and paying down debt.

### National Defense — 13.1¢
**Funded by:** general revenue (income tax).
**Why this rate:** A textbook public good. The US spends **~3.0% of GDP** on the
OMB budget-function basis (3.2-3.4% on the broader SIPRI/NATO measure) — above
NATO-Europe (~2%) and the 2% floor — so the rate is a **policy choice within a
defensible 2.0-3.5% band**, now pulled upward by the **2025 Hague 3.5% core-defence
target** (5% total including a 1.5% broader-security component). **Moves up** on new commitments or conflict; **moves down** with
allied burden-sharing or drawdown.

### Income Security & Family — 10.0¢
**Funded by:** income tax + dedicated unemployment payroll.
**Why this rate:** The safety net and family support. US family benefits are
**under 1% of GDP vs the OECD's 2.35%** — already below peers — so the international
anchor says protect or grow it, not cut. It must carry an automatic **downturn
shortfall rule** so a recession doesn't trigger cuts exactly when need peaks.
**Moves up** in recessions (automatic stabilizers); **moves down** in strong
economies.

### Veterans — 5.4¢
**Funded by:** general revenue.
**Why this rate:** Deferred compensation and an earned obligation — closer to a
contract than a discretionary choice. Protected for continuity. **Moves up** with
new cohorts and care needs; **moves down** slowly as caseloads decline.

---

## The mid and small lanes

| Lane | Rate | Why this rate (short) | Moves up on | Moves down on |
|---|---:|---|---|---|
| Transportation | 2.1¢ | Already a working dedicated trust-fund lane (Highway/Airport); clean candidate to formalize | infrastructure backlog; EV erosion of fuel tax | completed buildout |
| Constitutional Govt & Justice | 1.8¢ | Core rule-of-law public good; proposed corporate-funded (business relies on courts) | security/justice needs | efficiency |
| Nat. Resources/Energy/Environ. | 1.6¢ | Commons stewardship; mixed royalty/fee/excise | climate & disaster needs | royalty/fee recovery |
| Community/Disaster/Regional | 1.2¢ | Volatile (disasters) — reserve-rule candidate | disaster years | calm years (build reserve) |
| Education, Work & Social Services | 1.0¢ | Mostly state/local; near peer levels; proposed corporate-funded | skills gaps | state assumption |
| Agriculture & Food | 0.7¢ | Mixed support + insurance | farm crisis | strong farm economy |
| International Affairs | 0.6¢ | Far smaller than the public assumes; proposed corporate-funded | global crisis | drawdown |
| Science, Space & Civic Capacity | 0.6¢ | The research base no private actor fully funds; proposed corporate-funded | strategic R&D competition | mission completion |

---

## Sources

Per-lane costs and rates: `SRC-OMB-HIST-1-1-FY2027`, `SRC-OMB-HIST-3-2-FY2027`.
Funding sources: `SRC-OMB-HIST-2-1-FY2027`, `SRC-OMB-HIST-2-4-FY2027`. International
anchors: `SRC-OECD-REVSTATS-2025`, `SRC-OECD-HEALTH-2025`, `SRC-OECD-PENSIONS-2025`,
`SRC-SIPRI-MILEX-2024`, `SRC-NATO-DEFEXP-2025`, `SRC-WB-EDU-GDP`. Income base:
`SRC-IRS-SOI-1304`. OASDI rate/cap: `SRC-SSA-OACT`. All recorded in
`docs/sources/source-version-ledger.md`; derivations in the `docs/research/` notes.

## Public-use guardrail

These cards are an educational reform model. They use **modeled allocation** and
say so. They do not claim your specific income-tax dollars are legally dedicated to
any lane (except where they already are — payroll to Social Security and Medicare,
trust excise to Transportation). The borrowed share — about 25¢ of every dollar
today — stays visible until receipts cover outlays. No card is a fraud finding or
tax advice.
