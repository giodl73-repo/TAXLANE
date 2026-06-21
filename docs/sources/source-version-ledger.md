# Source Version Ledger

## Purpose

This ledger records the official source versions TAXLANE may use for rates,
receipts, outlays, accounting definitions, and taxpayer-facing allocation models.
No extracted data table should be published unless its source appears here with a
source ID, publisher, URL, observed date, coverage, and extraction rule.

## Ledger rules

1. Record the source before extracting or summarizing data.
2. Prefer primary government sources for legal authority, rates, receipts,
   outlays, fund accounting, and budget concepts.
3. Store source ID, publisher, URL, observed date, coverage years, and update
   cadence when known.
4. Record whether a source is authoritative, contextual, or candidate-only.
5. Do not mix fiscal years and tax years without an explicit field.
6. Keep raw downloaded data out of the repo until source custody and generated
   artifact rules are defined.

## Required source fields

| Field | Meaning |
|---|---|
| Source ID | Stable repo-local ID, for example `SRC-OMB-HIST-2-1-FY2027`. |
| Publisher | Official publisher or steward. |
| URL | Canonical source URL. |
| Observed date | Date this repo observed or fetched the source. |
| Coverage | Tax years, fiscal years, or legal date range covered. |
| Update cadence | Annual, historical/static, dynamic, or unknown. |
| Status | Authoritative, contextual, candidate, or deferred. |
| Extraction rule | What may be extracted and what must be labeled. |

## Legal authority sources

