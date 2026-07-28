# SRC-TREASURY-FISCALDATA-MSPD-DETAIL-2026-07-24 Metadata

- Source family: Treasury Fiscal Data Monthly Statement of the Public Debt detail tables
- Publisher: U.S. Department of the Treasury, Bureau of the Fiscal Service
- MSPD Table 3 source URL: <https://api.fiscaldata.treasury.gov/services/api/fiscal_service/v1/debt/mspd/mspd_table_3>
- MSPD Table 5 source URL: <https://api.fiscaldata.treasury.gov/services/api/fiscal_service/v1/debt/mspd/mspd_table_5>
- Retrieval date: 2026-07-24
- Pagination rule: complete CSV custody assembled from Fiscal Data API pages using `page[size]=5000`, preserving one header row.
- Local MSPD Table 3 artifact: `data/raw/treasury/SRC-TREASURY-FISCALDATA-MSPD-DETAIL-2026-07-24/2026-07-24/mspd_table_3.full.csv`
- MSPD Table 3 row count: 217144
- MSPD Table 3 byte count: 55726310
- MSPD Table 3 SHA-256: `347ff7878822ec7cf108b575cc440fe104dea4f0f9688cc0da89f5ce815a67c4`
- Local MSPD Table 5 artifact: `data/raw/treasury/SRC-TREASURY-FISCALDATA-MSPD-DETAIL-2026-07-24/2026-07-24/mspd_table_5.full.csv`
- MSPD Table 5 row count: 86145
- MSPD Table 5 byte count: 13156635
- MSPD Table 5 SHA-256: `801fceb68ece6ae48640697b7a6ab2dec65bcdb7c899b2b3fbe1c3bd133c5867`
- Access boundary: public Fiscal Data API CSV pages retrieved and combined into local bytes.
- Review status: source-custody context only; not reviewed for a solver-ready maturity schedule, debt-stock projection, primary-balance feedback fixture, savings estimate, rate calculation, or balanced-budget claim.

Use boundary: Table 3 contains issue dates, maturity dates, rates/yields, and amounts for MSPD security detail rows; Table 5 contains CUSIP-level strip/reconstitution detail. These files improve maturity-detail custody, but they do not by themselves create a remaining-maturity model for FY2025-FY2035 or reconcile Treasury monthly detail to CBO/OMB current-law projections.
