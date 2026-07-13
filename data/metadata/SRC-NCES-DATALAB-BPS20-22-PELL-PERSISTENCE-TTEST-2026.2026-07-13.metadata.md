# Source Metadata: SRC-NCES-DATALAB-BPS20-22-PELL-PERSISTENCE-TTEST-2026

| Field | Value |
|---|---|
| `source_id` | `SRC-NCES-DATALAB-BPS20-22-PELL-PERSISTENCE-TTEST-2026` |
| `publisher` | U.S. Department of Education, Institute of Education Sciences, National Center for Education Statistics |
| `source_table_url` | <https://nces.ed.gov/datalab/powerstats/table/zclxfu> |
| `api_endpoint_template` | `https://nces.ed.gov/datalab/api/v1/workspace/process/pvalue?datasetId=168&weight=WTA000&tValue={t_value}` |
| `study` | 2020/22 Beginning Postsecondary Students Longitudinal Study (BPS:20/22) |
| `dataset_id` | `168` |
| `weight` | `WTA000` |
| `source_query_id` | `396385` |
| `source_retrieval_code` | `zclxfu` |
| `observed_date` | 2026-07-13 |
| `capture_method` | Five GET requests to the official NCES DataLab p-value endpoint using t values calculated from the full-precision estimates and BRR standard errors in saved retrieval `zclxfu`. Each returned JSON payload is preserved in a separate repository text file; the repository file includes its normal final line feed. A checksum-verified request manifest preserves every input and URL. |
| `raw_directory` | `data/raw/nces/SRC-NCES-DATALAB-BPS20-22-PELL-PERSISTENCE-TTEST-2026/2026-07-13/` |
| `content_type` | `application/json` |
| `analysis` | DataLab Independent Estimates t-Test p-value lookup |
| `formula` | `t = (estimate_no_pell - estimate_pell) / sqrt(se_no_pell^2 + se_pell^2)` |
| `comparison_count` | `5` |
| `status` | `captured-checksum-verified-independent-estimates-screen` |

## Custody Checksums

| File | Bytes | SHA-256 |
|---|---:|---|
| `request-manifest.json` | 3468 | `9D09C714FA6CF290B5964AEF3A35ADA9225A2AA977F7AE0D8EE701AC7EC3CA57` |
| `pvalue-attained-bachelors-degree.json` | 77 | `2879EC8B8E28CAEB36FEB15125F152A10020A9CF8AAFD441A0A54CEA071AAB88` |
| `pvalue-attained-associates-degree.json` | 75 | `6BDA131E950FE31623FA034FDD81BFD99DEFF76D02552CABF3E25F6C0D8F0E53` |
| `pvalue-attained-certificate.json` | 79 | `4BD0676E5C74BA112F7108A688B75EED4A7B5AA87416C400C677A72BBF22EBA2` |
| `pvalue-no-degree-enrolled-ay2021-22.json` | 78 | `317B326ED9ED859B02B96D68B2D6E4A3CC4890D5C569AA6B157CD7DE30769111` |
| `pvalue-no-degree-not-enrolled-ay2021-22.json` | 79 | `16F76FD195F10C4A504A0CF3225A21D8D162AB013E3C3A8E59345CDB7F1C1D95` |
| **Packet total** | **3856** | Not applicable; validate the six file checksums individually. |

## Method And Evidence Boundaries

DataLab describes this tool as valid only for independent groups. The formula
sums the two squared standard errors and contains no covariance term. The two
`PELL20` cuts are mutually exclusive receipt groups, but this packet remains an
independent-estimates significance screen. It is not a replicate-weight or
otherwise covariance-aware difference estimate, and the stricter covariance
gate remains blocked.

The endpoint returns unadjusted p-values. TAXLANE's five-comparison Bonferroni
threshold and adjusted p-values are derived, not returned by DataLab. Preserve
both results and do not substitute statistical significance for practical
importance.

The captured endpoint requests use full-precision t values calculated from the
saved response, not DataLab's display-rounded values. As a sensitivity check,
the interface's one-decimal estimates and two-decimal standard errors produce
p-values of 0.0201324, 0.1529747, 7.9362e-16, 1.05595e-26, and 4.16699e-21 in
the same category order. The nominal-alpha and Bonferroni conclusions are
unchanged. These sensitivity values are not substituted for the captured
full-precision responses.

Positive `PELL20` is receipt, not eligibility. The groups are observational and
unadjusted, the five-category result is not the six-category First Look table,
and the pandemic-era three-year window is not mature. No causal Pell effect,
incremental program cost, compatible fiscal return, fraud, improper payment,
recovery, or savings claim is supported.
