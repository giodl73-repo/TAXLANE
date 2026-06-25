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
| `SRC-IRS-SOI-1304` | IRS Statistics of Income | <https://www.irs.gov/statistics/soi-tax-stats-individual-income-tax-returns-complete-report-publication-1304-basic-tables-part-1> | 2026-06-23 | Aggregate AGI, taxable income, income tax after credits, returns counts; complete report Tax Year 2022 | Annual/IRS-maintained | Authoritative | Use as the aggregate income base for statutory rate-on-base framing. Tax-year basis; do not equate with fiscal-year receipts. Record tax year. |

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

## International and comparative benchmark sources

These sources back the international and historical benchmark note
(`docs/research/2026-06-23-international-historical-benchmark.md`). They are
comparators for fair-rate design, not TAXLANE fiscal-spine data. Observe the
government-level and "compulsory vs general" scope caveats recorded in that note.

| Source ID | Publisher | URL | Observed date | Coverage | Update cadence | Status | Extraction rule |
|---|---|---|---|---|---|---|---|
| `SRC-OECD-REVSTATS-2025` | OECD | <https://www.oecd.org/en/publications/revenue-statistics-2025_3708be73-en.html> | 2026-06-23 | Total tax-to-GDP, all government, 38 members, through 2024 (provisional) | Annual | Authoritative comparator | Use for cross-country total tax burden. Always label as all-government basis, not US federal-only. |
| `SRC-OECD-PENSIONS-2025` | OECD | <https://www.oecd.org/en/publications/pensions-at-a-glance-2025_76510fe4-en.html> | 2026-06-23 | Public pension expenditure % GDP; latest harmonized 2021 | Biennial | Authoritative comparator | Use for public pension spend by country. Public-only; excludes private/occupational. |
| `SRC-OECD-HEALTH-2025` | OECD | <https://www.oecd.org/en/publications/health-at-a-glance-2025_a894f72e-en.html> | 2026-06-23 | Government/compulsory and total health spend % GDP, 2024 | Biennial | Authoritative comparator | Use for health spend. "Government/compulsory" includes US compulsory private insurance; label it. |
| `SRC-OECD-SOCX` | OECD | <https://www.oecd.org/en/data/datasets/social-expenditure-database-socx.html> | 2026-06-23 | Public social expenditure % GDP; family benefits; latest full year 2022 | Annual | Authoritative comparator | Use for total public social and family-benefit spend. Public basis; US net-total (incl. private) differs. |
| `SRC-OECD-GOV-GLANCE-2025` | OECD | <https://www.oecd.org/en/publications/government-at-a-glance-2025_0efd0bcd-en.html> | 2026-06-23 | General government expenditure % GDP, OECD basis, 2023 | Biennial | Authoritative comparator | Use for OECD-average general-government spending benchmark. |
| `SRC-IMF-FISCAL-MONITOR` | International Monetary Fund | <https://www.imf.org/external/datamapper/exp@FPP> | 2026-06-23 | General government total expenditure % GDP by country, 2024 est. | Periodic | Authoritative comparator | Use for single-vintage cross-country general-government spending. Estimates flagged. |
| `SRC-SIPRI-MILEX-2024` | Stockholm International Peace Research Institute | <https://www.sipri.org/sites/default/files/2025-04/2504_fs_milex_2024.pdf> | 2026-06-23 | Military expenditure % GDP by country, 2024 | Annual | Authoritative comparator | Use for defense burden. SIPRI definition is broader than NATO; do not merge series without a note. |
| `SRC-NATO-DEFEXP-2025` | NATO | <https://www.nato.int/content/dam/nato/webready/documents/finance/def-exp-2025-en.pdf> | 2026-06-23 | Defence expenditure % GDP, NATO definition, 2014-2025 (2024-25 est.) | Annual | Authoritative comparator | Use for NATO-definition defense burden and the 2%/Hague-5% guidelines. |
| `SRC-NATO-HAGUE-2025` | NATO | <https://www.nato.int/en/about-us/official-texts-and-resources/official-texts/2025/06/25/the-hague-summit-declaration> | 2026-06-24 | The Hague Summit Declaration, 25 June 2025: commitment to invest 5% of GDP annually on defence and security by 2035 (3.5% core defence on the NATO definition + up to 1.5% broader security); review in 2029 | Historical/static | Authoritative | Use for the Hague 5%-by-2035 commitment and its 3.5%+1.5% split. Quote the commitment text with citation; it is a political commitment, not appropriated spending. |
| `SRC-WB-EDU-GDP` | World Bank / UNESCO Institute for Statistics | <https://data.worldbank.org/indicator/SE.XPD.TOTL.GD.ZS> | 2026-06-23 | Government education expenditure % GDP by country, 2021-2022 | Annual | Authoritative comparator | Use for public education spend. Scope differs slightly from OECD EAG. |
| `SRC-SSA-OACT` | Social Security Administration, Office of the Chief Actuary | <https://www.ssa.gov/oact/cola/cbb.html> | 2026-06-23 | OASDI rate, taxable maximum, OASDI cost % GDP | Annual/projection | Authoritative | Use for US OASDI rate, wage-base cap, and cost-as-%-GDP projection. Trustees figures are projections. |