| Source ID | Publisher | URL | Observed date | Coverage | Update cadence | Status | Extraction rule |
|---|---|---|---|---|---|---|---|
| `SRC-ARCHIVES-16A` | National Archives | <https://www.archives.gov/milestone-documents/16th-amendment> | 2026-06-21 | Sixteenth Amendment; ratified 1913 | Historical/static | Authoritative | Use for constitutional authority text and ratification context. Quote only with citation. |
| `SRC-CAPITOL-16A` | U.S. Capitol Visitor Center / National Archives records | <https://www.visitthecapitol.gov/artifact/sj-res-40-joint-resolution-proposing-amendment-constitution-united-states-sixteenth> | 2026-06-21 | Joint resolution proposing Sixteenth Amendment; ratified 1913 | Historical/static | Contextual | Use to cross-check the amendment proposal and public artifact context. Do not use as the sole authority for constitutional text. |
| `SRC-SENATE-1861-REV-ACT` | U.S. Senate Historical Office | <https://www.senate.gov/artandhistory/history/common/civil_war/RevenueAct_FeaturedDoc.htm> | 2026-06-21 | Revenue Act of 1861 summary and Civil War context | Historical/static | Contextual | Use for official congressional history framing. Cite Statutes at Large or FRASER PDF for statutory extraction. |
| `SRC-FRASER-1861-REV-ACT` | FRASER / Federal Reserve Bank of St. Louis | <https://fraser.stlouisfed.org/title/revenue-act-1861-1117> | 2026-06-21 | Revenue Act of 1861, 12 Stat. 292 | Historical/static | Authoritative copy | Use for statutory page/section extraction if GovInfo source is unavailable or less accessible. |
| `SRC-LOC-POLLOCK-157-US-429` | Library of Congress, U.S. Reports | <https://tile.loc.gov/storage-services/service/ll/usrep/usrep157/usrep157429/usrep157429.pdf> | 2026-06-21 | Pollock v. Farmers' Loan & Trust Co., 157 U.S. 429 (1895) | Historical/static | Authoritative | Use for the 1895 constitutional constraint event. Record page and holding before quoting. |
| `SRC-GOVINFO-1913-REV-ACT` | GovInfo / U.S. Statutes at Large | <https://www.govinfo.gov/app/details/STATUTE-38/STATUTE-38-Pg114> | 2026-06-21 | Revenue Act of 1913, 38 Stat. 114 | Historical/static | Authoritative | Use for statutory starting point and original TAXLANE provisions. Record page/section before quoting. |
| `SRC-FRASER-1913-UNDERWOOD` | FRASER / Federal Reserve Bank of St. Louis | <https://fraser.stlouisfed.org/files/docs/historical/congressional/underwood-tariff-1913.pdf> | 2026-06-21 | Revenue Act of 1913 / Underwood Tariff Act copy | Historical/static | Authoritative copy | Use as accessible statutory text for the 1913 income-tax section. Cross-check with GovInfo before final extraction. |
| `SRC-IRS-HISTORY-HIGHLIGHTS` | Internal Revenue Service | <https://www.irs.gov/newsroom/historical-highlights-of-the-irs> | 2026-06-21 | IRS institutional and tax-administration timeline | IRS-maintained | Contextual | Use for official IRS timeline context, especially 1942 and 1943 administration milestones. Direct statute still required for legal extraction. |
| `SRC-DOL-1942-REV-ACT` | U.S. Department of Labor | <https://www.dol.gov/node/22387> | 2026-06-21 | Revenue Act of 1942 public history context | Historical/static | Contextual | Use for official public framing of wartime mass-tax expansion. Direct statute still required for rates and legal detail. |
| `SRC-SENATE-1943-CURRENT-PAYMENT` | U.S. Senate Finance Committee archive | <https://www.finance.senate.gov/download/1946/03/04/legislative-history-of-the-current-tax-payment-act-of-1943> | 2026-06-21 | Legislative history of Current Tax Payment Act of 1943 | Historical/static | Contextual | Use for withholding/pay-as-you-go legislative history. Add Statutes at Large source before extracting statutory rules. |
| `SRC-GOVINFO-1954-IRC` | GovInfo / U.S. Statutes at Large | <https://www.govinfo.gov/content/pkg/STATUTE-68/pdf/STATUTE-68A-Pg1.pdf> | 2026-06-21 | Internal Revenue Code of 1954, 68A Stat. 1 | Historical/static | Authoritative | Use for code-consolidation event and current-code lineage. |
| `SRC-GOVINFO-1986-TRA` | GovInfo / U.S. Statutes at Large | <https://www.govinfo.gov/app/details/STATUTE-100/STATUTE-100-Pg2085/summary> | 2026-06-21 | Tax Reform Act of 1986, Pub. L. 99-514, 100 Stat. 2085 | Historical/static | Authoritative | Use for modern reform baseline and 1986 code-reform event. |
| `SRC-USCODE-T26` | Office of the Law Revision Counsel, U.S. House of Representatives | <https://uscode.house.gov/browse/prelim@title26&edition=prelim> | 2026-06-21 | Current Title 26, Internal Revenue Code | Current/preliminary | Authoritative current code | Use for current-code references. Record access date and edition because current code changes. |
| `SRC-IRS-CODE-GUIDANCE` | Internal Revenue Service | <https://www.irs.gov/privacy-disclosure/tax-code-regulations-and-official-guidance> | 2026-06-21 | IRS public guidance on tax code, regulations, and official guidance | IRS-maintained | Contextual | Use for reader-facing explanation that federal tax law is generally enacted in the IRC and published in Title 26. |

## TAXLANE rate sources

| Source ID | Publisher | URL | Observed date | Coverage | Update cadence | Status | Extraction rule |
|---|---|---|---|---|---|---|---|
| `SRC-IRS-SOI-HT23` | IRS Statistics of Income | <https://www.irs.gov/statistics/soi-tax-stats-historical-table-23> | 2026-06-21; page reported last reviewed/updated 2026-03-17 | Individual income tax rates and brackets from 1913 through the table's current endpoint | Periodic/IRS-maintained | Authoritative | Use as rates spine. Extract tax year, filing status when available, rate, bracket threshold, exemption/standard deduction context, and table version. |

## Receipts and outlays sources

