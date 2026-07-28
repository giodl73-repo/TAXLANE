# SRC-TREASURY-FISCALDATA-MSPD-TABLE-1-2026-07-24 Metadata

- Source family: Treasury Fiscal Data monthly statement of the public debt and debt-to-the-penny context
- Publisher: U.S. Department of the Treasury, Bureau of the Fiscal Service
- MSPD Table 1 source URL: <https://api.fiscaldata.treasury.gov/services/api/fiscal_service/v1/debt/mspd/mspd_table_1?format=csv&page[size]=10000&sort=-record_date>
- Debt-to-the-penny source URL: <https://api.fiscaldata.treasury.gov/services/api/fiscal_service/v2/accounting/od/debt_to_penny?format=csv&page[size]=10000&sort=-record_date>
- Retrieval date: 2026-07-24
- Local MSPD artifact: `data/raw/treasury/SRC-TREASURY-FISCALDATA-MSPD-TABLE-1-2026-07-24/2026-07-24/mspd_table_1.csv`
- MSPD byte count: 572282
- MSPD SHA-256: `e69089cd531522d47e4ac9f9cbf9efc3d0328886d1d5816999cde31f4891be19`
- Local debt-to-the-penny artifact: `data/raw/treasury/SRC-TREASURY-FISCALDATA-MSPD-TABLE-1-2026-07-24/2026-07-24/debt_to_penny_recent.csv`
- Debt-to-the-penny byte count: 823082
- Debt-to-the-penny SHA-256: `cc5b3b0b3e96338ad5d47ef48266eb31a354f87dbd89ba34116db15aaabe53fc`
- Debt-to-the-penny row count: 8355
- Access boundary: public Fiscal Data API CSV exports retrieved as local bytes.
- Review status: source-custody context only; not reviewed for a solver-ready debt stock path, remaining-maturity schedule, primary-balance feedback, savings estimate, rate calculation, or balanced-budget claim.

Use boundary: these CSVs support debt stock and security-class context. They do not by themselves complete a remaining-maturity reconciliation, OMB/CBO projection bridge, or endogenous net-interest feedback fixture.