## US fiscal-ratio and projection sources

| Source ID | Publisher | URL | Observed date | Coverage | Update cadence | Status | Extraction rule |
|---|---|---|---|---|---|---|---|
| `SRC-OMB-HIST-1-2-FY2027` | OMB | <https://www.whitehouse.gov/omb/information-resources/budget/historical-tables/> | 2026-06-23 | Receipts, outlays, and surpluses/deficits as % of GDP, with GDP series, 1930-2031 | Annual | Authoritative | Use for federal receipts/outlays as % GDP and the GDP denominator. Federal/central scope only. |
| `SRC-OMB-HIST-6-1-FY2027` | OMB | <https://www.whitehouse.gov/wp-content/uploads/2026/04/hist06z1_fy2027.xlsx> | 2026-06-24 | Composition of Outlays, 1940-2031: national-defense vs nondefense outlays in current/constant dollars and as percentages of total outlays and of GDP | Annual | Authoritative | Use for the historical national-defense-as-%-GDP series (Korean-War-era peak, Reagan buildup, post-Cold-War low). OMB budget-function (050) basis, not the SIPRI/NATO definition; label projections (FY2026-2031) separately from actuals. |
| `SRC-CBO-BUDGET-OUTLOOK` | Congressional Budget Office | <https://www.cbo.gov/publication/61307> | 2026-06-23 | Federal receipts/outlays % GDP, historical averages and projections | Periodic | Authoritative | Use for 50-year averages and forward projections. Label projections explicitly. |

## Health outcomes and cost-driver sources

Back the research-program papers (health track). Outcome and price-driver
comparators; label scope per source.

| Source ID | Publisher | URL | Observed date | Coverage | Update cadence | Status | Extraction rule |
|---|---|---|---|---|---|---|---|
| `SRC-CENSUS-P60-288` | U.S. Census Bureau | <https://www.census.gov/library/publications/2025/demo/p60-288.html> | 2026-06-24 | Health insurance coverage: 8.0% uninsured (~27.1M), 2024 | Annual | Authoritative | Use for US uninsured rate/count. Calendar-year, not fiscal-year. |
| `SRC-COMMONWEALTH-USHC` | Commonwealth Fund | <https://www.commonwealthfund.org/publications/issue-briefs/2026/may/us-health-care-global-perspective-2026> | 2026-06-24 | US is the only high-income country without universal coverage; cross-country health comparison | Periodic | Contextual | Use for the universal-coverage point and peer context, not as primary spending data. |
| `SRC-JAMA-PAPANICOLAS-2018` | Papanicolas, Woskie, Jha — JAMA 2018;319(10):1024-1039 | <https://jamanetwork.com/journals/jama/fullarticle/2674671> | 2026-06-24 | Cross-country evidence that higher US health spending is driven by prices and administration, not utilization | Static (peer-reviewed) | Authoritative (literature) | Use to support "price, not volume." Cite as published study; figures are 2013-era. |
| `SRC-CBO-LTBO` | Congressional Budget Office | <https://www.cbo.gov/publication/61270> | 2026-06-24 | Long-Term Budget Outlook: Medicare 3.1%→5.2% of GDP, major health programs 5.8%→8.1% by 2055; 65+ entitlements 40%→>50% of noninterest spending | Annual | Authoritative | Use for the long-run federal health trajectory and the aging driver. Projections — label as such. |