| Source ID | Publisher | URL | Observed date | Coverage | Update cadence | Status | Extraction rule |
|---|---|---|---|---|---|---|---|
| `SRC-OMB-HIST-FY2027` | OMB | <https://www.whitehouse.gov/omb/information-resources/budget/historical-tables/> | 2026-06-21 | Historical tables listed as FY2027 release | Annual | Authoritative | Use as the index for OMB historical spreadsheet versions. Record table ID and FY release in extracted records. |
| `SRC-OMB-HIST-1-1-FY2027` | OMB | <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist01z1_fy2027.xlsx> | 2026-06-21 | Receipts, outlays, and surpluses/deficits, 1789-2025 | Annual | Authoritative | Use for top-level fiscal spine. Label as fiscal-year federal totals. |
| `SRC-OMB-HIST-1-4-FY2027` | OMB | <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist01z4_fy2027.xlsx> | 2026-06-21 | Receipts, outlays, and surpluses/deficits by fund group, 1934-2025 | Annual | Authoritative | Use for federal fund/trust fund split. Do not imply legal earmark without budget-concepts support. |
| `SRC-OMB-HIST-2-1-FY2027` | OMB | <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist02z1_fy2027.xlsx> | 2026-06-21 | Receipts by source, 1934-2031 | Annual | Authoritative | Use for individual income tax, corporate income tax, social insurance, excise, and other receipt categories. Label projections separately from actuals. |
| `SRC-OMB-HIST-2-2-FY2027` | OMB | <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist02z2_fy2027.xlsx> | 2026-06-21 | Percentage composition of receipts by source, 1934-2031 | Annual | Authoritative | Use for receipt-share views. Label percentages as OMB historical-table values. |
| `SRC-OMB-HIST-2-4-FY2027` | OMB | <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist02z4_fy2027.xlsx> | 2026-06-21 | Social insurance, retirement receipts, and excise tax composition, 1940-2031 | Annual | Authoritative | Use to keep payroll/social-insurance receipts separate from individual income taxes. |
| `SRC-OMB-HIST-3-1-FY2027` | OMB | <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist03z1_fy2027.xlsx> | 2026-06-21 | Outlays by superfunction and function, 1940-2031 | Annual | Authoritative | Use for public-purpose outlay categories. Label projections separately from actuals. |
| `SRC-OMB-HIST-3-2-FY2027` | OMB | <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist03z2_fy2027.xlsx> | 2026-06-21 | Outlays by function and subfunction, 1962-2031 | Annual | Authoritative | Use for layered public-purpose explanation below function level. |
| `SRC-OMB-HIST-4-1-FY2027` | OMB | <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist04z1_fy2027.xlsx> | 2026-06-21 | Outlays by agency, 1962-2031 | Annual | Authoritative | Use when explaining agency-level spending. Do not substitute agency for public-purpose function without a crosswalk. |
| `SRC-OMB-HIST-8-5-FY2027` | OMB | <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist08z5_fy2027.xlsx> | 2026-06-21 | Outlays for mandatory and related programs, 1962-2031 | Annual | Authoritative | Use for mandatory program views. Keep eligibility/beneficiary claims separate from spending totals. |
| `SRC-OMB-HIST-8-7-FY2027` | OMB | <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist08z7_fy2027.xlsx> | 2026-06-21 | Outlays for discretionary programs, 1962-2031 | Annual | Authoritative | Use for discretionary program views and comparison with mandatory spending. |
| `SRC-OMB-HIST-11-3-FY2027` | OMB | <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist11z3_fy2027.xlsx> | 2026-06-21 | Payments for individuals by category and major program, 1940-2031 | Annual | Authoritative | Use for beneficiary-facing spending views. Do not equate payments for individuals with all public benefits. |

## Budget accounting definition sources