## Revenue-option and debt-trajectory sources

Back the revenue/solvency track. Scored federal options and the long-run debt path;
all are projections — label as such.

| Source ID | Publisher | URL | Observed date | Coverage | Update cadence | Status | Extraction rule |
|---|---|---|---|---|---|---|---|
| `SRC-CBO-TAX-EXP-2025` | Congressional Budget Office | <https://www.cbo.gov/publication/61172> | 2026-06-24 | Federal tax expenditures ~$2.3T / 7.6% of GDP, FY2025 (compiles JCT/Treasury) | Annual | Authoritative | Use for the tax-expenditure total. Includes payroll-tax effects; larger than Treasury income-tax-only totals. |
| `SRC-TREASURY-OTA-TE` | U.S. Treasury, Office of Tax Analysis | <https://home.treasury.gov/policy-issues/tax-policy/tax-expenditures> | 2026-06-24 | Annual tax-expenditure estimates by provision | Annual | Authoritative | Use for per-provision tax-expenditure detail; income-tax basis. |
| `SRC-CBO-BUDGET-OPTIONS` | Congressional Budget Office | <https://www.cbo.gov/budget-options/60961> | 2026-06-24 | Options for Reducing the Deficit 2025-2034: 5% VAT (~$3.4-3.5T broad / $2.2-2.3T narrow, 10yr); SS taxable-max options (#60955: ~$0.8T at 90% earnings, ~$1.6T on earnings >$250k) | Periodic | Authoritative | Use for scored revenue-option yields. 10-year projections, net of income/payroll offsets; label as projections. |
| `SRC-CBO-LTBO` | Congressional Budget Office | <https://www.cbo.gov/publication/61270> | 2026-06-24 | Long-Term Budget Outlook 2025-2055: debt held by public ~100%→156% of GDP; deficit 6.2%→7.3%; net interest 3.2%→5.4% of GDP | Annual | Authoritative | Use for the long-run debt/deficit/interest trajectory and the r>g point. Projections. (Also registered in the health-outcomes section.) |
| `SRC-SSA-TRUSTEES-2026` | Social Security Trustees / SSA Office of the Chief Actuary | <https://www.ssa.gov/oact/TR/2026/II_A_highlights.html> | 2026-06-24 | 2026 OASDI Trustees Report: OASI depletion Q4 2032 (~78% payable), combined OASDI 2034 (83%→65% by 2100); 75-yr actuarial deficit 4.42% of taxable payroll (~1.5% of GDP); OASDI cost 5.3%→~6.9% of GDP; ~17-18% of covered earnings above the cap | Annual | Authoritative | Use for trust-fund depletion, the 75-year deficit, and the share of earnings above the cap. Intermediate-assumption projections — label as such. |

## Pension-rate, mortality, and solvency-scoring sources

Back the Social-Security track. Peer contribution rates carry pillar/cap labels;
mortality and actuarial-scoring figures support the distribution and lever analysis.