| Source ID | Publisher | URL | Observed date | Coverage | Update cadence | Status | Extraction rule |
|---|---|---|---|---|---|---|---|
| `SRC-OMB-AP-FY2027` | OMB | <https://www.whitehouse.gov/omb/information-resources/budget/analytical-perspectives/> | 2026-06-21 | Analytical Perspectives, FY2027 release | Annual | Authoritative | Use as the index for budget concept chapters. Quote exact definitions only from chapter PDFs. |
| `SRC-OMB-AP-6-CONCEPTS-FY2027` | OMB | <https://www.whitehouse.gov/wp-content/uploads/2026/04/ap_6_concepts_fy2027.pdf> | 2026-06-21 | Budget concepts | Annual | Authoritative | Use for exact definitions of budget terms before public receipt claims. |
| `SRC-OMB-AP-8-RECEIPTS-FY2027` | OMB | <https://www.whitehouse.gov/wp-content/uploads/2026/04/ap_8_receipts_fy2027.pdf> | 2026-06-21 | Governmental receipts | Annual | Authoritative | Use for receipt definitions and receipt-source treatment. |
| `SRC-OMB-AP-9-OFFSETTING-FY2027` | OMB | <https://www.whitehouse.gov/wp-content/uploads/2026/04/ap_9_offsetting_fy2027.pdf> | 2026-06-21 | Offsetting collections and offsetting receipts | Annual | Authoritative | Use to distinguish governmental receipts from business-like collections. |
| `SRC-OMB-AP-13-FUNDS-FY2027` | OMB | <https://www.whitehouse.gov/wp-content/uploads/2026/04/ap_13_funds_fy2027.pdf> | 2026-06-21 | Trust funds and federal funds | Annual | Authoritative | Use before any claim about earmarks, trust funds, or dedicated program financing. |
| `SRC-GAO-BUDGET-GLOSSARY-2005` | U.S. Government Accountability Office | <https://www.govinfo.gov/content/pkg/GAOREPORTS-GAO-05-734SP/html/GAOREPORTS-GAO-05-734SP.htm> | 2026-06-21 | Federal budget process glossary | Historical/static | Authoritative glossary | Use for budget-process terms such as earmarking, collections, obligations, and fund-account vocabulary. |
| `SRC-GAO-TRUST-DEDICATED-FUNDS-2020` | U.S. Government Accountability Office | <https://www.gao.gov/products/gao-20-156> | 2026-06-21 | Federal trust funds and other dedicated funds | Historical/static | Contextual | Use for trust/dedicated-fund design risks and oversight framing. Do not use as a substitute for statutory authority. |
| `SRC-GAO-USER-FEE-DESIGN-2008` | U.S. Government Accountability Office | <https://www.gao.gov/products/gao-08-386sp> | 2026-06-21 | Federal user-fee design guide | Historical/static | Contextual | Use to distinguish user fees from broad taxes and to identify beneficiary-pays design issues. |
| `SRC-CBO-BUDGET-TERMS-2021` | Congressional Budget Office | <https://www.cbo.gov/publication/57660> | 2026-06-21 | Common federal budget terms | CBO-maintained | Contextual | Use as plain-language cross-check for budget authority, obligations, outlays, deficits, and debt. |
| `SRC-CBO-TRUST-FUNDS-2020` | Congressional Budget Office | <https://www.cbo.gov/publication/56523> | 2026-06-21 | Outlook for major federal trust funds, 2020 to 2030 | Historical/static | Contextual | Use for trust-fund balance, income, and spending risk framing. Verify current projections before using for current-year numbers. |

## Public spending and receipt-context sources

| Source ID | Publisher | URL | Observed date | Coverage | Update cadence | Status | Extraction rule |
|---|---|---|---|---|---|---|---|
| `SRC-TREASURY-AFG` | U.S. Treasury Fiscal Data | <https://fiscaldata.treasury.gov/americas-finance-guide/> | 2026-06-21 | Dynamic public finance guide | Dynamic | Contextual | Use for public-facing framing and cross-checking Treasury presentation. Do not use as sole historical data source. |
| `SRC-TREASURY-MTS` | U.S. Treasury Fiscal Data | <https://fiscaldata.treasury.gov/datasets/monthly-treasury-statement/> | 2026-06-21 | Monthly Treasury Statement receipts, outlays, and deficit/surplus data | Monthly/dynamic | Contextual | Use for current-period receipt/outlay context after recording query date and period. Prefer OMB historical tables for annual spine. |
| `SRC-USASPENDING` | USAspending.gov | <https://www.usaspending.gov/> | 2026-06-21 | Federal spending search and account/program detail | Dynamic | Contextual | Use for program/account exploration after OMB function-level records. Record query parameters before citing. |
| `SRC-CBO-BUDGET-DATA` | CBO | <https://www.cbo.gov/data/budget-economic-data> | 2026-06-21; automated fetch blocked with 403 | Budget/economic data | Periodic | Candidate | Use only after manual verification or accessible source capture. Candidate cross-check, not required spine. |