| Source ID | Publisher | URL | Observed date | Coverage | Update cadence | Status | Extraction rule |
|---|---|---|---|---|---|---|---|
| `SRC-PENSION-CONTRIB-RATES` | National social-security agencies (Deutsche Rentenversicherung, Nenkin, CRA, gov.uk, INPS, Pensionsmyndigheten, Sécurité sociale) | <https://www.deutsche-rentenversicherung.de/> | 2026-06-24 | Combined public old-age pension contribution rates 2025-26: Germany 18.6% (single, capped €96,600); Japan 18.3% (multi, capped); Canada 11.9%+CPP2 (single, capped); UK NI not pension-earmarked; Italy 33% (single, mostly uncapped); Sweden 18.5% (multi, capped); France 15.45% capped + 2.42% uncapped + AGIRC-ARRCO (multi) | Annual | Authoritative (national agencies) | Cite per-country with pillar (single/multi) and cap status; do not aggregate into one comparable number. SSPTW edition is dated; use current national figures. |
| `SRC-CHETTY-2016` | Chetty et al., JAMA 2016;315(16):1750-1766 | <https://doi.org/10.1001/jama.2016.4226> | 2026-06-24 | US life-expectancy gap top-1% vs bottom-1% income: 14.6 yrs (men), 10.1 yrs (women) | Static (peer-reviewed) | Authoritative (literature) | Use for differential mortality by income. |
| `SRC-NAS-2015` | National Academies of Sciences, Engineering, and Medicine, 2015 | <https://doi.org/10.17226/19015> | 2026-06-24 | Differential mortality erodes/reverses Social Security lifetime progressivity (1930→1960 cohorts) | Static | Authoritative | Use for the lifetime-progressivity offset. Cohort-projection figures are model estimates. |
| `SRC-SSA-SOLVENCY` | SSA Office of the Chief Actuary, solvency provisions | <https://www.ssa.gov/oact/solvency/provisions/> | 2026-06-24 | Actuarial-balance improvement (% of taxable payroll; % of 75-yr shortfall) for cap provisions: eliminate cap no-credit +2.55% (67%); with-credit +1.85% (48%); >$250k +2.50% (65%); to 90% earnings +1.07% (28%) | Annual | Authoritative | Use for the actuarial scoring of cap reform (2025 TR assumptions; 75-yr deficit −3.82% of payroll on that basis). Projections. |

## Derived TAXLANE records (internal provenance)

These derived records are produced by `taxlane-tools` / repo scripts from the
sourced inputs above; registered here so papers and packets can cite them.

| Record | Path | Derived from | Observed date |
|---|---|---|---|
| Income-tax outlay model (decade summary) | `data/derived/income_tax_outlay_model/decade-summary.md` | `SRC-OMB-HIST-1-1-FY2027`, `SRC-OMB-HIST-2-1-FY2027`, `SRC-OMB-HIST-3-1-FY2027` | 2026-06-21 |
| Program-lane rate model | `data/derived/program_lane_rate_model/` | `SRC-OMB-HIST-1-1-FY2027`, `SRC-OMB-HIST-3-2-FY2027`, `SRC-OECD-REVSTATS-2025` | 2026-06-23 |
| Health-efficiency scenarios | `data/derived/program_lane_rate_model/health_efficiency_scenarios.fy2025.draft.jsonl` | `SRC-OMB-HIST-3-2-FY2027`, `SRC-OECD-HEALTH-2025` (GDP derived) | 2026-06-23 |
| Receipts-to-lane allocation | `data/derived/program_lane_rate_model/receipts_to_lane_allocation.fy2025.draft.jsonl` | `SRC-OMB-HIST-2-1-FY2027`, `SRC-OMB-HIST-2-4-FY2027`, `SRC-OMB-HIST-3-2-FY2027` | 2026-06-23 |

## Extraction status

| Source family | Status | Next action |
|---|---|---|
| Constitutional/statutory authority | Recorded | Build history timeline records with page/section anchors. |
| IRS historical rates | Recorded | Download table and define `rates_timeline` schema. |
| OMB receipts/outlays | Recorded | Download spreadsheets and define fiscal-year data dictionary. |
| OMB budget concepts | Recorded | Extract exact definitions for accounting explainer. |
| Treasury/USAspending context | Recorded | Use after OMB spine exists; record queries and dynamic dates. |
| CBO | Recorded | Use as cited fiscal-ratio and projection cross-check; label projections. |
| International/comparative benchmarks | Recorded | Use as fair-rate comparators with government-level and compulsory/general scope labels. |
| US fiscal ratios and GDP (Table 1.2) | Recorded | Extract federal %-GDP and GDP denominator for the rate model (Pulse 03). |