## Current taxpayer mechanics sources

| Source ID | Publisher | URL | Observed date | Coverage | Update cadence | Status | Extraction rule |
|---|---|---|---|---|---|---|---|
| `SRC-IRS-RATES-BRACKETS` | IRS | <https://www.irs.gov/filing/federal-income-tax-rates-and-brackets> | 2026-06-21; page reported last reviewed/updated 2026-02-10 | Current federal income-tax rate and bracket explainer, including 2025 bracket tables | Annual/IRS-maintained | Authoritative | Use for current marginal bracket explanation and current-year bracket extraction. Record tax year and filing status. |
| `SRC-IRS-P17` | IRS | <https://www.irs.gov/publications/p17> | 2026-06-21 | Publication 17, Your Federal Income Tax | Annual/IRS-maintained | Authoritative guidance | Use for general individual filing mechanics. Do not substitute for statute when a legal rule needs exact authority. |
| `SRC-IRS-F1040` | IRS | <https://www.irs.gov/forms-pubs/about-form-1040> | 2026-06-21 | Form 1040, U.S. Individual Income Tax Return | Annual/IRS-maintained | Authoritative form source | Use for return-flow mapping: income, deductions, tax, credits, payments, refund or amount due. |
| `SRC-IRS-CREDITS-DEDUCTIONS` | IRS | <https://www.irs.gov/credits-and-deductions> | 2026-06-21; page reported last reviewed/updated 2026-05-26 | Current IRS credits and deductions index | IRS-maintained | Authoritative guidance | Use for the distinction that credits reduce tax due and deductions reduce taxable income. |
| `SRC-IRS-WITHHOLDING-ESTIMATOR` | IRS | <https://www.irs.gov/individuals/tax-withholding-estimator> | 2026-06-21; page reported last reviewed/updated 2026-03-26 | Tax withholding estimator and W-4/W-4P update guidance | IRS-maintained | Authoritative guidance | Use to explain withholding as payment timing and employer/pension-provider withholding, not spending allocation. |
| `SRC-IRS-1040ES` | IRS | <https://www.irs.gov/forms-pubs/about-form-1040-es> | 2026-06-21; page reported last reviewed/updated 2026-04-15 | Form 1040-ES estimated tax for individuals | Annual/IRS-maintained | Authoritative guidance | Use to explain estimated tax for income not subject to withholding. |
| `SRC-IRS-DATA-BOOK` | IRS Statistics of Income | <https://www.irs.gov/statistics/soi-tax-stats-irs-data-book> | 2026-06-21 | IRS Data Book, including returns, collections, refunds, service, and enforcement statistics | Annual/IRS-maintained | Contextual | Use for administration scale and collection/refund context. Do not use as program allocation proof. |

## Extraction status

| Source family | Status | Next action |
|---|---|---|
| Constitutional/statutory authority | Recorded | Build history timeline records with page/section anchors. |
| IRS historical rates | Recorded | Download table and define `rates_timeline` schema. |
| OMB receipts/outlays | Recorded | Download spreadsheets and define fiscal-year data dictionary. |
| OMB budget concepts | Recorded | Extract exact definitions for accounting explainer. |
| Treasury/USAspending context | Recorded | Use after OMB spine exists; record queries and dynamic dates. |
| CBO | Candidate | Verify manually before using as a cited cross-check. |
