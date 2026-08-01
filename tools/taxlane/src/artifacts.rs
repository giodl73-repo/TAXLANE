//! Auto-split from main.rs (ROUTE-style domain layout).
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::*;
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use roxmltree::Document;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use taxlane_core::{
    AccountabilityEvidenceRecord, ArtifactMetadata, BreadthBenchmarkRecord, CostDownBacklogRecord,
    CostDownEvidenceQueueRecord, CostDownFirstPassRollupRecord, CostDownScoringReadinessRecord,
    CostDownSourcePacketRecord, DebtMaturityRiskTreasuryProbeRecord,
    DebtPrimaryBalanceFiscalProbeRecord, DefenseAuditControlProbeRecord,
    DefenseProcurementControlProbeRecord, DisasterDeclarationProbeRecord,
    DisasterMitigationProjectProbeRecord, EfficiencyPressureRecord,
    ExternalAccountabilityClaimIntakeRecord, ExternalClaimAmountDerivation,
    ExternalClaimAmountSemantic, ExternalClaimCustodyStatus, ExternalClaimEvidenceRelation,
    ExternalClaimLegalOrAdministrativeStatus, ExternalClaimPublicationKind,
    ExternalClaimResponseRequestStatus, ExternalClaimReviewStatus, ExternalClaimStatus,
    ExternalClaimType, HeadlineBasisRecord, HealthAdminSimplificationProbeRecord,
    HealthPriceDisciplineProbeRecord, PERFORMANCE_DEMAND_RESPONSE_INTAKE_USE_RULE,
    PUBLIC_CLAIM_ALLOWED_LABEL, PUBLIC_CLAIM_BLOCKED_LABEL,
    PaymentIntegrityClaimsTimelinessProbeRecord, PaymentIntegrityMethodologyClosureCoverageRecord,
    PaymentIntegrityMethodologyClosureDecisionRecord,
    PaymentIntegrityMethodologyClosureReadinessRecord,
    PaymentIntegrityMethodologyComponentGateBoundaryDecisionRecord,
    PaymentIntegrityMethodologyComponentGateBoundaryReadinessRecord,
    PaymentIntegrityMethodologyComponentGateNarrowCandidateRecord,
    PaymentIntegrityMethodologyComponentGateNarrowDecisionRecord,
    PaymentIntegrityMethodologyComponentGateProgressRecord,
    PaymentIntegrityMethodologyComponentGateProgressRequirementRecord,
    PaymentIntegrityMethodologyComponentGateProgressSourceQueryRecord,
    PaymentIntegrityMethodologyComponentGateProgressSourceQueryRunRecord,
    PaymentIntegrityMethodologyComponentGateProgressSourceTargetRecord,
    PaymentIntegrityMethodologyComponentGateRequirementRecord,
    PaymentIntegrityMethodologyComponentGateSourceCaptureRecord,
    PaymentIntegrityMethodologyComponentGateSourceCaptureRollupRecord,
    PaymentIntegrityMethodologyComponentGateSourceQueryRecord,
    PaymentIntegrityMethodologyComponentGateSourceQueryRunRecord,
    PaymentIntegrityMethodologyComponentGateSourceTargetRecord,
    PaymentIntegrityMethodologyFieldRecord, PaymentIntegrityMethodologyFieldReviewRecord,
    PaymentIntegrityMethodologyFieldUpdateRecord,
    PaymentIntegrityMethodologyFollowupBoundaryDecisionRecord,
    PaymentIntegrityMethodologyFollowupBoundaryReadinessRecord,
    PaymentIntegrityMethodologyFollowupSourceCaptureRecord,
    PaymentIntegrityMethodologyFollowupSourceCaptureRollupRecord,
    PaymentIntegrityMethodologyFollowupSourceQueryRecord,
    PaymentIntegrityMethodologyFollowupSourceQueryRunRecord,
    PaymentIntegrityMethodologyGapFollowupRecord,
    PaymentIntegrityMethodologyGapSourceCaptureRecord,
    PaymentIntegrityMethodologyNarrowClosureCandidateRecord,
    PaymentIntegrityMethodologyNarrowClosureDecisionRecord,
    PaymentIntegrityMethodologyOpenProgramComponentProgressRecord,
    PaymentIntegrityMethodologyOpenProgramStatusRecord, PaymentIntegrityMethodologyPlanRecord,
    PaymentIntegrityMethodologyPriorityReviewerActionRecord,
    PaymentIntegrityMethodologyPrioritySourceWorkRecord,
    PaymentIntegrityMethodologyProgramRollupRecord, PaymentIntegrityMethodologyQueryRecord,
    PaymentIntegrityMethodologyQueryRunRecord,
    PaymentIntegrityMethodologyResidualGapPriorityRecord,
    PaymentIntegrityMethodologyResidualSourceGapRecord, PaymentIntegrityMethodologyResultRecord,
    PaymentIntegrityMethodologyResultReviewReadinessRecord,
    PaymentIntegrityMethodologyScoringGateRecord,
    PaymentIntegrityMethodologySourceCaptureRollupRecord,
    PaymentIntegrityMethodologySourceTargetRecord, PaymentIntegrityNextProgramSelectionRecord,
    PaymentIntegrityPortalProbeRecord, PaymentIntegrityProgramReviewGateRecord,
    PaymentIntegrityProgramReviewStatusRecord, PaymentIntegrityProgramReviewTaskRecord,
    PaymentIntegrityScorecardProbeRecord, PerUnitDisplayReadinessRecord, PerUnitReceiptCardRecord,
    PerformanceDemandChecklistRecord, PerformanceDemandResponseBundleArtifact,
    PerformanceDemandResponseBundleManifest, PerformanceDemandResponseClass,
    PerformanceDemandResponseDeltaRow, PerformanceDemandResponseIntakeRecord,
    PerformanceDemandResponseLogClass, PerformanceDemandResponseLogRecord,
    PerformanceDemandResponseStatus, SpendCategoryMapRecord,
};

use zip::ZipArchive;

pub(crate) const CHART_SPECS: &[&str] = &[
    "docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json",
    "docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json",
    "docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json",
    "docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json",
    "docs/charts/income-tax-outlay-subfunction-model/fy2025-top-subfunctions.vl.json",
    "docs/charts/income-tax-outlay-subfunction-model/selected-subfunction-trends.vl.json",
    "docs/charts/income-tax-outlay-subfunction-model/decade-top-subfunctions.vl.json",
    "docs/charts/taxpayer-receipt-model/placeholder-lane-bars.vl.json",
    "docs/charts/taxpayer-receipt-model/placeholder-financing-context.vl.json",
];

pub(crate) const MANIFEST_PATH: &str = "data/derived/income_tax_outlay_model/MANIFEST.md";

pub(crate) const ANNUAL_JSONL_PATH: &str = "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl";

pub(crate) const DECADE_JSONL_PATH: &str = "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-summary.jsonl";

pub(crate) const ANNUAL_CSV_PATH: &str = "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv";

pub(crate) const DECADE_CSV_PATH: &str = "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-wide.csv";

pub(crate) const DECADE_MD_PATH: &str = "data/derived/income_tax_outlay_model/decade-summary.md";

pub(crate) const SOURCE_PROFILE_PATH: &str = "data/derived/income_tax_outlay_model/source-profile.md";

pub(crate) const SUBFUNCTION_MODEL_JSONL_PATH: &str = "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.draft.jsonl";

pub(crate) const SUBFUNCTION_ANNUAL_CSV_PATH: &str = "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.annual-long.csv";

pub(crate) const SUBFUNCTION_DECADE_CSV_PATH: &str = "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.decade-long.csv";

pub(crate) const SUBFUNCTION_FY2025_TOP_CSV_PATH: &str = "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.fy2025-top-subfunctions.csv";

pub(crate) const SUBFUNCTION_MODEL_PROFILE_PATH: &str =
    "data/derived/income_tax_outlay_subfunction_model/source-profile.md";

pub(crate) const SUBFUNCTION_MODEL_README_PATH: &str =
    "data/derived/income_tax_outlay_subfunction_model/README.md";

pub(crate) const PLACEHOLDER_RECEIPT_JSON_PATH: &str = "data/derived/taxpayer_receipt_model/taxpayer_receipt_model.placeholder-1000.fy2025.omb-fy2027-v1.draft.json";

pub(crate) const PLACEHOLDER_RECEIPT_LANE_BARS_SPEC_PATH: &str =
    "docs/charts/taxpayer-receipt-model/placeholder-lane-bars.vl.json";

pub(crate) const PLACEHOLDER_RECEIPT_FINANCING_CONTEXT_SPEC_PATH: &str =
    "docs/charts/taxpayer-receipt-model/placeholder-financing-context.vl.json";

pub(crate) const ACCOUNTABILITY_EVIDENCE_JSONL_PATH: &str = "data/derived/accountability_evidence/accountability_evidence.omb-fy2027-v1.2026-06-23.draft.jsonl";

pub(crate) const ACCOUNTABILITY_READINESS_REPORT_PATH: &str =
    "data/derived/accountability_evidence/readiness-report.md";

pub(crate) const ACCOUNTABILITY_ACTION_QUEUE_PATH: &str =
    "data/derived/accountability_evidence/action-queue.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_PACKET_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-packet.md";

pub(crate) const ACCOUNTABILITY_WORK_ITEMS_JSONL_PATH: &str =
    "data/derived/accountability_evidence/accountability-work-items.jsonl";

pub(crate) const ACCOUNTABILITY_CLAIM_GUARD_REPORT_PATH: &str =
    "data/derived/accountability_evidence/claim-guard-report.md";

pub(crate) const ACCOUNTABILITY_PUBLIC_QUESTIONS_PATH: &str =
    "data/derived/accountability_evidence/public-questions.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-checklist.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_JSONL_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-checklist.jsonl";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_CLAIM_GATES_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-claim-gates.json";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_DASHBOARD_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-dashboard.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_BRIEF_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-brief.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_LETTER_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-letter.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_RUBRIC_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-rubric.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_FOLLOWUP_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-followup.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-log.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_JSONL_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-log.jsonl";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_SCHEMA_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-log.schema.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-status.json";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DASHBOARD_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-dashboard.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_HANDOFF_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-handoff.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_INTAKE_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-intake.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_INTAKE_SCHEMA_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-intake.schema.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_INTAKE_EXAMPLE_JSONL_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-intake.example.jsonl";

pub(crate) const EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_JSONL_PATH: &str =
    "data/derived/accountability_evidence/external-accountability-claim-intake.v1.draft.jsonl";

pub(crate) const EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_SCHEMA_PATH: &str =
    "data/derived/accountability_evidence/external-accountability-claim-intake.schema.md";

pub(crate) const EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_READER_PATH: &str =
    "data/derived/accountability_evidence/external-accountability-claim-intake.md";

pub(crate) const EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_REVIEW_PATH: &str =
    "reviews/2026-07-14-external-accountability-claim-intake-role-review.md";

pub(crate) const HOUSE_SHIRLEY_TESTIMONY_RAW_PATH: &str =
    "data/raw/house/SRC-HOUSE-JUDICIARY-SHIRLEY-TESTIMONY-2026/2026-07-14/shirley-testimony.pdf";

pub(crate) const HOUSE_SHIRLEY_TESTIMONY_METADATA_PATH: &str =
    "data/metadata/SRC-HOUSE-JUDICIARY-SHIRLEY-TESTIMONY-2026.2026-07-14.metadata.md";

pub(crate) const HOUSE_SHIRLEY_TESTIMONY_REVIEW_PATH: &str =
    "reviews/2026-07-14-house-testimony-quality-learing-center-claim-atom-role-review.md";

pub(crate) const HOUSE_SHIRLEY_TESTIMONY_BYTES: u64 = 60_433;

pub(crate) const HOUSE_SHIRLEY_TESTIMONY_SHA256: &str =
    "e90266a876dcb6882593a1a63df70646270c7f9a037f6ba49d20f9e310c040c5";

pub(crate) const MN_DCYF_CCAP_PROVIDER_RAW_PATH: &str = "data/raw/minnesota-house/SRC-MN-HOUSE-DCYF-CCAP-PROVIDER-DATA-2026-04-22/2026-07-14/dcyf-ccap-provider-data.pdf";

pub(crate) const MN_DCYF_CCAP_PROVIDER_METADATA_PATH: &str =
    "data/metadata/SRC-MN-HOUSE-DCYF-CCAP-PROVIDER-DATA-2026-04-22.2026-07-14.metadata.md";

pub(crate) const MN_DCYF_CCAP_PROVIDER_REVIEW_PATH: &str =
    "reviews/2026-07-14-mn-house-dcyf-quality-learning-center-payment-context-role-review.md";

pub(crate) const PULSE_55_QUALITY_LEARNING_CENTER_OFFICIAL_CONTEXT_PATH: &str = "context/waves/2026-07-12-breadth-depth-benchmark-matrix/pulses/pulse-55-quality-learning-center-official-context.md";

pub(crate) const MN_DCYF_CCAP_PROVIDER_CLOSURE_REVIEW_PATH: &str = "reviews/2026-07-14-mn-house-dcyf-quality-learning-center-license-closure-context-role-review.md";

pub(crate) const PULSE_56_QUALITY_LEARNING_CENTER_LICENSE_CLOSURE_CONTEXT_PATH: &str = "context/waves/2026-07-12-breadth-depth-benchmark-matrix/pulses/pulse-56-quality-learning-center-license-closure-context.md";

pub(crate) const QUALITY_LEARNING_CENTER_CY2025_PERIOD_CORRECTION_REVIEW_PATH: &str =
    "reviews/2026-07-14-quality-learning-center-cy2025-period-correction-role-review.md";

pub(crate) const PULSE_57_QUALITY_LEARNING_CENTER_CY2025_PERIOD_CORRECTION_PATH: &str = "context/waves/2026-07-12-breadth-depth-benchmark-matrix/pulses/pulse-57-quality-learning-center-cy2025-period-correction.md";

pub(crate) const MN_CCAP_CY2025_REQUEST_SPEC_PATH: &str = "data/derived/accountability_evidence/minnesota-ccap-quality-learning-center-cy2025-existing-records-request-specification.v1.draft.json";

pub(crate) const MN_CCAP_CY2025_REQUEST_READER_PATH: &str = "data/derived/accountability_evidence/minnesota-ccap-quality-learning-center-cy2025-existing-records-request-specification.md";

pub(crate) const MN_CCAP_CY2025_REQUEST_TEMPLATE_PATH: &str =
    "docs/requests/minnesota-ccap-quality-learning-center-cy2025-data-request.md";

pub(crate) const MN_CCAP_CY2025_REQUEST_REVIEW_PATH: &str = "reviews/2026-07-15-minnesota-ccap-quality-learning-center-cy2025-existing-records-request-specification-role-review.md";

pub(crate) const PULSE_58_MN_CCAP_CY2025_REQUEST_SPEC_PATH: &str = "context/waves/2026-07-12-breadth-depth-benchmark-matrix/pulses/pulse-58-minnesota-ccap-quality-learning-center-cy2025-request-specification.md";

pub(crate) const MN_DCYF_DATA_REQUESTS_SOURCE_ID: &str = "SRC-MN-DCYF-DATA-REQUESTS";

pub(crate) const MN_DCYF_DATA_REQUESTS_RAW_PATH: &str =
    "data/raw/minnesota-dcyf/SRC-MN-DCYF-DATA-REQUESTS/2026-07-15/data-requests.html";

pub(crate) const MN_DCYF_DATA_REQUESTS_METADATA_PATH: &str =
    "data/metadata/SRC-MN-DCYF-DATA-REQUESTS.2026-07-15.metadata.md";

pub(crate) const MN_DCYF_DATA_REQUESTS_BYTES: u64 = 70_406;

pub(crate) const MN_DCYF_DATA_REQUESTS_SHA256: &str =
    "374baf99e073640d920526b0d033713bdeb63eeb11e6ccdc6cd917cefcaf027c";

pub(crate) const MN_DCYF_PUBLIC_DATA_GUIDE_SOURCE_ID: &str = "SRC-MN-DCYF-PUBLIC-DATA-GUIDE";

pub(crate) const MN_DCYF_PUBLIC_DATA_GUIDE_RAW_PATH: &str =
    "data/raw/minnesota-dcyf/SRC-MN-DCYF-PUBLIC-DATA-GUIDE/2026-07-15/dcyf-public-data-guide.pdf";

pub(crate) const MN_DCYF_PUBLIC_DATA_GUIDE_METADATA_PATH: &str =
    "data/metadata/SRC-MN-DCYF-PUBLIC-DATA-GUIDE.2026-07-15.metadata.md";

pub(crate) const MN_DCYF_PUBLIC_DATA_GUIDE_BYTES: u64 = 395_594;

pub(crate) const MN_DCYF_PUBLIC_DATA_GUIDE_SHA256: &str =
    "93f3bfe68e7835a70c3308c57a59efbd62cb75685e6ebf3b72af5b5572a370fc";

pub(crate) const MN_STAT_13_03_SOURCE_ID: &str = "SRC-MN-STAT-13-03-2025";

pub(crate) const MN_STAT_13_03_RAW_PATH: &str =
    "data/raw/minnesota-revisor/SRC-MN-STAT-13-03-2025/2026-07-15/mn-statute-13-03.pdf";

pub(crate) const MN_STAT_13_03_METADATA_PATH: &str =
    "data/metadata/SRC-MN-STAT-13-03-2025.2026-07-15.metadata.md";

pub(crate) const MN_STAT_13_03_BYTES: u64 = 310_898;

pub(crate) const MN_STAT_13_03_SHA256: &str =
    "af9a6751dca8770f98144fa39ad700a1b379e65f2612788098534a6ecd57b69d";

pub(crate) const MN_STAT_142E_02_SOURCE_ID: &str = "SRC-MN-STAT-142E-02-2025";

pub(crate) const MN_STAT_142E_02_RAW_PATH: &str =
    "data/raw/minnesota-revisor/SRC-MN-STAT-142E-02-2025/2026-07-15/mn-statute-142e-02.pdf";

pub(crate) const MN_STAT_142E_02_METADATA_PATH: &str =
    "data/metadata/SRC-MN-STAT-142E-02-2025.2026-07-15.metadata.md";

pub(crate) const MN_STAT_142E_02_BYTES: u64 = 360_390;

pub(crate) const MN_STAT_142E_02_SHA256: &str =
    "e54bd41260c89a77370e996c2aba8f3207e417a5a87ea7d989e8a0097144f02d";

pub(crate) const MN_DCYF_CCAP_PROVIDER_BYTES: u64 = 1_277_757;

pub(crate) const MN_DCYF_CCAP_PROVIDER_SHA256: &str =
    "e7068e1198d8dce851907b60fc4a2a16fedd5de7a1d41afcd2b02dcaabf3dec1";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_APPLIED_EXAMPLE_JSONL_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-log.applied-example.jsonl";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_APPLIED_EXAMPLE_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-status.applied-example.json";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DASHBOARD_APPLIED_EXAMPLE_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-dashboard.applied-example.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_HANDOFF_APPLIED_EXAMPLE_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-handoff.applied-example.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_APPLIED_EXAMPLE_SCHEMA_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-applied-example.schema.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-delta.applied-example.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_JSONL_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-delta.applied-example.jsonl";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_SCHEMA_PATH: &str = "data/derived/accountability_evidence/performance-demand-response-delta.applied-example.schema.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-bundle.applied-example.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_JSON_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-bundle.applied-example.json";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_SCHEMA_PATH: &str = "data/derived/accountability_evidence/performance-demand-response-bundle.applied-example.schema.md";

pub(crate) const ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_SCHEMA_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-checklist.schema.md";

pub(crate) const SPEND_CATEGORY_MAP_JSONL_PATH: &str =
    "data/derived/spend_category_map/spend_category_map.fy2025.omb-fy2027-v1.draft.jsonl";

pub(crate) const SPEND_CATEGORY_MAP_README_PATH: &str = "data/derived/spend_category_map/README.md";

pub(crate) const SPEND_CATEGORY_MAP_SCHEMA_PATH: &str =
    "data/derived/spend_category_map/spend_category_map.schema.md";

pub(crate) const SPEND_CATEGORY_MAP_HANDOFF_PATH: &str =
    "data/derived/spend_category_map/accountability-question-handoff.md";

pub(crate) const SPEND_CATEGORY_MAP_DASHBOARD_PATH: &str =
    "data/derived/spend_category_map/spend-category-dashboard.md";

pub(crate) const BREADTH_BENCHMARK_JSONL_PATH: &str =
    "data/derived/breadth_benchmark_matrix/breadth_benchmark_matrix.v1.draft.jsonl";

pub(crate) const BREADTH_BENCHMARK_README_PATH: &str = "data/derived/breadth_benchmark_matrix/README.md";

pub(crate) const BREADTH_BENCHMARK_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/breadth_benchmark_matrix.schema.md";

pub(crate) const BREADTH_BENCHMARK_SCOREBOARD_PATH: &str =
    "docs/reading/current-versus-benchmark-scoreboard.md";

pub(crate) const LANE_FULL_COVERAGE_MATRIX_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/lane_full_coverage_matrix.v1.draft.json";

pub(crate) const LANE_FULL_COVERAGE_MATRIX_READER_PATH: &str = "docs/reading/lane-full-coverage-matrix.md";

pub(crate) const PUBLIC_EXPLAINER_WAVE_C_PROMOTION_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/public_explainer_wave_c_promotion.v1.draft.json";

pub(crate) const PUBLIC_EXPLAINER_WAVE_C_PROMOTION_READER_PATH: &str =
    "docs/reading/public-explainer-wave-c-promotion.md";

pub(crate) const HEALTH_COST_DECOMPOSITION_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_cost_decomposition.v1.draft.json";

pub(crate) const HEALTH_COST_DECOMPOSITION_READER_PATH: &str = "docs/reading/health-cost-decomposition.md";

pub(crate) const HEALTH_SERVICE_BRIDGE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_service_price_volume_bridge.cy2024.v1.draft.json";

pub(crate) const HEALTH_SERVICE_BRIDGE_READER_PATH: &str =
    "docs/reading/health-service-price-volume-bridge.md";

pub(crate) const HEALTH_CATEGORY_BENCHMARK_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_category_benchmark_ladder.v1.draft.json";

pub(crate) const HEALTH_CATEGORY_BENCHMARK_READER_PATH: &str =
    "docs/reading/health-category-benchmark-ladder.md";

pub(crate) const HEALTH_TARGET_ADMISSIBILITY_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_target_admissibility.v1.draft.json";

pub(crate) const HEALTH_TARGET_ADMISSIBILITY_READER_PATH: &str = "docs/reading/health-target-admissibility.md";

pub(crate) const HEALTH_SCENARIOS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_medicare_relative_scenarios.v1.draft.json";

pub(crate) const HEALTH_SCENARIOS_READER_PATH: &str = "docs/reading/health-medicare-relative-scenarios.md";

pub(crate) const HEALTH_SAMPLE_SENSITIVITY_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_commercial_sample_sensitivity.v1.draft.json";

pub(crate) const HEALTH_SAMPLE_SENSITIVITY_READER_PATH: &str =
    "docs/reading/health-commercial-sample-sensitivity.md";

pub(crate) const HEALTH_NATIONAL_PHI_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_national_phi_sensitivity.v1.draft.json";

pub(crate) const HEALTH_NATIONAL_PHI_READER_PATH: &str = "docs/reading/health-national-phi-sensitivity.md";

pub(crate) const HEALTH_TARGET_COST_SCENARIO_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_target_cost_scenario.v1.draft.json";

pub(crate) const FISCAL_PATH_SCENARIOS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fiscal_path_scenarios.v1.draft.json";

pub(crate) const FISCAL_PATH_SCENARIOS_READER_PATH: &str = "docs/reading/fiscal-path-scenarios.md";

pub(crate) const FISCAL_DEBT_DYNAMICS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fiscal_debt_dynamics_2026_2036.v1.draft.json";

pub(crate) const FISCAL_DEBT_DYNAMICS_READER_PATH: &str = "docs/reading/fiscal-debt-dynamics.md";

pub(crate) const FISCAL_POLICY_BASKETS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fiscal_policy_scale_baskets.v1.draft.json";

pub(crate) const FISCAL_POLICY_BASKETS_READER_PATH: &str = "docs/reading/fiscal-policy-scale-baskets.md";

pub(crate) const FISCAL_POLICY_DISTRIBUTION_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fiscal_policy_distribution_screen.v1.draft.json";

pub(crate) const FISCAL_POLICY_DISTRIBUTION_READER_PATH: &str =
    "docs/reading/fiscal-policy-distribution-screen.md";

pub(crate) const BALANCED_RATE_READINESS_GATE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/balanced_rate_readiness_gate.v1.draft.json";

pub(crate) const BALANCED_RATE_READINESS_GATE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/balanced_rate_readiness_gate.schema.md";

pub(crate) const BALANCED_RATE_READINESS_GATE_READER_PATH: &str =
    "docs/reading/balanced-rate-readiness-gate.md";

pub(crate) const FINAL_CLOSURE_READINESS_GATE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/final_closure_readiness_gate.v1.draft.json";

pub(crate) const FINAL_CLOSURE_READINESS_GATE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/final_closure_readiness_gate.schema.md";

pub(crate) const FINAL_CLOSURE_READINESS_GATE_READER_PATH: &str =
    "docs/reading/final-closure-readiness-gate.md";

pub(crate) const ADAPTIVE_RATE_SYSTEM_CONTRACT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/adaptive_rate_system_contract.v1.draft.json";

pub(crate) const ADAPTIVE_RATE_SYSTEM_CONTRACT_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/adaptive_rate_system_contract.schema.md";

pub(crate) const ADAPTIVE_RATE_SYSTEM_CONTRACT_READER_PATH: &str =
    "docs/reading/adaptive-rate-system-contract.md";

pub(crate) const OVERSPENDING_RISK_TAXONOMY_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/overspending_risk_taxonomy.v1.draft.json";

pub(crate) const OVERSPENDING_RISK_TAXONOMY_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/overspending_risk_taxonomy.schema.md";

pub(crate) const OVERSPENDING_RISK_TAXONOMY_READER_PATH: &str = "docs/reading/overspending-risk-taxonomy.md";

pub(crate) const TECHNOLOGY_TRANSITION_OPERATING_MODEL_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/technology_transition_operating_model.v1.draft.json";

pub(crate) const TECHNOLOGY_TRANSITION_OPERATING_MODEL_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/technology_transition_operating_model.schema.md";

pub(crate) const TECHNOLOGY_TRANSITION_OPERATING_MODEL_READER_PATH: &str =
    "docs/reading/technology-transition-operating-model.md";

pub(crate) const PUBLIC_RATE_CARD_V2_CONTRACT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/public_rate_card_v2_contract.v1.draft.json";

pub(crate) const PUBLIC_RATE_CARD_V2_CONTRACT_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/public_rate_card_v2_contract.schema.md";

pub(crate) const PUBLIC_RATE_CARD_V2_CONTRACT_READER_PATH: &str =
    "docs/reading/public-rate-card-v2-contract.md";

pub(crate) const PILOT_LANE_SELECTION_GATE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/pilot_lane_selection_gate.v1.draft.json";

pub(crate) const PILOT_LANE_SELECTION_GATE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/pilot_lane_selection_gate.schema.md";

pub(crate) const PILOT_LANE_SELECTION_GATE_READER_PATH: &str = "docs/reading/pilot-lane-selection-gate.md";

pub(crate) const DETERMINISTIC_ANNUAL_UPDATE_SIMULATOR_CONTRACT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/deterministic_annual_update_simulator_contract.v1.draft.json";

pub(crate) const DETERMINISTIC_ANNUAL_UPDATE_SIMULATOR_CONTRACT_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/deterministic_annual_update_simulator_contract.schema.md";

pub(crate) const DETERMINISTIC_ANNUAL_UPDATE_SIMULATOR_CONTRACT_READER_PATH: &str =
    "docs/reading/deterministic-annual-update-simulator-contract.md";

pub(crate) const PUBLIC_THESIS_PACKET_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/public_thesis_packet.v1.draft.json";

pub(crate) const PUBLIC_THESIS_PACKET_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/public_thesis_packet.schema.md";

pub(crate) const PUBLIC_THESIS_PACKET_READER_PATH: &str = "docs/reading/public-thesis-packet.md";

pub(crate) const PUBLIC_THESIS_PACKET_ROLE_REVIEW_PATH: &str =
    "reviews/2026-07-18-public-thesis-packet-role-review.md";

pub(crate) const PILOT_LANE_SELECTION_DECISION_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/pilot_lane_selection_decision.v1.draft.json";

pub(crate) const PILOT_LANE_SELECTION_DECISION_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/pilot_lane_selection_decision.schema.md";

pub(crate) const PILOT_LANE_SELECTION_DECISION_READER_PATH: &str =
    "docs/reading/pilot-lane-selection-decision.md";

pub(crate) const PILOT_LANE_SELECTION_DECISION_ROLE_REVIEW_PATH: &str =
    "reviews/2026-07-18-pilot-lane-selection-decision-role-review.md";

pub(crate) const TRANSPORTATION_PILOT_SOURCE_PLAN_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/transportation_pilot_source_plan.v1.draft.json";

pub(crate) const TRANSPORTATION_PILOT_SOURCE_PLAN_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/transportation_pilot_source_plan.schema.md";

pub(crate) const TRANSPORTATION_PILOT_SOURCE_PLAN_READER_PATH: &str =
    "docs/reading/transportation-pilot-source-plan.md";

pub(crate) const TRANSPORTATION_PILOT_BASELINE_PATH_CONTRACT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_pilot_baseline_path_contract.v1.draft.json";

pub(crate) const TRANSPORTATION_PILOT_BASELINE_PATH_CONTRACT_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/transportation_pilot_baseline_path_contract.schema.md";

pub(crate) const TRANSPORTATION_PILOT_BASELINE_PATH_CONTRACT_READER_PATH: &str =
    "docs/reading/transportation-pilot-baseline-path-contract.md";

pub(crate) const TRANSPORTATION_PILOT_FLOOR_INDICATOR_CONTRACT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_pilot_floor_indicator_contract.v1.draft.json";

pub(crate) const TRANSPORTATION_PILOT_FLOOR_INDICATOR_CONTRACT_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/transportation_pilot_floor_indicator_contract.schema.md";

pub(crate) const TRANSPORTATION_PILOT_FLOOR_INDICATOR_CONTRACT_READER_PATH: &str =
    "docs/reading/transportation-pilot-floor-indicator-contract.md";

pub(crate) const TRANSPORTATION_PILOT_MODERNIZATION_PATH_CONTRACT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_pilot_modernization_path_contract.v1.draft.json";

pub(crate) const TRANSPORTATION_PILOT_MODERNIZATION_PATH_CONTRACT_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_pilot_modernization_path_contract.schema.md";

pub(crate) const TRANSPORTATION_PILOT_MODERNIZATION_PATH_CONTRACT_READER_PATH: &str =
    "docs/reading/transportation-pilot-modernization-path-contract.md";

pub(crate) const TRANSPORTATION_PILOT_STRESS_PATH_CONTRACT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/transportation_pilot_stress_path_contract.v1.draft.json";

pub(crate) const TRANSPORTATION_PILOT_STRESS_PATH_CONTRACT_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/transportation_pilot_stress_path_contract.schema.md";

pub(crate) const TRANSPORTATION_PILOT_STRESS_PATH_CONTRACT_READER_PATH: &str =
    "docs/reading/transportation-pilot-stress-path-contract.md";

pub(crate) const TRANSPORTATION_PILOT_FY2025_ANCHOR_CUSTODY_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_pilot_fy2025_anchor_custody.v1.draft.json";

pub(crate) const TRANSPORTATION_PILOT_FY2025_ANCHOR_CUSTODY_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/transportation_pilot_fy2025_anchor_custody.schema.md";

pub(crate) const TRANSPORTATION_PILOT_FY2025_ANCHOR_CUSTODY_READER_PATH: &str =
    "docs/reading/transportation-pilot-fy2025-anchor-custody.md";

pub(crate) const TRANSPORTATION_PILOT_PARTIAL_FEDERAL_OUTLAY_PATH_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_pilot_partial_federal_outlay_path.v1.draft.json";

pub(crate) const TRANSPORTATION_PILOT_PARTIAL_FEDERAL_OUTLAY_PATH_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_pilot_partial_federal_outlay_path.schema.md";

pub(crate) const TRANSPORTATION_PILOT_PARTIAL_FEDERAL_OUTLAY_PATH_READER_PATH: &str =
    "docs/reading/transportation-pilot-partial-federal-outlay-path.md";

pub(crate) const TRANSPORTATION_PILOT_TRUST_FUND_SOURCE_CUSTODY_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_pilot_trust_fund_source_custody.v1.draft.json";

pub(crate) const TRANSPORTATION_PILOT_TRUST_FUND_SOURCE_CUSTODY_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_pilot_trust_fund_source_custody.schema.md";

pub(crate) const TRANSPORTATION_PILOT_TRUST_FUND_SOURCE_CUSTODY_READER_PATH: &str =
    "docs/reading/transportation-pilot-trust-fund-source-custody.md";

pub(crate) const TRANSPORTATION_PILOT_TRUST_FUND_ACCOUNTING_BOUNDARY_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_pilot_trust_fund_accounting_boundary.v1.draft.json";

pub(crate) const TRANSPORTATION_PILOT_TRUST_FUND_ACCOUNTING_BOUNDARY_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_pilot_trust_fund_accounting_boundary.schema.md";

pub(crate) const TRANSPORTATION_PILOT_TRUST_FUND_ACCOUNTING_BOUNDARY_READER_PATH: &str =
    "docs/reading/transportation-pilot-trust-fund-accounting-boundary.md";

pub(crate) const TRANSPORTATION_TRUST_FUND_TABLE_13_4_FY2025_2031_CONTEXT_PATH_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_trust_fund_table_13_4_fy2025_2031_context_path.v1.draft.json";

pub(crate) const TRANSPORTATION_TRUST_FUND_TABLE_13_4_FY2025_2031_CONTEXT_PATH_READER_PATH: &str =
    "docs/reading/transportation-trust-fund-table-13-4-fy2025-2031-context-path.md";

pub(crate) const FUND_GROUP_FY2025_RECONCILIATION_FIXTURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fund_group_fy2025_reconciliation_fixture.v1.draft.json";

pub(crate) const FUND_GROUP_FY2025_RECONCILIATION_FIXTURE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fund_group_fy2025_reconciliation_fixture.schema.md";

pub(crate) const FUND_GROUP_FY2025_RECONCILIATION_FIXTURE_READER_PATH: &str =
    "docs/reading/fund-group-fy2025-reconciliation-fixture.md";

pub(crate) const SOLVER_ACCOUNTING_READINESS_GATE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/solver_accounting_readiness_gate.v1.draft.json";

pub(crate) const SOLVER_ACCOUNTING_READINESS_GATE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/solver_accounting_readiness_gate.schema.md";

pub(crate) const SOLVER_ACCOUNTING_READINESS_GATE_READER_PATH: &str =
    "docs/reading/solver-accounting-readiness-gate.md";

pub(crate) const SOLVER_INPUT_INVENTORY_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/solver_input_inventory.v1.draft.json";

pub(crate) const SOLVER_INPUT_INVENTORY_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/solver_input_inventory.schema.md";

pub(crate) const SOLVER_INPUT_INVENTORY_READER_PATH: &str = "docs/reading/solver-input-inventory.md";

pub(crate) const RESERVE_RULE_CONTRACT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/reserve_rule_contract.v1.draft.json";

pub(crate) const RESERVE_RULE_CONTRACT_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/reserve_rule_contract.schema.md";

pub(crate) const RESERVE_RULE_CONTRACT_READER_PATH: &str = "docs/reading/reserve-rule-contract.md";

pub(crate) const RESERVE_PARAMETER_READINESS_GATE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/reserve_parameter_readiness_gate.v1.draft.json";

pub(crate) const RESERVE_PARAMETER_READINESS_GATE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/reserve_parameter_readiness_gate.schema.md";

pub(crate) const RESERVE_PARAMETER_READINESS_GATE_READER_PATH: &str =
    "docs/reading/reserve-parameter-readiness-gate.md";

pub(crate) const NET_INTEREST_FORMULA_CONTRACT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_interest_formula_contract.v1.draft.json";

pub(crate) const NET_INTEREST_FORMULA_CONTRACT_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_interest_formula_contract.schema.md";

pub(crate) const NET_INTEREST_FORMULA_CONTRACT_READER_PATH: &str =
    "docs/reading/net-interest-formula-contract.md";

pub(crate) const NET_BASELINE_COMPATIBILITY_AUDIT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_current_law_baseline_compatibility_audit.v1.draft.json";
pub(crate) const NET_BASELINE_COMPATIBILITY_AUDIT_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_current_law_baseline_compatibility_audit.schema.md";
pub(crate) const NET_BASELINE_COMPATIBILITY_AUDIT_READER_PATH: &str =
    "docs/reading/net-current-law-baseline-compatibility-audit.md";
pub(crate) const NET_BASELINE_COMPATIBILITY_AUDIT_REVIEW_PATH: &str =
    "reviews/2026-07-31-net-current-law-baseline-compatibility-role-review.md";
pub(crate) const NET_OMB_GROSS_TO_NET_BRIDGE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_interest_omb_pbd_gross_to_net_bridge.v1.draft.json";
pub(crate) const NET_OMB_GROSS_TO_NET_BRIDGE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_interest_omb_pbd_gross_to_net_bridge.schema.md";
pub(crate) const NET_OMB_GROSS_TO_NET_BRIDGE_READER_PATH: &str =
    "docs/reading/net-interest-omb-pbd-gross-to-net-bridge.md";
pub(crate) const NET_NEW_BORROWING_TIMING_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_interest_new_borrowing_timing_convention.v1.draft.json";
pub(crate) const NET_NEW_BORROWING_TIMING_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_interest_new_borrowing_timing_convention.schema.md";
pub(crate) const NET_NEW_BORROWING_TIMING_READER_PATH: &str =
    "docs/reading/net-interest-new-borrowing-timing-convention.md";
pub(crate) const NET_ACCOUNTING_BRIDGE_REVIEW_PATH: &str =
    "reviews/2026-07-31-net-accounting-bridge-role-review.md";
pub(crate) const NET_CBO_AVERAGE_RATE_FEEDBACK_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_interest_cbo_average_rate_feedback.v1.draft.json";
pub(crate) const NET_CBO_AVERAGE_RATE_FEEDBACK_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_interest_cbo_average_rate_feedback.schema.md";
pub(crate) const NET_CBO_AVERAGE_RATE_FEEDBACK_READER_PATH: &str =
    "docs/reading/net-interest-cbo-average-rate-feedback.md";
pub(crate) const NET_CBO_AVERAGE_RATE_FEEDBACK_REVIEW_PATH: &str =
    "reviews/2026-08-01-net-cbo-average-rate-feedback-role-review.md";
pub(crate) const NET_PUBLIC_MATURITY_ENVELOPE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_interest_mspd_public_maturity_envelope.v1.draft.json";
pub(crate) const NET_PUBLIC_MATURITY_ENVELOPE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_interest_mspd_public_maturity_envelope.schema.md";
pub(crate) const NET_PUBLIC_MATURITY_ENVELOPE_READER_PATH: &str =
    "docs/reading/net-interest-mspd-public-maturity-envelope.md";
pub(crate) const NET_PUBLIC_MATURITY_ENVELOPE_REVIEW_PATH: &str =
    "reviews/2026-08-01-net-public-maturity-envelope-role-review.md";
pub(crate) const NET_EMPIRICAL_ROLLOVER_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_interest_mspd_empirical_rollover_convention.v1.draft.json";
pub(crate) const NET_EMPIRICAL_ROLLOVER_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_interest_mspd_empirical_rollover_convention.schema.md";
pub(crate) const NET_EMPIRICAL_ROLLOVER_READER_PATH: &str =
    "docs/reading/net-interest-mspd-empirical-rollover-convention.md";
pub(crate) const NET_EMPIRICAL_ROLLOVER_REVIEW_PATH: &str =
    "reviews/2026-08-01-net-empirical-rollover-role-review.md";

pub(crate) const NET_INTEREST_PBD_FY2025_2031_CURRENT_LAW_CONTEXT_PATH_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/net_interest_pbd_fy2025_2031_current_law_context_path.v1.draft.json";

pub(crate) const NET_INTEREST_PBD_FY2025_2031_CURRENT_LAW_CONTEXT_PATH_READER_PATH: &str =
    "docs/reading/net-interest-pbd-fy2025-2031-current-law-context-path.md";

pub(crate) const ASSIGNED_RECEIPT_BASE_INVENTORY_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/assigned_receipt_base_inventory.v1.draft.json";

pub(crate) const ASSIGNED_RECEIPT_BASE_INVENTORY_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/assigned_receipt_base_inventory.schema.md";

pub(crate) const ASSIGNED_RECEIPT_BASE_INVENTORY_READER_PATH: &str =
    "docs/reading/assigned-receipt-base-inventory.md";

pub(crate) const ASSIGNED_RECEIPT_BASE_SOURCE_GAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/assigned_receipt_base_source_gap.v1.draft.json";

pub(crate) const ASSIGNED_RECEIPT_BASE_SOURCE_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/assigned_receipt_base_source_gap.schema.md";

pub(crate) const ASSIGNED_RECEIPT_BASE_SOURCE_GAP_READER_PATH: &str =
    "docs/reading/assigned-receipt-base-source-gap.md";

pub(crate) const DISTRIBUTIONAL_EFFECT_PLACEHOLDER_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/distributional_effect_placeholder.v1.draft.json";

pub(crate) const DISTRIBUTIONAL_EFFECT_PLACEHOLDER_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/distributional_effect_placeholder.schema.md";

pub(crate) const DISTRIBUTIONAL_EFFECT_PLACEHOLDER_READER_PATH: &str =
    "docs/reading/distributional-effect-placeholder.md";

pub(crate) const DISTRIBUTION_INCIDENCE_SOURCE_GAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/distribution_incidence_source_gap.v1.draft.json";

pub(crate) const DISTRIBUTION_INCIDENCE_SOURCE_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/distribution_incidence_source_gap.schema.md";

pub(crate) const DISTRIBUTION_INCIDENCE_SOURCE_GAP_READER_PATH: &str =
    "docs/reading/distribution-incidence-source-gap.md";

pub(crate) const ADMINISTRATION_COMPLIANCE_BURDEN_SOURCE_GAP_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/administration_compliance_burden_source_gap.v1.draft.json";

pub(crate) const ADMINISTRATION_COMPLIANCE_BURDEN_SOURCE_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/administration_compliance_burden_source_gap.schema.md";

pub(crate) const ADMINISTRATION_COMPLIANCE_BURDEN_SOURCE_GAP_READER_PATH: &str =
    "docs/reading/administration-compliance-burden-source-gap.md";

pub(crate) const RATE_PUBLICATION_READINESS_ROLLUP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rate_publication_readiness_rollup.v1.draft.json";

pub(crate) const RATE_PUBLICATION_READINESS_ROLLUP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rate_publication_readiness_rollup.schema.md";

pub(crate) const RATE_PUBLICATION_READINESS_ROLLUP_READER_PATH: &str =
    "docs/reading/rate-publication-readiness-rollup.md";

pub(crate) const RECEIPT_BASE_LOCAL_SOURCE_INVENTORY_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/receipt_base_local_source_inventory.v1.draft.json";

pub(crate) const RECEIPT_BASE_LOCAL_SOURCE_INVENTORY_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/receipt_base_local_source_inventory.schema.md";

pub(crate) const RECEIPT_BASE_LOCAL_SOURCE_INVENTORY_READER_PATH: &str =
    "docs/reading/receipt-base-local-source-inventory.md";

pub(crate) const RECEIPT_BASE_SOURCE_WORK_QUEUE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/receipt_base_source_work_queue.v1.draft.json";

pub(crate) const RECEIPT_BASE_SOURCE_WORK_QUEUE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/receipt_base_source_work_queue.schema.md";

pub(crate) const RECEIPT_BASE_SOURCE_WORK_QUEUE_READER_PATH: &str =
    "docs/reading/receipt-base-source-work-queue.md";

pub(crate) const OMB_RECEIPT_CATEGORY_CONTEXT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/omb_receipt_category_context.fy2025.v1.draft.json";

pub(crate) const OMB_RECEIPT_CATEGORY_CONTEXT_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/omb_receipt_category_context.schema.md";

pub(crate) const OMB_RECEIPT_CATEGORY_CONTEXT_READER_PATH: &str =
    "docs/reading/omb-receipt-category-context.md";

pub(crate) const OMB_RECEIPT_CATEGORY_FY2025_2031_CONTEXT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/omb_receipt_category_fy2025_2031_context.v1.draft.json";

pub(crate) const OMB_RECEIPT_CATEGORY_FY2025_2031_CONTEXT_READER_PATH: &str =
    "docs/reading/omb-receipt-category-fy2025-2031-context.md";

pub(crate) const OMB_RECEIPT_DETAIL_TABLE_2_4_FY2025_2031_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/omb_receipt_detail_table_2_4_fy2025_2031_context.v1.draft.json";

pub(crate) const OMB_RECEIPT_DETAIL_TABLE_2_4_FY2025_2031_CONTEXT_READER_PATH: &str =
    "docs/reading/omb-receipt-detail-table-2-4-fy2025-2031-context.md";

pub(crate) const OMB_RECEIPT_SHARE_TABLE_2_2_FY2025_2031_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/omb_receipt_share_table_2_2_fy2025_2031_context.v1.draft.json";

pub(crate) const OMB_RECEIPT_SHARE_TABLE_2_2_FY2025_2031_CONTEXT_READER_PATH: &str =
    "docs/reading/omb-receipt-share-table-2-2-fy2025-2031-context.md";

pub(crate) const OMB_RECEIPT_AMOUNT_SHARE_RECONCILIATION_FY2025_2031_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/omb_receipt_amount_share_reconciliation_fy2025_2031_context.v1.draft.json";

pub(crate) const OMB_RECEIPT_AMOUNT_SHARE_RECONCILIATION_FY2025_2031_CONTEXT_READER_PATH: &str =
    "docs/reading/omb-receipt-amount-share-reconciliation-fy2025-2031-context.md";

pub(crate) const OMB_CBO_REVENUE_OVERLAP_RECONCILIATION_FY2026_2031_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/omb_cbo_revenue_overlap_reconciliation_fy2026_2031_context.v1.draft.json";

pub(crate) const OMB_CBO_REVENUE_OVERLAP_RECONCILIATION_FY2026_2031_CONTEXT_READER_PATH: &str =
    "docs/reading/omb-cbo-revenue-overlap-reconciliation-fy2026-2031-context.md";

pub(crate) const RECEIPT_BASE_WORK_ITEM_COMPLETION_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/receipt_base_work_item_completion.v1.draft.json";

pub(crate) const RECEIPT_BASE_WORK_ITEM_COMPLETION_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/receipt_base_work_item_completion.schema.md";

pub(crate) const RECEIPT_BASE_WORK_ITEM_COMPLETION_READER_PATH: &str =
    "docs/reading/receipt-base-work-item-completion.md";

pub(crate) const TRANSPORTATION_RECEIPT_BASE_WORK_ITEM_PROGRESS_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_receipt_base_work_item_progress.v1.draft.json";

pub(crate) const TRANSPORTATION_RECEIPT_BASE_WORK_ITEM_PROGRESS_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_receipt_base_work_item_progress.schema.md";

pub(crate) const TRANSPORTATION_RECEIPT_BASE_WORK_ITEM_PROGRESS_READER_PATH: &str =
    "docs/reading/transportation-receipt-base-work-item-progress.md";

pub(crate) const RECEIPT_BASE_OFFICIAL_SOURCE_CAPTURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/receipt_base_official_source_capture.v1.draft.json";

pub(crate) const RECEIPT_BASE_OFFICIAL_SOURCE_CAPTURE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/receipt_base_official_source_capture.schema.md";

pub(crate) const RECEIPT_BASE_OFFICIAL_SOURCE_CAPTURE_READER_PATH: &str =
    "docs/reading/receipt-base-official-source-capture.md";

pub(crate) const IRS_SOI_PUB1304_TY2023_INDIVIDUAL_INCOME_BASE_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/irs_soi_pub1304_ty2023_individual_income_base_context.v1.draft.json";

pub(crate) const IRS_SOI_PUB1304_TY2023_INDIVIDUAL_INCOME_BASE_CONTEXT_READER_PATH: &str =
    "docs/reading/irs-soi-pub1304-ty2023-individual-income-base-context.md";

pub(crate) const IRS_SOI_CORPORATION_COMPLETE_TABLE_2_3_TY2022_CORPORATE_INCOME_BASE_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/irs_soi_corporation_complete_table_2_3_ty2022_corporate_income_base_context.v1.draft.json";

pub(crate) const IRS_SOI_CORPORATION_COMPLETE_TABLE_2_3_TY2022_CORPORATE_INCOME_BASE_CONTEXT_READER_PATH:
    &str =
    "docs/reading/irs-soi-corporation-complete-table-2-3-ty2022-corporate-income-base-context.md";

pub(crate) const RECEIPT_BASE_RECONCILIATION_GAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/receipt_base_reconciliation_gap.v1.draft.json";

pub(crate) const RECEIPT_BASE_RECONCILIATION_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/receipt_base_reconciliation_gap.schema.md";

pub(crate) const RECEIPT_BASE_RECONCILIATION_GAP_READER_PATH: &str =
    "docs/reading/receipt-base-reconciliation-gap.md";

pub(crate) const RECEIPT_BASE_RATE_BRIDGE_READINESS_ROLLUP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/receipt_base_rate_bridge_readiness_rollup.v1.draft.json";

pub(crate) const RECEIPT_BASE_RATE_BRIDGE_READINESS_ROLLUP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/receipt_base_rate_bridge_readiness_rollup.schema.md";

pub(crate) const RECEIPT_BASE_RATE_BRIDGE_READINESS_ROLLUP_READER_PATH: &str =
    "docs/reading/receipt-base-rate-bridge-readiness-rollup.md";

pub(crate) const MEDICARE_HI_RECEIPT_BASE_RECONCILIATION_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_receipt_base_reconciliation.v1.draft.json";

pub(crate) const MEDICARE_HI_RECEIPT_BASE_RECONCILIATION_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_receipt_base_reconciliation.schema.md";

pub(crate) const MEDICARE_HI_RECEIPT_BASE_RECONCILIATION_READER_PATH: &str =
    "docs/reading/medicare-hi-receipt-base-reconciliation.md";

pub(crate) const MEDICARE_HI_PERIMETER_BRIDGE_REQUIREMENTS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_perimeter_bridge_requirements.v1.draft.json";

pub(crate) const MEDICARE_HI_PERIMETER_BRIDGE_REQUIREMENTS_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_perimeter_bridge_requirements.schema.md";

pub(crate) const MEDICARE_HI_PERIMETER_BRIDGE_REQUIREMENTS_READER_PATH: &str =
    "docs/reading/medicare-hi-perimeter-bridge-requirements.md";

pub(crate) const MEDICARE_HI_PAYROLL_TAX_PERIMETER_BRIDGE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_payroll_tax_perimeter_bridge.v1.draft.json";

pub(crate) const MEDICARE_HI_PAYROLL_TAX_PERIMETER_BRIDGE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_payroll_tax_perimeter_bridge.schema.md";

pub(crate) const MEDICARE_HI_PAYROLL_TAX_PERIMETER_BRIDGE_READER_PATH: &str =
    "docs/reading/medicare-hi-payroll-tax-perimeter-bridge.md";

pub(crate) const MEDICARE_HI_BENEFITS_TAX_INCOME_SPLIT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_benefits_tax_income_split.v1.draft.json";

pub(crate) const MEDICARE_HI_BENEFITS_TAX_INCOME_SPLIT_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_benefits_tax_income_split.schema.md";

pub(crate) const MEDICARE_HI_BENEFITS_TAX_INCOME_SPLIT_READER_PATH: &str =
    "docs/reading/medicare-hi-benefits-tax-income-split.md";

pub(crate) const MEDICARE_HI_LEGAL_BASE_DEFINITION_GAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_legal_base_definition_gap.v1.draft.json";

pub(crate) const MEDICARE_HI_LEGAL_BASE_DEFINITION_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_legal_base_definition_gap.schema.md";

pub(crate) const MEDICARE_HI_LEGAL_BASE_DEFINITION_GAP_READER_PATH: &str =
    "docs/reading/medicare-hi-legal-base-definition-gap.md";

pub(crate) const MEDICARE_HI_ECONOMIC_BASE_DEFINITION_GAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_economic_base_definition_gap.v1.draft.json";

pub(crate) const MEDICARE_HI_ECONOMIC_BASE_DEFINITION_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_economic_base_definition_gap.schema.md";

pub(crate) const MEDICARE_HI_ECONOMIC_BASE_DEFINITION_GAP_READER_PATH: &str =
    "docs/reading/medicare-hi-economic-base-definition-gap.md";

pub(crate) const MEDICARE_HI_SOLVER_YIELD_MAPPING_GAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_solver_yield_mapping_gap.v1.draft.json";

pub(crate) const MEDICARE_HI_SOLVER_YIELD_MAPPING_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_solver_yield_mapping_gap.schema.md";

pub(crate) const MEDICARE_HI_SOLVER_YIELD_MAPPING_GAP_READER_PATH: &str =
    "docs/reading/medicare-hi-solver-yield-mapping-gap.md";

pub(crate) const MEDICARE_HI_BEHAVIOR_REFORM_YIELD_GAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_behavior_reform_yield_gap.v1.draft.json";

pub(crate) const MEDICARE_HI_BEHAVIOR_REFORM_YIELD_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_behavior_reform_yield_gap.schema.md";

pub(crate) const MEDICARE_HI_BEHAVIOR_REFORM_YIELD_GAP_READER_PATH: &str =
    "docs/reading/medicare-hi-behavior-reform-yield-gap.md";

pub(crate) const MEDICARE_HI_BRIDGE_STATUS_ROLLUP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_bridge_status_rollup.v1.draft.json";

pub(crate) const MEDICARE_HI_BRIDGE_STATUS_ROLLUP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_bridge_status_rollup.schema.md";

pub(crate) const MEDICARE_HI_BRIDGE_STATUS_ROLLUP_READER_PATH: &str =
    "docs/reading/medicare-hi-bridge-status-rollup.md";

pub(crate) const MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_bridge_closure_work_queue.v1.draft.json";

pub(crate) const MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_bridge_closure_work_queue.schema.md";

pub(crate) const MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_READER_PATH: &str =
    "docs/reading/medicare-hi-bridge-closure-work-queue.md";

pub(crate) const MEDICARE_HI_OMB_CMS_RECEIPT_ROW_PERIMETER_EVIDENCE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_hi_omb_cms_receipt_row_perimeter_evidence.v1.draft.json";

pub(crate) const MEDICARE_HI_OMB_CMS_RECEIPT_ROW_PERIMETER_EVIDENCE_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_hi_omb_cms_receipt_row_perimeter_evidence.schema.md";

pub(crate) const MEDICARE_HI_OMB_CMS_RECEIPT_ROW_PERIMETER_EVIDENCE_READER_PATH: &str =
    "docs/reading/medicare-hi-omb-cms-receipt-row-perimeter-evidence.md";

pub(crate) const MEDICARE_HI_INCOME_CATEGORY_OMB_MAPPING_GAP_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_hi_income_category_omb_mapping_gap.v1.draft.json";

pub(crate) const MEDICARE_HI_INCOME_CATEGORY_OMB_MAPPING_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_income_category_omb_mapping_gap.schema.md";

pub(crate) const MEDICARE_HI_INCOME_CATEGORY_OMB_MAPPING_GAP_READER_PATH: &str =
    "docs/reading/medicare-hi-income-category-omb-mapping-gap.md";

pub(crate) const MEDICARE_HI_LEGAL_BASE_CLOSURE_GAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_legal_base_closure_gap.v1.draft.json";

pub(crate) const MEDICARE_HI_LEGAL_BASE_CLOSURE_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_legal_base_closure_gap.schema.md";

pub(crate) const MEDICARE_HI_LEGAL_BASE_CLOSURE_GAP_READER_PATH: &str =
    "docs/reading/medicare-hi-legal-base-closure-gap.md";

pub(crate) const MEDICARE_HI_ECONOMIC_BASE_CLOSURE_GAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_economic_base_closure_gap.v1.draft.json";

pub(crate) const MEDICARE_HI_ECONOMIC_BASE_CLOSURE_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_economic_base_closure_gap.schema.md";

pub(crate) const MEDICARE_HI_ECONOMIC_BASE_CLOSURE_GAP_READER_PATH: &str =
    "docs/reading/medicare-hi-economic-base-closure-gap.md";

pub(crate) const MEDICARE_HI_TRUST_FUND_SOLVER_YIELD_CLOSURE_GAP_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_hi_trust_fund_solver_yield_closure_gap.v1.draft.json";

pub(crate) const MEDICARE_HI_TRUST_FUND_SOLVER_YIELD_CLOSURE_GAP_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_hi_trust_fund_solver_yield_closure_gap.schema.md";

pub(crate) const MEDICARE_HI_TRUST_FUND_SOLVER_YIELD_CLOSURE_GAP_READER_PATH: &str =
    "docs/reading/medicare-hi-trust-fund-solver-yield-closure-gap.md";

pub(crate) const MEDICARE_HI_POLICY_BEHAVIOR_REFORM_YIELD_CLOSURE_GAP_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_hi_policy_behavior_reform_yield_closure_gap.v1.draft.json";

pub(crate) const MEDICARE_HI_POLICY_BEHAVIOR_REFORM_YIELD_CLOSURE_GAP_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_hi_policy_behavior_reform_yield_closure_gap.schema.md";

pub(crate) const MEDICARE_HI_POLICY_BEHAVIOR_REFORM_YIELD_CLOSURE_GAP_READER_PATH: &str =
    "docs/reading/medicare-hi-policy-behavior-reform-yield-closure-gap.md";

pub(crate) const MEDICARE_HI_RATE_SOLVER_READINESS_REVIEW_CLOSURE_GAP_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_hi_rate_solver_readiness_review_closure_gap.v1.draft.json";

pub(crate) const MEDICARE_HI_RATE_SOLVER_READINESS_REVIEW_CLOSURE_GAP_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_hi_rate_solver_readiness_review_closure_gap.schema.md";

pub(crate) const MEDICARE_HI_RATE_SOLVER_READINESS_REVIEW_CLOSURE_GAP_READER_PATH: &str =
    "docs/reading/medicare-hi-rate-solver-readiness-review-closure-gap.md";

pub(crate) const MEDICARE_HI_CLOSURE_SERIES_ROLLUP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_closure_series_rollup.v1.draft.json";

pub(crate) const MEDICARE_HI_CLOSURE_SERIES_ROLLUP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/medicare_hi_closure_series_rollup.schema.md";

pub(crate) const MEDICARE_HI_CLOSURE_SERIES_ROLLUP_READER_PATH: &str =
    "docs/reading/medicare-hi-closure-series-rollup.md";

pub(crate) const POST_MEDICARE_HI_NEXT_READINESS_QUEUE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/post_medicare_hi_next_readiness_queue.v1.draft.json";

pub(crate) const POST_MEDICARE_HI_NEXT_READINESS_QUEUE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/post_medicare_hi_next_readiness_queue.schema.md";

pub(crate) const POST_MEDICARE_HI_NEXT_READINESS_QUEUE_READER_PATH: &str =
    "docs/reading/post-medicare-hi-next-readiness-queue.md";

pub(crate) const SOURCE_CUSTODY_CURRENT_LAW_PATHS_GAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/source_custody_current_law_paths_gap.v1.draft.json";

pub(crate) const SOURCE_CUSTODY_CURRENT_LAW_PATHS_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/source_custody_current_law_paths_gap.schema.md";

pub(crate) const SOURCE_CUSTODY_CURRENT_LAW_PATHS_GAP_READER_PATH: &str =
    "docs/reading/source-custody-current-law-paths-gap.md";

pub(crate) const TRUST_FUND_FUND_GROUP_RECONCILIATION_GAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trust_fund_fund_group_reconciliation_gap.v1.draft.json";

pub(crate) const TRUST_FUND_FUND_GROUP_RECONCILIATION_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trust_fund_fund_group_reconciliation_gap.schema.md";

pub(crate) const TRUST_FUND_FUND_GROUP_RECONCILIATION_GAP_READER_PATH: &str =
    "docs/reading/trust-fund-fund-group-reconciliation-gap.md";

pub(crate) const OUTCOME_FLOOR_THRESHOLDS_GAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/outcome_floor_thresholds_gap.v1.draft.json";

pub(crate) const OUTCOME_FLOOR_THRESHOLDS_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/outcome_floor_thresholds_gap.schema.md";

pub(crate) const OUTCOME_FLOOR_THRESHOLDS_GAP_READER_PATH: &str =
    "docs/reading/outcome-floor-thresholds-gap.md";

pub(crate) const HEALTH_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_outcome_floor_definition_packet.v1.draft.json";

pub(crate) const HEALTH_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_outcome_floor_definition_packet.schema.md";

pub(crate) const HEALTH_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH: &str =
    "docs/reading/health-outcome-floor-definition-packet.md";

pub(crate) const HEALTH_MEDICARE_PROVIDER_ADEQUACY_MARGIN_FLOOR_VALUE_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/health_medicare_provider_adequacy_margin_floor_value_packet.v1.draft.json";

pub(crate) const HEALTH_MEDICARE_PROVIDER_ADEQUACY_MARGIN_FLOOR_VALUE_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/health_medicare_provider_adequacy_margin_floor_value_packet.schema.md";

pub(crate) const HEALTH_MEDICARE_PROVIDER_ADEQUACY_MARGIN_FLOOR_VALUE_PACKET_READER_PATH: &str =
    "docs/reading/health-medicare-provider-adequacy-margin-floor-value-packet.md";

pub(crate) const SOCIAL_SECURITY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_outcome_floor_definition_packet.v1.draft.json";

pub(crate) const SOCIAL_SECURITY_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_outcome_floor_definition_packet.schema.md";

pub(crate) const SOCIAL_SECURITY_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH: &str =
    "docs/reading/social-security-outcome-floor-definition-packet.md";

pub(crate) const SOCIAL_SECURITY_SOURCE_READINESS_GAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/social_security_source_readiness_gap.v1.draft.json";

pub(crate) const SOCIAL_SECURITY_SOURCE_READINESS_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/social_security_source_readiness_gap.schema.md";

pub(crate) const SOCIAL_SECURITY_SOURCE_READINESS_GAP_READER_PATH: &str =
    "docs/reading/social-security-source-readiness-gap.md";

pub(crate) const SOCIAL_SECURITY_SOURCE_CAPTURE_QUEUE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/social_security_source_capture_queue.v1.draft.json";

pub(crate) const SOCIAL_SECURITY_SOURCE_CAPTURE_QUEUE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/social_security_source_capture_queue.schema.md";

pub(crate) const SOCIAL_SECURITY_SOURCE_CAPTURE_QUEUE_READER_PATH: &str =
    "docs/reading/social-security-source-capture-queue.md";

pub(crate) const SOCIAL_SECURITY_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_trustees_source_capture_status.v1.draft.json";

pub(crate) const SOCIAL_SECURITY_TRUSTEES_SOURCE_CAPTURE_STATUS_READER_PATH: &str =
    "docs/reading/social-security-trustees-source-capture-status.md";

pub(crate) const SOCIAL_SECURITY_OASDI_FY2025_2035_CURRENT_LAW_PATH_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_oasdi_fy2025_2035_current_law_path.v1.draft.json";

pub(crate) const SOCIAL_SECURITY_OASDI_FY2025_2035_CURRENT_LAW_PATH_READER_PATH: &str =
    "docs/reading/social-security-oasdi-fy2025-2035-current-law-path.md";

pub(crate) const SOCIAL_SECURITY_TAXABLE_PAYROLL_BASE_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_taxable_payroll_base_bridge.cy2025-2035.v1.draft.json";

pub(crate) const SOCIAL_SECURITY_TAXABLE_PAYROLL_BASE_BRIDGE_READER_PATH: &str =
    "docs/reading/social-security-taxable-payroll-base-bridge.md";

pub(crate) const SOCIAL_SECURITY_OASDI_RECEIPT_YIELD_BOUNDARY_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_oasdi_receipt_yield_boundary.fy2025-cy2025.v1.draft.json";

pub(crate) const SOCIAL_SECURITY_OASDI_RECEIPT_YIELD_BOUNDARY_READER_PATH: &str =
    "docs/reading/social-security-oasdi-receipt-yield-boundary.md";

pub(crate) const SOCIAL_SECURITY_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_source_capture_status_rollup.v1.draft.json";

pub(crate) const SOCIAL_SECURITY_SOURCE_CAPTURE_STATUS_ROLLUP_READER_PATH: &str =
    "docs/reading/social-security-source-capture-status-rollup.md";

pub(crate) const SOCIAL_SECURITY_BENEFIT_ADEQUACY_CONTEXT_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_benefit_adequacy_context_bridge.v1.draft.json";

pub(crate) const SOCIAL_SECURITY_BENEFIT_ADEQUACY_CONTEXT_BRIDGE_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_benefit_adequacy_context_bridge.schema.md";

pub(crate) const SOCIAL_SECURITY_BENEFIT_ADEQUACY_CONTEXT_BRIDGE_READER_PATH: &str =
    "docs/reading/social-security-benefit-adequacy-context-bridge.md";

pub(crate) const SOCIAL_SECURITY_OLD_AGE_POVERTY_CONTEXT_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_old_age_poverty_context_bridge.v1.draft.json";

pub(crate) const SOCIAL_SECURITY_OLD_AGE_POVERTY_CONTEXT_BRIDGE_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_old_age_poverty_context_bridge.schema.md";

pub(crate) const SOCIAL_SECURITY_OLD_AGE_POVERTY_CONTEXT_BRIDGE_READER_PATH: &str =
    "docs/reading/social-security-old-age-poverty-context-bridge.md";

pub(crate) const SOCIAL_SECURITY_DOMESTIC_OLD_AGE_POVERTY_CONTEXT_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_domestic_old_age_poverty_context_bridge.v1.draft.json";

pub(crate) const SOCIAL_SECURITY_DOMESTIC_OLD_AGE_POVERTY_CONTEXT_BRIDGE_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_domestic_old_age_poverty_context_bridge.schema.md";

pub(crate) const SOCIAL_SECURITY_DOMESTIC_OLD_AGE_POVERTY_CONTEXT_BRIDGE_READER_PATH: &str =
    "docs/reading/social-security-domestic-old-age-poverty-context-bridge.md";

pub(crate) const SOCIAL_SECURITY_ADMINISTRATION_SERVICE_CONTEXT_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_administration_service_context_bridge.v1.draft.json";

pub(crate) const SOCIAL_SECURITY_ADMINISTRATION_SERVICE_CONTEXT_BRIDGE_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_administration_service_context_bridge.schema.md";

pub(crate) const SOCIAL_SECURITY_ADMINISTRATION_SERVICE_CONTEXT_BRIDGE_READER_PATH: &str =
    "docs/reading/social-security-administration-service-context-bridge.md";

pub(crate) const SOCIAL_SECURITY_OLD_AGE_POVERTY_FLOOR_VALUE_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_old_age_poverty_floor_value_packet.v1.draft.json";

pub(crate) const SOCIAL_SECURITY_OLD_AGE_POVERTY_FLOOR_VALUE_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/social_security_old_age_poverty_floor_value_packet.schema.md";

pub(crate) const SOCIAL_SECURITY_OLD_AGE_POVERTY_FLOOR_VALUE_PACKET_READER_PATH: &str =
    "docs/reading/social-security-old-age-poverty-floor-value-packet.md";

pub(crate) const DENOMINATOR_VALUES_CY2025_SSA_TRUSTEES_JSONL_PATH: &str =
    "data/derived/denominator_requirements/denominator_values.cy2025.ssa-trustees-2026.draft.jsonl";

pub(crate) const DEFENSE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/defense_outcome_floor_definition_packet.v1.draft.json";

pub(crate) const DEFENSE_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/defense_outcome_floor_definition_packet.schema.md";

pub(crate) const DEFENSE_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH: &str =
    "docs/reading/defense-outcome-floor-definition-packet.md";

pub(crate) const DEFENSE_SOURCE_READINESS_GAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/defense_source_readiness_gap.v1.draft.json";

pub(crate) const DEFENSE_SOURCE_READINESS_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/defense_source_readiness_gap.schema.md";

pub(crate) const DEFENSE_SOURCE_READINESS_GAP_READER_PATH: &str =
    "docs/reading/defense-source-readiness-gap.md";

pub(crate) const DEFENSE_SOURCE_CAPTURE_QUEUE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/defense_source_capture_queue.v1.draft.json";

pub(crate) const DEFENSE_SOURCE_CAPTURE_QUEUE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/defense_source_capture_queue.schema.md";

pub(crate) const DEFENSE_SOURCE_CAPTURE_QUEUE_READER_PATH: &str =
    "docs/reading/defense-source-capture-queue.md";

pub(crate) const DEFENSE_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/defense_source_capture_status_rollup.v1.draft.json";

pub(crate) const DEFENSE_SOURCE_CAPTURE_STATUS_ROLLUP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/defense_source_capture_status_rollup.schema.md";

pub(crate) const DEFENSE_SOURCE_CAPTURE_STATUS_ROLLUP_READER_PATH: &str =
    "docs/reading/defense-source-capture-status-rollup.md";

pub(crate) const DEFENSE_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/defense_source_capture_closure_work_queue.v1.draft.json";

pub(crate) const DEFENSE_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/defense_source_capture_closure_work_queue.schema.md";

pub(crate) const DEFENSE_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_READER_PATH: &str =
    "docs/reading/defense-source-capture-closure-work-queue.md";

pub(crate) const INCOME_SECURITY_FAMILY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_outcome_floor_definition_packet.v1.draft.json";

pub(crate) const INCOME_SECURITY_FAMILY_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_outcome_floor_definition_packet.schema.md";

pub(crate) const INCOME_SECURITY_FAMILY_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH: &str =
    "docs/reading/income-security-family-outcome-floor-definition-packet.md";

pub(crate) const INCOME_SECURITY_FAMILY_SOURCE_READINESS_GAP_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_source_readiness_gap.v1.draft.json";

pub(crate) const INCOME_SECURITY_FAMILY_SOURCE_READINESS_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/income_security_family_source_readiness_gap.schema.md";

pub(crate) const INCOME_SECURITY_FAMILY_SOURCE_READINESS_GAP_READER_PATH: &str =
    "docs/reading/income-security-family-source-readiness-gap.md";

pub(crate) const INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_source_capture_queue.v1.draft.json";

pub(crate) const INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/income_security_family_source_capture_queue.schema.md";

pub(crate) const INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_READER_PATH: &str =
    "docs/reading/income-security-family-source-capture-queue.md";

pub(crate) const INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_source_capture_status_rollup.v1.draft.json";

pub(crate) const INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_STATUS_ROLLUP_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_source_capture_status_rollup.schema.md";

pub(crate) const INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_STATUS_ROLLUP_READER_PATH: &str =
    "docs/reading/income-security-family-source-capture-status-rollup.md";

pub(crate) const INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_source_capture_closure_work_queue.v1.draft.json";

pub(crate) const INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_source_capture_closure_work_queue.schema.md";

pub(crate) const INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_READER_PATH: &str =
    "docs/reading/income-security-family-source-capture-closure-work-queue.md";

pub(crate) const INCOME_SECURITY_FAMILY_FEDERAL_PROGRAM_PERIMETER_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_federal_program_perimeter_bridge.fy2025.v1.draft.json";

pub(crate) const INCOME_SECURITY_FAMILY_FEDERAL_PROGRAM_PERIMETER_BRIDGE_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_federal_program_perimeter_bridge.schema.md";

pub(crate) const INCOME_SECURITY_FAMILY_FEDERAL_PROGRAM_PERIMETER_BRIDGE_READER_PATH: &str =
    "docs/reading/income-security-family-federal-program-perimeter-bridge.md";

pub(crate) const INCOME_SECURITY_FAMILY_CBO_BASELINE_TAKEUP_CAPTURE_GAP_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_cbo_baseline_takeup_capture_gap.v1.draft.json";

pub(crate) const INCOME_SECURITY_FAMILY_CBO_BASELINE_TAKEUP_CAPTURE_GAP_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_cbo_baseline_takeup_capture_gap.schema.md";

pub(crate) const INCOME_SECURITY_FAMILY_CBO_BASELINE_TAKEUP_CAPTURE_GAP_READER_PATH: &str =
    "docs/reading/income-security-family-cbo-baseline-takeup-capture-gap.md";

pub(crate) const INCOME_SECURITY_FAMILY_CHILD_RELATIVE_POVERTY_CONTEXT_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_child_relative_poverty_context_bridge.v1.draft.json";

pub(crate) const INCOME_SECURITY_FAMILY_CHILD_RELATIVE_POVERTY_CONTEXT_BRIDGE_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_child_relative_poverty_context_bridge.schema.md";

pub(crate) const INCOME_SECURITY_FAMILY_CHILD_RELATIVE_POVERTY_CONTEXT_BRIDGE_READER_PATH: &str =
    "docs/reading/income-security-family-child-relative-poverty-context-bridge.md";

pub(crate) const INCOME_SECURITY_FAMILY_SOCX_FAMILY_BENEFIT_COMPARATOR_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_socx_family_benefit_comparator_bridge.v1.draft.json";

pub(crate) const INCOME_SECURITY_FAMILY_SOCX_FAMILY_BENEFIT_COMPARATOR_BRIDGE_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_socx_family_benefit_comparator_bridge.schema.md";

pub(crate) const INCOME_SECURITY_FAMILY_SOCX_FAMILY_BENEFIT_COMPARATOR_BRIDGE_READER_PATH: &str =
    "docs/reading/income-security-family-socx-family-benefit-comparator-bridge.md";

pub(crate) const INCOME_SECURITY_FAMILY_CHILDCARE_FAMILY_SERVICE_CAPTURE_GAP_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_childcare_family_service_capture_gap.v1.draft.json";

pub(crate) const INCOME_SECURITY_FAMILY_CHILDCARE_FAMILY_SERVICE_CAPTURE_GAP_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_childcare_family_service_capture_gap.schema.md";

pub(crate) const INCOME_SECURITY_FAMILY_CHILDCARE_FAMILY_SERVICE_CAPTURE_GAP_READER_PATH: &str =
    "docs/reading/income-security-family-childcare-family-service-capture-gap.md";

pub(crate) const INCOME_SECURITY_FAMILY_FOOD_HARDSHIP_NUTRITION_CAPTURE_GAP_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_food_hardship_nutrition_capture_gap.v1.draft.json";

pub(crate) const INCOME_SECURITY_FAMILY_FOOD_HARDSHIP_NUTRITION_CAPTURE_GAP_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_food_hardship_nutrition_capture_gap.schema.md";

pub(crate) const INCOME_SECURITY_FAMILY_FOOD_HARDSHIP_NUTRITION_CAPTURE_GAP_READER_PATH: &str =
    "docs/reading/income-security-family-food-hardship-nutrition-capture-gap.md";

pub(crate) const INCOME_SECURITY_FAMILY_CENSUS_CHILD_POVERTY_INCOME_CAPTURE_GAP_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_census_child_poverty_income_capture_gap.v1.draft.json";

pub(crate) const INCOME_SECURITY_FAMILY_CENSUS_CHILD_POVERTY_INCOME_CAPTURE_GAP_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_census_child_poverty_income_capture_gap.schema.md";

pub(crate) const INCOME_SECURITY_FAMILY_CENSUS_CHILD_POVERTY_INCOME_CAPTURE_GAP_READER_PATH: &str =
    "docs/reading/income-security-family-census-child-poverty-income-capture-gap.md";

pub(crate) const INCOME_SECURITY_FAMILY_CHILD_POVERTY_FLOOR_VALUE_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_child_poverty_floor_value_packet.v1.draft.json";

pub(crate) const INCOME_SECURITY_FAMILY_CHILD_POVERTY_FLOOR_VALUE_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/income_security_family_child_poverty_floor_value_packet.schema.md";

pub(crate) const INCOME_SECURITY_FAMILY_CHILD_POVERTY_FLOOR_VALUE_PACKET_READER_PATH: &str =
    "docs/reading/income-security-family-child-poverty-floor-value-packet.md";

pub(crate) const TAXLANE_SHOWCASE_READINESS_SUMMARY_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/taxlane_showcase_readiness_summary.v1.draft.json";

pub(crate) const TAXLANE_SHOWCASE_READINESS_SUMMARY_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/taxlane_showcase_readiness_summary.schema.md";

pub(crate) const TAXLANE_SHOWCASE_READINESS_SUMMARY_READER_PATH: &str =
    "docs/reading/taxlane-showcase-readiness-summary.md";

pub(crate) const REVENUE_SOLVENCY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/revenue_solvency_outcome_floor_definition_packet.v1.draft.json";

pub(crate) const REVENUE_SOLVENCY_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/revenue_solvency_outcome_floor_definition_packet.schema.md";

pub(crate) const REVENUE_SOLVENCY_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH: &str =
    "docs/reading/revenue-solvency-outcome-floor-definition-packet.md";

pub(crate) const REVENUE_SOLVENCY_TOTAL_RECEIPTS_FLOOR_VALUE_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/revenue_solvency_total_receipts_floor_value_packet.v1.draft.json";

pub(crate) const REVENUE_SOLVENCY_TOTAL_RECEIPTS_FLOOR_VALUE_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/revenue_solvency_total_receipts_floor_value_packet.schema.md";

pub(crate) const REVENUE_SOLVENCY_TOTAL_RECEIPTS_FLOOR_VALUE_PACKET_READER_PATH: &str =
    "docs/reading/revenue-solvency-total-receipts-floor-value-packet.md";

pub(crate) const NET_INTEREST_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/net_interest_outcome_floor_definition_packet.v1.draft.json";

pub(crate) const NET_INTEREST_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_interest_outcome_floor_definition_packet.schema.md";

pub(crate) const NET_INTEREST_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH: &str =
    "docs/reading/net-interest-outcome-floor-definition-packet.md";

pub(crate) const NET_INTEREST_AVERAGE_RATE_FLOOR_VALUE_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/net_interest_average_rate_floor_value_packet.v1.draft.json";

pub(crate) const NET_INTEREST_AVERAGE_RATE_FLOOR_VALUE_PACKET_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_interest_average_rate_floor_value_packet.schema.md";

pub(crate) const NET_INTEREST_AVERAGE_RATE_FLOOR_VALUE_PACKET_READER_PATH: &str =
    "docs/reading/net-interest-average-rate-floor-value-packet.md";

pub(crate) const TRANSPORTATION_ROADWAY_FATALITY_RATE_FLOOR_VALUE_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_roadway_fatality_rate_floor_value_packet.v1.draft.json";

pub(crate) const TRANSPORTATION_ROADWAY_FATALITY_RATE_FLOOR_VALUE_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_roadway_fatality_rate_floor_value_packet.schema.md";

pub(crate) const TRANSPORTATION_ROADWAY_FATALITY_RATE_FLOOR_VALUE_PACKET_READER_PATH: &str =
    "docs/reading/transportation-roadway-fatality-rate-floor-value-packet.md";

pub(crate) const PAYMENT_INTEGRITY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/payment_integrity_outcome_floor_definition_packet.v1.draft.json";

pub(crate) const PAYMENT_INTEGRITY_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/payment_integrity_outcome_floor_definition_packet.schema.md";

pub(crate) const PAYMENT_INTEGRITY_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH: &str =
    "docs/reading/payment-integrity-outcome-floor-definition-packet.md";

pub(crate) const PAYMENT_INTEGRITY_FCIC_PAYMENT_ACCURACY_FLOOR_VALUE_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/payment_integrity_fcic_payment_accuracy_floor_value_packet.v1.draft.json";

pub(crate) const PAYMENT_INTEGRITY_FCIC_PAYMENT_ACCURACY_FLOOR_VALUE_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/payment_integrity_fcic_payment_accuracy_floor_value_packet.schema.md";

pub(crate) const PAYMENT_INTEGRITY_FCIC_PAYMENT_ACCURACY_FLOOR_VALUE_PACKET_READER_PATH: &str =
    "docs/reading/payment-integrity-fcic-payment-accuracy-floor-value-packet.md";

pub(crate) const VETERANS_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/veterans_outcome_floor_definition_packet.v1.draft.json";

pub(crate) const VETERANS_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/veterans_outcome_floor_definition_packet.schema.md";

pub(crate) const VETERANS_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH: &str =
    "docs/reading/veterans-outcome-floor-definition-packet.md";

pub(crate) const VETERANS_CLAIMS_BACKLOG_FLOOR_VALUE_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/veterans_claims_backlog_floor_value_packet.v1.draft.json";

pub(crate) const VETERANS_CLAIMS_BACKLOG_FLOOR_VALUE_PACKET_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/veterans_claims_backlog_floor_value_packet.schema.md";

pub(crate) const VETERANS_CLAIMS_BACKLOG_FLOOR_VALUE_PACKET_READER_PATH: &str =
    "docs/reading/veterans-claims-backlog-floor-value-packet.md";

pub(crate) const TRANSPORTATION_INFRASTRUCTURE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_infrastructure_outcome_floor_definition_packet.v1.draft.json";

pub(crate) const TRANSPORTATION_INFRASTRUCTURE_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_infrastructure_outcome_floor_definition_packet.schema.md";

pub(crate) const TRANSPORTATION_INFRASTRUCTURE_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH: &str =
    "docs/reading/transportation-infrastructure-outcome-floor-definition-packet.md";

pub(crate) const EDUCATION_WORKFORCE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/education_workforce_outcome_floor_definition_packet.v1.draft.json";

pub(crate) const EDUCATION_WORKFORCE_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/education_workforce_outcome_floor_definition_packet.schema.md";

pub(crate) const EDUCATION_WORKFORCE_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH: &str =
    "docs/reading/education-workforce-outcome-floor-definition-packet.md";

pub(crate) const EDUCATION_WORKFORCE_GRADUATION_FLOOR_VALUE_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/education_workforce_graduation_floor_value_packet.v1.draft.json";

pub(crate) const EDUCATION_WORKFORCE_GRADUATION_FLOOR_VALUE_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/education_workforce_graduation_floor_value_packet.schema.md";

pub(crate) const EDUCATION_WORKFORCE_GRADUATION_FLOOR_VALUE_PACKET_READER_PATH: &str =
    "docs/reading/education-workforce-graduation-floor-value-packet.md";

pub(crate) const DISASTER_RESILIENCE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/disaster_resilience_outcome_floor_definition_packet.v1.draft.json";

pub(crate) const DISASTER_RESILIENCE_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/disaster_resilience_outcome_floor_definition_packet.schema.md";

pub(crate) const DISASTER_RESILIENCE_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH: &str =
    "docs/reading/disaster-resilience-outcome-floor-definition-packet.md";

pub(crate) const JUSTICE_COURTS_PUBLIC_SAFETY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/justice_courts_public_safety_outcome_floor_definition_packet.v1.draft.json";

pub(crate) const JUSTICE_COURTS_PUBLIC_SAFETY_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/justice_courts_public_safety_outcome_floor_definition_packet.schema.md";

pub(crate) const JUSTICE_COURTS_PUBLIC_SAFETY_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH: &str =
    "docs/reading/justice-courts-public-safety-outcome-floor-definition-packet.md";

pub(crate) const SCIENCE_ENERGY_ENVIRONMENT_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/science_energy_environment_outcome_floor_definition_packet.v1.draft.json";

pub(crate) const SCIENCE_ENERGY_ENVIRONMENT_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/science_energy_environment_outcome_floor_definition_packet.schema.md";

pub(crate) const SCIENCE_ENERGY_ENVIRONMENT_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH: &str =
    "docs/reading/science-energy-environment-outcome-floor-definition-packet.md";

pub(crate) const AGRICULTURE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/agriculture_outcome_floor_definition_packet.v1.draft.json";

pub(crate) const AGRICULTURE_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/agriculture_outcome_floor_definition_packet.schema.md";

pub(crate) const AGRICULTURE_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH: &str =
    "docs/reading/agriculture-outcome-floor-definition-packet.md";

pub(crate) const INTERNATIONAL_AFFAIRS_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/international_affairs_outcome_floor_definition_packet.v1.draft.json";

pub(crate) const INTERNATIONAL_AFFAIRS_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/international_affairs_outcome_floor_definition_packet.schema.md";

pub(crate) const INTERNATIONAL_AFFAIRS_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH: &str =
    "docs/reading/international-affairs-outcome-floor-definition-packet.md";

pub(crate) const LANE_FLOOR_READINESS_ROLLUP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/lane_floor_readiness_rollup.v1.draft.json";

pub(crate) const LANE_FLOOR_READINESS_ROLLUP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/lane_floor_readiness_rollup.schema.md";

pub(crate) const LANE_FLOOR_READINESS_ROLLUP_READER_PATH: &str = "docs/reading/lane-floor-readiness-rollup.md";

pub(crate) const LANE_FLOOR_SOURCE_WORK_QUEUE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/lane_floor_source_work_queue.v1.draft.json";

pub(crate) const LANE_FLOOR_SOURCE_WORK_QUEUE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/lane_floor_source_work_queue.schema.md";

pub(crate) const LANE_FLOOR_SOURCE_WORK_QUEUE_READER_PATH: &str =
    "docs/reading/lane-floor-source-work-queue.md";

pub(crate) const OUTCOME_FLOOR_WAVE_D_VALUE_READINESS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/outcome_floor_wave_d_value_readiness.v1.draft.json";

pub(crate) const OUTCOME_FLOOR_WAVE_D_VALUE_READINESS_READER_PATH: &str =
    "docs/reading/outcome-floor-wave-d-value-readiness.md";

pub(crate) const LANE_SCENARIO_PACK_WAVE_E_READINESS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/lane_scenario_pack_wave_e_readiness.v1.draft.json";

pub(crate) const LANE_SCENARIO_PACK_WAVE_E_READINESS_READER_PATH: &str =
    "docs/reading/lane-scenario-pack-wave-e-readiness.md";

pub(crate) const WAVE_E_REFERENCE_SCENARIO_PACKS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/wave_e_reference_scenario_packs.v1.draft.json";

pub(crate) const WAVE_E_REFERENCE_SCENARIO_PACKS_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/wave_e_reference_scenario_packs.schema.md";

pub(crate) const WAVE_E_REFERENCE_SCENARIO_PACKS_READER_PATH: &str =
    "docs/reading/wave-e-reference-scenario-packs.md";

pub(crate) const WAVE_E_REFERENCE_SCENARIO_PACKS_ROLE_REVIEW_PATH: &str =
    "reviews/2026-07-26-wave-e-reference-scenario-packs-role-review.md";

pub(crate) const WAVE_F_TRANSPORTATION_CALIBRATION_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/wave_f_transportation_deterministic_calibration.v1.draft.json";

pub(crate) const WAVE_F_TRANSPORTATION_CALIBRATION_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/wave_f_transportation_deterministic_calibration.schema.md";

pub(crate) const WAVE_F_TRANSPORTATION_CALIBRATION_READER_PATH: &str =
    "docs/reading/wave-f-transportation-deterministic-calibration.md";

pub(crate) const WAVE_F_TRANSPORTATION_CALIBRATION_ROLE_REVIEW_PATH: &str =
    "reviews/2026-07-26-wave-f-transportation-deterministic-calibration-role-review.md";

pub(crate) const WAVE_G_SOLVER_SPINE_CONTRACT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/wave_g_official_current_law_solver_spine_contract.v1.draft.json";

pub(crate) const WAVE_G_SOLVER_SPINE_CONTRACT_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/wave_g_official_current_law_solver_spine_contract.schema.md";

pub(crate) const WAVE_G_SOLVER_SPINE_CONTRACT_READER_PATH: &str =
    "docs/reading/wave-g-official-current-law-solver-spine-contract.md";

pub(crate) const POST_F_WAVE_ROADMAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/adaptive_rate_post_f_wave_roadmap.v1.draft.json";

pub(crate) const POST_F_WAVE_ROADMAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/adaptive_rate_post_f_wave_roadmap.schema.md";

pub(crate) const POST_F_WAVE_ROADMAP_READER_PATH: &str = "docs/reading/adaptive-rate-post-f-wave-roadmap.md";

pub(crate) const CORPUS_TRACK_PLAN_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/adaptive_rate_corpus_track_plan.v1.draft.json";

pub(crate) const CORPUS_TRACK_PLAN_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/adaptive_rate_corpus_track_plan.schema.md";

pub(crate) const CORPUS_TRACK_PLAN_READER_PATH: &str = "docs/reading/adaptive-rate-corpus-track-plan.md";

pub(crate) const CORE_G_SOLVER_SPINE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/core_g_official_current_law_solver_spine.v1.draft.json";

pub(crate) const CORE_G_SOLVER_SPINE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/core_g_official_current_law_solver_spine.schema.md";

pub(crate) const CORE_G_SOLVER_SPINE_READER_PATH: &str =
    "docs/reading/core-g-official-current-law-solver-spine.md";

pub(crate) const CORE_G_SOLVER_SPINE_ROLE_REVIEW_PATH: &str =
    "reviews/2026-07-26-core-g-official-current-law-solver-spine-role-review.md";

pub(crate) const TRN_A_BASELINE_SPINE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/trn_a_transportation_baseline_source_spine.v1.draft.json";

pub(crate) const TRN_A_BASELINE_SPINE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_a_transportation_baseline_source_spine.schema.md";

pub(crate) const TRN_A_BASELINE_SPINE_READER_PATH: &str =
    "docs/reading/trn-a-transportation-baseline-source-spine.md";

pub(crate) const TRN_A_BASELINE_SPINE_ROLE_REVIEW_PATH: &str =
    "reviews/2026-07-26-trn-a-transportation-baseline-source-spine-role-review.md";

pub(crate) const CORE_H_ACCOUNTING_SUBSTRATE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/core_h_shared_accounting_substrate.v1.draft.json";

pub(crate) const CORE_H_ACCOUNTING_SUBSTRATE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/core_h_shared_accounting_substrate.schema.md";

pub(crate) const CORE_H_ACCOUNTING_SUBSTRATE_READER_PATH: &str =
    "docs/reading/core-h-shared-accounting-substrate.md";

pub(crate) const CORE_H_ACCOUNTING_SUBSTRATE_ROLE_REVIEW_PATH: &str =
    "reviews/2026-07-26-core-h-shared-accounting-substrate-role-review.md";

pub(crate) const TRN_B_START_GATE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/trn_b_transportation_accounting_start_gate.v1.draft.json";

pub(crate) const TRN_B_START_GATE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_b_transportation_accounting_start_gate.schema.md";

pub(crate) const TRN_B_START_GATE_READER_PATH: &str =
    "docs/reading/trn-b-transportation-accounting-start-gate.md";

pub(crate) const TRN_B_NAMED_FUND_ADAPTER_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_b_named_fund_adapter_rows.v1.draft.json";

pub(crate) const TRN_B_NAMED_FUND_ADAPTER_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_b_named_fund_adapter_rows.schema.md";

pub(crate) const TRN_B_SOURCE_BRIDGE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_b_source_bridge_decisions.v1.draft.json";

pub(crate) const TRN_B_SOURCE_BRIDGE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_b_source_bridge_decisions.schema.md";

pub(crate) const TRN_B_FUNCTION_400_MAPPING_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_b_function_400_mapping.v1.draft.json";

pub(crate) const TRN_B_FUNCTION_400_MAPPING_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_b_function_400_mapping.schema.md";

pub(crate) const TRN_B_ACCOUNTING_SCHEDULES_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_b_accounting_schedules.v1.draft.json";

pub(crate) const TRN_B_ACCOUNTING_SCHEDULES_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_b_accounting_schedules.schema.md";

pub(crate) const TRN_B_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_b_transportation_accounting_closure.v1.draft.json";

pub(crate) const TRN_B_CLOSURE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_b_transportation_accounting_closure.schema.md";

pub(crate) const TRN_B_CLOSURE_READER_PATH: &str = "docs/reading/trn-b-transportation-accounting-closure.md";

pub(crate) const TRN_B_CLOSURE_ROLE_REVIEW_PATH: &str =
    "reviews/2026-07-26-trn-b-transportation-accounting-closure-role-review.md";

pub(crate) const TRN_C_START_GATE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_c_real_reform_start_gate.v1.draft.json";

pub(crate) const TRN_C_START_GATE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_c_real_reform_start_gate.schema.md";

pub(crate) const TRN_C_START_GATE_READER_PATH: &str = "docs/reading/trn-c-real-reform-start-gate.md";

pub(crate) const TRN_C_CANDIDATE_SCREEN_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_c_candidate_screen.v1.draft.json";

pub(crate) const TRN_C_CANDIDATE_SCREEN_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_c_candidate_screen.schema.md";

pub(crate) const TRN_C_CANDIDATE_SCREEN_READER_PATH: &str = "docs/reading/trn-c-candidate-screen.md";

pub(crate) const TRN_C_SCENARIO_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_c_airmen_certificate_reform_scenario.v1.draft.json";

pub(crate) const TRN_C_SCENARIO_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_c_airmen_certificate_reform_scenario.schema.md";

pub(crate) const TRN_C_SCENARIO_READER_PATH: &str = "docs/reading/trn-c-airmen-certificate-reform-scenario.md";

pub(crate) const CORE_I_REFORM_ADMISSION_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/core_i_shared_reform_admission_contract.v1.draft.json";

pub(crate) const CORE_I_REFORM_ADMISSION_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/core_i_shared_reform_admission_contract.schema.md";

pub(crate) const CORE_I_REFORM_ADMISSION_READER_PATH: &str =
    "docs/reading/core-i-shared-reform-admission-contract.md";

pub(crate) const TRN_C_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_c_real_reform_closure.v1.draft.json";

pub(crate) const TRN_C_CLOSURE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_c_real_reform_closure.schema.md";

pub(crate) const TRN_C_CLOSURE_READER_PATH: &str = "docs/reading/trn-c-real-reform-closure.md";

pub(crate) const TRN_C_CLOSURE_ROLE_REVIEW_PATH: &str =
    "reviews/2026-07-26-trn-c-real-reform-closure-role-review.md";

pub(crate) const TRN_D_START_GATE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_d_financing_fairness_start_gate.v1.draft.json";

pub(crate) const TRN_D_START_GATE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_d_financing_fairness_start_gate.schema.md";

pub(crate) const TRN_D_START_GATE_READER_PATH: &str = "docs/reading/trn-d-financing-fairness-start-gate.md";

pub(crate) const TRN_D_LEGAL_PERIMETER_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_d_legal_financing_perimeter_decision.v1.draft.json";

pub(crate) const TRN_D_LEGAL_PERIMETER_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_d_legal_financing_perimeter_decision.schema.md";

pub(crate) const TRN_D_LEGAL_PERIMETER_READER_PATH: &str =
    "docs/reading/trn-d-legal-financing-perimeter-decision.md";

pub(crate) const TRN_D_ADMIN_BEHAVIOR_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/trn_d_administration_compliance_behavior_boundary.v1.draft.json";

pub(crate) const TRN_D_ADMIN_BEHAVIOR_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/trn_d_administration_compliance_behavior_boundary.schema.md";

pub(crate) const TRN_D_ADMIN_BEHAVIOR_READER_PATH: &str =
    "docs/reading/trn-d-administration-compliance-behavior-boundary.md";

pub(crate) const TRN_D_INCIDENCE_FAIRNESS_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/trn_d_incidence_distribution_fairness_boundary.v1.draft.json";

pub(crate) const TRN_D_INCIDENCE_FAIRNESS_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/trn_d_incidence_distribution_fairness_boundary.schema.md";

pub(crate) const TRN_D_INCIDENCE_FAIRNESS_READER_PATH: &str =
    "docs/reading/trn-d-incidence-distribution-fairness-boundary.md";

pub(crate) const HLT_A_START_GATE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_a_health_baseline_start_gate.v1.draft.json";

pub(crate) const HLT_A_START_GATE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_a_health_baseline_start_gate.schema.md";

pub(crate) const HLT_A_START_GATE_READER_PATH: &str = "docs/reading/hlt-a-health-baseline-start-gate.md";

pub(crate) const HLT_A_PERIMETER_INVENTORY_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/hlt_a_health_lane_perimeter_source_inventory.v1.draft.json";

pub(crate) const HLT_A_PERIMETER_INVENTORY_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_a_health_lane_perimeter_source_inventory.schema.md";

pub(crate) const HLT_A_PERIMETER_INVENTORY_READER_PATH: &str =
    "docs/reading/hlt-a-health-lane-perimeter-source-inventory.md";

pub(crate) const HLT_A_BASELINE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_a_federal_health_baseline_path.v1.draft.json";

pub(crate) const HLT_A_BASELINE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_a_federal_health_baseline_path.schema.md";

pub(crate) const HLT_A_BASELINE_READER_PATH: &str = "docs/reading/hlt-a-federal-health-baseline-path.md";

pub(crate) const HLT_A_FINANCING_LINEAGE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_a_health_financing_lineage.v1.draft.json";

pub(crate) const HLT_A_FINANCING_LINEAGE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_a_health_financing_lineage.schema.md";

pub(crate) const HLT_A_FINANCING_LINEAGE_READER_PATH: &str = "docs/reading/hlt-a-health-financing-lineage.md";

pub(crate) const EDU_A_START_GATE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/edu_a_education_baseline_start_gate.v1.draft.json";

pub(crate) const EDU_A_START_GATE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/edu_a_education_baseline_start_gate.schema.md";

pub(crate) const EDU_A_START_GATE_READER_PATH: &str = "docs/reading/edu-a-education-baseline-start-gate.md";

pub(crate) const EDU_A_PERIMETER_INVENTORY_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/edu_a_education_workforce_perimeter_source_inventory.v1.draft.json";

pub(crate) const EDU_A_PERIMETER_INVENTORY_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/edu_a_education_workforce_perimeter_source_inventory.schema.md";

pub(crate) const EDU_A_PERIMETER_INVENTORY_READER_PATH: &str =
    "docs/reading/edu-a-education-workforce-perimeter-source-inventory.md";

pub(crate) const EDU_A_BASELINE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/edu_a_federal_education_workforce_baseline_path.v1.draft.json";

pub(crate) const EDU_A_BASELINE_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/edu_a_federal_education_workforce_baseline_path.schema.md";

pub(crate) const EDU_A_BASELINE_READER_PATH: &str =
    "docs/reading/edu-a-federal-education-workforce-baseline-path.md";

pub(crate) const EDU_A_FINANCING_LINEAGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/edu_a_education_workforce_financing_lineage.v1.draft.json";

pub(crate) const EDU_A_FINANCING_LINEAGE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/edu_a_education_workforce_financing_lineage.schema.md";

pub(crate) const EDU_A_FINANCING_LINEAGE_READER_PATH: &str =
    "docs/reading/edu-a-education-workforce-financing-lineage.md";

pub(crate) const TRN_D_INTERACTIONS_BRIDGE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_d_interactions_macro_fund_bridge.v1.draft.json";

pub(crate) const TRN_D_INTERACTIONS_BRIDGE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_d_interactions_macro_fund_bridge.schema.md";

pub(crate) const TRN_D_INTERACTIONS_BRIDGE_READER_PATH: &str =
    "docs/reading/trn-d-interactions-macro-fund-bridge.md";

pub(crate) const TRN_D_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_d_financing_fairness_closure.v1.draft.json";

pub(crate) const TRN_D_CLOSURE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_d_financing_fairness_closure.schema.md";

pub(crate) const TRN_D_CLOSURE_READER_PATH: &str = "docs/reading/trn-d-financing-fairness-closure.md";

pub(crate) const TRN_D_CLOSURE_REVIEW_PATH: &str =
    "reviews/2026-07-26-trn-d-financing-fairness-closure-role-review.md";

pub(crate) const TRN_E_START_GATE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/trn_e_integrated_candidate_solver_start_gate.v1.draft.json";

pub(crate) const TRN_E_START_GATE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_e_integrated_candidate_solver_start_gate.schema.md";

pub(crate) const TRN_E_START_GATE_READER_PATH: &str =
    "docs/reading/trn-e-integrated-candidate-solver-start-gate.md";

pub(crate) const HLT_A_SERVICE_FLOOR_SPINE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_a_health_service_floor_source_spine.v1.draft.json";

pub(crate) const HLT_A_SERVICE_FLOOR_SPINE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_a_health_service_floor_source_spine.schema.md";

pub(crate) const HLT_A_SERVICE_FLOOR_SPINE_READER_PATH: &str =
    "docs/reading/hlt-a-health-service-floor-source-spine.md";

pub(crate) const HLT_A_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_a_health_baseline_closure.v1.draft.json";

pub(crate) const HLT_A_CLOSURE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_a_health_baseline_closure.schema.md";

pub(crate) const HLT_A_CLOSURE_READER_PATH: &str = "docs/reading/hlt-a-health-baseline-closure.md";

pub(crate) const HLT_A_CLOSURE_REVIEW_PATH: &str =
    "reviews/2026-07-26-hlt-a-health-baseline-closure-role-review.md";

pub(crate) const HLT_B_START_GATE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_b_health_accounting_start_gate.v1.draft.json";

pub(crate) const HLT_B_START_GATE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_b_health_accounting_start_gate.schema.md";

pub(crate) const HLT_B_START_GATE_READER_PATH: &str = "docs/reading/hlt-b-health-accounting-start-gate.md";

pub(crate) const EDU_A_SERVICE_FLOOR_SPINE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/edu_a_education_workforce_service_floor_source_spine.v1.draft.json";

pub(crate) const EDU_A_SERVICE_FLOOR_SPINE_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/edu_a_education_workforce_service_floor_source_spine.schema.md";

pub(crate) const EDU_A_SERVICE_FLOOR_SPINE_READER_PATH: &str =
    "docs/reading/edu-a-education-workforce-service-floor-source-spine.md";

pub(crate) const EDU_A_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/edu_a_education_workforce_baseline_closure.v1.draft.json";

pub(crate) const EDU_A_CLOSURE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/edu_a_education_workforce_baseline_closure.schema.md";

pub(crate) const EDU_A_CLOSURE_READER_PATH: &str =
    "docs/reading/edu-a-education-workforce-baseline-closure.md";

pub(crate) const EDU_A_CLOSURE_REVIEW_PATH: &str =
    "reviews/2026-07-26-edu-a-education-workforce-baseline-closure-role-review.md";

pub(crate) const EDU_B_START_GATE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/edu_b_education_workforce_accounting_start_gate.v1.draft.json";

pub(crate) const EDU_B_START_GATE_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/edu_b_education_workforce_accounting_start_gate.schema.md";

pub(crate) const EDU_B_START_GATE_READER_PATH: &str =
    "docs/reading/edu-b-education-workforce-accounting-start-gate.md";

pub(crate) const TRN_E_INPUT_READINESS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_e_solver_input_readiness_bridge.v1.draft.json";

pub(crate) const TRN_E_INPUT_READINESS_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_e_solver_input_readiness_bridge.schema.md";

pub(crate) const TRN_E_INPUT_READINESS_READER_PATH: &str =
    "docs/reading/trn-e-solver-input-readiness-bridge.md";

pub(crate) const HLT_B_COMPONENT_MAPPING_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_b_health_component_account_mapping.v1.draft.json";

pub(crate) const HLT_B_COMPONENT_MAPPING_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_b_health_component_account_mapping.schema.md";

pub(crate) const HLT_B_COMPONENT_MAPPING_READER_PATH: &str =
    "docs/reading/hlt-b-health-component-account-mapping.md";

pub(crate) const EDU_B_COMPONENT_MAPPING_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/edu_b_education_workforce_component_account_mapping.v1.draft.json";

pub(crate) const EDU_B_COMPONENT_MAPPING_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/edu_b_education_workforce_component_account_mapping.schema.md";

pub(crate) const EDU_B_COMPONENT_MAPPING_READER_PATH: &str =
    "docs/reading/edu-b-education-workforce-component-account-mapping.md";

pub(crate) const CORE_J_CONTRACT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/core_j_bounded_closure_handoff_contract.v1.draft.json";

pub(crate) const CORE_J_CONTRACT_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/core_j_bounded_closure_handoff_contract.schema.md";

pub(crate) const CORE_J_CONTRACT_READER_PATH: &str = "docs/reading/core-j-bounded-closure-handoff-contract.md";

pub(crate) const CORE_J_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/core_j_bounded_closure_handoff_closure.v1.draft.json";

pub(crate) const CORE_J_CLOSURE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/core_j_bounded_closure_handoff_closure.schema.md";

pub(crate) const CORE_J_CLOSURE_READER_PATH: &str = "docs/reading/core-j-bounded-closure-handoff-closure.md";

pub(crate) const CORE_J_CLOSURE_REVIEW_PATH: &str =
    "reviews/2026-07-26-core-j-bounded-closure-handoff-role-review.md";

pub(crate) const HLT_B_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_b_health_accounting_closure.v1.draft.json";

pub(crate) const EDU_B_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/edu_b_education_workforce_accounting_closure.v1.draft.json";

pub(crate) const OAS_A_SPINE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/oas_a_social_security_baseline_source_spine.v1.draft.json";

pub(crate) const CORE_K_CONTRACT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/core_k_temporal_composite_accounting_contract.v1.draft.json";

pub(crate) const CORE_K_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/core_k_temporal_composite_accounting_closure.v1.draft.json";

pub(crate) const OAS_B_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/oas_b_social_security_accounting_closure.v1.draft.json";

pub(crate) const HLT_C_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_c_health_scenario_admission_closure.v1.draft.json";

pub(crate) const EDU_C_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/edu_c_education_workforce_scenario_admission_closure.v1.draft.json";

pub(crate) const OAS_C_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/oas_c_social_security_scenario_admission_closure.v1.draft.json";

pub(crate) const STAGE_C_CATCHUP_BUNDLE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/four_lane_stage_c_catchup_bundle.v1.draft.json";

pub(crate) const STAGE_C_CATCHUP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/stage_c_catchup_bundle.schema.md";

pub(crate) const STAGE_C_CATCHUP_READER_PATH: &str = "docs/reading/stage-c-catchup-bundle.md";

pub(crate) const STAGE_C_CATCHUP_REVIEW_PATH: &str =
    "reviews/2026-07-26-stage-c-catchup-bundle-role-review.md";

pub(crate) const HLT_D_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_d_health_financing_fairness_closure.v1.draft.json";

pub(crate) const EDU_D_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/edu_d_education_workforce_financing_fairness_closure.v1.draft.json";

pub(crate) const OAS_D_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/oas_d_social_security_financing_fairness_closure.v1.draft.json";

pub(crate) const TRN_E_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_e_integrated_candidate_solver_closure.v1.draft.json";

pub(crate) const ISF_A_SPINE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/isf_a_income_security_family_baseline_source_spine.v1.draft.json";

pub(crate) const VET_A_SPINE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/vet_a_veterans_baseline_source_spine.v1.draft.json";

pub(crate) const AGR_A_SPINE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/agr_a_agriculture_baseline_source_spine.v1.draft.json";

pub(crate) const CORE_L_CONTRACT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/core_l_cross_lane_overlap_allocation_contract.v1.draft.json";

pub(crate) const CORE_L_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/core_l_cross_lane_overlap_allocation_closure.v1.draft.json";

pub(crate) const ISF_B_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/isf_b_income_security_family_accounting_closure.v1.draft.json";

pub(crate) const VET_B_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/vet_b_veterans_accounting_closure.v1.draft.json";

pub(crate) const AGR_B_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/agr_b_agriculture_accounting_closure.v1.draft.json";

pub(crate) const SEVEN_LANE_CATCHUP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/seven_lane_d_b_catchup_bundle.v1.draft.json";

pub(crate) const MULTI_TRACK_D_B_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/multi_track_d_b_catchup_bundle.schema.md";

pub(crate) const MULTI_TRACK_D_B_READER_PATH: &str = "docs/reading/multi-track-d-b-catchup-bundle.md";

pub(crate) const MULTI_TRACK_D_B_REVIEW_PATH: &str =
    "reviews/2026-07-26-multi-track-d-b-catchup-role-review.md";

pub(crate) const ISF_C_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/isf_c_income_security_family_scenario_admission_closure.v1.draft.json";

pub(crate) const VET_C_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/vet_c_veterans_scenario_admission_closure.v1.draft.json";

pub(crate) const AGR_C_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/agr_c_agriculture_scenario_admission_closure.v1.draft.json";

pub(crate) const THREE_LANE_STAGE_C_BUNDLE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/three_lane_stage_c_discovery_bundle.v1.draft.json";

pub(crate) const THREE_LANE_STAGE_C_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/three_lane_stage_c_discovery_bundle.schema.md";

pub(crate) const THREE_LANE_STAGE_C_READER_PATH: &str = "docs/reading/three-lane-stage-c-discovery-bundle.md";

pub(crate) const THREE_LANE_STAGE_C_REVIEW_PATH: &str =
    "reviews/2026-07-26-three-lane-stage-c-discovery-role-review.md";

pub(crate) const FIFTEEN_LANE_STAGE_MATRIX_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fifteen_lane_track_stage_matrix.v1.draft.json";

pub(crate) const FIFTEEN_LANE_STAGE_MATRIX_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fifteen_lane_track_stage_matrix.schema.md";

pub(crate) const FIFTEEN_LANE_STAGE_MATRIX_READER_PATH: &str =
    "docs/reading/fifteen-lane-track-stage-matrix.md";

pub(crate) const ISF_D_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/isf_d_income_security_family_financing_fairness_closure.v1.draft.json";

pub(crate) const VET_D_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/vet_d_veterans_financing_fairness_closure.v1.draft.json";

pub(crate) const AGR_D_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/agr_d_agriculture_financing_fairness_closure.v1.draft.json";

pub(crate) const THREE_LANE_STAGE_D_BUNDLE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/three_lane_stage_d_bounded_bundle.v1.draft.json";

pub(crate) const THREE_LANE_STAGE_D_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/three_lane_stage_d_bounded_bundle.schema.md";

pub(crate) const THREE_LANE_STAGE_D_READER_PATH: &str = "docs/reading/three-lane-stage-d-bounded-bundle.md";

pub(crate) const THREE_LANE_STAGE_D_REVIEW_PATH: &str =
    "reviews/2026-07-27-three-lane-stage-d-bounded-role-review.md";

pub(crate) const DEF_D_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/def_d_national_defense_bounded_stage_chain_closure.v1.draft.json";

pub(crate) const DIS_D_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/dis_d_disaster_resilience_bounded_stage_chain_closure.v1.draft.json";

pub(crate) const JUS_D_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/jus_d_justice_public_safety_bounded_stage_chain_closure.v1.draft.json";

pub(crate) const SEE_D_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/see_d_science_energy_environment_bounded_stage_chain_closure.v1.draft.json";

pub(crate) const INT_D_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/int_d_international_affairs_bounded_stage_chain_closure.v1.draft.json";

pub(crate) const PAY_D_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/pay_d_payment_integrity_bounded_stage_chain_closure.v1.draft.json";

pub(crate) const REV_D_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_d_revenue_solvency_bounded_stage_chain_closure.v1.draft.json";

pub(crate) const NET_D_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/net_d_net_interest_bounded_stage_chain_closure.v1.draft.json";

pub(crate) const EIGHT_LANE_A_D_BUNDLE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/eight_lane_a_d_bounded_bundle.v1.draft.json";

pub(crate) const EIGHT_LANE_A_D_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/eight_lane_a_d_bounded_bundle.schema.md";

pub(crate) const EIGHT_LANE_A_D_READER_PATH: &str = "docs/reading/eight-lane-a-d-bounded-bundle.md";

pub(crate) const EIGHT_LANE_A_D_REVIEW_PATH: &str = "reviews/2026-07-27-eight-lane-a-d-bounded-role-review.md";

pub(crate) const FIFTEEN_LANE_D_PORTFOLIO_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fifteen_lane_stage_d_portfolio_closure.v1.draft.json";

pub(crate) const FIFTEEN_LANE_D_PORTFOLIO_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fifteen_lane_stage_d_portfolio_closure.schema.md";

pub(crate) const FIFTEEN_LANE_D_PORTFOLIO_READER_PATH: &str =
    "docs/reading/fifteen-lane-stage-d-portfolio-closure.md";

pub(crate) const FIFTEEN_LANE_D_PORTFOLIO_REVIEW_PATH: &str =
    "reviews/2026-07-27-fifteen-lane-stage-d-portfolio-role-review.md";

pub(crate) const LANE_E_CONTRACT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/lane_e_bounded_selection_solver_gate_contract.v1.draft.json";

pub(crate) const LANE_E_CONTRACT_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/lane_e_bounded_selection_solver_gate_contract.schema.md";

pub(crate) const LANE_E_CONTRACT_READER_PATH: &str =
    "docs/reading/lane-e-bounded-selection-solver-gate-contract.md";

pub(crate) const HLT_E_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/hlt_e_health_bounded_selection_solver_closure.v1.draft.json";

pub(crate) const EDU_E_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/edu_e_education_workforce_bounded_selection_solver_closure.v1.draft.json";

pub(crate) const OAS_E_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/oas_e_social_security_bounded_selection_solver_closure.v1.draft.json";

pub(crate) const ISF_E_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/isf_e_income_security_family_bounded_selection_solver_closure.v1.draft.json";

pub(crate) const VET_E_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/vet_e_veterans_bounded_selection_solver_closure.v1.draft.json";

pub(crate) const AGR_E_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/agr_e_agriculture_bounded_selection_solver_closure.v1.draft.json";

pub(crate) const DEF_E_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/def_e_national_defense_bounded_selection_solver_closure.v1.draft.json";

pub(crate) const DIS_E_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/dis_e_disaster_resilience_bounded_selection_solver_closure.v1.draft.json";

pub(crate) const JUS_E_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/jus_e_justice_public_safety_bounded_selection_solver_closure.v1.draft.json";

pub(crate) const SEE_E_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/see_e_science_energy_environment_bounded_selection_solver_closure.v1.draft.json";

pub(crate) const INT_E_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/int_e_international_affairs_bounded_selection_solver_closure.v1.draft.json";

pub(crate) const PAY_E_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/pay_e_payment_integrity_bounded_selection_solver_closure.v1.draft.json";

pub(crate) const REV_E_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_e_revenue_solvency_bounded_selection_solver_closure.v1.draft.json";

pub(crate) const NET_E_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/net_e_net_interest_bounded_selection_solver_closure.v1.draft.json";

pub(crate) const FOURTEEN_LANE_E_BUNDLE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fourteen_lane_stage_e_bounded_bundle.v1.draft.json";

pub(crate) const FOURTEEN_LANE_E_BUNDLE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fourteen_lane_stage_e_bounded_bundle.schema.md";

pub(crate) const FOURTEEN_LANE_E_BUNDLE_READER_PATH: &str =
    "docs/reading/fourteen-lane-stage-e-bounded-bundle.md";

pub(crate) const FOURTEEN_LANE_E_REVIEW_PATH: &str =
    "reviews/2026-07-27-fourteen-lane-stage-e-bounded-role-review.md";

pub(crate) const FIFTEEN_LANE_E_PORTFOLIO_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fifteen_lane_stage_e_portfolio_closure.v1.draft.json";

pub(crate) const FIFTEEN_LANE_E_PORTFOLIO_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fifteen_lane_stage_e_portfolio_closure.schema.md";

pub(crate) const FIFTEEN_LANE_E_PORTFOLIO_READER_PATH: &str =
    "docs/reading/fifteen-lane-stage-e-portfolio-closure.md";

pub(crate) const FIFTEEN_LANE_E_PORTFOLIO_REVIEW_PATH: &str =
    "reviews/2026-07-27-fifteen-lane-stage-e-portfolio-role-review.md";

pub(crate) const LANE_F_CONTRACT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/lane_f_public_release_gate_contract.v1.draft.json";

pub(crate) const LANE_F_CONTRACT_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/lane_f_public_release_gate_contract.schema.md";

pub(crate) const LANE_F_CONTRACT_READER_PATH: &str = "docs/reading/lane-f-public-release-gate-contract.md";

pub(crate) const FIFTEEN_LANE_F_READINESS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fifteen_lane_stage_f_start_readiness.v1.draft.json";

pub(crate) const FIFTEEN_LANE_F_READINESS_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fifteen_lane_stage_f_start_readiness.schema.md";

pub(crate) const FIFTEEN_LANE_F_READINESS_READER_PATH: &str =
    "docs/reading/fifteen-lane-stage-f-start-readiness.md";

pub(crate) const FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/fifteen_lane_two_level_f_advancement_queue.v1.draft.json";

pub(crate) const FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fifteen_lane_two_level_f_advancement_queue.schema.md";

pub(crate) const FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_READER_PATH: &str =
    "docs/reading/fifteen-lane-two-level-f-advancement-queue.md";

pub(crate) const FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_REVIEW_PATH: &str =
    "reviews/2026-07-27-fifteen-lane-two-level-f-advancement-role-review.md";

pub(crate) const TRN_LEVEL_1_CORE_LESSONS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_level_1_core_lessons_audit.v1.draft.json";

pub(crate) const TRN_LEVEL_1_CORE_LESSONS_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_level_1_core_lessons_audit.schema.md";

pub(crate) const TRN_LEVEL_1_CORE_LESSONS_READER_PATH: &str = "docs/reading/trn-level-1-core-lessons-audit.md";

pub(crate) const TRN_LEVEL_1_CORE_LESSONS_REVIEW_PATH: &str =
    "reviews/2026-07-27-trn-level-1-core-lessons-role-review.md";

pub(crate) const CORE_M_CONTRACT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/core_m_candidate_dossier_typed_release_contract.v1.draft.json";

pub(crate) const CORE_M_CONTRACT_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/core_m_candidate_dossier_typed_release_contract.schema.md";

pub(crate) const CORE_M_CONTRACT_READER_PATH: &str =
    "docs/reading/core-m-candidate-dossier-typed-release-contract.md";

pub(crate) const CORE_M_CONTRACT_REVIEW_PATH: &str =
    "reviews/2026-07-27-core-m-candidate-dossier-typed-release-role-review.md";

pub(crate) const CORE_M_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/core_m_candidate_dossier_typed_release_closure.v1.draft.json";

pub(crate) const TRN_LEVEL_1_DOSSIER_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_level_1_hr2247_candidate_dossier.v1.draft.json";

pub(crate) const TRN_LEVEL_1_DOSSIER_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_level_1_hr2247_candidate_dossier.schema.md";

pub(crate) const TRN_LEVEL_1_DOSSIER_READER_PATH: &str =
    "docs/reading/trn-level-1-hr2247-candidate-dossier.md";

pub(crate) const TRN_LEVEL_1_DOSSIER_REVIEW_PATH: &str =
    "reviews/2026-07-27-trn-level-1-hr2247-candidate-dossier-role-review.md";

pub(crate) const TRN_LEVEL_2_E_RERUN_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_level_2_hr2247_output_ready_e_rerun.v1.draft.json";

pub(crate) const TRN_LEVEL_2_E_RERUN_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_level_2_hr2247_output_ready_e_rerun.schema.md";

pub(crate) const TRN_LEVEL_2_E_RERUN_READER_PATH: &str =
    "docs/reading/trn-level-2-hr2247-output-ready-e-rerun.md";

pub(crate) const TRN_LEVEL_2_E_RERUN_REVIEW_PATH: &str =
    "reviews/2026-07-27-trn-level-2-hr2247-output-ready-e-rerun-role-review.md";

pub(crate) const CORE_N_CONTRACT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/core_n_typed_public_release_surface_contract.v1.draft.json";

pub(crate) const CORE_N_CONTRACT_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/core_n_typed_public_release_surface_contract.schema.md";

pub(crate) const CORE_N_CONTRACT_READER_PATH: &str =
    "docs/reading/core-n-typed-public-release-surface-contract.md";

pub(crate) const CORE_N_CONTRACT_REVIEW_PATH: &str =
    "reviews/2026-07-27-core-n-typed-public-release-surface-role-review.md";

pub(crate) const CORE_N_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/core_n_typed_public_release_surface_closure.v1.draft.json";

pub(crate) const TRN_F_COST_NOTE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_f_hr2247_cost_note.v1.draft.json";

pub(crate) const TRN_F_COST_NOTE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/trn_f_hr2247_cost_note.schema.md";

pub(crate) const TRN_F_COST_NOTE_READER_PATH: &str = "docs/reading/trn-f-hr2247-cost-note.md";

pub(crate) const TRN_F_COST_NOTE_REVIEW_PATH: &str =
    "reviews/2026-07-27-trn-f-hr2247-cost-note-role-review.md";

pub(crate) const REV_LEVEL_1_START_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_level_1_individual_income_rate_candidate_start.v1.draft.json";

pub(crate) const REV_LEVEL_1_START_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_level_1_individual_income_rate_candidate_start.schema.md";

pub(crate) const REV_LEVEL_1_START_READER_PATH: &str =
    "docs/reading/rev-level-1-individual-income-rate-candidate-start.md";

pub(crate) const REV_LEVEL_1_START_REVIEW_PATH: &str =
    "reviews/2026-07-27-rev-level-1-individual-income-rate-candidate-start-role-review.md";

pub(crate) const REV_LEVEL_1_BASE_PERIMETER_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_level_1_individual_income_legal_economic_base_perimeter.v1.draft.json";

pub(crate) const REV_LEVEL_1_BASE_PERIMETER_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_level_1_individual_income_legal_economic_base_perimeter.schema.md";

pub(crate) const REV_LEVEL_1_BASE_PERIMETER_READER_PATH: &str =
    "docs/reading/rev-level-1-individual-income-legal-economic-base-perimeter.md";

pub(crate) const REV_LEVEL_1_BASE_PERIMETER_REVIEW_PATH: &str =
    "reviews/2026-07-27-rev-level-1-individual-income-legal-economic-base-perimeter-role-review.md";

pub(crate) const REV_LEVEL_1_TIMING_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_level_1_individual_income_tax_fiscal_timing_bridge.v1.draft.json";

pub(crate) const REV_LEVEL_1_TIMING_BRIDGE_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_level_1_individual_income_tax_fiscal_timing_bridge.schema.md";

pub(crate) const REV_LEVEL_1_TIMING_BRIDGE_READER_PATH: &str =
    "docs/reading/rev-level-1-individual-income-tax-fiscal-timing-bridge.md";

pub(crate) const REV_LEVEL_1_TIMING_BRIDGE_REVIEW_PATH: &str =
    "reviews/2026-07-27-rev-level-1-individual-income-tax-fiscal-timing-bridge-role-review.md";

pub(crate) const REV_LEVEL_1_RATE_LADDER_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_level_1_individual_income_rate_planning_ladder.v1.draft.json";

pub(crate) const REV_LEVEL_1_RATE_LADDER_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_level_1_individual_income_rate_planning_ladder.schema.md";

pub(crate) const REV_LEVEL_1_RATE_LADDER_READER_PATH: &str =
    "docs/reading/rev-level-1-individual-income-rate-planning-ladder.md";

pub(crate) const REV_LEVEL_1_RATE_LADDER_REVIEW_PATH: &str =
    "reviews/2026-07-27-rev-level-1-individual-income-rate-planning-ladder-role-review.md";

pub(crate) const REV_LEVEL_1_POST_2025_PROXY_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_level_1_post_2025_rate_rescore_proxy.v1.draft.json";

pub(crate) const REV_LEVEL_1_POST_2025_PROXY_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_level_1_post_2025_rate_rescore_proxy.schema.md";

pub(crate) const REV_LEVEL_1_POST_2025_PROXY_READER_PATH: &str =
    "docs/reading/rev-level-1-post-2025-rate-rescore-proxy.md";

pub(crate) const REV_LEVEL_1_POST_2025_PROXY_REVIEW_PATH: &str =
    "reviews/2026-07-27-rev-level-1-post-2025-rate-rescore-proxy-role-review.md";

pub(crate) const REV_LEVEL_1_GUARDED_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_level_1_guarded_proxy_closure.v1.draft.json";

pub(crate) const FISCALLY_DECISIVE_LEVEL_1_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/fiscally_decisive_track_level_1_candidate_envelopes.v1.draft.json";

pub(crate) const FISCALLY_DECISIVE_LEVEL_2_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/fiscally_decisive_track_level_2_dependency_rerun.v1.draft.json";

pub(crate) const HLT_LEVEL_3_MA_PROXY_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/hlt_level_3_ma_benchmark_current_law_scale_proxy.v1.draft.json";

pub(crate) const HLT_LEVEL_4_FLOOR_AUDIT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/hlt_level_4_ma_benchmark_beneficiary_floor_audit.v1.draft.json";

pub(crate) const FISCAL_PACKAGE_HLT_BRIDGE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fiscal_package_hlt_contribution_bridge.v1.draft.json";

pub(crate) const DEF_LEVEL_3_SCALE_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/def_level_3_defense_current_law_scale_bridge.v1.draft.json";

pub(crate) const DEF_LEVEL_4_ALLOCATION_AUDIT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/def_level_4_force_readiness_allocation_audit.v1.draft.json";

pub(crate) const FISCAL_PACKAGE_DEF_BRIDGE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fiscal_package_def_contribution_bridge.v1.draft.json";

pub(crate) const PAY_LEVEL_3_CONTROL_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/pay_level_3_full_dmf_causal_control_spine.v1.draft.json";

pub(crate) const PAY_LEVEL_4_AUDIT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/pay_level_4_full_dmf_accounting_floor_audit.v1.draft.json";

pub(crate) const FIFTEEN_LANE_CANDIDATE_FRONTIER_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fifteen_lane_candidate_execution_frontier.v1.draft.json";

pub(crate) const PAY_FULL_DMF_EVIDENCE_CEILING_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/pay_full_dmf_public_evidence_ceiling.v1.draft.json";

pub(crate) const FISCAL_PACKAGE_PAY_BRIDGE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fiscal_package_pay_contribution_bridge.v1.draft.json";

pub(crate) const OAS_LEVEL_3_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/oas_level_3_taxable_max_2026_trustees_bridge.v1.draft.json";

pub(crate) const OAS_LEVEL_4_AUDIT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/oas_level_4_taxable_max_cohort_incidence_audit.v1.draft.json";

pub(crate) const FISCAL_PACKAGE_OAS_BRIDGE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fiscal_package_oas_contribution_bridge.v1.draft.json";

pub(crate) const NET_LEVEL_3_RECOMPUTATION_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/net_level_3_zero_input_endogenous_recomputation.v1.draft.json";

pub(crate) const NET_LEVEL_4_AUDIT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_level_4_endogenous_dependency_audit.v1.draft.json";

pub(crate) const FISCAL_PACKAGE_NET_BRIDGE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fiscal_package_net_contribution_bridge.v1.draft.json";

pub(crate) const REV_LEVEL_2_RECONCILIATION_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_level_2_zero_admitted_spending_reconciliation.v1.draft.json";

pub(crate) const REV_LEVEL_2_AUDIT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_level_2_formal_rate_gate_audit.v1.draft.json";

pub(crate) const FISCAL_PACKAGE_RATE_READINESS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fiscal_package_rate_readiness_bridge.v1.draft.json";

pub(crate) const REV_LEVEL_3_MICROSIMULATION_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_level_3_taxcalc_microsimulation_score.v1.draft.json";

pub(crate) const REV_LEVEL_3_AUDIT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_level_3_rate_admission_audit.v1.draft.json";

pub(crate) const FISCAL_PACKAGE_PROVISIONAL_RATE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fiscal_package_provisional_rate_bridge.v1.draft.json";

pub(crate) const REV_LEVEL_4_TIMING_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_level_4_first_year_cash_timing_bridge.v1.draft.json";

pub(crate) const REV_LEVEL_4_AUDIT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_level_4_assignment_cost_macro_audit.v1.draft.json";

pub(crate) const FISCAL_PACKAGE_FISCAL_TIMING_RATE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fiscal_package_fiscal_timing_rate_bridge.v1.draft.json";

pub(crate) const REV_LEVEL_5_ADMINISTRATION_CEILING_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_level_5_administration_implementation_ceiling.v1.draft.json";

pub(crate) const REV_LEVEL_5_MACRO_AUDIT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_level_5_macro_assignment_methodology_audit.v1.draft.json";

pub(crate) const FISCAL_PACKAGE_ADMINISTRATION_BOUNDED_RATE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/fiscal_package_administration_bounded_rate_bridge.v1.draft.json";

pub(crate) const REV_LEVEL_6_POLICY_DECISION_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_level_6_policy_rate_decision.v1.draft.json";

pub(crate) const REV_LEVEL_6_DOSSIER_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_level_6_revenue_instrument_dossier.v1.draft.json";

pub(crate) const REV_F_PLANNING_RATE_CARD_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_f_public_planning_rate_card.v1.draft.json";

pub(crate) const HLT_LEVEL_5_SITE_NEUTRAL_AUDIT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/hlt_level_5_site_neutral_imaging_candidate_audit.v1.draft.json";

pub(crate) const DEF_LEVEL_5_STRATEGY_AUDIT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/def_level_5_specific_force_strategy_candidate_audit.v1.draft.json";

pub(crate) const RATE_DOWN_BUNDLE_RERUN_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rate_down_bundle_dependency_rerun.v1.draft.json";

pub(crate) const EIGHT_TRACK_TWO_LEVEL_CATCHUP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/eight_track_two_level_candidate_catchup.v1.draft.json";

pub(crate) const FIFTEEN_TRACK_INTEGRATED_RERUN_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/fifteen_track_integrated_dependency_admission_rerun.v1.draft.json";

pub(crate) const REV_LEVEL_7_CERTIFICATION_HANDOFF_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_level_7_integrated_certification_and_score_handoff.v1.draft.json";

pub(crate) const REV_LEVEL_7_POLICY_SPEC_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_level_7_scorer_ready_legislative_specification.v1.draft.json";

pub(crate) const REV_LEVEL_7_SCORE_WORKBOOK_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_level_7_official_score_request_workbook.v1.draft.json";

pub(crate) const REV_LEVEL_7_DISCUSSION_DRAFT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_level_7_nonofficial_conforming_discussion_draft.v1.draft.json";

pub(crate) const REV_LEVEL_7_EXTERNAL_SUBMISSION_CONTROL_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_level_7_external_submission_control.v1.draft.json";

pub(crate) const REV_LEVEL_7_EXTERNAL_RESPONSE_INTAKE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_level_7_external_response_intake.v1.draft.json";

pub(crate) const REV_LEVEL_7_SUBMISSION_BUILDER_PATH: &str = "tools/build-rev-level-7-submission.ps1";

pub(crate) const REV_INTERNAL_NEXT_TEN_STEPS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_internal_analysis_next_ten_steps.v1.draft.json";

pub(crate) const REV_INTERNAL_BASELINE_FREEZE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_internal_analysis_baseline_freeze.v1.draft.json";

pub(crate) const REV_INTERNAL_GRID_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_internal_rate_sensitivity_grid_run.v1.generated.json";

pub(crate) const REV_INTERNAL_GRID_EXTENSION_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/rev_internal_rate_sensitivity_grid_extension.v1.generated.json";

pub(crate) const REV_INTERNAL_CANDIDATE_ANALYSIS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_internal_rate_candidate_analysis.v1.generated.json";

pub(crate) const REV_INTERNAL_COMPLETION_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/rev_internal_rate_analysis_completion.v1.draft.json";

pub(crate) const TARGETED_SPENDING_RATE_DECISION_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/targeted_spending_rate_decision.v1.draft.json";

pub(crate) const FIFTEEN_TRACK_TERMINAL_DISPOSITION_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fifteen_track_terminal_disposition.v1.draft.json";

pub(crate) const FINAL_RESULT_EXPLANATION_PROGRAM_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/taxlane_final_result_explanation_program.v1.draft.json";

pub(crate) const EXPL_A_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/expl_a_narrative_evidence_foundation_closure.v1.draft.json";

pub(crate) const EXPL_B_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/expl_b_citizen_teaching_guides_closure.v1.draft.json";

pub(crate) const EXPL_C_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/expl_c_research_paper_series_closure.v1.draft.json";

pub(crate) const EXPL_D_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/expl_d_presentation_system_closure.v1.draft.json";

pub(crate) const EXPL_E_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/expl_e_local_html_experience_closure.v1.draft.json";

pub(crate) const EXPL_F_CLOSURE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/expl_f_integrated_repository_readiness_closure.v1.draft.json";

pub(crate) const FIFTEEN_TRACK_NEXT_TWO_LEVEL_WAVE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/fifteen_track_next_two_level_advancement_wave.v1.draft.json";

pub(crate) const HLT_NEXT_LEVEL_A_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/hlt_next_level_a_site_neutral_evidence_closure.v1.draft.json";

pub(crate) const HLT_NEXT_LEVEL_B_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hlt_next_level_b_admission_rerun.v1.draft.json";

pub(crate) const BATCH_1_REMAINING_FOUR_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/batch_1_remaining_four_two_level_closure.v1.draft.json";

pub(crate) const BATCH_2_EIGHT_TRACK_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/batch_2_eight_track_two_level_closure.v1.draft.json";

pub(crate) const BATCH_3_TRN_REV_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/batch_3_trn_rev_two_level_closure.v1.draft.json";

pub(crate) const PAY_NET_REV_POST_FIFTEEN_RECONCILIATION_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/pay_net_rev_post_fifteen_track_reconciliation.v1.draft.json";

pub(crate) const GOVINFO_HR2137_REPORTED_BILL_PDF_PATH: &str =
    "data/raw/gpo/SRC-GOVINFO-HR2137-RH-2026/2026-07-26/BILLS-119hr2137rh.pdf";

pub(crate) const GOVINFO_HR2137_REPORTED_BILL_HTML_PATH: &str =
    "data/raw/gpo/SRC-GOVINFO-HR2137-RH-2026/2026-07-26/BILLS-119hr2137rh.htm";

pub(crate) const GOVINFO_HR2137_METADATA_PATH: &str =
    "data/metadata/SRC-GOVINFO-HR2137-RH-2026.2026-07-26.metadata.md";

pub(crate) const MULTI_TRACK_FRONTIER_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/adaptive_rate_multi_track_frontier.v1.draft.json";

pub(crate) const MULTI_TRACK_FRONTIER_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/adaptive_rate_multi_track_frontier.schema.md";

pub(crate) const MULTI_TRACK_FRONTIER_READER_PATH: &str = "docs/reading/adaptive-rate-multi-track-frontier.md";

pub(crate) const HR2247_BILL_SOURCE_PATH: &str =
    "data/raw/gpo/SRC-GPO-HR2247-RFS-2026/2026-07-26/hr2247-rfs-excerpt.txt";

pub(crate) const HR2247_SCORE_SOURCE_PATH: &str =
    "data/raw/gpo/SRC-GPO-HRPT119-551-2026/2026-07-26/hrpt119-551-cbo-excerpt.txt";

pub(crate) const SOLVER_RATE_WAVE_F_READINESS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/solver_rate_wave_f_readiness.v1.draft.json";

pub(crate) const SOLVER_RATE_WAVE_F_READINESS_READER_PATH: &str =
    "docs/reading/solver-rate-wave-f-readiness.md";

pub(crate) const HEALTH_FLOOR_SOURCE_CAPTURE_STATUS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_floor_source_capture_status.v1.draft.json";

pub(crate) const HEALTH_FLOOR_SOURCE_CAPTURE_STATUS_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_floor_source_capture_status.schema.md";

pub(crate) const HEALTH_FLOOR_SOURCE_CAPTURE_STATUS_READER_PATH: &str =
    "docs/reading/health-floor-source-capture-status.md";

pub(crate) const HEALTH_MEDICARE_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/health_medicare_trustees_source_capture_status.v1.draft.json";

pub(crate) const HEALTH_MEDICARE_TRUSTEES_SOURCE_CAPTURE_STATUS_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/health_medicare_trustees_source_capture_status.schema.md";

pub(crate) const HEALTH_MEDICARE_TRUSTEES_SOURCE_CAPTURE_STATUS_READER_PATH: &str =
    "docs/reading/health-medicare-trustees-source-capture-status.md";

pub(crate) const MEDICARE_HI_CY2025_2035_CURRENT_LAW_CONTEXT_PATH_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_hi_cy2025_2035_current_law_context_path.v1.draft.json";

pub(crate) const MEDICARE_HI_CY2025_2035_CURRENT_LAW_CONTEXT_PATH_READER_PATH: &str =
    "docs/reading/medicare-hi-cy2025-2035-current-law-context-path.md";

pub(crate) const MEDICARE_HI_CMS_OMB_FY2025_TIMING_PERIMETER_DIAGNOSTIC_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_hi_cms_omb_fy2025_timing_perimeter_diagnostic.v1.draft.json";

pub(crate) const MEDICARE_HI_CMS_OMB_FY2025_TIMING_PERIMETER_DIAGNOSTIC_READER_PATH: &str =
    "docs/reading/medicare-hi-cms-omb-fy2025-timing-perimeter-diagnostic.md";

pub(crate) const MEDICARE_HI_TREASURY_MTS_FY2025_TRUST_FUND_ANCHOR_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_hi_treasury_mts_fy2025_trust_fund_anchor_context.v1.draft.json";

pub(crate) const MEDICARE_HI_TREASURY_MTS_FY2025_TRUST_FUND_ANCHOR_CONTEXT_READER_PATH: &str =
    "docs/reading/medicare-hi-treasury-mts-fy2025-trust-fund-anchor-context.md";

pub(crate) const HEALTH_NHE_SOURCE_CUSTODY_GAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_nhe_source_custody_gap.v1.draft.json";

pub(crate) const HEALTH_NHE_SOURCE_CUSTODY_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_nhe_source_custody_gap.schema.md";

pub(crate) const HEALTH_NHE_SOURCE_CUSTODY_GAP_READER_PATH: &str =
    "docs/reading/health-nhe-source-custody-gap.md";

pub(crate) const HEALTH_CBO_SOURCE_CUSTODY_GAP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_cbo_source_custody_gap.v1.draft.json";

pub(crate) const HEALTH_CBO_SOURCE_CUSTODY_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_cbo_source_custody_gap.schema.md";

pub(crate) const HEALTH_CBO_SOURCE_CUSTODY_GAP_READER_PATH: &str =
    "docs/reading/health-cbo-source-custody-gap.md";

pub(crate) const HEALTH_QUALITY_ACCESS_INDICATOR_SOURCE_GAP_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/health_quality_access_indicator_source_gap.v1.draft.json";

pub(crate) const HEALTH_QUALITY_ACCESS_INDICATOR_SOURCE_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_quality_access_indicator_source_gap.schema.md";

pub(crate) const HEALTH_QUALITY_ACCESS_INDICATOR_SOURCE_GAP_READER_PATH: &str =
    "docs/reading/health-quality-access-indicator-source-gap.md";

pub(crate) const HEALTH_SOURCE_READINESS_ROLLUP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_source_readiness_rollup.v1.draft.json";

pub(crate) const HEALTH_SOURCE_READINESS_ROLLUP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_source_readiness_rollup.schema.md";

pub(crate) const HEALTH_SOURCE_READINESS_ROLLUP_READER_PATH: &str =
    "docs/reading/health-source-readiness-rollup.md";

pub(crate) const MEDICARE_PART_FINANCING_CY2025_CMS_TRUSTEES_JSONL_PATH: &str = "data/derived/contribution_alignment/medicare_part_financing.cy2025.cms-trustees-2026.draft.jsonl";

pub(crate) const MEDICARE_DENOMINATOR_VALUES_CY2025_CMS_TRUSTEES_JSONL_PATH: &str = "data/derived/denominator_requirements/denominator_values.cy2025.cms-medicare-trustees-2026.draft.jsonl";

pub(crate) const SOLVER_INPUT_READINESS_ROLLUP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/solver_input_readiness_rollup.v1.draft.json";

pub(crate) const SOLVER_INPUT_READINESS_ROLLUP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/solver_input_readiness_rollup.schema.md";

pub(crate) const SOLVER_INPUT_READINESS_ROLLUP_READER_PATH: &str =
    "docs/reading/solver-input-readiness-rollup.md";

pub(crate) const CURRENT_LAW_PATH_INVENTORY_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/current_law_path_inventory.v1.draft.json";

pub(crate) const CURRENT_LAW_PATH_INVENTORY_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/current_law_path_inventory.schema.md";

pub(crate) const CURRENT_LAW_PATH_INVENTORY_READER_PATH: &str = "docs/reading/current-law-path-inventory.md";

pub(crate) const CURRENT_LAW_SOURCE_CUSTODY_PREFLIGHT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/current_law_source_custody_preflight.v1.draft.json";

pub(crate) const CURRENT_LAW_SOURCE_CUSTODY_PREFLIGHT_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/current_law_source_custody_preflight.schema.md";

pub(crate) const CURRENT_LAW_SOURCE_CUSTODY_PREFLIGHT_READER_PATH: &str =
    "docs/reading/current-law-source-custody-preflight.md";

pub(crate) const LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/lane_depth_explainability_tracker.v1.draft.json";

pub(crate) const LANE_DEPTH_EXPLAINABILITY_TRACKER_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/lane_depth_explainability_tracker.schema.md";

pub(crate) const LANE_DEPTH_EXPLAINABILITY_TRACKER_READER_PATH: &str =
    "docs/reading/lane-depth-explainability-tracker.md";

pub(crate) const LANE_AGENT_WORK_ORDER_PLAN_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/lane_agent_work_order_plan.v1.draft.json";

pub(crate) const LANE_AGENT_WORK_ORDER_PLAN_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/lane_agent_work_order_plan.schema.md";

pub(crate) const LANE_AGENT_WORK_ORDER_PLAN_READER_PATH: &str = "docs/reading/lane-agent-work-order-plan.md";

pub(crate) const WAVE1_PUBLIC_TOPLINE_LANE_DEPTH_PACKETS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/wave1_public_topline_lane_depth_packets.v1.draft.json";

pub(crate) const WAVE1_PUBLIC_TOPLINE_LANE_DEPTH_PACKETS_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/wave1_public_topline_lane_depth_packets.schema.md";

pub(crate) const WAVE1_PUBLIC_TOPLINE_LANE_DEPTH_PACKETS_READER_PATH: &str =
    "docs/reading/wave1-public-topline-lane-depth-packets.md";

pub(crate) const WAVE2_HUMAN_SERVICES_LANE_DEPTH_PACKETS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/wave2_human_services_lane_depth_packets.v1.draft.json";

pub(crate) const WAVE2_HUMAN_SERVICES_LANE_DEPTH_PACKETS_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/wave2_human_services_lane_depth_packets.schema.md";

pub(crate) const WAVE2_HUMAN_SERVICES_LANE_DEPTH_PACKETS_READER_PATH: &str =
    "docs/reading/wave2-human-services-lane-depth-packets.md";

pub(crate) const WAVE3_PUBLIC_GOODS_LANE_DEPTH_PACKETS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/wave3_public_goods_lane_depth_packets.v1.draft.json";

pub(crate) const WAVE3_PUBLIC_GOODS_LANE_DEPTH_PACKETS_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/wave3_public_goods_lane_depth_packets.schema.md";

pub(crate) const WAVE3_PUBLIC_GOODS_LANE_DEPTH_PACKETS_READER_PATH: &str =
    "docs/reading/wave3-public-goods-lane-depth-packets.md";

pub(crate) const WAVE4_COMPONENT_AND_PILOT_LANE_DEPTH_PACKETS_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/wave4_component_and_pilot_lane_depth_packets.v1.draft.json";

pub(crate) const WAVE4_COMPONENT_AND_PILOT_LANE_DEPTH_PACKETS_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/wave4_component_and_pilot_lane_depth_packets.schema.md";

pub(crate) const WAVE4_COMPONENT_AND_PILOT_LANE_DEPTH_PACKETS_READER_PATH: &str =
    "docs/reading/wave4-component-and-pilot-lane-depth-packets.md";

pub(crate) const WAVE5_FISCAL_CONTROL_OVERLAY_DEPTH_PACKETS_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/wave5_fiscal_control_overlay_depth_packets.v1.draft.json";

pub(crate) const WAVE5_FISCAL_CONTROL_OVERLAY_DEPTH_PACKETS_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/wave5_fiscal_control_overlay_depth_packets.schema.md";

pub(crate) const WAVE5_FISCAL_CONTROL_OVERLAY_DEPTH_PACKETS_READER_PATH: &str =
    "docs/reading/wave5-fiscal-control-overlay-depth-packets.md";

pub(crate) const WAVE_LANE_DEPTH_SCAFFOLD_ROLLUP_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/wave_lane_depth_scaffold_rollup.v1.draft.json";

pub(crate) const WAVE_LANE_DEPTH_SCAFFOLD_ROLLUP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/wave_lane_depth_scaffold_rollup.schema.md";

pub(crate) const WAVE_LANE_DEPTH_SCAFFOLD_ROLLUP_READER_PATH: &str =
    "docs/reading/wave-lane-depth-scaffold-rollup.md";

pub(crate) const POST_ROLLUP_READINESS_WORK_QUEUE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/post_rollup_readiness_work_queue.v1.draft.json";

pub(crate) const POST_ROLLUP_READINESS_WORK_QUEUE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/post_rollup_readiness_work_queue.schema.md";

pub(crate) const POST_ROLLUP_READINESS_WORK_QUEUE_READER_PATH: &str =
    "docs/reading/post-rollup-readiness-work-queue.md";

pub(crate) const CURRENT_LAW_SOURCE_CUSTODY_BATCH_PLAN_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/current_law_source_custody_batch_plan.v1.draft.json";

pub(crate) const CURRENT_LAW_SOURCE_CUSTODY_BATCH_PLAN_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/current_law_source_custody_batch_plan.schema.md";

pub(crate) const CURRENT_LAW_SOURCE_CUSTODY_BATCH_PLAN_READER_PATH: &str =
    "docs/reading/current-law-source-custody-batch-plan.md";

pub(crate) const CURRENT_LAW_SOURCE_CUSTODY_PACKET_TEMPLATE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/current_law_source_custody_packet_template.v1.draft.json";

pub(crate) const CURRENT_LAW_SOURCE_CUSTODY_PACKET_TEMPLATE_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/current_law_source_custody_packet_template.schema.md";

pub(crate) const CURRENT_LAW_SOURCE_CUSTODY_PACKET_TEMPLATE_READER_PATH: &str =
    "docs/reading/current-law-source-custody-packet-template.md";

pub(crate) const CURRENT_LAW_FY2025_17_ROW_LEDGER_CUSTODY_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/current_law_fy2025_17_row_ledger_custody.v1.draft.json";

pub(crate) const CURRENT_LAW_FY2025_17_ROW_LEDGER_CUSTODY_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/current_law_fy2025_17_row_ledger_custody.schema.md";

pub(crate) const CURRENT_LAW_FY2025_17_ROW_LEDGER_CUSTODY_READER_PATH: &str =
    "docs/reading/current-law-fy2025-17-row-ledger-custody.md";

pub(crate) const CURRENT_LAW_17_ROW_PBD_FY2025_2031_CONTEXT_PATH_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/current_law_17_row_pbd_fy2025_2031_context_path.v1.draft.json";

pub(crate) const CURRENT_LAW_17_ROW_PBD_FY2025_2031_CONTEXT_PATH_READER_PATH: &str =
    "docs/reading/current-law-17-row-pbd-fy2025-2031-context-path.md";

pub(crate) const CBO_OPEN_DATA_FY2032_2035_CURRENT_LAW_EXTENSION_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/cbo_open_data_fy2032_2035_current_law_extension_context.v1.draft.json";

pub(crate) const CBO_OPEN_DATA_FY2032_2035_CURRENT_LAW_EXTENSION_CONTEXT_READER_PATH: &str =
    "docs/reading/cbo-open-data-fy2032-2035-current-law-extension-context.md";

pub(crate) const CBO_MAJOR_OUTLAY_CATEGORY_FY2032_2035_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/cbo_major_outlay_category_fy2032_2035_context.v1.draft.json";

pub(crate) const CBO_MAJOR_OUTLAY_CATEGORY_FY2032_2035_CONTEXT_READER_PATH: &str =
    "docs/reading/cbo-major-outlay-category-fy2032-2035-context.md";

pub(crate) const CBO_REVENUE_DETAIL_FY2026_2035_CONTEXT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/cbo_revenue_detail_fy2026_2035_context.v1.draft.json";

pub(crate) const CBO_REVENUE_DETAIL_FY2026_2035_CONTEXT_READER_PATH: &str =
    "docs/reading/cbo-revenue-detail-fy2026-2035-context.md";

pub(crate) const CBO_HEALTH_INSURANCE_BASELINE_BROWSER_CONTEXT_FY2026_2036_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/cbo_health_insurance_baseline_browser_context_fy2026_2036.v1.draft.json";

pub(crate) const CBO_HEALTH_INSURANCE_BASELINE_BROWSER_CONTEXT_FY2026_2036_READER_PATH: &str =
    "docs/reading/cbo-health-insurance-baseline-browser-context-fy2026-2036.md";

pub(crate) const CBO_HEALTH_INSURANCE_TABLE2_BROWSER_ROWMAP_FY2026_2036_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/cbo_health_insurance_table2_browser_rowmap_fy2026_2036.v1.draft.json";

pub(crate) const CBO_HEALTH_INSURANCE_TABLE2_BROWSER_ROWMAP_FY2026_2036_READER_PATH: &str =
    "docs/reading/cbo-health-insurance-table2-browser-rowmap-fy2026-2036.md";

pub(crate) const OMB_PBD_FY2027_USER_GUIDE_HORIZON_BOUNDARY_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/omb_pbd_fy2027_user_guide_horizon_boundary_context.v1.draft.json";

pub(crate) const OMB_PBD_FY2027_USER_GUIDE_HORIZON_BOUNDARY_CONTEXT_READER_PATH: &str =
    "docs/reading/omb-pbd-fy2027-user-guide-horizon-boundary-context.md";

pub(crate) const CURRENT_LAW_BASELINE_ANNUAL_PATH_PARTIAL_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/current_law_baseline_annual_path_partial.v1.draft.json";

pub(crate) const CURRENT_LAW_BASELINE_ANNUAL_PATH_PARTIAL_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/current_law_baseline_annual_path_partial.schema.md";

pub(crate) const CURRENT_LAW_BASELINE_ANNUAL_PATH_PARTIAL_READER_PATH: &str =
    "docs/reading/current-law-baseline-annual-path-partial.md";

pub(crate) const CURRENT_LAW_BASELINE_RECEIPTS_DEFICIT_PATH_PARTIAL_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/current_law_baseline_receipts_deficit_path_partial.v1.draft.json";

pub(crate) const CURRENT_LAW_BASELINE_RECEIPTS_DEFICIT_PATH_PARTIAL_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/current_law_baseline_receipts_deficit_path_partial.schema.md";

pub(crate) const CURRENT_LAW_BASELINE_RECEIPTS_DEFICIT_PATH_PARTIAL_READER_PATH: &str =
    "docs/reading/current-law-baseline-receipts-deficit-path-partial.md";

pub(crate) const CURRENT_LAW_FY2025_FUND_GROUP_PATH_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/current_law_fy2025_fund_group_path.v1.draft.json";

pub(crate) const CURRENT_LAW_FY2025_FUND_GROUP_PATH_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/current_law_fy2025_fund_group_path.schema.md";

pub(crate) const CURRENT_LAW_FY2025_FUND_GROUP_PATH_READER_PATH: &str =
    "docs/reading/current-law-fy2025-fund-group-path.md";

pub(crate) const TREASURY_MTS_TABLE_8_FEDERAL_FUND_FY2025_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/treasury_mts_table_8_federal_fund_fy2025_context.v1.draft.json";

pub(crate) const TREASURY_MTS_TABLE_8_FEDERAL_FUND_FY2025_CONTEXT_READER_PATH: &str =
    "docs/reading/treasury-mts-table-8-federal-fund-fy2025-context.md";

pub(crate) const CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/current_law_fy2025_dedicated_receipt_anchors.v1.draft.json";

pub(crate) const CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/current_law_fy2025_dedicated_receipt_anchors.schema.md";

pub(crate) const CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_READER_PATH: &str =
    "docs/reading/current-law-fy2025-dedicated-receipt-anchors.md";

pub(crate) const CURRENT_LAW_FY2025_NAMED_TRUST_FUND_OUTLAY_ANCHORS_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/current_law_fy2025_named_trust_fund_outlay_anchors.v1.draft.json";

pub(crate) const CURRENT_LAW_FY2025_NAMED_TRUST_FUND_OUTLAY_ANCHORS_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/current_law_fy2025_named_trust_fund_outlay_anchors.schema.md";

pub(crate) const CURRENT_LAW_FY2025_NAMED_TRUST_FUND_OUTLAY_ANCHORS_READER_PATH: &str =
    "docs/reading/current-law-fy2025-named-trust-fund-outlay-anchors.md";

pub(crate) const CURRENT_LAW_NAMED_FUND_BALANCE_TRANSFER_GAP_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/current_law_named_fund_balance_transfer_gap.v1.draft.json";

pub(crate) const CURRENT_LAW_NAMED_FUND_BALANCE_TRANSFER_GAP_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/current_law_named_fund_balance_transfer_gap.schema.md";

pub(crate) const CURRENT_LAW_NAMED_FUND_BALANCE_TRANSFER_GAP_READER_PATH: &str =
    "docs/reading/current-law-named-fund-balance-transfer-gap.md";

pub(crate) const CURRENT_LAW_SOURCE_CUSTODY_PROGRESS_ROLLUP_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/current_law_source_custody_progress_rollup.v1.draft.json";

pub(crate) const CURRENT_LAW_SOURCE_CUSTODY_PROGRESS_ROLLUP_READER_PATH: &str =
    "docs/reading/current-law-source-custody-progress-rollup.md";

pub(crate) const CURRENT_LAW_SOURCE_CUSTODY_WAVE_B_CLOSURE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/current_law_source_custody_wave_b_closure.v1.draft.json";

pub(crate) const CURRENT_LAW_SOURCE_CUSTODY_WAVE_B_CLOSURE_READER_PATH: &str =
    "docs/reading/current-law-source-custody-wave-b-closure.md";

pub(crate) const DATA_ACQUISITION_EIGHT_GAP_STATUS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/data_acquisition_eight_gap_status.v1.draft.json";

pub(crate) const DATA_ACQUISITION_EIGHT_GAP_STATUS_READER_PATH: &str =
    "docs/reading/data-acquisition-eight-gap-status.md";

pub(crate) const CMS_HOSPITAL_QUALITY_METHODOLOGY_SURFACE_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/cms_hospital_quality_methodology_surface_context.v1.draft.json";

pub(crate) const CMS_HOSPITAL_QUALITY_METHODOLOGY_SURFACE_CONTEXT_READER_PATH: &str =
    "docs/reading/cms-hospital-quality-methodology-surface-context.md";

pub(crate) const CMS_HOSPITAL_MEASURE_METHODOLOGY_REPORT_CUSTODY_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/cms_hospital_measure_methodology_report_custody.v1.draft.json";

pub(crate) const CMS_HOSPITAL_MEASURE_METHODOLOGY_REPORT_CUSTODY_READER_PATH: &str =
    "docs/reading/cms-hospital-measure-methodology-report-custody.md";

pub(crate) const CMS_HOSPITAL_QUALITY_DATASET_FIELD_CROSSWALK_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/cms_hospital_quality_dataset_field_crosswalk.v1.draft.json";

pub(crate) const CMS_HOSPITAL_QUALITY_DATASET_FIELD_CROSSWALK_READER_PATH: &str =
    "docs/reading/cms-hospital-quality-dataset-field-crosswalk.md";

pub(crate) const CMS_HRSA_RURAL_SAFETY_NET_CAPACITY_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/cms_hrsa_rural_safety_net_capacity_context.v1.draft.json";

pub(crate) const CMS_HRSA_RURAL_SAFETY_NET_CAPACITY_CONTEXT_READER_PATH: &str =
    "docs/reading/cms-hrsa-rural-safety-net-capacity-context.md";

pub(crate) const NET_INTEREST_TREASURY_MSPD_MATURITY_DETAIL_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/net_interest_treasury_mspd_maturity_detail_context.v1.draft.json";

pub(crate) const NET_INTEREST_TREASURY_MSPD_MATURITY_DETAIL_CONTEXT_READER_PATH: &str =
    "docs/reading/net-interest-treasury-mspd-maturity-detail-context.md";

pub(crate) const NET_INTEREST_TREASURY_MSPD_REMAINING_MATURITY_BUCKET_DIAGNOSTIC_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/net_interest_treasury_mspd_remaining_maturity_bucket_diagnostic.v1.draft.json";

pub(crate) const NET_INTEREST_TREASURY_MSPD_REMAINING_MATURITY_BUCKET_DIAGNOSTIC_READER_PATH: &str =
    "docs/reading/net-interest-treasury-mspd-remaining-maturity-bucket-diagnostic.md";

pub(crate) const NET_INTEREST_TREASURY_MSPD_SNAPSHOT_RECONCILIATION_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_interest_treasury_mspd_snapshot_reconciliation.v1.draft.json";
pub(crate) const NET_INTEREST_TREASURY_MSPD_SNAPSHOT_RECONCILIATION_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/net_interest_treasury_mspd_snapshot_reconciliation.schema.md";
pub(crate) const NET_INTEREST_TREASURY_MSPD_SNAPSHOT_RECONCILIATION_READER_PATH: &str =
    "docs/reading/net-interest-treasury-mspd-snapshot-reconciliation.md";
pub(crate) const NET_INTEREST_TREASURY_MSPD_SNAPSHOT_RECONCILIATION_REVIEW_PATH: &str =
    "reviews/2026-07-31-net-interest-treasury-mspd-snapshot-reconciliation-role-review.md";

pub(crate) const NET_INTEREST_TREASURY_AVERAGE_INTEREST_RATE_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/net_interest_treasury_average_interest_rate_context.v1.draft.json";

pub(crate) const NET_INTEREST_TREASURY_AVERAGE_INTEREST_RATE_CONTEXT_READER_PATH: &str =
    "docs/reading/net-interest-treasury-average-interest-rate-context.md";

pub(crate) const OMB_AP13_FUND_GROUP_RECONCILIATION_DETAIL_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/omb_ap13_fund_group_reconciliation_detail_fy2025_context.v1.draft.json";

pub(crate) const OMB_AP13_FUND_GROUP_RECONCILIATION_DETAIL_CONTEXT_READER_PATH: &str =
    "docs/reading/omb-ap13-fund-group-reconciliation-detail-fy2025-context.md";

pub(crate) const TRANSPORTATION_TRUST_FUND_TABLE_13_4_AGGREGATE_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_trust_fund_table_13_4_aggregate_fy2025_2031_context.v1.draft.json";

pub(crate) const TRANSPORTATION_TRUST_FUND_TABLE_13_4_AGGREGATE_CONTEXT_READER_PATH: &str =
    "docs/reading/transportation-trust-fund-table-13-4-aggregate-fy2025-2031-context.md";

pub(crate) const TRANSPORTATION_TRUST_FUND_TABLE_13_4_IDENTITY_DIAGNOSTIC_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_trust_fund_table_13_4_identity_diagnostic.v1.draft.json";

pub(crate) const TRANSPORTATION_TRUST_FUND_TABLE_13_4_IDENTITY_DIAGNOSTIC_READER_PATH: &str =
    "docs/reading/transportation-trust-fund-table-13-4-identity-diagnostic.md";

pub(crate) const TRANSPORTATION_TRUST_FUND_CBO_BALANCE_EXTENSION_FY2032_2035_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_trust_fund_cbo_balance_extension_fy2032_2035_context.v1.draft.json";

pub(crate) const TRANSPORTATION_TRUST_FUND_CBO_BALANCE_EXTENSION_FY2032_2035_CONTEXT_READER_PATH: &str =
    "docs/reading/transportation-trust-fund-cbo-balance-extension-fy2032-2035-context.md";

pub(crate) const TRANSPORTATION_TRUST_FUND_CROSS_SOURCE_RECONCILIATION_STATUS_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_trust_fund_cross_source_reconciliation_status.v1.draft.json";

pub(crate) const TRANSPORTATION_TRUST_FUND_CROSS_SOURCE_RECONCILIATION_STATUS_SCHEMA_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_trust_fund_cross_source_reconciliation_status.schema.md";

pub(crate) const TRANSPORTATION_TRUST_FUND_CROSS_SOURCE_RECONCILIATION_STATUS_READER_PATH: &str =
    "docs/reading/transportation-trust-fund-cross-source-reconciliation-status.md";

pub(crate) const TRANSPORTATION_TRUST_FUND_TREASURY_MTS_FY2025_ANCHOR_CONTEXT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/transportation_trust_fund_treasury_mts_fy2025_anchor_context.v1.draft.json";

pub(crate) const TRANSPORTATION_TRUST_FUND_TREASURY_MTS_FY2025_ANCHOR_CONTEXT_READER_PATH: &str =
    "docs/reading/transportation-trust-fund-treasury-mts-fy2025-anchor-context.md";

pub(crate) const BUDGET_BALLOT_CONFIG_PATH: &str = "experiments/annual-budget-ballot/config.v1.json";

pub(crate) const BUDGET_BALLOT_OUTPUT_PATH: &str =
    "experiments/annual-budget-ballot/outputs/synthetic-run.v1.json";

pub(crate) const BUDGET_BALLOT_READER_PATH: &str =
    "experiments/annual-budget-ballot/outputs/synthetic-run.v1.md";

pub(crate) const BUDGET_BALLOT_V2_CONFIG_PATH: &str = "experiments/annual-budget-ballot/config.v2.json";

pub(crate) const BUDGET_BALLOT_V2_OUTPUT_PATH: &str =
    "experiments/annual-budget-ballot/outputs/diverse-run.v2.json";

pub(crate) const BUDGET_BALLOT_V2_READER_PATH: &str =
    "experiments/annual-budget-ballot/outputs/diverse-run.v2.md";

pub(crate) const VETERANS_DEPTH_CARD_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/veterans_depth_card.fy2025.v1.draft.json";

pub(crate) const VETERANS_DEPTH_CARD_READER_PATH: &str = "docs/reading/veterans-depth-card.md";

pub(crate) const TRANSPORTATION_DEPTH_CARD_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/transportation_depth_card.fy2025.v1.draft.json";

pub(crate) const TRANSPORTATION_DEPTH_CARD_READER_PATH: &str = "docs/reading/transportation-depth-card.md";

pub(crate) const EDUCATION_DEPTH_CARD_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/education_depth_card.fy2025.v1.draft.json";

pub(crate) const EDUCATION_DEPTH_CARD_READER_PATH: &str = "docs/reading/education-depth-card.md";

pub(crate) const HIGHER_EDUCATION_BRIDGE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/higher_education_account_bridge.fy2025.v1.draft.json";

pub(crate) const HIGHER_EDUCATION_BRIDGE_READER_PATH: &str = "docs/reading/higher-education-account-bridge.md";

pub(crate) const PELL_SHORT_TRAINING_IMPACT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/pell_short_training_impact_evidence.2012-2021.v1.draft.json";

pub(crate) const PELL_SHORT_TRAINING_IMPACT_READER_PATH: &str =
    "docs/reading/pell-short-training-impact-evidence.md";

pub(crate) const FSA_TITLE_IV_STUDENT_ACCESS_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/fsa_title_iv_student_access_baseline.fy2024.v1.draft.json";

pub(crate) const FSA_TITLE_IV_STUDENT_ACCESS_READER_PATH: &str =
    "docs/reading/fsa-title-iv-student-access-baseline.md";

pub(crate) const PELL_BACHELOR_OUTCOME_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/pell_bachelor_recipient_outcome_baseline.bb2016-2020.v1.draft.json";

pub(crate) const PELL_BACHELOR_OUTCOME_READER_PATH: &str =
    "docs/reading/pell-bachelor-recipient-outcome-baseline.md";

pub(crate) const PELL_BACHELOR_OUTCOME_SOURCE_ID: &str = "SRC-NCES-BB16-20-PELL-OUTCOMES-2022";

pub(crate) const PELL_BACHELOR_OUTCOME_METADATA_PATH: &str =
    "data/metadata/SRC-NCES-BB16-20-PELL-OUTCOMES-2022.2026-07-13.metadata.md";

pub(crate) const PELL_BACHELOR_OUTCOME_RAW_PATH: &str =
    "data/raw/nces/SRC-NCES-BB16-20-PELL-OUTCOMES-2022/2026-07-13/2022241.pdf";

pub(crate) const PELL_BACHELOR_OUTCOME_RAW_BYTES: u64 = 2_254_611;

pub(crate) const PELL_BACHELOR_OUTCOME_RAW_SHA256: &str =
    "7f8b2021b4bda8fc52a783c4b9ebe9c378a300f75165c491d52d153e9c41f9ee";

pub(crate) const BPS_FIRST_TIME_STUDENT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/bps_first_time_student_longitudinal_bridge.ay2019-2022.v1.draft.json";

pub(crate) const BPS_FIRST_TIME_STUDENT_READER_PATH: &str =
    "docs/reading/bps-first-time-student-longitudinal-bridge.md";

pub(crate) const BPS_FIRST_LOOK_SOURCE_ID: &str = "SRC-NCES-BPS20-22-FIRST-LOOK-2024";

pub(crate) const BPS_FIRST_LOOK_METADATA_PATH: &str =
    "data/metadata/SRC-NCES-BPS20-22-FIRST-LOOK-2024.2026-07-13.metadata.md";

pub(crate) const BPS_FIRST_LOOK_RAW_PATH: &str =
    "data/raw/nces/SRC-NCES-BPS20-22-FIRST-LOOK-2024/2026-07-13/2024401.pdf";

pub(crate) const BPS_FIRST_LOOK_RAW_BYTES: u64 = 2_167_816;

pub(crate) const BPS_FIRST_LOOK_RAW_SHA256: &str =
    "37e099c660c1db5a288091e43122aac89a6e00095cdbdbd907fc1104b15ff0f3";

pub(crate) const BPS_DFD_SOURCE_ID: &str = "SRC-NCES-BPS20-22-DFD-2026";

pub(crate) const BPS_DFD_METADATA_PATH: &str =
    "data/metadata/SRC-NCES-BPS20-22-DFD-2026.2026-07-13.metadata.md";

pub(crate) const BPS_DFD_RAW_PATH: &str = "data/raw/nces/SRC-NCES-BPS20-22-DFD-2026/2026-07-13/2026013.pdf";

pub(crate) const BPS_DFD_RAW_BYTES: u64 = 8_928_093;

pub(crate) const BPS_DFD_RAW_SHA256: &str = "5cd2f7ca96c76da3a683dc79ff12dcfa056aed92f9532db1afb9d14edf9aa524";

pub(crate) const PELL_CURRENT_ENTRANT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/pell_current_entrant_persistence_baseline.bps2020-2022.v1.draft.json";

pub(crate) const PELL_CURRENT_ENTRANT_READER_PATH: &str =
    "docs/reading/pell-current-entrant-persistence-baseline.md";

pub(crate) const PELL_CURRENT_ENTRANT_SOURCE_ID: &str = "SRC-NCES-DATALAB-BPS20-22-PELL-PERSISTENCE-2026";

pub(crate) const PELL_CURRENT_ENTRANT_METADATA_PATH: &str =
    "data/metadata/SRC-NCES-DATALAB-BPS20-22-PELL-PERSISTENCE-2026.2026-07-13.metadata.md";

pub(crate) const PELL_CURRENT_ENTRANT_RAW_PATH: &str = "data/raw/nces/SRC-NCES-DATALAB-BPS20-22-PELL-PERSISTENCE-2026/2026-07-13/workspace-retrieve-zclxfu.json";

pub(crate) const PELL_CURRENT_ENTRANT_RAW_BYTES: u64 = 8_023;

pub(crate) const PELL_CURRENT_ENTRANT_RAW_SHA256: &str =
    "aedc7781ddc8da4a9f59942e16b398f58cffb20128cc6fe44cf24d6f04795dc5";

pub(crate) const PELL_CURRENT_ENTRANT_RETRIEVAL_CODE: &str = "zclxfu";

pub(crate) const PELL_CURRENT_ENTRANT_QUERY_ID: i64 = 396_385;

pub(crate) const PELL_CURRENT_ENTRANT_SIGNIFICANCE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/pell_current_entrant_persistence_significance_screen.bps2020-2022.v1.draft.json";

pub(crate) const PELL_CURRENT_ENTRANT_SIGNIFICANCE_READER_PATH: &str =
    "docs/reading/pell-current-entrant-persistence-significance-screen.md";

pub(crate) const PELL_CURRENT_ENTRANT_SIGNIFICANCE_SOURCE_ID: &str =
    "SRC-NCES-DATALAB-BPS20-22-PELL-PERSISTENCE-TTEST-2026";

pub(crate) const PELL_CURRENT_ENTRANT_SIGNIFICANCE_METADATA_PATH: &str =
    "data/metadata/SRC-NCES-DATALAB-BPS20-22-PELL-PERSISTENCE-TTEST-2026.2026-07-13.metadata.md";

pub(crate) const PELL_CURRENT_ENTRANT_SIGNIFICANCE_RAW_DIR: &str =
    "data/raw/nces/SRC-NCES-DATALAB-BPS20-22-PELL-PERSISTENCE-TTEST-2026/2026-07-13";

pub(crate) const PELL_CURRENT_ENTRANT_SIGNIFICANCE_MANIFEST_PATH: &str = "data/raw/nces/SRC-NCES-DATALAB-BPS20-22-PELL-PERSISTENCE-TTEST-2026/2026-07-13/request-manifest.json";

pub(crate) const PELL_CURRENT_ENTRANT_SIGNIFICANCE_MANIFEST_BYTES: u64 = 3_468;

pub(crate) const PELL_CURRENT_ENTRANT_SIGNIFICANCE_MANIFEST_SHA256: &str =
    "9d09c714fa6cf290b5964aef3a35ada9225a2aa977f7ae0d8ee701ac7ec3ca57";

pub(crate) const FCIC_PAYMENT_INTEGRITY_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/federal_crop_insurance_payment_integrity_bridge.fy2024-q4-2025.v1.draft.json";

pub(crate) const FCIC_PAYMENT_INTEGRITY_BRIDGE_READER_PATH: &str =
    "docs/reading/federal-crop-insurance-payment-integrity-bridge.md";

pub(crate) const FCIC_SCORECARD_SOURCE_ID: &str = "SRC-OMB-PAYMENTACCURACY-FCIC-Q4-2025";

pub(crate) const FCIC_SCORECARD_METADATA_PATH: &str =
    "data/metadata/SRC-OMB-PAYMENTACCURACY-FCIC-Q4-2025.2026-07-13.metadata.md";

pub(crate) const FCIC_SCORECARD_RAW_PATH: &str = "data/raw/omb/SRC-OMB-PAYMENTACCURACY-FCIC-Q4-2025/2026-07-13/Federal Crop Insurance Corporation (FCIC).pdf";

pub(crate) const FCIC_SCORECARD_RAW_BYTES: u64 = 217_443;

pub(crate) const FCIC_SCORECARD_RAW_SHA256: &str =
    "64486352e268061b05554255f5fbb43ded57401549efc19e31268ae97e945ed2";

pub(crate) const FCIC_COM_23_SOURCE_ID: &str = "SRC-USDA-RMA-COM-23-001";

pub(crate) const FCIC_COM_23_METADATA_PATH: &str =
    "data/metadata/SRC-USDA-RMA-COM-23-001.2026-07-13.metadata.md";

pub(crate) const FCIC_COM_23_RAW_PATH: &str =
    "data/raw/usda/SRC-USDA-RMA-COM-23-001/2026-07-13/COM-23-001.pdf";

pub(crate) const FCIC_COM_23_RAW_BYTES: u64 = 43_246;

pub(crate) const FCIC_COM_23_RAW_SHA256: &str =
    "e28d06d615bb4af8447c6d53d1017a0f9d8a74d7f2399966d6bd0551d31a8a6c";

pub(crate) const FCIC_SRA_2022_SOURCE_ID: &str = "SRC-USDA-RMA-SRA-2022";

pub(crate) const FCIC_SRA_2022_METADATA_PATH: &str =
    "data/metadata/SRC-USDA-RMA-SRA-2022.2026-07-13.metadata.md";

pub(crate) const FCIC_SRA_2022_RAW_PATH: &str = "data/raw/usda/SRC-USDA-RMA-SRA-2022/2026-07-13/SRA_2022.pdf";

pub(crate) const FCIC_SRA_2022_RAW_BYTES: u64 = 372_206;

pub(crate) const FCIC_SRA_2022_RAW_SHA256: &str =
    "589adbc9219012ae487ee567e8e0a0c6b351ff08abbdb66820f3ce1130e551dd";

pub(crate) const FCIC_ROOT_CAUSE_DEFINITION_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/federal_crop_insurance_root_cause_definition_bridge.fy2024.v1.draft.json";

pub(crate) const FCIC_ROOT_CAUSE_DEFINITION_BRIDGE_READER_PATH: &str =
    "docs/reading/federal-crop-insurance-root-cause-definition-bridge.md";

pub(crate) const FCIC_AFR_SOURCE_ID: &str = "SRC-USDA-AFR-FY2024";

pub(crate) const FCIC_AFR_METADATA_PATH: &str = "data/metadata/SRC-USDA-AFR-FY2024.2026-07-13.metadata.md";

pub(crate) const FCIC_AFR_RAW_PATH: &str =
    "data/raw/usda/SRC-USDA-AFR-FY2024/2026-07-13/fy-2024-agency-financial-report.pdf";

pub(crate) const FCIC_AFR_RAW_BYTES: u64 = 15_170_759;

pub(crate) const FCIC_AFR_RAW_SHA256: &str =
    "f573ac22ddcc64a1ce2dd9c13370eb1e02e83f2467f3a87146c3e3d521e8de22";

pub(crate) const FCIC_PAYMENT_UNIVERSE_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/federal_crop_insurance_payment_universe_bridge.fy2024.v1.draft.json";

pub(crate) const FCIC_PAYMENT_UNIVERSE_BRIDGE_READER_PATH: &str =
    "docs/reading/federal-crop-insurance-payment-universe-bridge.md";

pub(crate) const FCIC_OIG_FS_SOURCE_ID: &str = "SRC-USDA-OIG-FCIC-RMA-FS-FY2024";

pub(crate) const FCIC_OIG_FS_METADATA_PATH: &str =
    "data/metadata/SRC-USDA-OIG-FCIC-RMA-FS-FY2024.2026-07-13.metadata.md";

pub(crate) const FCIC_OIG_FS_RAW_PATH: &str =
    "data/raw/usda/SRC-USDA-OIG-FCIC-RMA-FS-FY2024/2026-07-13/05403-0001-11_FR_508.pdf";

pub(crate) const FCIC_OIG_FS_RAW_BYTES: u64 = 7_242_677;

pub(crate) const FCIC_OIG_FS_RAW_SHA256: &str =
    "0797bd2ccb1027b568bce3b640849e89f30a235f528b1b1a2b249d525695ed32";

pub(crate) const FCIC_SAMPLE_DESIGN_COMPONENT_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/federal_crop_insurance_sample_design_component_bridge.fy2024.v1.draft.json";

pub(crate) const FCIC_SAMPLE_DESIGN_COMPONENT_BRIDGE_READER_PATH: &str =
    "docs/reading/federal-crop-insurance-sample-design-component-bridge.md";

pub(crate) const FCIC_OIG_PIIA_SOURCE_ID: &str = "SRC-USDA-OIG-PIIA-COMPLIANCE-FY2024";

pub(crate) const FCIC_OIG_PIIA_METADATA_PATH: &str =
    "data/metadata/SRC-USDA-OIG-PIIA-COMPLIANCE-FY2024.2026-07-13.metadata.md";

pub(crate) const FCIC_OIG_PIIA_RAW_PATH: &str =
    "data/raw/usda/SRC-USDA-OIG-PIIA-COMPLIANCE-FY2024/2026-07-13/50024-0016-11_FR_508.pdf";

pub(crate) const FCIC_OIG_PIIA_RAW_BYTES: u64 = 5_619_427;

pub(crate) const FCIC_OIG_PIIA_RAW_SHA256: &str =
    "a3cebe04d34d926737995ee9b176f5d7f43eff0e60dc20574ddb8d4fa7b5c60f";

pub(crate) const FCIC_HISTORICAL_SAMPLING_METHOD_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/federal_crop_insurance_historical_sampling_method_bridge.fy2020.v1.draft.json";

pub(crate) const FCIC_HISTORICAL_SAMPLING_METHOD_BRIDGE_READER_PATH: &str =
    "docs/reading/federal-crop-insurance-historical-sampling-method-bridge.md";

pub(crate) const FCIC_OIG_FS_FY2020_SOURCE_ID: &str = "SRC-USDA-OIG-FCIC-RMA-FS-FY2020";

pub(crate) const FCIC_OIG_FS_FY2020_METADATA_PATH: &str =
    "data/metadata/SRC-USDA-OIG-FCIC-RMA-FS-FY2020.2026-07-13.metadata.md";

pub(crate) const FCIC_OIG_FS_FY2020_RAW_PATH: &str =
    "data/raw/usda/SRC-USDA-OIG-FCIC-RMA-FS-FY2020/2026-07-13/05401-0012-11FRFOIA.pdf";

pub(crate) const FCIC_OIG_FS_FY2020_RAW_BYTES: u64 = 13_696_922;

pub(crate) const FCIC_OIG_FS_FY2020_RAW_SHA256: &str =
    "55fd128f191c3d0892f819f35a92929efb02dbd7354626d3c844a84c3253ac4b";

pub(crate) const FCIC_PUBLIC_METHODOLOGY_EVIDENCE_CEILING_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/federal_crop_insurance_public_methodology_evidence_ceiling.fy2025.v1.draft.json";

pub(crate) const FCIC_PUBLIC_METHODOLOGY_EVIDENCE_CEILING_READER_PATH: &str =
    "docs/reading/federal-crop-insurance-public-methodology-evidence-ceiling.md";

pub(crate) const OMB_M_21_19_SOURCE_ID: &str = "SRC-OMB-M-21-19";

pub(crate) const OMB_M_21_19_METADATA_PATH: &str = "data/metadata/SRC-OMB-M-21-19.2026-07-13.metadata.md";

pub(crate) const OMB_M_21_19_RAW_PATH: &str = "data/raw/omb/SRC-OMB-M-21-19/2026-07-13/M-21-19.pdf";

pub(crate) const OMB_M_21_19_RAW_BYTES: u64 = 2_808_576;

pub(crate) const OMB_M_21_19_RAW_SHA256: &str =
    "12a1d448b1d5eb7040e2377e7e04bb721a2a513a9220768037b0f094c03d14aa";

pub(crate) const FCIC_OIG_FS_FY2025_SOURCE_ID: &str = "SRC-USDA-OIG-FCIC-RMA-FS-FY2025";

pub(crate) const FCIC_OIG_FS_FY2025_METADATA_PATH: &str =
    "data/metadata/SRC-USDA-OIG-FCIC-RMA-FS-FY2025.2026-07-13.metadata.md";

pub(crate) const FCIC_OIG_FS_FY2025_RAW_PATH: &str =
    "data/raw/usda/SRC-USDA-OIG-FCIC-RMA-FS-FY2025/2026-07-13/05403-0002-11-FR-508-signed.pdf";

pub(crate) const FCIC_OIG_FS_FY2025_RAW_BYTES: u64 = 6_099_095;

pub(crate) const FCIC_OIG_FS_FY2025_RAW_SHA256: &str =
    "0bcd7df0c7e3f78a7bb4b4c896c718e38adf1d6fba36b0efe45bd84d8738272a";

pub(crate) const FCIC_RECOVERY_LINEAGE_BOUNDARY_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/federal_crop_insurance_recovery_lineage_boundary_bridge.fy2024.v1.draft.json";

pub(crate) const FCIC_RECOVERY_LINEAGE_BOUNDARY_BRIDGE_READER_PATH: &str =
    "docs/reading/federal-crop-insurance-recovery-lineage-boundary-bridge.md";

pub(crate) const FCIC_MANAGER_SEP_2023_SOURCE_ID: &str = "SRC-USDA-RMA-FCIC-MANAGER-2023-09-20";

pub(crate) const FCIC_MANAGER_SEP_2023_METADATA_PATH: &str =
    "data/metadata/SRC-USDA-RMA-FCIC-MANAGER-2023-09-20.2026-07-13.metadata.md";

pub(crate) const FCIC_MANAGER_SEP_2023_RAW_PATH: &str =
    "data/raw/usda/SRC-USDA-RMA-FCIC-MANAGER-2023-09-20/2026-07-13/092023managers.pdf";

pub(crate) const FCIC_MANAGER_SEP_2023_RAW_BYTES: u64 = 463_794;

pub(crate) const FCIC_MANAGER_SEP_2023_RAW_SHA256: &str =
    "a5f9267782552b331c294ce73836552879eadf47dda672ed5a40501d971cfcff";

pub(crate) const FCIC_MANAGER_FEB_2024_SOURCE_ID: &str = "SRC-USDA-RMA-FCIC-MANAGER-2024-02-22";

pub(crate) const FCIC_MANAGER_FEB_2024_METADATA_PATH: &str =
    "data/metadata/SRC-USDA-RMA-FCIC-MANAGER-2024-02-22.2026-07-13.metadata.md";

pub(crate) const FCIC_MANAGER_FEB_2024_RAW_PATH: &str =
    "data/raw/usda/SRC-USDA-RMA-FCIC-MANAGER-2024-02-22/2026-07-13/022224managers.pdf";

pub(crate) const FCIC_MANAGER_FEB_2024_RAW_BYTES: u64 = 168_423;

pub(crate) const FCIC_MANAGER_FEB_2024_RAW_SHA256: &str =
    "e89fa8202fb0eb4ff4e28211b10eed10e4cf06839172657a771da2f6b9d75be0";

pub(crate) const FCIC_MANAGER_MAY_2024_SOURCE_ID: &str = "SRC-USDA-RMA-FCIC-MANAGER-2024-05-23";

pub(crate) const FCIC_MANAGER_MAY_2024_METADATA_PATH: &str =
    "data/metadata/SRC-USDA-RMA-FCIC-MANAGER-2024-05-23.2026-07-13.metadata.md";

pub(crate) const FCIC_MANAGER_MAY_2024_RAW_PATH: &str =
    "data/raw/usda/SRC-USDA-RMA-FCIC-MANAGER-2024-05-23/2026-07-13/052324managers.pdf";

pub(crate) const FCIC_MANAGER_MAY_2024_RAW_BYTES: u64 = 195_464;

pub(crate) const FCIC_MANAGER_MAY_2024_RAW_SHA256: &str =
    "3da5b844433fb12db8f35e186fc227482118b118714021694ff4620e1cde4502";

pub(crate) const FCIC_MANAGER_AUG_2024_SOURCE_ID: &str = "SRC-USDA-RMA-FCIC-MANAGER-2024-08-22";

pub(crate) const FCIC_MANAGER_AUG_2024_METADATA_PATH: &str =
    "data/metadata/SRC-USDA-RMA-FCIC-MANAGER-2024-08-22.2026-07-13.metadata.md";

pub(crate) const FCIC_MANAGER_AUG_2024_RAW_PATH: &str =
    "data/raw/usda/SRC-USDA-RMA-FCIC-MANAGER-2024-08-22/2026-07-13/082224managers.pdf";

pub(crate) const FCIC_MANAGER_AUG_2024_RAW_BYTES: u64 = 232_689;

pub(crate) const FCIC_MANAGER_AUG_2024_RAW_SHA256: &str =
    "d090de9299d1dd7cf8b8c249e53c9b37056b648af3ef238ca3ad2951dc210026";

pub(crate) const FCIC_APPEAL_COLLECTIBILITY_GOVERNANCE_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/federal_crop_insurance_appeal_collectibility_governance_bridge.fy2024.v1.draft.json";

pub(crate) const FCIC_APPEAL_COLLECTIBILITY_GOVERNANCE_BRIDGE_READER_PATH: &str =
    "docs/reading/federal-crop-insurance-appeal-collectibility-governance-bridge.md";

pub(crate) const FCIC_COM_16_002_SOURCE_ID: &str = "SRC-USDA-RMA-COM-16-002";

pub(crate) const FCIC_COM_16_002_METADATA_PATH: &str =
    "data/metadata/SRC-USDA-RMA-COM-16-002.2026-07-13.metadata.md";

pub(crate) const FCIC_COM_16_002_RAW_PATH: &str =
    "data/raw/usda/SRC-USDA-RMA-COM-16-002/2026-07-13/COM-16-002.pdf";

pub(crate) const FCIC_COM_16_002_RAW_BYTES: u64 = 43_627;

pub(crate) const FCIC_COM_16_002_RAW_SHA256: &str =
    "6c4c90f25ad5bdd7dcb76e4cae9545321f029d7fb1a07d8f2680d16313b56830";

pub(crate) const FCIC_COM_16_004_SOURCE_ID: &str = "SRC-USDA-RMA-COM-16-004";

pub(crate) const FCIC_COM_16_004_METADATA_PATH: &str =
    "data/metadata/SRC-USDA-RMA-COM-16-004.2026-07-13.metadata.md";

pub(crate) const FCIC_COM_16_004_RAW_PATH: &str =
    "data/raw/usda/SRC-USDA-RMA-COM-16-004/2026-07-13/COM-16-004.pdf";

pub(crate) const FCIC_COM_16_004_RAW_BYTES: u64 = 43_452;

pub(crate) const FCIC_COM_16_004_RAW_SHA256: &str =
    "a78e792079689356a1327fbf334ca024f3cb2aaa15aa48d4427dcf83251e64da";

pub(crate) const FCIC_PUBLIC_COHORT_OUTCOME_EVIDENCE_CEILING_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/federal_crop_insurance_public_cohort_outcome_evidence_ceiling.fy2024.v1.draft.json";

pub(crate) const FCIC_PUBLIC_COHORT_OUTCOME_EVIDENCE_CEILING_READER_PATH: &str =
    "docs/reading/federal-crop-insurance-public-cohort-outcome-evidence-ceiling.md";

pub(crate) const FCIC_MANAGER_SEP_2024_SOURCE_ID: &str = "SRC-USDA-RMA-FCIC-MANAGER-2024-09-25";

pub(crate) const FCIC_MANAGER_SEP_2024_METADATA_PATH: &str =
    "data/metadata/SRC-USDA-RMA-FCIC-MANAGER-2024-09-25.2026-07-14.metadata.md";

pub(crate) const FCIC_MANAGER_SEP_2024_RAW_PATH: &str =
    "data/raw/usda/SRC-USDA-RMA-FCIC-MANAGER-2024-09-25/2026-07-14/092524managers.pdf";

pub(crate) const FCIC_MANAGER_SEP_2024_RAW_BYTES: u64 = 105_445;

pub(crate) const FCIC_MANAGER_SEP_2024_RAW_SHA256: &str =
    "acd35b47587751305f1199e675b715ccb5c074e2ad88d4ddb9d1d1e148ee9d22";

pub(crate) const FCIC_MANAGER_NOV_2024_SOURCE_ID: &str = "SRC-USDA-RMA-FCIC-MANAGER-2024-11-21";

pub(crate) const FCIC_MANAGER_NOV_2024_METADATA_PATH: &str =
    "data/metadata/SRC-USDA-RMA-FCIC-MANAGER-2024-11-21.2026-07-14.metadata.md";

pub(crate) const FCIC_MANAGER_NOV_2024_RAW_PATH: &str =
    "data/raw/usda/SRC-USDA-RMA-FCIC-MANAGER-2024-11-21/2026-07-14/112124managers.pdf";

pub(crate) const FCIC_MANAGER_NOV_2024_RAW_BYTES: u64 = 144_513;

pub(crate) const FCIC_MANAGER_NOV_2024_RAW_SHA256: &str =
    "f581f11fb99a690743894ba932fc6e460f57e82b3ffc68e2952169523947e443";

pub(crate) const FCIC_COHORT_DISPOSITION_REQUEST_SPEC_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/federal_crop_insurance_cohort_disposition_request_specification.fy2024.v1.draft.json";

pub(crate) const FCIC_COHORT_DISPOSITION_REQUEST_SPEC_READER_PATH: &str =
    "docs/reading/federal-crop-insurance-cohort-disposition-request-specification.md";

pub(crate) const FCIC_COHORT_DISPOSITION_REQUEST_TEMPLATE_PATH: &str =
    "docs/requests/federal-crop-insurance-fy2024-cohort-disposition-foia-request.md";

pub(crate) const RMA_FOIA_SOURCE_ID: &str = "SRC-USDA-RMA-FOIA";

pub(crate) const RMA_FOIA_METADATA_PATH: &str = "data/metadata/SRC-USDA-RMA-FOIA.2026-07-14.metadata.md";

pub(crate) const RMA_FOIA_RAW_PATH: &str = "data/raw/usda/SRC-USDA-RMA-FOIA/2026-07-14/rma-foia.html";

pub(crate) const RMA_FOIA_RAW_BYTES: u64 = 85_950;

pub(crate) const RMA_FOIA_RAW_SHA256: &str =
    "07baaef274de84d1e4569af3e820a0bfb5a827a4c176edaba86350dcdc21a671";

pub(crate) const USCODE_7_1502_SOURCE_ID: &str = "SRC-USCODE-7-1502";

pub(crate) const USCODE_7_1502_METADATA_PATH: &str = "data/metadata/SRC-USCODE-7-1502.2026-07-14.metadata.md";

pub(crate) const USCODE_7_1502_RAW_PATH: &str = "data/raw/uscode/SRC-USCODE-7-1502/2026-07-14/7-usc-1502.html";

pub(crate) const USCODE_7_1502_RAW_BYTES: u64 = 168_877;

pub(crate) const USCODE_7_1502_RAW_SHA256: &str =
    "cd71097ef55e3d1311d5e504b8b7c2b72d03338e5fb6c2d6b05ddf652486accd";

pub(crate) const ECFR_7_CFR_1_SOURCE_ID: &str = "SRC-ECFR-7-CFR-1-SUBPART-A";

pub(crate) const ECFR_7_CFR_1_METADATA_PATH: &str =
    "data/metadata/SRC-ECFR-7-CFR-1-SUBPART-A.2026-07-14.metadata.md";

pub(crate) const ECFR_7_CFR_1_RAW_PATH: &str =
    "data/raw/ecfr/SRC-ECFR-7-CFR-1-SUBPART-A/2026-07-14/7-cfr-1-subpart-a.html";

pub(crate) const ECFR_7_CFR_1_RAW_BYTES: u64 = 197_132;

pub(crate) const ECFR_7_CFR_1_RAW_SHA256: &str =
    "4069aa69cd67b04ed87b032022bcdd143ecd97fd42e78285c6f448edbde6bdd1";

pub(crate) const FCIC_FOIA_RESPONSE_INTAKE_CONTRACT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/federal_crop_insurance_foia_response_intake_contract.v1.draft.json";

pub(crate) const FCIC_FOIA_RESPONSE_INTAKE_TEMPLATE_PATH: &str =
    "data/templates/federal_crop_insurance_foia_response_intake.v1.template.json";

pub(crate) const FCIC_FOIA_PREFLIGHT_PATH: &str =
    "docs/requests/federal-crop-insurance-foia-submission-preflight.md";

pub(crate) const FCIC_FOIA_RESPONSE_INTAKE_READER_PATH: &str =
    "docs/reading/federal-crop-insurance-foia-preflight-response-intake.md";

pub(crate) const RMA_FOIA_FEES_SOURCE_ID: &str = "SRC-USDA-RMA-FOIA-FEES";

pub(crate) const RMA_FOIA_FEES_METADATA_PATH: &str =
    "data/metadata/SRC-USDA-RMA-FOIA-FEES.2026-07-14.metadata.md";

pub(crate) const RMA_FOIA_FEES_RAW_PATH: &str =
    "data/raw/usda/SRC-USDA-RMA-FOIA-FEES/2026-07-14/rma-foia-fees.html";

pub(crate) const RMA_FOIA_FEES_RAW_BYTES: u64 = 84_531;

pub(crate) const RMA_FOIA_FEES_RAW_SHA256: &str =
    "874816d0ec560706d8f74a118ab9d5d903c151742b9a760a86085d6a22f06749";

pub(crate) const MEDICARE_PART_D_PAYMENT_TYPE_COMPOSITION_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_part_d_payment_type_composition_bridge.fy2024.v1.draft.json";

pub(crate) const MEDICARE_PART_D_PAYMENT_TYPE_COMPOSITION_BRIDGE_READER_PATH: &str =
    "docs/reading/medicare-part-d-payment-type-composition-bridge.md";

pub(crate) const CMS_PART_D_IPM_FY2024_FINDINGS_SOURCE_ID: &str = "SRC-CMS-PART-D-IPM-FY2024-FINDINGS";

pub(crate) const CMS_PART_D_IPM_FY2024_FINDINGS_METADATA_PATH: &str =
    "data/metadata/SRC-CMS-PART-D-IPM-FY2024-FINDINGS.2026-07-14.metadata.md";

pub(crate) const CMS_PART_D_IPM_FY2024_FINDINGS_RAW_PATH: &str = "data/raw/cms/SRC-CMS-PART-D-IPM-FY2024-FINDINGS/2026-07-14/fy-2024-medicare-part-d-error-rate-findings-and-results.pdf";

pub(crate) const CMS_PART_D_IPM_FY2024_FINDINGS_RAW_BYTES: u64 = 121_610;

pub(crate) const CMS_PART_D_IPM_FY2024_FINDINGS_RAW_SHA256: &str =
    "36afd362b5e5af4dac098c2502c9c548d79d7aa001725fe92e757bb4cf5e7be8";

pub(crate) const MEDICARE_PART_D_SPONSOR_DOCUMENTATION_DEPENDENCY_CEILING_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_part_d_sponsor_documentation_dependency_evidence_ceiling.fy2024.v1.draft.json";

pub(crate) const MEDICARE_PART_D_SPONSOR_DOCUMENTATION_DEPENDENCY_CEILING_READER_PATH: &str =
    "docs/reading/medicare-part-d-sponsor-documentation-dependency-evidence-ceiling.md";

pub(crate) const MEDICARE_PART_D_SAMPLE_DESIGN_EVIDENCE_CEILING_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_part_d_sample_design_evidence_ceiling.fy2024.v1.draft.json";

pub(crate) const MEDICARE_PART_D_SAMPLE_DESIGN_EVIDENCE_CEILING_READER_PATH: &str =
    "docs/reading/medicare-part-d-sample-design-evidence-ceiling.md";

pub(crate) const MEDICARE_PART_D_ESTIMATION_METHOD_EVIDENCE_CEILING_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_part_d_estimation_method_evidence_ceiling.fy2024.v1.draft.json";

pub(crate) const MEDICARE_PART_D_ESTIMATION_METHOD_EVIDENCE_CEILING_READER_PATH: &str =
    "docs/reading/medicare-part-d-estimation-method-evidence-ceiling.md";

pub(crate) const MEDICARE_PART_D_MISSING_DOCUMENT_EXCLUSION_TREATMENT_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_part_d_missing_document_exclusion_treatment_bridge.fy2024.v1.draft.json";

pub(crate) const MEDICARE_PART_D_MISSING_DOCUMENT_EXCLUSION_TREATMENT_BRIDGE_READER_PATH: &str =
    "docs/reading/medicare-part-d-missing-document-exclusion-treatment-bridge.md";

pub(crate) const MEDICARE_PART_D_PAYMENT_UNIVERSE_MEASUREMENT_OBJECT_DENOMINATOR_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_part_d_payment_universe_measurement_object_denominator_bridge.fy2024.v1.draft.json";

pub(crate) const MEDICARE_PART_D_PAYMENT_UNIVERSE_MEASUREMENT_OBJECT_DENOMINATOR_BRIDGE_READER_PATH: &str =
    "docs/reading/medicare-part-d-payment-universe-measurement-object-denominator-bridge.md";

pub(crate) const MEDICARE_PART_D_AUDIT_CLOSEOUT_RECOVERY_PROCESS_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_part_d_audit_closeout_recovery_process_bridge.q4-2025.v1.draft.json";

pub(crate) const MEDICARE_PART_D_AUDIT_CLOSEOUT_RECOVERY_PROCESS_BRIDGE_READER_PATH: &str =
    "docs/reading/medicare-part-d-audit-closeout-recovery-process-bridge.md";

pub(crate) const MEDICARE_PART_D_PUBLISHED_UNCERTAINTY_OUTPUT_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_part_d_published_uncertainty_output_bridge.fy2024.v1.draft.json";

pub(crate) const MEDICARE_PART_D_PUBLISHED_UNCERTAINTY_OUTPUT_BRIDGE_READER_PATH: &str =
    "docs/reading/medicare-part-d-published-uncertainty-output-bridge.md";

pub(crate) const MEDICARE_PART_D_RECONCILIATION_PDE_ADJUSTMENT_DOCUMENTATION_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_part_d_reconciliation_pde_adjustment_documentation_bridge.cy2022.v1.draft.json";

pub(crate) const MEDICARE_PART_D_RECONCILIATION_PDE_ADJUSTMENT_DOCUMENTATION_BRIDGE_READER_PATH: &str =
    "docs/reading/medicare-part-d-reconciliation-pde-adjustment-documentation-bridge.md";

pub(crate) const MEDICARE_PART_D_SAMPLING_ESTIMATION_PLAN_ACCESS_EVIDENCE_CEILING_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_part_d_sampling_estimation_plan_access_evidence_ceiling.fy2024.v1.draft.json";

pub(crate) const MEDICARE_PART_D_SAMPLING_ESTIMATION_PLAN_ACCESS_EVIDENCE_CEILING_READER_PATH: &str =
    "docs/reading/medicare-part-d-sampling-estimation-plan-access-evidence-ceiling.md";

pub(crate) const MEDICARE_PART_D_SAMPLING_ESTIMATION_METHODOLOGY_PLAN_REQUEST_SPEC_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_part_d_sampling_estimation_methodology_plan_request_specification.fy2024.v1.draft.json";

pub(crate) const MEDICARE_PART_D_SAMPLING_ESTIMATION_METHODOLOGY_PLAN_REQUEST_SPEC_READER_PATH: &str =
    "docs/reading/medicare-part-d-sampling-estimation-methodology-plan-request-specification.md";

pub(crate) const MEDICARE_PART_D_SAMPLING_ESTIMATION_METHODOLOGY_PLAN_REQUEST_TEMPLATE_PATH: &str =
    "docs/requests/medicare-part-d-fy2024-sampling-estimation-methodology-plan-foia-request.md";

pub(crate) const CMS_FOIA_FILING_SOURCE_ID: &str = "SRC-CMS-FOIA-FILING";

pub(crate) const CMS_FOIA_FILING_METADATA_PATH: &str =
    "data/metadata/SRC-CMS-FOIA-FILING.2026-07-14.metadata.md";

pub(crate) const CMS_FOIA_FILING_RAW_PATH: &str =
    "data/raw/cms/SRC-CMS-FOIA-FILING/2026-07-14/cms-foia-filing.html";

pub(crate) const CMS_FOIA_FILING_RAW_BYTES: u64 = 212_862;

pub(crate) const CMS_FOIA_FILING_RAW_SHA256: &str =
    "58c82eda98850caaccf6c1802dcea1bb75f6457530a948dfa5a20eb1204f77bb";

pub(crate) const ECFR_45_CFR_5_SOURCE_ID: &str = "SRC-ECFR-45-CFR-5";

pub(crate) const ECFR_45_CFR_5_METADATA_PATH: &str = "data/metadata/SRC-ECFR-45-CFR-5.2026-07-14.metadata.md";

pub(crate) const ECFR_45_CFR_5_RAW_PATH: &str = "data/raw/ecfr/SRC-ECFR-45-CFR-5/2026-07-14/45-cfr-part-5.pdf";

pub(crate) const ECFR_45_CFR_5_RAW_BYTES: u64 = 249_694;

pub(crate) const ECFR_45_CFR_5_RAW_SHA256: &str =
    "c5c1a301acd0353fb66ea0850ac188a22e0391aa499766cb68137bccb29158f1";

pub(crate) const MEDICARE_PART_D_FOIA_RESPONSE_INTAKE_CONTRACT_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_part_d_sampling_estimation_methodology_plan_foia_response_intake_contract.fy2024.v1.draft.json";

pub(crate) const MEDICARE_PART_D_FOIA_RESPONSE_INTAKE_TEMPLATE_PATH: &str = "data/templates/medicare_part_d_sampling_estimation_methodology_plan_foia_response_intake.v1.template.json";

pub(crate) const MEDICARE_PART_D_FOIA_PREFLIGHT_PATH: &str = "docs/requests/medicare-part-d-fy2024-sampling-estimation-methodology-plan-foia-submission-preflight.md";

pub(crate) const MEDICARE_PART_D_FOIA_RESPONSE_INTAKE_READER_PATH: &str = "docs/reading/medicare-part-d-sampling-estimation-methodology-plan-foia-preflight-response-intake.md";

pub(crate) const PAYMENT_INTEGRITY_FY2024_ANNUAL_EXTRACTION_ROLE_REVIEW_PATH: &str =
    "reviews/2026-07-14-payment-integrity-fy2024-annual-extraction-role-review.md";

pub(crate) const PAYMENT_ACCURACY_FY2024_WORKBOOK_RAW_PATH: &str =
    "data/raw/omb/SRC-OMB-PAYMENTACCURACY/2026-07-12/FY2024_Dataset.xlsx";

pub(crate) const PAYMENT_ACCURACY_FY2024_WORKBOOK_RAW_BYTES: u64 = 700_992;

pub(crate) const PAYMENT_ACCURACY_FY2024_WORKBOOK_RAW_SHA256: &str =
    "595369da4c32965c457543e2695b5738bc131049318537948cd396323391e28c";

pub(crate) const MEDICARE_PART_D_SPONSOR_DOCUMENTATION_DEPENDENCY_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/medicare_part_d_sponsor_documentation_dependency_bridge.fy2024.v1.draft.json";

pub(crate) const MEDICARE_PART_D_SPONSOR_DOCUMENTATION_DEPENDENCY_BRIDGE_READER_PATH: &str =
    "docs/reading/medicare-part-d-sponsor-documentation-dependency-bridge.md";

pub(crate) const CMS_PART_D_IPM_CY2022_SUBMISSION_GUIDE_SOURCE_ID: &str =
    "SRC-CMS-PART-D-IPM-CY2022-SUBMISSION-GUIDE";

pub(crate) const CMS_PART_D_IPM_CY2022_SUBMISSION_GUIDE_METADATA_PATH: &str =
    "data/metadata/SRC-CMS-PART-D-IPM-CY2022-SUBMISSION-GUIDE.2026-07-14.metadata.md";

pub(crate) const CMS_PART_D_IPM_CY2022_SUBMISSION_GUIDE_RAW_PATH: &str = "data/raw/cms/SRC-CMS-PART-D-IPM-CY2022-SUBMISSION-GUIDE/2026-07-14/part-d-ipm-cy22-submission-instruction-guide.pdf";

pub(crate) const CMS_PART_D_IPM_CY2022_SUBMISSION_GUIDE_RAW_BYTES: u64 = 5_962_810;

pub(crate) const CMS_PART_D_IPM_CY2022_SUBMISSION_GUIDE_RAW_SHA256: &str =
    "52a76c9910bb66edd387d127744f864d78e59826bc5ff0162bc81ede428c7199";

pub(crate) const CMS_PART_D_IPM_CY2022_FAQ_SOURCE_ID: &str = "SRC-CMS-PART-D-IPM-CY2022-FAQ";

pub(crate) const CMS_PART_D_IPM_CY2022_FAQ_METADATA_PATH: &str =
    "data/metadata/SRC-CMS-PART-D-IPM-CY2022-FAQ.2026-07-14.metadata.md";

pub(crate) const CMS_PART_D_IPM_CY2022_FAQ_RAW_PATH: &str =
    "data/raw/cms/SRC-CMS-PART-D-IPM-CY2022-FAQ/2026-07-14/part-d-ipm-cy22-faqs.pdf";

pub(crate) const CMS_PART_D_IPM_CY2022_FAQ_RAW_BYTES: u64 = 304_485;

pub(crate) const CMS_PART_D_IPM_CY2022_FAQ_RAW_SHA256: &str =
    "1f3e3fddb8a954a0096810ce24e6ede482938191dcff6c2d3b779727330891d4";

pub(crate) const PAYMENT_ACCURACY_PART_D_Q4_2025_SOURCE_ID: &str = "SRC-OMB-PAYMENTACCURACY-PART-D-Q4-2025";

pub(crate) const PAYMENT_ACCURACY_PART_D_Q4_2025_METADATA_PATH: &str =
    "data/metadata/SRC-OMB-PAYMENTACCURACY-PART-D-Q4-2025.2026-07-14.metadata.md";

pub(crate) const PAYMENT_ACCURACY_PART_D_Q4_2025_RAW_PATH: &str = "data/raw/payment_accuracy/SRC-OMB-PAYMENTACCURACY-PART-D-Q4-2025/2026-07-14/medicare-part-d-q4-2025-scorecard.pdf";

pub(crate) const PAYMENT_ACCURACY_PART_D_Q4_2025_RAW_BYTES: u64 = 215_862;

pub(crate) const PAYMENT_ACCURACY_PART_D_Q4_2025_RAW_SHA256: &str =
    "7a3287ad75cfd5f4a53ac1cbfa8992e9223bb57c363c6794d3959471fbed6097";

pub(crate) const VA_PLTSS_PAYMENT_TYPE_COMPOSITION_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/va_pltss_payment_type_composition_bridge.fy2025.v1.draft.json";

pub(crate) const VA_PLTSS_PAYMENT_TYPE_COMPOSITION_BRIDGE_READER_PATH: &str =
    "docs/reading/va-pltss-payment-type-composition-bridge.md";

pub(crate) const VA_AFR_SECTION_III_FY2025_SOURCE_ID: &str = "SRC-VA-AFR-SECTION-III-FY2025";

pub(crate) const VA_AFR_SECTION_III_FY2025_METADATA_PATH: &str =
    "data/metadata/SRC-VA-AFR-SECTION-III-FY2025.2026-07-14.metadata.md";

pub(crate) const VA_AFR_SECTION_III_FY2025_RAW_PATH: &str =
    "data/raw/va/SRC-VA-AFR-SECTION-III-FY2025/2026-07-14/2025-Section-III-Other-Information.pdf";

pub(crate) const VA_AFR_SECTION_III_FY2025_RAW_BYTES: u64 = 1_204_753;

pub(crate) const VA_AFR_SECTION_III_FY2025_RAW_SHA256: &str =
    "3f3e19818ecf5f59f241d8deac4bc2ffa2e377c7be4daf65666c33d75cb2f7c1";

pub(crate) const VA_PLTSS_DOCUMENTATION_RECOVERABILITY_BOUNDARY_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/va_pltss_documentation_recoverability_boundary.fy2025.v1.draft.json";

pub(crate) const VA_PLTSS_DOCUMENTATION_RECOVERABILITY_BOUNDARY_READER_PATH: &str =
    "docs/reading/va-pltss-documentation-recoverability-boundary.md";

pub(crate) const VA_PLTSS_OIG_VERIFICATION_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/va_pltss_oig_verification.fy2024.v1.draft.json";

pub(crate) const VA_PLTSS_OIG_VERIFICATION_READER_PATH: &str = "docs/reading/va-pltss-oig-verification.md";

pub(crate) const VA_PLTSS_SAME_COHORT_LINEAGE_CEILING_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/va_pltss_same_cohort_debt_collection_lineage_evidence_ceiling.fy2024-q4-2025.v1.draft.json";

pub(crate) const VA_PLTSS_SAME_COHORT_LINEAGE_CEILING_READER_PATH: &str =
    "docs/reading/va-pltss-same-cohort-debt-collection-lineage-evidence-ceiling.md";

pub(crate) const VA_PLTSS_Q4_2025_SOURCE_ID: &str = "SRC-OMB-PAYMENTACCURACY-VA-PLTSS-Q4-2025";

pub(crate) const VA_PLTSS_Q4_2025_RAW_PATH: &str =
    "data/raw/omb/SRC-OMB-PAYMENTACCURACY/2026-07-13/Purchased Long Term Services and Supports.pdf";

pub(crate) const VA_PLTSS_Q4_2025_RAW_BYTES: u64 = 222_751;

pub(crate) const VA_PLTSS_Q4_2025_RAW_SHA256: &str =
    "9f365d7c42bb5ec2b50cc3cba8dfda3a12229ece27e430a234b950af93ca8692";

pub(crate) const VA_OIG_PIIA_FY2024_SOURCE_ID: &str = "SRC-VA-OIG-PIIA-FY2024";

pub(crate) const VA_OIG_PIIA_FY2024_RAW_PATH: &str =
    "data/raw/va-oig/SRC-VA-OIG-PIIA-FY2024/2026-07-13/vaoig-24-03777-113.pdf";

pub(crate) const VA_OIG_PIIA_FY2024_RAW_BYTES: u64 = 1_972_706;

pub(crate) const VA_OIG_PIIA_FY2024_RAW_SHA256: &str =
    "b3010f3635be8dcfca3d6762a33c583c4b91ed0ff05bdb8a709ed193883e6c3b";

pub(crate) const PAYMENT_INTEGRITY_BOUNDED_EXAMPLES_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/payment_integrity_bounded_factual_examples.fy2024.v1.draft.json";

pub(crate) const PAYMENT_INTEGRITY_BOUNDED_EXAMPLES_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/payment_integrity_bounded_factual_examples.schema.md";

pub(crate) const PAYMENT_INTEGRITY_BOUNDED_EXAMPLES_READER_PATH: &str =
    "docs/reading/payment-integrity-bounded-factual-examples.md";

pub(crate) const PAYMENT_INTEGRITY_BOUNDED_EXAMPLES_REVIEW_PATH: &str =
    "reviews/2026-07-14-payment-integrity-bounded-factual-examples-role-review.md";

pub(crate) const VA_PAYMENT_INTEGRITY_POLICY_SOURCE_ID: &str = "SRC-VA-FIN-POLICY-PAYMENT-INTEGRITY";

pub(crate) const VA_PAYMENT_INTEGRITY_POLICY_METADATA_PATH: &str =
    "data/metadata/SRC-VA-FIN-POLICY-PAYMENT-INTEGRITY.2026-07-14.metadata.md";

pub(crate) const VA_PAYMENT_INTEGRITY_POLICY_RAW_PATH: &str = "data/raw/va/SRC-VA-FIN-POLICY-PAYMENT-INTEGRITY/2026-07-14/chapter-03-payment-integrity-and-fraud-reduction.html";

pub(crate) const VA_PAYMENT_INTEGRITY_POLICY_RAW_BYTES: u64 = 249_093;

pub(crate) const VA_PAYMENT_INTEGRITY_POLICY_RAW_SHA256: &str =
    "b8a12f68789c42001f4a3f76ae92858c84ccc3628a2a519fa4819e0b07037d75";

pub(crate) const VA_OVERPAYMENT_AUDIT_POLICY_SOURCE_ID: &str =
    "SRC-VA-FIN-POLICY-OVERPAYMENT-AUDIT-RECOVERIES";

pub(crate) const VA_OVERPAYMENT_AUDIT_POLICY_METADATA_PATH: &str =
    "data/metadata/SRC-VA-FIN-POLICY-OVERPAYMENT-AUDIT-RECOVERIES.2026-07-14.metadata.md";

pub(crate) const VA_OVERPAYMENT_AUDIT_POLICY_RAW_PATH: &str = "data/raw/va/SRC-VA-FIN-POLICY-OVERPAYMENT-AUDIT-RECOVERIES/2026-07-14/chapter-30-overpayment-audit-recoveries.html";

pub(crate) const VA_OVERPAYMENT_AUDIT_POLICY_RAW_BYTES: u64 = 122_908;

pub(crate) const VA_OVERPAYMENT_AUDIT_POLICY_RAW_SHA256: &str =
    "eb3adc8783fc40bd0f15c3c2acec8dfb77bddfdda8938b7c9ef17864fcb87f59";

pub(crate) const K12_FEDERALISM_BRIDGE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/k12_federalism_finance_bridge.fy2024.v1.draft.json";

pub(crate) const K12_FEDERALISM_BRIDGE_READER_PATH: &str = "docs/reading/k12-federalism-finance-bridge.md";

pub(crate) const K12_OUTCOME_BASELINE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/k12_outcome_baseline.naep2024-acgr2021-22.v1.draft.json";

pub(crate) const K12_OUTCOME_BASELINE_READER_PATH: &str = "docs/reading/k12-outcome-baseline.md";

pub(crate) const K12_PISA_PEER_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/k12_pisa2022_peer_comparison.v1.draft.json";

pub(crate) const K12_PISA_PEER_READER_PATH: &str = "docs/reading/k12-pisa-2022-peer-comparison.md";

pub(crate) const GLOBAL_COUNTRY_COMPARISON_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/global_country_comparison_coverage.v1.draft.json";

pub(crate) const GLOBAL_COUNTRY_COMPARISON_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/global_country_comparison_coverage.schema.md";

pub(crate) const GLOBAL_COUNTRY_COMPARISON_READER_PATH: &str =
    "docs/reading/global-country-comparison-coverage.md";

pub(crate) const INTERNATIONAL_COMPARATOR_TARGET_RUBRIC_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/international_comparator_target_rubric.v1.draft.json";

pub(crate) const PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/program_lane_target_cost_contract.v1.draft.json";

pub(crate) const OECD_COFOG_PANEL_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/oecd_cofog_country_panel.data2022.v1.draft.json";

pub(crate) const OECD_COFOG_PANEL_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/oecd_cofog_country_panel.schema.md";

pub(crate) const OECD_COFOG_PANEL_READER_PATH: &str = "docs/reading/oecd-cofog-country-panel-2022.md";

pub(crate) const OECD_COFOG_PANEL_METADATA_PATH: &str =
    "data/metadata/SRC-OECD-COFOG-GLOBAL-PANEL-2022.2026-07-15.metadata.md";

pub(crate) const OECD_COFOG_RAW_PATH: &str =
    "data/raw/oecd/SRC-OECD-COFOG-GLOBAL-PANEL-2022/2026-07-15/oecd-cofog-panel-2022.csv";

pub(crate) const OECD_GDP_RAW_PATH: &str =
    "data/raw/oecd/SRC-OECD-COFOG-GLOBAL-PANEL-2022/2026-07-15/oecd-gdp-panel-2022.csv";

pub(crate) const OECD_COFOG_RAW_SHA256: &str =
    "66d0af19fea30a0390240e6ef558148f83eec9285acb8c8bce75b243c0817fd6";

pub(crate) const OECD_GDP_RAW_SHA256: &str =
    "5ad56a019e9d2a03423604f1c5fe6292c4df73548ac1fb1f36da4613acc960c5";

pub(crate) const HYBRID_COFOG_PANEL_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/hybrid_cofog_country_panel.data2022.v1.draft.json";

pub(crate) const IMF_COFOG_RAW_PATH: &str =
    "data/raw/imf/SRC-IMF-GFS-COFOG-GLOBAL-PANEL-2022/2026-07-15/imf-gfs-cofog-panel-2022.csv";

pub(crate) const IMF_COFOG_RAW_SHA256: &str =
    "11526dcb1a140dc6b211ce3c5a4b24f6b223bdc21b8cce3e79f6591295f80a5b";

pub(crate) const FISCAL_COUNTRY_PANEL_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/fiscal_country_panel.data2022.v1.draft.json";

pub(crate) const IMF_FISCAL_PANEL_RAW_PATH: &str =
    "data/raw/imf/SRC-IMF-FM-OCT2025-FISCAL-PANEL-2022/2026-07-15/imf-fm-oct2025-fiscal-panel.csv";

pub(crate) const IMF_FISCAL_PANEL_RAW_SHA256: &str =
    "704ce9ab5ebe519471e099abf9cd820acdbd344fa84f85e61431d36345ba80b7";

pub(crate) const OECD_TOTAL_TAX_REVENUE_RAW_PATH: &str =
    "data/raw/oecd/SRC-OECD-REVSTATS-PANEL-2022/2026-07-15/oecd-total-tax-revenue-panel-2022.csv";

pub(crate) const OECD_TOTAL_TAX_REVENUE_RAW_SHA256: &str =
    "02f0748155de38c01229679dc60ea1d745cecd33428d9370ecd2022fe871127a";

pub(crate) const OECD_TAX_MIX_RAW_PATH: &str =
    "data/raw/oecd/SRC-OECD-REVSTATS-PANEL-2022/2026-07-15/oecd-tax-mix-panel-2022.csv";

pub(crate) const OECD_TAX_MIX_RAW_SHA256: &str =
    "b1541a72eec61758d68c3c0968b76537646f1fd8508e9deeedb153f9a105f87c";

pub(crate) const OECD_GOV_INTEREST_RAW_PATH: &str = "data/raw/oecd/SRC-OECD-GOV-INTEREST-PANEL-2022/2026-07-15/oecd-general-government-interest-payable-panel-2022.csv";

pub(crate) const OECD_GOV_INTEREST_RAW_SHA256: &str =
    "ec3fb87302ddec5a24bf0c737078477acf73f9c8fdd95c0feee3972388f9fd54";

pub(crate) const OECD_GOV_NET_INTEREST_RAW_PATH: &str = "data/raw/oecd/SRC-OECD-GOV-NET-INTEREST-PANEL-2022/2026-07-15/oecd-general-government-net-interest-panel-2022.csv";

pub(crate) const OECD_GOV_NET_INTEREST_RAW_SHA256: &str =
    "e6dc8722de87a47cc737c0cb5c815cfb6830f54abec59461230447bb12c582f4";

pub(crate) const QPSD_MATURITY_PANEL_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/qpsd_maturity_country_panel.2022q4.v1.draft.json";

pub(crate) const QPSD_SHORT_ORIGINAL_RAW_PATH: &str = "data/raw/world_bank/SRC-WB-IMF-QPSD-MATURITY-PANEL-2022Q4/2026-07-15/short-original-2022q4.json";

pub(crate) const QPSD_SHORT_ORIGINAL_RAW_SHA256: &str =
    "9b5129bad20e691deb2c0a43345e224abed15f352e6d4770fd1b64365b8af9b6";

pub(crate) const QPSD_LONG_DUE_WITHIN_ONE_YEAR_RAW_PATH: &str = "data/raw/world_bank/SRC-WB-IMF-QPSD-MATURITY-PANEL-2022Q4/2026-07-15/long-due-within-one-year-2022q4.json";

pub(crate) const QPSD_LONG_DUE_WITHIN_ONE_YEAR_RAW_SHA256: &str =
    "892805fe0d6fcd19c89912a9a14487348bb488a5fcfeeccf830eaac4a9828630";

pub(crate) const QPSD_LONG_DUE_OVER_ONE_YEAR_RAW_PATH: &str = "data/raw/world_bank/SRC-WB-IMF-QPSD-MATURITY-PANEL-2022Q4/2026-07-15/long-due-over-one-year-2022q4.json";

pub(crate) const QPSD_LONG_DUE_OVER_ONE_YEAR_RAW_SHA256: &str =
    "8c1c1b48c854f4b0744c081a9754f63ca1e4444b02a5c4199b56aa643a4dd603";

pub(crate) const QPSD_LONG_ORIGINAL_TOTAL_RAW_PATH: &str = "data/raw/world_bank/SRC-WB-IMF-QPSD-MATURITY-PANEL-2022Q4/2026-07-15/long-original-total-2022q4.json";

pub(crate) const QPSD_LONG_ORIGINAL_TOTAL_RAW_SHA256: &str =
    "b256bd9d7457f5d0999377d4f02f64e38e91c4e5fed80c8796e0157980e64226";

pub(crate) const SOCX_OLDAGE_FAMILY_PANEL_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/socx_oldage_family_country_panel.data2022.v1.draft.json";

pub(crate) const SOCX_OLDAGE_FAMILY_RAW_PATH: &str = "data/raw/oecd/SRC-OECD-SOCX-OLDAGE-FAMILY-PANEL-2022/2026-07-15/oecd-socx-oldage-family-panel-2022.csv";

pub(crate) const SOCX_OLDAGE_FAMILY_RAW_SHA256: &str =
    "0f138dc4e1dd3424890357cdbf4610645dd1d00bd3848d19509fe24860e8c253";

pub(crate) const AGE_RELATIVE_POVERTY_PANEL_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/age_relative_poverty_country_panel.v1.draft.json";

pub(crate) const IDD_OLD_AGE_POVERTY_RAW_PATH: &str = "data/raw/oecd/SRC-OECD-IDD-AGE-POVERTY-PANELS/2026-07-15/oecd-idd-old-age-poverty-2020-2024.csv";

pub(crate) const IDD_OLD_AGE_POVERTY_RAW_SHA256: &str =
    "910f741388bfa1c35cb9b68ac1588b6d51f08b016555a4209b784a905fb5e351";

pub(crate) const IDD_CHILD_POVERTY_RAW_PATH: &str =
    "data/raw/oecd/SRC-OECD-IDD-AGE-POVERTY-PANELS/2026-07-15/oecd-idd-child-poverty-2020-2021.csv";

pub(crate) const IDD_CHILD_POVERTY_RAW_SHA256: &str =
    "3ec2eaa896abf855ffa75067b99aaac340fbb9061a4f8e0b3f70333c77b33d08";

pub(crate) const CENSUS_P60_287_TABLE_A3_RAW_PATH: &str = "data/raw/census/SRC-CENSUS-P60-287-POVERTY-2024/2026-07-24/tableA3_hist_pov_by_all_and_age.xlsx";

pub(crate) const CENSUS_P60_287_TABLE_A3_RAW_SHA256: &str =
    "a72a881ce64b1d32bacaa35a43a291fb75119503a793195c252d560c253b0ed2";

pub(crate) const CENSUS_P60_287_TABLE_B2_RAW_PATH: &str =
    "data/raw/census/SRC-CENSUS-P60-287-POVERTY-2024/2026-07-24/tableB-2.xlsx";

pub(crate) const CENSUS_P60_287_TABLE_B2_RAW_SHA256: &str =
    "8cdb688380c543c1bd3bc47e2124ec6872511eff8c03c8340b1adacdbd1525fe";

pub(crate) const CENSUS_P60_287_TABLE_B7_RAW_PATH: &str =
    "data/raw/census/SRC-CENSUS-P60-287-POVERTY-2024/2026-07-24/tableB-7.xlsx";

pub(crate) const CENSUS_P60_287_TABLE_B7_RAW_SHA256: &str =
    "ceea883550e7453b3002d90afb4caa7b52612cb9fb24846fe1df424468ca46f7";

pub(crate) const CENSUS_P60_287_INCOME_TO_POVERTY_RAW_PATH: &str =
    "data/raw/census/SRC-CENSUS-P60-287-POVERTY-2024/2026-07-24/Income-to-Poverty-Ratios.xlsx";

pub(crate) const CENSUS_P60_287_INCOME_TO_POVERTY_RAW_SHA256: &str =
    "fb5b9c60b02cef2acc49d1674271839623082fad4ab00395d42d4949de00938f";

pub(crate) const PENSION_REPLACEMENT_PANEL_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/pension_replacement_country_panel.model2024.v1.draft.json";

pub(crate) const PENSION_REPLACEMENT_GROSS_RAW_PATH: &str = "data/raw/oecd/SRC-OECD-PAG-PENSION-REPLACEMENT-PANEL-2024/2026-07-15/oecd-pag-gross-replacement-average-earner-mandatory-2024.csv";

pub(crate) const PENSION_REPLACEMENT_GROSS_RAW_SHA256: &str =
    "ec19bf44a336d484afc67115e07367ec6f601698d94f0034d7dc616a3ccfc85f";

pub(crate) const PENSION_REPLACEMENT_NET_RAW_PATH: &str = "data/raw/oecd/SRC-OECD-PAG-PENSION-REPLACEMENT-PANEL-2024/2026-07-15/oecd-pag-net-replacement-average-earner-mandatory-2024.csv";

pub(crate) const PENSION_REPLACEMENT_NET_RAW_SHA256: &str =
    "cd2f6dba48e44ebea7d7005c000557bef02a5466d233011d6f9fba4d19c30698";

pub(crate) const K12_OECD_RESOURCE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/k12_oecd_resource_comparison.eag2025-data2022.v1.draft.json";

pub(crate) const K12_OECD_RESOURCE_READER_PATH: &str = "docs/reading/k12-oecd-resource-comparison.md";

pub(crate) const CPS_EDUCATION_ACCESS_TRANSITION_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/census_cps_education_access_transition_baseline.oct2024.v1.draft.json";

pub(crate) const CPS_EDUCATION_ACCESS_TRANSITION_READER_PATH: &str =
    "docs/reading/census-cps-education-access-transition-baseline.md";

pub(crate) const WIOA_OUTCOME_BASELINE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/wioa_national_outcome_baseline.py2024.v1.draft.json";

pub(crate) const WIOA_OUTCOME_BASELINE_READER_PATH: &str = "docs/reading/wioa-national-outcome-baseline.md";

pub(crate) const BLS_CPS_WORKER_BASELINE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/bls_cps_worker_baseline.cy2024.v1.draft.json";

pub(crate) const BLS_CPS_WORKER_BASELINE_READER_PATH: &str = "docs/reading/bls-cps-worker-baseline.md";

pub(crate) const TRAINING_EMPLOYMENT_BRIDGE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/training_employment_account_bridge.fy2025.v1.draft.json";

pub(crate) const TRAINING_EMPLOYMENT_BRIDGE_READER_PATH: &str =
    "docs/reading/training-employment-account-bridge.md";

pub(crate) const WIA_GOLD_STANDARD_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/wia_gold_standard_impact_evidence.2011-2013.v1.draft.json";

pub(crate) const WIA_GOLD_STANDARD_READER_PATH: &str = "docs/reading/wia-gold-standard-impact-evidence.md";

pub(crate) const DISASTER_DEPTH_CARD_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/disaster_depth_card.fy2025.v1.draft.json";

pub(crate) const DISASTER_DEPTH_CARD_READER_PATH: &str = "docs/reading/disaster-depth-card.md";

pub(crate) const JUSTICE_DEPTH_CARD_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/justice_depth_card.fy2025.v1.draft.json";

pub(crate) const JUSTICE_DEPTH_CARD_READER_PATH: &str = "docs/reading/justice-depth-card.md";

pub(crate) const SCIENCE_DEPTH_CARD_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/science_energy_environment_depth_card.fy2025.v1.draft.json";

pub(crate) const SCIENCE_DEPTH_CARD_READER_PATH: &str =
    "docs/reading/science-energy-environment-depth-card.md";

pub(crate) const AGRICULTURE_DEPTH_CARD_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/agriculture_depth_card.fy2025.v1.draft.json";

pub(crate) const AGRICULTURE_DEPTH_CARD_READER_PATH: &str = "docs/reading/agriculture-depth-card.md";

pub(crate) const INTERNATIONAL_DEPTH_CARD_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/international_affairs_depth_card.fy2025.v1.draft.json";

pub(crate) const INTERNATIONAL_DEPTH_CARD_READER_PATH: &str =
    "docs/reading/international-affairs-depth-card.md";

pub(crate) const INTERNATIONAL_FINANCIAL_BRIDGE_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/international_financial_programs_account_bridge.fy2025.v1.draft.json";

pub(crate) const INTERNATIONAL_FINANCIAL_BRIDGE_READER_PATH: &str =
    "docs/reading/international-financial-programs-account-bridge.md";

pub(crate) const HEADLINE_BASIS_JSONL_PATH: &str =
    "data/derived/headline_basis_crosswalk/headline_basis_crosswalk.v1.draft.jsonl";

pub(crate) const HEADLINE_BASIS_README_PATH: &str = "data/derived/headline_basis_crosswalk/README.md";

pub(crate) const HEADLINE_BASIS_SCHEMA_PATH: &str =
    "data/derived/headline_basis_crosswalk/headline_basis_crosswalk.schema.md";

pub(crate) const HEADLINE_BASIS_GUIDE_PATH: &str = "docs/reading/headline-number-selection-guide.md";

pub(crate) const EFFICIENCY_PRESSURE_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/efficiency_pressure.fy2025.v1.draft.jsonl";

pub(crate) const EFFICIENCY_PRESSURE_README_PATH: &str = "data/derived/efficiency_pressure/README.md";

pub(crate) const EFFICIENCY_PRESSURE_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/efficiency_pressure.schema.md";

pub(crate) const COST_DOWN_BACKLOG_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_backlog.fy2025.v1.draft.jsonl";

pub(crate) const COST_DOWN_BACKLOG_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_backlog.schema.md";

pub(crate) const COST_DOWN_BACKLOG_READER_PATH: &str = "docs/reading/cost-down-backlog.md";

pub(crate) const COST_DOWN_SOURCE_PACKETS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_source_packets.fy2025.v1.draft.jsonl";

pub(crate) const COST_DOWN_SOURCE_PACKETS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_source_packets.schema.md";

pub(crate) const COST_DOWN_EVIDENCE_QUEUE_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_evidence_queue.fy2025.v1.draft.jsonl";

pub(crate) const COST_DOWN_EVIDENCE_QUEUE_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_evidence_queue.schema.md";

pub(crate) const COST_DOWN_EVIDENCE_QUEUE_READER_PATH: &str = "docs/reading/cost-down-evidence-queue.md";

pub(crate) const COST_DOWN_FIRST_PASS_ROLLUP_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_first_pass_rollup.v1.draft.jsonl";

pub(crate) const COST_DOWN_FIRST_PASS_ROLLUP_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_first_pass_rollup.schema.md";

pub(crate) const COST_DOWN_FIRST_PASS_ROLLUP_READER_PATH: &str = "docs/reading/cost-down-first-pass-rollup.md";

pub(crate) const COST_DOWN_SCORING_READINESS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_scoring_readiness.v1.draft.jsonl";

pub(crate) const COST_DOWN_SCORING_READINESS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_scoring_readiness.schema.md";

pub(crate) const COST_DOWN_SCORING_READINESS_READER_PATH: &str = "docs/reading/cost-down-scoring-readiness.md";

pub(crate) const PAYMENT_INTEGRITY_ELIGIBILITY_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/payment_integrity_eligibility_first_pass.jsonl";

pub(crate) const PAYMENT_INTEGRITY_ELIGIBILITY_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/payment_integrity_eligibility_first_pass.schema.md";

pub(crate) const PAYMENT_INTEGRITY_FIRST_PASS_READER_PATH: &str =
    "docs/reading/payment-integrity-first-pass-extract.md";

pub(crate) const PAYMENT_INTEGRITY_SCORECARDS_Q4_2025_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_scorecards_q4_2025_first_pass.jsonl";

pub(crate) const PAYMENT_INTEGRITY_SCORECARDS_Q4_2025_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_scorecards_q4_2025_first_pass.schema.md";

pub(crate) const PAYMENT_INTEGRITY_SCORECARD_READER_PATH: &str =
    "docs/reading/payment-integrity-scorecard-extract.md";

pub(crate) const PAYMENT_INTEGRITY_PROGRAM_REVIEW_GATE_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_gates_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_PROGRAM_REVIEW_GATE_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_gates_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_PROGRAM_REVIEW_GATE_READER_PATH: &str =
    "docs/reading/payment-integrity-program-review-gates.md";

pub(crate) const PAYMENT_INTEGRITY_PROGRAM_REVIEW_TASKS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_tasks_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_PROGRAM_REVIEW_TASKS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_tasks_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_PROGRAM_REVIEW_TASKS_READER_PATH: &str =
    "docs/reading/payment-integrity-program-review-tasks.md";

pub(crate) const PAYMENT_INTEGRITY_PROGRAM_REVIEW_STATUS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_status_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_PROGRAM_REVIEW_STATUS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_status_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_PROGRAM_REVIEW_STATUS_READER_PATH: &str =
    "docs/reading/payment-integrity-program-review-status.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_PLANS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_plans_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_PLANS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_plans_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_PLANS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-plans.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FIELDS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_fields_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FIELDS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_fields_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FIELDS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-fields.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_TARGETS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_source_targets_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_TARGETS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_source_targets_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_TARGETS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-source-targets.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_QUERIES_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_queries_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_QUERIES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_queries_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_QUERIES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-queries.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_QUERY_RUNS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_query_runs_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_QUERY_RUNS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_query_runs_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_QUERY_RUNS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-query-runs.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_RESULTS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_results_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_RESULTS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_results_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_RESULTS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-results.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_RESULT_REVIEW_READINESS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_result_review_readiness_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_RESULT_REVIEW_READINESS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_result_review_readiness_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_RESULT_REVIEW_READINESS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-result-review-readiness.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FIELD_REVIEWS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_field_reviews_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FIELD_REVIEWS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_field_reviews_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FIELD_REVIEWS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-field-reviews.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_GAP_FOLLOWUPS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_gap_followups_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_GAP_FOLLOWUPS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_gap_followups_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_GAP_FOLLOWUPS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-gap-followups.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_GAP_SOURCE_CAPTURES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_gap_source_captures_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_GAP_SOURCE_CAPTURES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_gap_source_captures_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_GAP_SOURCE_CAPTURES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-gap-source-captures.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_CAPTURE_ROLLUP_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_source_capture_rollup_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_CAPTURE_ROLLUP_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_source_capture_rollup_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_CAPTURE_ROLLUP_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-source-capture-rollup.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_READINESS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_readiness_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_READINESS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_readiness_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_READINESS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-closure-readiness.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_DECISIONS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_decisions_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_DECISIONS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_decisions_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_DECISIONS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-closure-decisions.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_SOURCE_GAPS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_residual_source_gaps_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_SOURCE_GAPS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_residual_source_gaps_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_SOURCE_GAPS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-residual-source-gaps.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_COVERAGE_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_coverage_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_COVERAGE_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_coverage_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_COVERAGE_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-closure-coverage.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_SCORING_GATE_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_scoring_gate_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_SCORING_GATE_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_scoring_gate_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_SCORING_GATE_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-scoring-gate.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_PROGRAM_ROLLUP_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_program_rollup_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_PROGRAM_ROLLUP_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_program_rollup_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_PROGRAM_ROLLUP_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-program-rollup.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_STATUS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_open_program_status_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_STATUS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_open_program_status_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_STATUS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-open-program-status.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_GAP_PRIORITY_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_residual_gap_priority_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_GAP_PRIORITY_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_residual_gap_priority_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_GAP_PRIORITY_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-residual-gap-priority.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_SOURCE_WORK_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_priority_source_work_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_SOURCE_WORK_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_priority_source_work_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_SOURCE_WORK_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-priority-source-work.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_REVIEWER_ACTIONS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_priority_reviewer_actions_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_REVIEWER_ACTIONS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_priority_reviewer_actions_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_REVIEWER_ACTIONS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-priority-reviewer-actions.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FIELD_UPDATES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_field_updates_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FIELD_UPDATES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_field_updates_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FIELD_UPDATES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-field-updates.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERIES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_queries_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERIES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_queries_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERIES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-followup-source-queries.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERY_RUNS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_query_runs_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERY_RUNS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_query_runs_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERY_RUNS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-followup-source-query-runs.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_captures_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_captures_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-followup-source-captures.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURE_ROLLUP_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_capture_rollup_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURE_ROLLUP_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_capture_rollup_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURE_ROLLUP_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-followup-source-capture-rollup.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_DECISIONS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_boundary_decisions_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_DECISIONS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_boundary_decisions_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_DECISIONS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-followup-boundary-decisions.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_READINESS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_boundary_readiness_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_READINESS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_boundary_readiness_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_READINESS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-followup-boundary-readiness.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_CANDIDATES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_narrow_closure_candidates_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_CANDIDATES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_narrow_closure_candidates_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_CANDIDATES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-narrow-closure-candidates.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_DECISIONS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_narrow_closure_decisions_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_DECISIONS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_narrow_closure_decisions_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_DECISIONS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-narrow-closure-decisions.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_COMPONENT_PROGRESS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_open_program_component_progress_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_COMPONENT_PROGRESS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_open_program_component_progress_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_COMPONENT_PROGRESS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-open-program-component-progress.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_REQUIREMENTS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_requirements_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_REQUIREMENTS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_requirements_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_REQUIREMENTS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-requirements.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_TARGETS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_targets_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_TARGETS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_targets_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_TARGETS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-source-targets.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERIES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_queries_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERIES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_queries_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERIES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-source-queries.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERY_RUNS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_query_runs_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERY_RUNS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_query_runs_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERY_RUNS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-source-query-runs.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_captures_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_captures_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-source-captures.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURE_ROLLUPS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_capture_rollups_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURE_ROLLUPS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_capture_rollups_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURE_ROLLUPS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-source-capture-rollups.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_DECISIONS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_boundary_decisions_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_DECISIONS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_boundary_decisions_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_DECISIONS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-boundary-decisions.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_READINESS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_boundary_readiness_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_READINESS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_boundary_readiness_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_READINESS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-boundary-readiness.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_CANDIDATES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_narrow_candidates_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_CANDIDATES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_narrow_candidates_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_CANDIDATES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-narrow-candidates.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_DECISIONS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_narrow_decisions_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_DECISIONS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_narrow_decisions_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_DECISIONS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-narrow-decisions.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-progress.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_REQUIREMENTS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_requirements_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_REQUIREMENTS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_requirements_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_REQUIREMENTS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-progress-requirements.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_TARGETS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_targets_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_TARGETS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_targets_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_TARGETS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-progress-source-targets.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERIES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_queries_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERIES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_queries_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERIES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-progress-source-queries.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERY_RUNS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_query_runs_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERY_RUNS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_query_runs_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERY_RUNS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-progress-source-query-runs.md";

pub(crate) const PAYMENT_INTEGRITY_NEXT_PROGRAM_SELECTION_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_next_program_selection_q4_2025.jsonl";

pub(crate) const PAYMENT_INTEGRITY_NEXT_PROGRAM_SELECTION_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_next_program_selection_q4_2025.schema.md";

pub(crate) const PAYMENT_INTEGRITY_NEXT_PROGRAM_SELECTION_READER_PATH: &str =
    "docs/reading/payment-integrity-next-program-selection.md";

pub(crate) const PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_claims_timeliness_first_pass.jsonl";

pub(crate) const PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_claims_timeliness_first_pass.schema.md";

pub(crate) const PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_READER_PATH: &str =
    "docs/reading/payment-integrity-claims-timeliness-extract.md";

pub(crate) const DEBT_MATURITY_RISK_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/debt_maturity_risk_first_pass.jsonl";

pub(crate) const DEBT_MATURITY_RISK_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/debt_maturity_risk_first_pass.schema.md";

pub(crate) const DEBT_MATURITY_RISK_EXTRACT_READER_PATH: &str = "docs/reading/debt-maturity-risk-extract.md";

pub(crate) const DEBT_PRIMARY_BALANCE_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/debt_primary_balance_first_pass.jsonl";

pub(crate) const DEBT_PRIMARY_BALANCE_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/debt_primary_balance_first_pass.schema.md";

pub(crate) const DEBT_PRIMARY_BALANCE_EXTRACT_READER_PATH: &str =
    "docs/reading/debt-primary-balance-extract.md";

pub(crate) const DISASTER_SUPPLEMENTAL_TRACKING_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/disaster_supplemental_tracking_first_pass.jsonl";

pub(crate) const DISASTER_SUPPLEMENTAL_TRACKING_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/disaster_supplemental_tracking_first_pass.schema.md";

pub(crate) const DISASTER_SUPPLEMENTAL_TRACKING_EXTRACT_READER_PATH: &str =
    "docs/reading/disaster-supplemental-tracking-extract.md";

pub(crate) const DISASTER_MITIGATION_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/disaster_mitigation_first_pass.jsonl";

pub(crate) const DISASTER_MITIGATION_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/disaster_mitigation_first_pass.schema.md";

pub(crate) const DISASTER_MITIGATION_EXTRACT_READER_PATH: &str = "docs/reading/disaster-mitigation-extract.md";

pub(crate) const DEFENSE_AUDIT_CONTROL_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/defense_audit_control_first_pass.jsonl";

pub(crate) const DEFENSE_AUDIT_CONTROL_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/defense_audit_control_first_pass.schema.md";

pub(crate) const DEFENSE_AUDIT_CONTROL_EXTRACT_READER_PATH: &str =
    "docs/reading/defense-audit-control-extract.md";

pub(crate) const DEFENSE_PROCUREMENT_CONTROL_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/defense_procurement_control_first_pass.jsonl";

pub(crate) const DEFENSE_PROCUREMENT_CONTROL_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/defense_procurement_control_first_pass.schema.md";

pub(crate) const DEFENSE_PROCUREMENT_CONTROL_EXTRACT_READER_PATH: &str =
    "docs/reading/defense-procurement-control-extract.md";

pub(crate) const HEALTH_PRICE_DISCIPLINE_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/health_price_discipline_first_pass.jsonl";

pub(crate) const HEALTH_PRICE_DISCIPLINE_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/health_price_discipline_first_pass.schema.md";

pub(crate) const HEALTH_PRICE_DISCIPLINE_EXTRACT_READER_PATH: &str =
    "docs/reading/health-price-discipline-extract.md";

pub(crate) const HEALTH_ADMIN_SIMPLIFICATION_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/health_admin_simplification_first_pass.jsonl";

pub(crate) const HEALTH_ADMIN_SIMPLIFICATION_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/health_admin_simplification_first_pass.schema.md";

pub(crate) const HEALTH_ADMIN_SIMPLIFICATION_EXTRACT_READER_PATH: &str =
    "docs/reading/health-administrative-simplification-extract.md";

pub(crate) const HEALTH_PRICE_DISCIPLINE_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/health-price-discipline-source-packet.md";

pub(crate) const HEALTH_ADMIN_SIMPLIFICATION_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/health-administrative-simplification-source-packet.md";

pub(crate) const DEBT_PRIMARY_BALANCE_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/debt-primary-balance-source-packet.md";

pub(crate) const DEBT_MATURITY_RISK_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/debt-maturity-risk-source-packet.md";

pub(crate) const DEFENSE_PROCUREMENT_CONTROL_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/defense-procurement-control-source-packet.md";

pub(crate) const DEFENSE_AUDIT_CONTROL_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/defense-audit-control-source-packet.md";

pub(crate) const DISASTER_MITIGATION_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/disaster-mitigation-source-packet.md";

pub(crate) const DISASTER_SUPPLEMENTAL_TRACKING_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/disaster-supplemental-tracking-source-packet.md";

pub(crate) const PAYMENT_INTEGRITY_ELIGIBILITY_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/payment-integrity-eligibility-source-packet.md";

pub(crate) const PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/payment-integrity-claims-timeliness-source-packet.md";

pub(crate) const EFFICIENCY_PRESSURE_RESEARCH_PATH: &str =
    "docs/research/2026-06-28-efficiency-pressure-framework.md";

pub(crate) const PER_UNIT_DISPLAY_READINESS_JSONL_PATH: &str =
    "data/derived/denominator_requirements/per_unit_display_readiness.v1.draft.jsonl";

pub(crate) const PER_UNIT_RECEIPT_CARDS_JSONL_PATH: &str =
    "data/derived/denominator_requirements/per_unit_receipt_cards.v1.draft.jsonl";

pub(crate) const PER_UNIT_DISPLAY_READINESS_DASHBOARD_PATH: &str =
    "data/derived/denominator_requirements/per-unit-display-readiness.md";

pub(crate) const PER_UNIT_RECEIPT_CARDS_READER_PATH: &str = "docs/reading/per-unit-receipt-cards.md";

pub(crate) const ACCOUNTABILITY_ARTIFACT_MAP_PATH: &str =
    "data/derived/accountability_evidence/artifact-map.md";

pub(crate) const ACCOUNTABILITY_PUBLIC_BRIEF_PATH: &str = "docs/reading/accountability-public-brief.md";

pub(crate) const README_PATH: &str = "README.md";

pub(crate) const READING_INDEX_PATH: &str = "docs/reading/README.md";

pub(crate) const SOURCE_VERSION_LEDGER_PATH: &str = "docs/sources/source-version-ledger.md";

pub(crate) const OBSERVED_DATE: &str = "2026-06-21";

pub(crate) const MODEL_ID: &str = "individual-income-tax-proportional-outlays-v1";

pub(crate) const SUBFUNCTION_MODEL_ID: &str = "individual-income-tax-proportional-subfunction-outlays-v1";

pub(crate) const TABLE_1_1_PATH: &str = "data/raw/omb/SRC-OMB-HIST-1-1-FY2027/2026-06-21/hist01z1_fy2027.xlsx";

pub(crate) const TABLE_2_1_PATH: &str = "data/raw/omb/SRC-OMB-HIST-2-1-FY2027/2026-06-21/hist02z1_fy2027.xlsx";

pub(crate) const TABLE_2_2_PATH: &str = "data/raw/omb/SRC-OMB-HIST-2-2-FY2027/2026-06-21/hist02z2_fy2027.xlsx";

pub(crate) const TABLE_3_1_PATH: &str = "data/raw/omb/SRC-OMB-HIST-3-1-FY2027/2026-06-21/hist03z1_fy2027.xlsx";

pub(crate) const TABLE_3_2_PATH: &str = "data/raw/omb/SRC-OMB-HIST-3-2-FY2027/2026-06-21/hist03z2_fy2027.xlsx";

pub(crate) const RECEIPT_SHARE_JSONL_PATH: &str =
    "data/extracted/receipt_source/receipt_source.SRC-OMB-HIST-2-2-FY2027.2026-06-21.draft.jsonl";

pub(crate) const RECEIPT_SHARE_PROFILE_PATH: &str = "data/extracted/receipt_source/table-2-2-profile.md";

pub(crate) const OUTLAY_FUNCTION_3_1_JSONL_PATH: &str =
    "data/extracted/outlay_function/outlay_function.SRC-OMB-HIST-3-1-FY2027.2026-06-21.draft.jsonl";

pub(crate) const OUTLAY_FUNCTION_3_1_PROFILE_PATH: &str =
    "data/extracted/outlay_function/table-3-1-profile.md";

pub(crate) const OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_JSONL_PATH: &str = "data/extracted/outlay_function/outlay_function.SRC-OMB-HIST-3-2-FY2027.2026-06-21.national-defense.draft.jsonl";

pub(crate) const OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_PROFILE_PATH: &str =
    "data/extracted/outlay_function/table-3-2-national-defense-profile.md";

pub(crate) const OUTLAY_FUNCTION_3_2_JSONL_PATH: &str =
    "data/extracted/outlay_function/outlay_function.SRC-OMB-HIST-3-2-FY2027.2026-06-21.draft.jsonl";

pub(crate) const OUTLAY_FUNCTION_3_2_PROFILE_PATH: &str =
    "data/extracted/outlay_function/table-3-2-profile.md";

pub(crate) const TABLE_6_1_PATH: &str = "data/raw/omb/SRC-OMB-HIST-6-1-FY2027/2026-06-24/hist06z1_fy2027.xlsx";

pub(crate) const OUTLAY_COMPOSITION_6_1_NATIONAL_DEFENSE_JSONL_PATH: &str = "data/extracted/outlay_composition/outlay_composition.SRC-OMB-HIST-6-1-FY2027.2026-06-24.national-defense-gdp.draft.jsonl";

pub(crate) const OUTLAY_COMPOSITION_6_1_NATIONAL_DEFENSE_PROFILE_PATH: &str =
    "data/extracted/outlay_composition/table-6-1-national-defense-gdp-profile.md";

pub(crate) const OBSERVED_DATE_6_1: &str = "2026-06-24";

pub(crate) const SOURCE_IDS: &[&str] = &[
    "SRC-OMB-HIST-1-1-FY2027",
    "SRC-OMB-HIST-2-1-FY2027",
    "SRC-OMB-HIST-3-1-FY2027",
];

pub(crate) const BROAD_CATEGORIES: &[(&str, &str, i64)] = &[
    ("national-defense", "National Defense", 4),
    ("human-resources", "Human resources", 5),
    ("physical-resources", "Physical resources", 14),
    ("net-interest", "Net interest", 22),
    ("other-functions", "Other functions", 25),
    (
        "undistributed-offsetting-receipts",
        "Undistributed offsetting receipts",
        32,
    ),
];

pub(crate) const ANNUAL_HEADERS: &[&str] = &[
    "fiscal_year",
    "coverage_note",
    "individual_income_tax_receipts_millions",
    "total_outlays_millions",
    "total_receipts_millions",
    "deficit_gap_millions",
    "borrowed_share_percent_of_outlays",
    "income_tax_coverage_percent_of_outlays",
    "allocation_method",
    "legal_allocation_status",
    "actual_or_projection",
    "national_defense_percent",
    "human_resources_percent",
    "physical_resources_percent",
    "net_interest_percent",
    "other_functions_percent",
    "offsetting_receipts_percent",
    "category_percent_sum",
];

pub(crate) const DECADE_HEADERS: &[&str] = &[
    "decade",
    "start_fiscal_year",
    "end_fiscal_year",
    "year_count",
    "coverage_note",
    "cumulative_individual_income_tax_receipts_millions",
    "cumulative_total_outlays_millions",
    "cumulative_total_receipts_millions",
    "cumulative_deficit_gap_millions",
    "borrowed_share_percent_of_outlays",
    "income_tax_coverage_percent_of_outlays",
    "allocation_method",
    "legal_allocation_status",
    "actual_or_projection",
    "national_defense_percent",
    "human_resources_percent",
    "physical_resources_percent",
    "net_interest_percent",
    "other_functions_percent",
    "offsetting_receipts_percent",
    "category_percent_sum",
];

pub(crate) const CATEGORY_FIELDS: &[(&str, &str)] = &[
    ("national-defense", "national_defense_percent"),
    ("human-resources", "human_resources_percent"),
    ("physical-resources", "physical_resources_percent"),
    ("net-interest", "net_interest_percent"),
    ("other-functions", "other_functions_percent"),
    (
        "undistributed-offsetting-receipts",
        "offsetting_receipts_percent",
    ),
];

pub(crate) const SUBFUNCTION_ANNUAL_HEADERS: &[&str] = &[
    "fiscal_year",
    "function_code",
    "function_label",
    "subfunction_code",
    "subfunction_label",
    "individual_income_tax_receipts_millions",
    "total_outlays_millions",
    "subfunction_outlays_millions",
    "modeled_income_tax_allocation_millions",
    "allocation_share_percent",
    "outlay_share_percent",
    "allocation_method",
    "legal_allocation_status",
    "actual_or_projection",
];

pub(crate) const SUBFUNCTION_TOP_HEADERS: &[&str] = &[
    "rank",
    "fiscal_year",
    "function_code",
    "function_label",
    "subfunction_code",
    "subfunction_label",
    "modeled_income_tax_allocation_millions",
    "allocation_share_percent",
    "subfunction_outlays_millions",
    "allocation_method",
    "legal_allocation_status",
];

pub(crate) const SUBFUNCTION_DECADE_HEADERS: &[&str] = &[
    "decade",
    "start_fiscal_year",
    "end_fiscal_year",
    "year_count",
    "coverage_note",
    "function_code",
    "function_label",
    "subfunction_code",
    "subfunction_label",
    "cumulative_individual_income_tax_receipts_millions",
    "cumulative_subfunction_outlays_millions",
    "cumulative_modeled_income_tax_allocation_millions",
    "decade_allocation_share_percent",
    "allocation_method",
    "legal_allocation_status",
    "actual_or_projection",
];

pub(crate) const ARTIFACTS: &[Artifact] = &[
    Artifact {
        path: "README.md",
        role: "Repository overview",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl",
        role: "Canonical annual model rows",
        grain: "fiscal year by broad category",
        kind: "jsonl",
        canonical: "yes",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-summary.jsonl",
        role: "Canonical decade summary rows",
        grain: "decade by broad category",
        kind: "jsonl",
        canonical: "yes",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv",
        role: "Chart-ready annual wide view",
        grain: "fiscal year",
        kind: "csv",
        canonical: "no",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-wide.csv",
        role: "Chart-ready decade wide view",
        grain: "decade",
        kind: "csv",
        canonical: "no",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.draft.jsonl",
        role: "Canonical annual subfunction model rows",
        grain: "fiscal year by Table 3.2 subfunction",
        kind: "jsonl",
        canonical: "yes",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.annual-long.csv",
        role: "Chart-ready annual subfunction long view",
        grain: "fiscal year by Table 3.2 subfunction",
        kind: "csv",
        canonical: "no",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.decade-long.csv",
        role: "Chart-ready decade subfunction long view",
        grain: "decade by Table 3.2 subfunction",
        kind: "csv",
        canonical: "no",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.fy2025-top-subfunctions.csv",
        role: "Chart-ready FY2025 top subfunction view",
        grain: "ranked FY2025 subfunction",
        kind: "csv",
        canonical: "no",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_subfunction_model/README.md",
        role: "Subfunction model method and schema note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/spend_category_map/spend_category_map.fy2025.omb-fy2027-v1.draft.jsonl",
        role: "Top FY2025 spend category question-routing rows",
        grain: "ranked FY2025 OMB subfunction",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/spend_category_map/README.md",
        role: "Spend category map method note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/spend_category_map/spend_category_map.schema.md",
        role: "Spend category map row schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/spend_category_map/accountability-question-handoff.md",
        role: "Spend category accountability question handoff",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/spend_category_map/spend-category-dashboard.md",
        role: "Spend category dashboard",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/breadth_benchmark_matrix.v1.draft.jsonl",
        role: "Breadth, depth, and current-versus-benchmark matrix",
        grain: "fiscal lane metric or explicit coverage gap",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/breadth_benchmark_matrix.schema.md",
        role: "Breadth benchmark matrix schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/README.md",
        role: "Breadth benchmark matrix method note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/lane_full_coverage_matrix.v1.draft.json",
        role: "Lane full coverage matrix",
        grain: "15-lane full-coverage gate status",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/lane-full-coverage-matrix.md",
        role: "Lane full coverage matrix reader",
        grain: "public coverage dashboard",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/public_explainer_wave_c_promotion.v1.draft.json",
        role: "Public explainer Wave C promotion",
        grain: "15-lane public-explainer completion gate",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/public-explainer-wave-c-promotion.md",
        role: "Public explainer Wave C promotion reader",
        grain: "public Wave C completion packet",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/current_law_source_custody_progress_rollup.v1.draft.json",
        role: "Current-law source custody progress rollup",
        grain: "eight-path current-law custody progress",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/current-law-source-custody-progress-rollup.md",
        role: "Current-law source custody progress reader",
        grain: "public current-law custody progress packet",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/cbo_open_data_fy2032_2035_current_law_extension_context.v1.draft.json",
        role: "CBO FY2032-FY2035 current-law extension context",
        grain: "official CBO top-line, revenue, debt, net-interest, and trust-fund context",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/cbo-open-data-fy2032-2035-current-law-extension-context.md",
        role: "CBO FY2032-FY2035 current-law extension context reader",
        grain: "public current-law extension context boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/cbo_major_outlay_category_fy2032_2035_context.v1.draft.json",
        role: "CBO FY2026-FY2035 major outlay category context",
        grain: "official CBO major outlay category context",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/cbo-major-outlay-category-fy2032-2035-context.md",
        role: "CBO FY2026-FY2035 major outlay category context reader",
        grain: "public category context boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/cbo_revenue_detail_fy2026_2035_context.v1.draft.json",
        role: "CBO FY2026-FY2035 revenue detail context",
        grain: "annual receipt-category context boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/cbo-revenue-detail-fy2026-2035-context.md",
        role: "CBO FY2026-FY2035 revenue detail context reader",
        grain: "public receipt-category context boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/cbo_health_insurance_baseline_browser_context_fy2026_2036.v1.draft.json",
        role: "CBO health-insurance baseline browser context",
        grain: "browser-visible CBO health baseline context boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/cbo-health-insurance-baseline-browser-context-fy2026-2036.md",
        role: "CBO health-insurance baseline browser context reader",
        grain: "public CBO health baseline access boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/cbo_health_insurance_table2_browser_rowmap_fy2026_2036.v1.draft.json",
        role: "CBO health-insurance Table 2 browser rowmap",
        grain: "browser-verified CBO Table 2 row assignment boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/cbo-health-insurance-table2-browser-rowmap-fy2026-2036.md",
        role: "CBO health-insurance Table 2 browser rowmap reader",
        grain: "public CBO health rowmap boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/omb_pbd_fy2027_user_guide_horizon_boundary_context.v1.draft.json",
        role: "OMB PBD FY2027 user guide horizon-boundary context",
        grain: "official OMB PBD documentation and horizon boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/omb-pbd-fy2027-user-guide-horizon-boundary-context.md",
        role: "OMB PBD FY2027 user guide horizon-boundary context reader",
        grain: "public OMB PBD file-boundary context",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/irs_soi_pub1304_ty2023_individual_income_base_context.v1.draft.json",
        role: "IRS SOI Pub. 1304 TY2023 individual income base context",
        grain: "official TY2023 individual-income base context boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/irs-soi-pub1304-ty2023-individual-income-base-context.md",
        role: "IRS SOI Pub. 1304 TY2023 individual income base context reader",
        grain: "public individual-income base context boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/irs_soi_corporation_complete_table_2_3_ty2022_corporate_income_base_context.v1.draft.json",
        role: "IRS SOI Publication 16 TY2022 corporate income base context",
        grain: "official TY2022 corporate-income base context boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/irs-soi-corporation-complete-table-2-3-ty2022-corporate-income-base-context.md",
        role: "IRS SOI Publication 16 TY2022 corporate income base context reader",
        grain: "public corporate-income base context boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/receipt_base_rate_bridge_readiness_rollup.v1.draft.json",
        role: "Receipt-base and rate-bridge readiness rollup",
        grain: "receipt-base context and rate bridge blocker summary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/receipt_base_rate_bridge_readiness_rollup.schema.md",
        role: "Receipt-base and rate-bridge readiness rollup schema",
        grain: "receipt-base readiness contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/receipt-base-rate-bridge-readiness-rollup.md",
        role: "Receipt-base and rate-bridge readiness rollup reader",
        grain: "public receipt-base/rate bridge blocker summary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/omb_receipt_category_fy2025_2031_context.v1.draft.json",
        role: "OMB FY2025-FY2031 receipt category context",
        grain: "official fiscal-year receipt-category context boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/omb-receipt-category-fy2025-2031-context.md",
        role: "OMB FY2025-FY2031 receipt category context reader",
        grain: "public receipt-category context boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/omb_receipt_detail_table_2_4_fy2025_2031_context.v1.draft.json",
        role: "OMB Table 2.4 FY2025-FY2031 social insurance and excise receipt detail context",
        grain: "official fiscal-year social-insurance and excise receipt-detail boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/omb-receipt-detail-table-2-4-fy2025-2031-context.md",
        role: "OMB Table 2.4 FY2025-FY2031 receipt detail context reader",
        grain: "public social-insurance and excise receipt-detail boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/omb_receipt_share_table_2_2_fy2025_2031_context.v1.draft.json",
        role: "OMB Table 2.2 FY2025-FY2031 receipt source share context",
        grain: "official fiscal-year receipt-source share boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/omb-receipt-share-table-2-2-fy2025-2031-context.md",
        role: "OMB Table 2.2 FY2025-FY2031 receipt share context reader",
        grain: "public receipt-source share boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/omb_receipt_amount_share_reconciliation_fy2025_2031_context.v1.draft.json",
        role: "OMB FY2025-FY2031 receipt amount/share reconciliation context",
        grain: "official fiscal-year receipt amount/share reconciliation boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/omb-receipt-amount-share-reconciliation-fy2025-2031-context.md",
        role: "OMB FY2025-FY2031 receipt amount/share reconciliation context reader",
        grain: "public receipt amount/share reconciliation boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/omb_cbo_revenue_overlap_reconciliation_fy2026_2031_context.v1.draft.json",
        role: "OMB/CBO revenue overlap reconciliation FY2026-FY2031 context",
        grain: "revenue source-vintage overlap reconciliation boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/omb-cbo-revenue-overlap-reconciliation-fy2026-2031-context.md",
        role: "OMB/CBO revenue overlap reconciliation FY2026-FY2031 reader",
        grain: "public revenue source-vintage overlap reconciliation boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/medicare_hi_cms_omb_fy2025_timing_perimeter_diagnostic.v1.draft.json",
        role: "Medicare HI CMS/OMB FY2025 timing-perimeter diagnostic",
        grain: "Medicare HI calendar/fiscal timing and perimeter boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/medicare-hi-cms-omb-fy2025-timing-perimeter-diagnostic.md",
        role: "Medicare HI CMS/OMB FY2025 timing-perimeter diagnostic reader",
        grain: "public Medicare HI timing and perimeter boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/medicare_hi_treasury_mts_fy2025_trust_fund_anchor_context.v1.draft.json",
        role: "Medicare HI Treasury MTS FY2025 trust-fund anchor context",
        grain: "Medicare HI FY2025 fiscal-year receipt and outlay anchors",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/medicare-hi-treasury-mts-fy2025-trust-fund-anchor-context.md",
        role: "Medicare HI Treasury MTS FY2025 trust-fund anchor reader",
        grain: "public Medicare HI fiscal-year anchor boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/current_law_source_custody_wave_b_closure.v1.draft.json",
        role: "Current-law source custody Wave B closure",
        grain: "Wave B source-custody closure gate",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/current-law-source-custody-wave-b-closure.md",
        role: "Current-law source custody Wave B closure reader",
        grain: "public Wave B closure packet",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/data_acquisition_eight_gap_status.v1.draft.json",
        role: "Eight-gap data acquisition status",
        grain: "official-source acquisition status across the remaining gaps",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/data-acquisition-eight-gap-status.md",
        role: "Eight-gap data acquisition status reader",
        grain: "public data-acquisition status boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/cms_hospital_quality_methodology_surface_context.v1.draft.json",
        role: "CMS hospital quality methodology surface context",
        grain: "CMS/QualityNet methodology surface custody boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/cms-hospital-quality-methodology-surface-context.md",
        role: "CMS hospital quality methodology surface reader",
        grain: "public CMS/QualityNet methodology surface custody boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/cms_hospital_measure_methodology_report_custody.v1.draft.json",
        role: "CMS hospital measure methodology report custody",
        grain: "CMS hospital methodology report custody boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/cms-hospital-measure-methodology-report-custody.md",
        role: "CMS hospital measure methodology report custody reader",
        grain: "public CMS hospital methodology report custody boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/cms_hospital_quality_dataset_field_crosswalk.v1.draft.json",
        role: "CMS hospital quality dataset field crosswalk",
        grain: "CMS hospital quality/access field crosswalk boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/cms-hospital-quality-dataset-field-crosswalk.md",
        role: "CMS hospital quality dataset field crosswalk reader",
        grain: "public CMS hospital quality/access field crosswalk boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/cms_hrsa_rural_safety_net_capacity_context.v1.draft.json",
        role: "CMS/HRSA rural safety-net capacity context",
        grain: "CMS/HRSA rural safety-net capacity boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/cms-hrsa-rural-safety-net-capacity-context.md",
        role: "CMS/HRSA rural safety-net capacity context reader",
        grain: "public CMS/HRSA rural safety-net capacity boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/net_interest_treasury_mspd_maturity_detail_context.v1.draft.json",
        role: "Net-interest Treasury MSPD maturity detail context",
        grain: "Treasury maturity-detail source custody boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/net-interest-treasury-mspd-maturity-detail-context.md",
        role: "Net-interest Treasury MSPD maturity detail reader",
        grain: "public maturity-detail context boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/net_interest_treasury_mspd_remaining_maturity_bucket_diagnostic.v1.draft.json",
        role: "Net-interest Treasury MSPD remaining-maturity bucket diagnostic",
        grain: "Treasury maturity-bucket diagnostic boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/net-interest-treasury-mspd-remaining-maturity-bucket-diagnostic.md",
        role: "Net-interest Treasury MSPD remaining-maturity bucket diagnostic reader",
        grain: "public maturity-bucket diagnostic boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_INTEREST_TREASURY_MSPD_SNAPSHOT_RECONCILIATION_JSON_PATH,
        role: "Net-interest Treasury MSPD snapshot reconciliation",
        grain: "units, overlap, marketable-debt identity, and snapshot seed",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: NET_INTEREST_TREASURY_MSPD_SNAPSHOT_RECONCILIATION_SCHEMA_PATH,
        role: "Net-interest Treasury MSPD snapshot reconciliation schema",
        grain: "snapshot identity and annual-model boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_INTEREST_TREASURY_MSPD_SNAPSHOT_RECONCILIATION_READER_PATH,
        role: "Net-interest Treasury MSPD snapshot reconciliation reader",
        grain: "public snapshot result and remaining rollover gaps",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_INTEREST_TREASURY_MSPD_SNAPSHOT_RECONCILIATION_REVIEW_PATH,
        role: "Net-interest Treasury MSPD snapshot reconciliation role review",
        grain: "source and accounting fixed point",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/net_interest_treasury_average_interest_rate_context.v1.draft.json",
        role: "Net-interest Treasury average-interest-rate context",
        grain: "Treasury average-rate context boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/net-interest-treasury-average-interest-rate-context.md",
        role: "Net-interest Treasury average-interest-rate context reader",
        grain: "public Treasury average-rate context boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/omb_ap13_fund_group_reconciliation_detail_fy2025_context.v1.draft.json",
        role: "OMB AP13 FY2025 fund-group reconciliation detail context",
        grain: "fund-group reconciliation detail boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/omb-ap13-fund-group-reconciliation-detail-fy2025-context.md",
        role: "OMB AP13 FY2025 fund-group reconciliation detail reader",
        grain: "public fund-group reconciliation detail boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/treasury_mts_table_8_federal_fund_fy2025_context.v1.draft.json",
        role: "Treasury MTS Table 8 FY2025 federal-fund context",
        grain: "federal-fund general-fund boundary diagnostic",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/treasury-mts-table-8-federal-fund-fy2025-context.md",
        role: "Treasury MTS Table 8 FY2025 federal-fund context reader",
        grain: "public federal-fund general-fund boundary diagnostic",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/transportation_trust_fund_table_13_4_aggregate_fy2025_2031_context.v1.draft.json",
        role: "Transportation Table 13-4 aggregate context",
        grain: "diagnostic transportation trust-fund aggregate boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/transportation-trust-fund-table-13-4-aggregate-fy2025-2031-context.md",
        role: "Transportation Table 13-4 aggregate context reader",
        grain: "public diagnostic transportation trust-fund aggregate boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/transportation_trust_fund_table_13_4_identity_diagnostic.v1.draft.json",
        role: "Transportation Table 13-4 identity diagnostic",
        grain: "transportation trust-fund internal identity boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/transportation-trust-fund-table-13-4-identity-diagnostic.md",
        role: "Transportation Table 13-4 identity diagnostic reader",
        grain: "public transportation trust-fund internal identity boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/transportation_trust_fund_cbo_balance_extension_fy2032_2035_context.v1.draft.json",
        role: "Transportation CBO FY2032-FY2035 trust-fund balance extension context",
        grain: "diagnostic transportation trust-fund balance extension boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/transportation-trust-fund-cbo-balance-extension-fy2032-2035-context.md",
        role: "Transportation CBO FY2032-FY2035 trust-fund balance extension reader",
        grain: "public transportation balance extension boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/transportation_trust_fund_cross_source_reconciliation_status.v1.draft.json",
        role: "Transportation trust-fund cross-source reconciliation status",
        grain: "transportation OMB/CBO/Treasury reconciliation boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/transportation_trust_fund_cross_source_reconciliation_status.schema.md",
        role: "Transportation trust-fund cross-source reconciliation status schema",
        grain: "transportation reconciliation status contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/transportation-trust-fund-cross-source-reconciliation-status.md",
        role: "Transportation trust-fund cross-source reconciliation status reader",
        grain: "public transportation reconciliation boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/transportation_trust_fund_treasury_mts_fy2025_anchor_context.v1.draft.json",
        role: "Transportation Treasury MTS FY2025 trust-fund anchor context",
        grain: "transportation FY2025 receipt and selected outlay anchor context",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/transportation-trust-fund-treasury-mts-fy2025-anchor-context.md",
        role: "Transportation Treasury MTS FY2025 trust-fund anchor reader",
        grain: "public transportation fiscal-year anchor boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/outcome_floor_wave_d_value_readiness.v1.draft.json",
        role: "Outcome floor Wave D value readiness",
        grain: "15-lane floor-value readiness audit",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/outcome-floor-wave-d-value-readiness.md",
        role: "Outcome floor Wave D value readiness reader",
        grain: "public Wave D blocker packet",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/health_medicare_provider_adequacy_margin_floor_value_packet.v1.draft.json",
        role: "Health/Medicare provider adequacy margin floor value packet",
        grain: "Health/Medicare provider adequacy margin threshold and baseline value packet",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/health_medicare_provider_adequacy_margin_floor_value_packet.schema.md",
        role: "Health/Medicare provider adequacy margin floor value packet schema",
        grain: "Health/Medicare provider adequacy margin floor value packet contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/health-medicare-provider-adequacy-margin-floor-value-packet.md",
        role: "Health/Medicare provider adequacy margin floor value packet reader",
        grain: "public Health/Medicare provider adequacy margin floor value boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/social_security_benefit_adequacy_context_bridge.v1.draft.json",
        role: "Social Security benefit adequacy context bridge",
        grain: "OECD modeled pension replacement-rate context boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/social_security_benefit_adequacy_context_bridge.schema.md",
        role: "Social Security benefit adequacy context bridge schema",
        grain: "Social Security benefit adequacy bridge contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/social-security-benefit-adequacy-context-bridge.md",
        role: "Social Security benefit adequacy context bridge reader",
        grain: "public OECD replacement-rate context boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/social_security_domestic_old_age_poverty_context_bridge.v1.draft.json",
        role: "Social Security domestic old-age poverty context bridge",
        grain: "Census domestic 65-plus poverty context boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/social_security_domestic_old_age_poverty_context_bridge.schema.md",
        role: "Social Security domestic old-age poverty context bridge schema",
        grain: "Social Security domestic old-age poverty bridge contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/social-security-domestic-old-age-poverty-context-bridge.md",
        role: "Social Security domestic old-age poverty context bridge reader",
        grain: "public Census 65-plus poverty context boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/social_security_administration_service_context_bridge.v1.draft.json",
        role: "Social Security administration service context bridge",
        grain: "SSA browser-visible service context boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/social_security_administration_service_context_bridge.schema.md",
        role: "Social Security administration service context bridge schema",
        grain: "Social Security administration service bridge contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/social-security-administration-service-context-bridge.md",
        role: "Social Security administration service context bridge reader",
        grain: "public SSA service context boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/social_security_old_age_poverty_floor_value_packet.v1.draft.json",
        role: "Social Security old-age poverty floor value packet",
        grain: "Social Security old-age poverty threshold and baseline value packet",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/social_security_old_age_poverty_floor_value_packet.schema.md",
        role: "Social Security old-age poverty floor value packet schema",
        grain: "Social Security old-age poverty floor value packet contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/social-security-old-age-poverty-floor-value-packet.md",
        role: "Social Security old-age poverty floor value packet reader",
        grain: "public Social Security old-age poverty floor value boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/education_workforce_graduation_floor_value_packet.v1.draft.json",
        role: "Education/workforce graduation floor value packet",
        grain: "Education/workforce graduation threshold and baseline value packet",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/education_workforce_graduation_floor_value_packet.schema.md",
        role: "Education/workforce graduation floor value packet schema",
        grain: "Education/workforce graduation floor value packet contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/education-workforce-graduation-floor-value-packet.md",
        role: "Education/workforce graduation floor value packet reader",
        grain: "public education/workforce graduation floor value boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/income_security_family_child_poverty_floor_value_packet.v1.draft.json",
        role: "Income-security/family child-poverty floor value packet",
        grain: "Income-security/family child-poverty threshold and baseline value packet",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/income_security_family_child_poverty_floor_value_packet.schema.md",
        role: "Income-security/family child-poverty floor value packet schema",
        grain: "Income-security/family child-poverty floor value packet contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/income-security-family-child-poverty-floor-value-packet.md",
        role: "Income-security/family child-poverty floor value packet reader",
        grain: "public income-security/family child-poverty floor value boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/veterans_claims_backlog_floor_value_packet.v1.draft.json",
        role: "Veterans claims backlog floor value packet",
        grain: "Veterans claims backlog threshold and baseline value packet",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/veterans_claims_backlog_floor_value_packet.schema.md",
        role: "Veterans claims backlog floor value packet schema",
        grain: "Veterans claims backlog floor value packet contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/veterans-claims-backlog-floor-value-packet.md",
        role: "Veterans claims backlog floor value packet reader",
        grain: "public Veterans claims backlog floor value boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/net_interest_average_rate_floor_value_packet.v1.draft.json",
        role: "Net-interest average-rate floor value packet",
        grain: "Net-interest average-rate threshold and baseline value packet",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/net_interest_average_rate_floor_value_packet.schema.md",
        role: "Net-interest average-rate floor value packet schema",
        grain: "Net-interest average-rate floor value packet contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/net-interest-average-rate-floor-value-packet.md",
        role: "Net-interest average-rate floor value packet reader",
        grain: "public net-interest average-rate floor value boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/transportation_roadway_fatality_rate_floor_value_packet.v1.draft.json",
        role: "Transportation roadway fatality-rate floor value packet",
        grain: "Transportation roadway fatality-rate threshold and baseline value packet",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/transportation_roadway_fatality_rate_floor_value_packet.schema.md",
        role: "Transportation roadway fatality-rate floor value packet schema",
        grain: "Transportation roadway fatality-rate floor value packet contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/transportation-roadway-fatality-rate-floor-value-packet.md",
        role: "Transportation roadway fatality-rate floor value packet reader",
        grain: "public transportation roadway safety floor value boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/defense_total_force_suicide_rate_floor_value_packet.v1.draft.json",
        role: "Defense Total Force suicide-rate floor value packet",
        grain: "Defense personnel-safety anchor threshold and baseline",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/defense_total_force_suicide_rate_floor_value_packet.schema.md",
        role: "Defense Total Force suicide-rate floor value schema",
        grain: "Defense personnel-safety anchor contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/defense-total-force-suicide-rate-floor-value-packet.md",
        role: "Defense Total Force suicide-rate floor value reader",
        grain: "public defense personnel-safety anchor boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/disaster_billion_dollar_disaster_deaths_floor_value_packet.v1.draft.json",
        role: "Disaster associated-deaths floor value packet",
        grain: "Disaster life-safety anchor threshold and baseline",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/disaster_billion_dollar_disaster_deaths_floor_value_packet.schema.md",
        role: "Disaster associated-deaths floor value schema",
        grain: "Disaster life-safety anchor contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/disaster-billion-dollar-disaster-deaths-floor-value-packet.md",
        role: "Disaster associated-deaths floor value reader",
        grain: "public disaster life-safety anchor boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/justice_violent_victimization_rate_floor_value_packet.v1.draft.json",
        role: "Justice violent-victimization-rate floor value packet",
        grain: "Justice public-safety anchor threshold and baseline",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/justice_violent_victimization_rate_floor_value_packet.schema.md",
        role: "Justice violent-victimization-rate floor value schema",
        grain: "Justice public-safety anchor contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/justice-violent-victimization-rate-floor-value-packet.md",
        role: "Justice violent-victimization-rate floor value reader",
        grain: "public justice safety anchor boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/science_rd_intensity_floor_value_packet.v1.draft.json",
        role: "Science R&D-intensity floor value packet",
        grain: "Science capacity anchor threshold and baseline",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/science_rd_intensity_floor_value_packet.schema.md",
        role: "Science R&D-intensity floor value schema",
        grain: "Science capacity anchor contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/science-rd-intensity-floor-value-packet.md",
        role: "Science R&D-intensity floor value reader",
        grain: "public science capacity anchor boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/agriculture_debt_to_asset_ratio_floor_value_packet.v1.draft.json",
        role: "Agriculture debt-to-asset-ratio floor value packet",
        grain: "Agriculture resilience anchor threshold and baseline",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/agriculture_debt_to_asset_ratio_floor_value_packet.schema.md",
        role: "Agriculture debt-to-asset-ratio floor value schema",
        grain: "Agriculture resilience anchor contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/agriculture-debt-to-asset-ratio-floor-value-packet.md",
        role: "Agriculture debt-to-asset-ratio floor value reader",
        grain: "public agriculture resilience anchor boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/international_foreign_assistance_geographic_coverage_floor_value_packet.v1.draft.json",
        role: "International foreign-assistance coverage floor value packet",
        grain: "International reporting-coverage anchor threshold and baseline",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/international_foreign_assistance_geographic_coverage_floor_value_packet.schema.md",
        role: "International foreign-assistance coverage floor value schema",
        grain: "International reporting-coverage anchor contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/international-foreign-assistance-geographic-coverage-floor-value-packet.md",
        role: "International foreign-assistance coverage floor value reader",
        grain: "public international reporting-coverage anchor boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/wave_e_reference_scenario_packs.v1.draft.json",
        role: "Wave E reference scenario packs",
        grain: "15-lane current-policy continuation comparator calibrations",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/wave_e_reference_scenario_packs.schema.md",
        role: "Wave E reference scenario pack schema",
        grain: "reference and adverse comparator calibration contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/wave-e-reference-scenario-packs.md",
        role: "Wave E reference scenario packs reader",
        grain: "public reference-calibration and claim boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-07-26-wave-e-reference-scenario-packs-role-review.md",
        role: "Wave E reference scenario packs role review",
        grain: "eight-role reference-calibration approval",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/lane_scenario_pack_wave_e_readiness.v1.draft.json",
        role: "Lane scenario pack Wave E readiness",
        grain: "15-lane policy-scenario readiness audit",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/lane-scenario-pack-wave-e-readiness.md",
        role: "Lane scenario pack Wave E readiness reader",
        grain: "public Wave E reference-calibration completion boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/wave_f_transportation_deterministic_calibration.v1.draft.json",
        role: "Wave F transportation deterministic calibration",
        grain: "FY2025-FY2035 synthetic simulator mechanics fixture",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/wave_f_transportation_deterministic_calibration.schema.md",
        role: "Wave F transportation deterministic calibration schema",
        grain: "simulator calibration completion and claim boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/wave-f-transportation-deterministic-calibration.md",
        role: "Wave F transportation deterministic calibration reader",
        grain: "public calibration-only completion packet",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-07-26-wave-f-transportation-deterministic-calibration-role-review.md",
        role: "Wave F transportation deterministic calibration role review",
        grain: "eight-role mechanical-fixture approval",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/adaptive_rate_corpus_track_plan.v1.draft.json",
        role: "Adaptive-rate CORE and lane-track corpus plan",
        grain: "extensible CORE namespace and TRN-A-F recipe",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/adaptive_rate_corpus_track_plan.schema.md",
        role: "Adaptive-rate CORE and lane-track corpus plan schema",
        grain: "namespace, dependency, and reuse contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/adaptive-rate-corpus-track-plan.md",
        role: "Adaptive-rate CORE and lane-track corpus plan reader",
        grain: "public CORE and transportation recipe",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/core_g_official_current_law_solver_spine.v1.draft.json",
        role: "CORE-G official current-law solver spine",
        grain: "FY2025-FY2035 CBO federal topline path",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/core_g_official_current_law_solver_spine.schema.md",
        role: "CORE-G official current-law solver spine schema",
        grain: "source, fiscal identity, and track-start contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/core-g-official-current-law-solver-spine.md",
        role: "CORE-G official current-law solver spine reader",
        grain: "public CORE-G completion boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-07-26-core-g-official-current-law-solver-spine-role-review.md",
        role: "CORE-G official current-law solver spine role review",
        grain: "eight-role source and accounting approval",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_A_BASELINE_SPINE_JSON_PATH,
        role: "TRN-A transportation baseline and source spine",
        grain: "bounded Function 400 and named-fund source spine",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_A_BASELINE_SPINE_SCHEMA_PATH,
        role: "TRN-A transportation baseline and source spine schema",
        grain: "source admission and non-stitching contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_A_BASELINE_SPINE_READER_PATH,
        role: "TRN-A transportation baseline and source spine reader",
        grain: "public TRN-A completion boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_A_BASELINE_SPINE_ROLE_REVIEW_PATH,
        role: "TRN-A transportation baseline and source spine role review",
        grain: "eight-role baseline and source approval",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_H_ACCOUNTING_SUBSTRATE_JSON_PATH,
        role: "CORE-H shared accounting substrate",
        grain: "fund reserve debt and interest interfaces",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_H_ACCOUNTING_SUBSTRATE_SCHEMA_PATH,
        role: "CORE-H shared accounting substrate schema",
        grain: "checked accounting interface contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_H_ACCOUNTING_SUBSTRATE_READER_PATH,
        role: "CORE-H shared accounting substrate reader",
        grain: "public CORE-H completion boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_H_ACCOUNTING_SUBSTRATE_ROLE_REVIEW_PATH,
        role: "CORE-H shared accounting substrate role review",
        grain: "eight-role accounting engine approval",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_B_START_GATE_JSON_PATH,
        role: "TRN-B transportation accounting start gate",
        grain: "dependency proof and accounting work packages",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_B_START_GATE_SCHEMA_PATH,
        role: "TRN-B transportation accounting start gate schema",
        grain: "dependency and claim-boundary contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_B_START_GATE_READER_PATH,
        role: "TRN-B transportation accounting start gate reader",
        grain: "public TRN-B start boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_B_NAMED_FUND_ADAPTER_JSON_PATH,
        role: "TRN-B named-fund adapter rows",
        grain: "initial FY2025 Highway and Airport/Airway CORE-H rows",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_B_NAMED_FUND_ADAPTER_SCHEMA_PATH,
        role: "TRN-B named-fund adapter row schema",
        grain: "source-preserving accounting adapter contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_B_SOURCE_BRIDGE_JSON_PATH,
        role: "TRN-B source bridge decisions",
        grain: "FY2025 OMB/Treasury and FY2031 OMB/CBO boundaries",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_B_SOURCE_BRIDGE_SCHEMA_PATH,
        role: "TRN-B source bridge decision schema",
        grain: "separate perimeter and non-stitching contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_B_FUNCTION_400_MAPPING_JSON_PATH,
        role: "TRN-B Function 400 mapping",
        grain: "FY2025-FY2031 named-fund perimeter crosswalk",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_B_FUNCTION_400_MAPPING_SCHEMA_PATH,
        role: "TRN-B Function 400 mapping schema",
        grain: "crosswalk identity and residual boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_B_ACCOUNTING_SCHEDULES_JSON_PATH,
        role: "TRN-B accounting schedules",
        grain: "adjustment rounding and explicit null schedules",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_B_ACCOUNTING_SCHEDULES_SCHEMA_PATH,
        role: "TRN-B accounting schedule schema",
        grain: "evidence-supported value and null contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_B_CLOSURE_JSON_PATH,
        role: "TRN-B transportation accounting closure",
        grain: "six-work-package bounded accounting closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_B_CLOSURE_SCHEMA_PATH,
        role: "TRN-B transportation accounting closure schema",
        grain: "reconciliation and track dependency contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_B_CLOSURE_READER_PATH,
        role: "TRN-B transportation accounting closure reader",
        grain: "public bounded accounting closure",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_B_CLOSURE_ROLE_REVIEW_PATH,
        role: "TRN-B transportation accounting closure role review",
        grain: "eight-role closure approval",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_C_START_GATE_JSON_PATH,
        role: "TRN-C real reform start gate",
        grain: "TRN-B dependency and reform work packages",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_C_START_GATE_SCHEMA_PATH,
        role: "TRN-C real reform start gate schema",
        grain: "dependency and reform claim boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_C_START_GATE_READER_PATH,
        role: "TRN-C real reform start gate reader",
        grain: "public TRN-C start boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_C_CANDIDATE_SCREEN_JSON_PATH,
        role: "TRN-C candidate screen",
        grain: "current instrument source screen",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_C_CANDIDATE_SCREEN_SCHEMA_PATH,
        role: "TRN-C candidate screen schema",
        grain: "selection and claim-boundary contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_C_CANDIDATE_SCREEN_READER_PATH,
        role: "TRN-C candidate screen reader",
        grain: "public candidate selection boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_C_SCENARIO_JSON_PATH,
        role: "TRN-C cost-only reform scenario",
        grain: "FY2025-FY2031 conditional federal effects",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_C_SCENARIO_SCHEMA_PATH,
        role: "TRN-C reform scenario schema",
        grain: "bounded score and floor contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_C_SCENARIO_READER_PATH,
        role: "TRN-C reform scenario reader",
        grain: "public cost-only scenario boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_I_REFORM_ADMISSION_JSON_PATH,
        role: "CORE-I shared reform admission contract",
        grain: "reusable conditional reform interfaces",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_I_REFORM_ADMISSION_SCHEMA_PATH,
        role: "CORE-I reform admission schema",
        grain: "shared interface contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_I_REFORM_ADMISSION_READER_PATH,
        role: "CORE-I reform admission reader",
        grain: "public shared interface boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_C_CLOSURE_JSON_PATH,
        role: "TRN-C real reform closure",
        grain: "five-work-package cost-only closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_C_CLOSURE_SCHEMA_PATH,
        role: "TRN-C real reform closure schema",
        grain: "closure and dependency contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_C_CLOSURE_READER_PATH,
        role: "TRN-C real reform closure reader",
        grain: "public bounded closure",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_C_CLOSURE_ROLE_REVIEW_PATH,
        role: "TRN-C real reform role review",
        grain: "eight-role closure approval",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_START_GATE_JSON_PATH,
        role: "TRN-D financing fairness start gate",
        grain: "TRN-C dependency and financing work packages",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_START_GATE_SCHEMA_PATH,
        role: "TRN-D start gate schema",
        grain: "financing dependency contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_START_GATE_READER_PATH,
        role: "TRN-D start gate reader",
        grain: "public financing start boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_LEGAL_PERIMETER_JSON_PATH,
        role: "TRN-D legal financing perimeter decision",
        grain: "FAA appropriation and null fund assignment",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_LEGAL_PERIMETER_SCHEMA_PATH,
        role: "TRN-D legal financing perimeter schema",
        grain: "legal funding-source decision contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_LEGAL_PERIMETER_READER_PATH,
        role: "TRN-D legal financing perimeter reader",
        grain: "public null-assignment boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_ADMIN_BEHAVIOR_JSON_PATH,
        role: "TRN-D administration compliance behavior boundary",
        grain: "scored FAA cost and null burden values",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_ADMIN_BEHAVIOR_SCHEMA_PATH,
        role: "TRN-D administration behavior schema",
        grain: "cost component and null-burden contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_ADMIN_BEHAVIOR_READER_PATH,
        role: "TRN-D administration behavior reader",
        grain: "public burden and savings boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_INCIDENCE_FAIRNESS_JSON_PATH,
        role: "TRN-D incidence distribution fairness boundary",
        grain: "qualitative incidence and null quantitative values",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_INCIDENCE_FAIRNESS_SCHEMA_PATH,
        role: "TRN-D incidence fairness schema",
        grain: "distribution and access protection contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_INCIDENCE_FAIRNESS_READER_PATH,
        role: "TRN-D incidence fairness reader",
        grain: "public qualitative incidence boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_START_GATE_JSON_PATH,
        role: "HLT-A health baseline start gate",
        grain: "CORE-G dependency and health baseline packages",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_START_GATE_SCHEMA_PATH,
        role: "HLT-A start gate schema",
        grain: "health baseline dependency contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_START_GATE_READER_PATH,
        role: "HLT-A start gate reader",
        grain: "public health baseline start boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_PERIMETER_INVENTORY_JSON_PATH,
        role: "HLT-A health perimeter source inventory",
        grain: "five-family bounded source inventory",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_PERIMETER_INVENTORY_SCHEMA_PATH,
        role: "HLT-A health perimeter inventory schema",
        grain: "federal and non-additive context contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_PERIMETER_INVENTORY_READER_PATH,
        role: "HLT-A health perimeter inventory reader",
        grain: "public health inventory boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_BASELINE_JSON_PATH,
        role: "HLT-A federal health baseline path",
        grain: "FY2025-FY2035 exact and null function rows",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_BASELINE_SCHEMA_PATH,
        role: "HLT-A federal health baseline schema",
        grain: "single-vintage bounded horizon contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_BASELINE_READER_PATH,
        role: "HLT-A federal health baseline reader",
        grain: "public exact and null horizon boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_FINANCING_LINEAGE_JSON_PATH,
        role: "HLT-A health financing lineage",
        grain: "HI SMI Medicaid marketplace financing relationships",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_FINANCING_LINEAGE_SCHEMA_PATH,
        role: "HLT-A financing lineage schema",
        grain: "calendar fiscal and non-additive context contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_FINANCING_LINEAGE_READER_PATH,
        role: "HLT-A financing lineage reader",
        grain: "public health financing boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_START_GATE_JSON_PATH,
        role: "EDU-A education baseline start gate",
        grain: "CORE-G dependency and education baseline packages",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_START_GATE_SCHEMA_PATH,
        role: "EDU-A start gate schema",
        grain: "education baseline dependency contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_START_GATE_READER_PATH,
        role: "EDU-A start gate reader",
        grain: "public education baseline start boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_PERIMETER_INVENTORY_JSON_PATH,
        role: "EDU-A education workforce perimeter source inventory",
        grain: "function-500 bounded source inventory",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_PERIMETER_INVENTORY_SCHEMA_PATH,
        role: "EDU-A perimeter inventory schema",
        grain: "credit and federalism boundary contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_PERIMETER_INVENTORY_READER_PATH,
        role: "EDU-A perimeter inventory reader",
        grain: "public education workforce inventory boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_BASELINE_JSON_PATH,
        role: "EDU-A federal education workforce baseline path",
        grain: "FY2025-FY2035 exact and null function rows",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_BASELINE_SCHEMA_PATH,
        role: "EDU-A federal baseline schema",
        grain: "single-vintage credit-sign horizon contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_BASELINE_READER_PATH,
        role: "EDU-A federal baseline reader",
        grain: "public exact and null horizon boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_FINANCING_LINEAGE_JSON_PATH,
        role: "EDU-A education workforce financing lineage",
        grain: "appropriation grant credit and federalism relationships",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_FINANCING_LINEAGE_SCHEMA_PATH,
        role: "EDU-A financing lineage schema",
        grain: "credit sign and non-additive recipient finance contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_FINANCING_LINEAGE_READER_PATH,
        role: "EDU-A financing lineage reader",
        grain: "public education financing boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_INTERACTIONS_BRIDGE_JSON_PATH,
        role: "TRN-D interactions macro fund bridge",
        grain: "appropriation fund behavior macro and interest boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_INTERACTIONS_BRIDGE_SCHEMA_PATH,
        role: "TRN-D interactions bridge schema",
        grain: "bounded interaction contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_INTERACTIONS_BRIDGE_READER_PATH,
        role: "TRN-D interactions bridge reader",
        grain: "public interaction boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_CLOSURE_JSON_PATH,
        role: "TRN-D financing fairness closure",
        grain: "six-package bounded closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_CLOSURE_SCHEMA_PATH,
        role: "TRN-D closure schema",
        grain: "bounded closure contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_CLOSURE_READER_PATH,
        role: "TRN-D closure reader",
        grain: "public closure boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_D_CLOSURE_REVIEW_PATH,
        role: "TRN-D closure role review",
        grain: "eight-role closure decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_E_START_GATE_JSON_PATH,
        role: "TRN-E integrated solver start gate",
        grain: "input-readiness discovery start",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_E_START_GATE_SCHEMA_PATH,
        role: "TRN-E start gate schema",
        grain: "solver discovery dependency contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_E_START_GATE_READER_PATH,
        role: "TRN-E start gate reader",
        grain: "public solver discovery boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_SERVICE_FLOOR_SPINE_JSON_PATH,
        role: "HLT-A service floor source spine",
        grain: "five-class bounded source lineage",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_SERVICE_FLOOR_SPINE_SCHEMA_PATH,
        role: "HLT-A service floor schema",
        grain: "partial custody and source-gap contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_SERVICE_FLOOR_SPINE_READER_PATH,
        role: "HLT-A service floor reader",
        grain: "public health source lineage boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_CLOSURE_JSON_PATH,
        role: "HLT-A health baseline closure",
        grain: "five-package bounded source-spine closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_CLOSURE_SCHEMA_PATH,
        role: "HLT-A closure schema",
        grain: "structural closure contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_CLOSURE_READER_PATH,
        role: "HLT-A closure reader",
        grain: "public bounded health closure",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_A_CLOSURE_REVIEW_PATH,
        role: "HLT-A closure role review",
        grain: "eight-role health closure decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_B_START_GATE_JSON_PATH,
        role: "HLT-B health accounting start gate",
        grain: "component-to-account discovery start",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_B_START_GATE_SCHEMA_PATH,
        role: "HLT-B start gate schema",
        grain: "health accounting dependency contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_B_START_GATE_READER_PATH,
        role: "HLT-B start gate reader",
        grain: "public health accounting boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_SERVICE_FLOOR_SPINE_JSON_PATH,
        role: "EDU-A service floor source spine",
        grain: "five-class bounded education lineage",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_SERVICE_FLOOR_SPINE_SCHEMA_PATH,
        role: "EDU-A service floor schema",
        grain: "federalism cohort and floor contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_SERVICE_FLOOR_SPINE_READER_PATH,
        role: "EDU-A service floor reader",
        grain: "public education source lineage boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_CLOSURE_JSON_PATH,
        role: "EDU-A education workforce closure",
        grain: "five-package bounded source-spine closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_CLOSURE_SCHEMA_PATH,
        role: "EDU-A closure schema",
        grain: "structural closure contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_CLOSURE_READER_PATH,
        role: "EDU-A closure reader",
        grain: "public bounded education closure",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_A_CLOSURE_REVIEW_PATH,
        role: "EDU-A closure role review",
        grain: "eight-role education closure decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_B_START_GATE_JSON_PATH,
        role: "EDU-B education accounting start gate",
        grain: "component-to-account discovery start",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_B_START_GATE_SCHEMA_PATH,
        role: "EDU-B start gate schema",
        grain: "education accounting dependency contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_B_START_GATE_READER_PATH,
        role: "EDU-B start gate reader",
        grain: "public education accounting boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_E_INPUT_READINESS_JSON_PATH,
        role: "TRN-E solver input readiness bridge",
        grain: "eight input evidence-state handoffs",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_E_INPUT_READINESS_SCHEMA_PATH,
        role: "TRN-E input readiness schema",
        grain: "evidence-state and owner contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_E_INPUT_READINESS_READER_PATH,
        role: "TRN-E input readiness reader",
        grain: "public solver-input boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_B_COMPONENT_MAPPING_JSON_PATH,
        role: "HLT-B component account mapping",
        grain: "six health component role handoffs",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_B_COMPONENT_MAPPING_SCHEMA_PATH,
        role: "HLT-B component mapping schema",
        grain: "health account-role contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_B_COMPONENT_MAPPING_READER_PATH,
        role: "HLT-B component mapping reader",
        grain: "public health mapping boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_B_COMPONENT_MAPPING_JSON_PATH,
        role: "EDU-B component account mapping",
        grain: "six education component role handoffs",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_B_COMPONENT_MAPPING_SCHEMA_PATH,
        role: "EDU-B component mapping schema",
        grain: "signed education account-role contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_B_COMPONENT_MAPPING_READER_PATH,
        role: "EDU-B component mapping reader",
        grain: "public education mapping boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_J_CONTRACT_JSON_PATH,
        role: "CORE-J bounded closure handoff contract",
        grain: "completion evidence account-role and handoff interfaces",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_J_CONTRACT_SCHEMA_PATH,
        role: "CORE-J contract schema",
        grain: "shared closure and handoff contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_J_CONTRACT_READER_PATH,
        role: "CORE-J contract reader",
        grain: "public shared-interface boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_J_CLOSURE_JSON_PATH,
        role: "CORE-J bounded closure handoff closure",
        grain: "eight-check shared-interface closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_J_CLOSURE_SCHEMA_PATH,
        role: "CORE-J closure schema",
        grain: "role-reviewed closure contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_J_CLOSURE_READER_PATH,
        role: "CORE-J closure reader",
        grain: "public CORE-J closure boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_J_CLOSURE_REVIEW_PATH,
        role: "CORE-J closure role review",
        grain: "eight-role shared-interface decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_B_CLOSURE_JSON_PATH,
        role: "HLT-B bounded accounting closure",
        grain: "four-package health accounting closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_B_CLOSURE_JSON_PATH,
        role: "EDU-B bounded accounting closure",
        grain: "four-package education accounting closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: OAS_A_SPINE_JSON_PATH,
        role: "OAS-A baseline source spine",
        grain: "five-package Social Security source closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_K_CONTRACT_JSON_PATH,
        role: "CORE-K temporal composite contract",
        grain: "time basis horizon composite and transfer interfaces",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_K_CLOSURE_JSON_PATH,
        role: "CORE-K temporal composite closure",
        grain: "eight-check shared accounting closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: OAS_B_CLOSURE_JSON_PATH,
        role: "OAS-B bounded accounting closure",
        grain: "four-package OASDI fund accounting closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_C_CLOSURE_JSON_PATH,
        role: "HLT-C candidate screen closure",
        grain: "four-package no-reform health screen",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_C_CLOSURE_JSON_PATH,
        role: "EDU-C candidate screen closure",
        grain: "four-package no-reform education screen",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: OAS_C_CLOSURE_JSON_PATH,
        role: "OAS-C candidate screen closure",
        grain: "four-package no-reform OASDI screen",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: STAGE_C_CATCHUP_BUNDLE_JSON_PATH,
        role: "Four-lane stage-C catch-up bundle",
        grain: "four tracks at or beyond bounded C",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: STAGE_C_CATCHUP_SCHEMA_PATH,
        role: "Stage-C catch-up schema",
        grain: "bounded multi-lane catch-up contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: STAGE_C_CATCHUP_READER_PATH,
        role: "Stage-C catch-up reader",
        grain: "public multi-lane catch-up boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: STAGE_C_CATCHUP_REVIEW_PATH,
        role: "Stage-C catch-up role review",
        grain: "eight-role multi-lane closure decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_D_CLOSURE_JSON_PATH,
        role: "HLT-D bounded financing fairness closure",
        grain: "six-package health D closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_D_CLOSURE_JSON_PATH,
        role: "EDU-D bounded financing fairness closure",
        grain: "six-package education D closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: OAS_D_CLOSURE_JSON_PATH,
        role: "OAS-D bounded financing fairness closure",
        grain: "six-package OASDI D closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_E_CLOSURE_JSON_PATH,
        role: "TRN-E bounded solver-gate closure",
        grain: "five-package no-candidate closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: ISF_A_SPINE_JSON_PATH,
        role: "ISF-A baseline source spine",
        grain: "five-package income-security source closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: VET_A_SPINE_JSON_PATH,
        role: "VET-A baseline source spine",
        grain: "five-package veterans source closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: AGR_A_SPINE_JSON_PATH,
        role: "AGR-A baseline source spine",
        grain: "five-package agriculture source closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_L_CONTRACT_JSON_PATH,
        role: "CORE-L cross-lane overlap contract",
        grain: "ownership non-additivity and allocation interfaces",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_L_CLOSURE_JSON_PATH,
        role: "CORE-L cross-lane overlap closure",
        grain: "eight-check shared overlap closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: ISF_B_CLOSURE_JSON_PATH,
        role: "ISF-B bounded accounting closure",
        grain: "four-package income-security accounting closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: VET_B_CLOSURE_JSON_PATH,
        role: "VET-B bounded accounting closure",
        grain: "four-package veterans accounting closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: AGR_B_CLOSURE_JSON_PATH,
        role: "AGR-B bounded accounting closure",
        grain: "four-package agriculture accounting closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: SEVEN_LANE_CATCHUP_JSON_PATH,
        role: "Seven-lane D/B catch-up bundle",
        grain: "seven requested stage minimums",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: MULTI_TRACK_D_B_SCHEMA_PATH,
        role: "Multi-track D/B catch-up schema",
        grain: "bounded multi-lane closure contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: MULTI_TRACK_D_B_READER_PATH,
        role: "Multi-track D/B catch-up reader",
        grain: "public seven-lane boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: MULTI_TRACK_D_B_REVIEW_PATH,
        role: "Multi-track D/B catch-up role review",
        grain: "eight-role seven-lane closure decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: ISF_C_CLOSURE_JSON_PATH,
        role: "ISF-C bounded scenario-admission closure",
        grain: "four-package income-security candidate screen",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: VET_C_CLOSURE_JSON_PATH,
        role: "VET-C bounded scenario-admission closure",
        grain: "four-package veterans candidate screen",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: AGR_C_CLOSURE_JSON_PATH,
        role: "AGR-C bounded scenario-admission closure",
        grain: "four-package agriculture candidate screen",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: THREE_LANE_STAGE_C_BUNDLE_JSON_PATH,
        role: "Three-lane stage-C discovery bundle",
        grain: "ISF VET AGR bounded candidate screens",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: THREE_LANE_STAGE_C_SCHEMA_PATH,
        role: "Three-lane stage-C discovery schema",
        grain: "bounded candidate-screen contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: THREE_LANE_STAGE_C_READER_PATH,
        role: "Three-lane stage-C discovery reader",
        grain: "public bounded-screen boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: THREE_LANE_STAGE_C_REVIEW_PATH,
        role: "Three-lane stage-C discovery role review",
        grain: "eight-role discovery closure",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_STAGE_MATRIX_JSON_PATH,
        role: "Fifteen-lane canonical stage matrix",
        grain: "one bounded stage row per canonical lane",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_STAGE_MATRIX_SCHEMA_PATH,
        role: "Fifteen-lane canonical stage matrix schema",
        grain: "fifteen-lane bounded stage contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_STAGE_MATRIX_READER_PATH,
        role: "Fifteen-lane canonical stage matrix reader",
        grain: "public bounded-stage boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: ISF_D_CLOSURE_JSON_PATH,
        role: "ISF-D bounded financing and fairness closure",
        grain: "six-package income-security bounded D closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: VET_D_CLOSURE_JSON_PATH,
        role: "VET-D bounded financing and fairness closure",
        grain: "six-package veterans bounded D closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: AGR_D_CLOSURE_JSON_PATH,
        role: "AGR-D bounded financing and fairness closure",
        grain: "six-package agriculture bounded D closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: THREE_LANE_STAGE_D_BUNDLE_JSON_PATH,
        role: "Three-lane bounded stage-D bundle",
        grain: "ISF VET AGR bounded D evidence",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: THREE_LANE_STAGE_D_SCHEMA_PATH,
        role: "Three-lane bounded stage-D schema",
        grain: "bounded D closure contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: THREE_LANE_STAGE_D_READER_PATH,
        role: "Three-lane bounded stage-D reader",
        grain: "public bounded D boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: THREE_LANE_STAGE_D_REVIEW_PATH,
        role: "Three-lane bounded stage-D role review",
        grain: "eight-role three-lane closure decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: DEF_D_CLOSURE_JSON_PATH,
        role: "DEF-A through DEF-D bounded stage-chain closure",
        grain: "four-stage national-defense chain",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: DIS_D_CLOSURE_JSON_PATH,
        role: "DIS-A through DIS-D bounded stage-chain closure",
        grain: "four-stage disaster-resilience chain",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: JUS_D_CLOSURE_JSON_PATH,
        role: "JUS-A through JUS-D bounded stage-chain closure",
        grain: "four-stage justice chain",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: SEE_D_CLOSURE_JSON_PATH,
        role: "SEE-A through SEE-D bounded stage-chain closure",
        grain: "four-stage science-energy-environment chain",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: INT_D_CLOSURE_JSON_PATH,
        role: "INT-A through INT-D bounded stage-chain closure",
        grain: "four-stage international-affairs chain",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: PAY_D_CLOSURE_JSON_PATH,
        role: "PAY-A through PAY-D bounded stage-chain closure",
        grain: "four-stage non-additive payment-integrity overlay",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: REV_D_CLOSURE_JSON_PATH,
        role: "REV-A through REV-D bounded stage-chain closure",
        grain: "four-stage non-additive revenue overlay",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: NET_D_CLOSURE_JSON_PATH,
        role: "NET-A through NET-D bounded stage-chain closure",
        grain: "four-stage endogenous net-interest chain",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: EIGHT_LANE_A_D_BUNDLE_JSON_PATH,
        role: "Eight-lane A-through-D bounded bundle",
        grain: "eight new bounded stage chains",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: EIGHT_LANE_A_D_SCHEMA_PATH,
        role: "Eight-lane A-through-D bounded schema",
        grain: "eight-lane stage-chain contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EIGHT_LANE_A_D_READER_PATH,
        role: "Eight-lane A-through-D bounded reader",
        grain: "public eight-lane stage boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EIGHT_LANE_A_D_REVIEW_PATH,
        role: "Eight-lane A-through-D bounded role review",
        grain: "eight-role stage-chain decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_D_PORTFOLIO_JSON_PATH,
        role: "Fifteen-lane bounded stage-D portfolio closure",
        grain: "one reviewed completion row per canonical lane",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_D_PORTFOLIO_SCHEMA_PATH,
        role: "Fifteen-lane bounded stage-D portfolio schema",
        grain: "portfolio completion audit contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_D_PORTFOLIO_READER_PATH,
        role: "Fifteen-lane bounded stage-D portfolio reader",
        grain: "public portfolio completion boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_D_PORTFOLIO_REVIEW_PATH,
        role: "Fifteen-lane bounded stage-D portfolio role review",
        grain: "eight-role portfolio closure decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: LANE_E_CONTRACT_JSON_PATH,
        role: "Canonical lane-E bounded selection and solver-gate contract",
        grain: "five-package lane-E closure rule",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: LANE_E_CONTRACT_SCHEMA_PATH,
        role: "Canonical lane-E contract schema",
        grain: "bounded E dependency and output contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: LANE_E_CONTRACT_READER_PATH,
        role: "Canonical lane-E contract reader",
        grain: "public bounded E boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HLT_E_CLOSURE_JSON_PATH,
        role: "HLT-E bounded selection and solver-gate closure",
        grain: "five-package health E closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: EDU_E_CLOSURE_JSON_PATH,
        role: "EDU-E bounded selection and solver-gate closure",
        grain: "five-package education-workforce E closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: OAS_E_CLOSURE_JSON_PATH,
        role: "OAS-E bounded selection and solver-gate closure",
        grain: "five-package Social Security E closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: ISF_E_CLOSURE_JSON_PATH,
        role: "ISF-E bounded selection and solver-gate closure",
        grain: "five-package income-security E closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: VET_E_CLOSURE_JSON_PATH,
        role: "VET-E bounded selection and solver-gate closure",
        grain: "five-package veterans E closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: AGR_E_CLOSURE_JSON_PATH,
        role: "AGR-E bounded selection and solver-gate closure",
        grain: "five-package agriculture E closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: DEF_E_CLOSURE_JSON_PATH,
        role: "DEF-E bounded selection and solver-gate closure",
        grain: "five-package defense E closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: DIS_E_CLOSURE_JSON_PATH,
        role: "DIS-E bounded selection and solver-gate closure",
        grain: "five-package disaster E closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: JUS_E_CLOSURE_JSON_PATH,
        role: "JUS-E bounded selection and solver-gate closure",
        grain: "five-package justice E closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: SEE_E_CLOSURE_JSON_PATH,
        role: "SEE-E bounded selection and solver-gate closure",
        grain: "five-package science-energy-environment E closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: INT_E_CLOSURE_JSON_PATH,
        role: "INT-E bounded selection and solver-gate closure",
        grain: "five-package international-affairs E closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: PAY_E_CLOSURE_JSON_PATH,
        role: "PAY-E bounded selection and solver-gate closure",
        grain: "five-package non-additive overlay E closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: REV_E_CLOSURE_JSON_PATH,
        role: "REV-E bounded selection and solver-gate closure",
        grain: "five-package non-additive revenue E closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: NET_E_CLOSURE_JSON_PATH,
        role: "NET-E bounded selection and solver-gate closure",
        grain: "five-package endogenous interest E closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: FOURTEEN_LANE_E_BUNDLE_JSON_PATH,
        role: "Fourteen-lane bounded stage-E bundle",
        grain: "fourteen D-to-E closure rows",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: FOURTEEN_LANE_E_BUNDLE_SCHEMA_PATH,
        role: "Fourteen-lane bounded stage-E schema",
        grain: "fourteen-lane evidence contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: FOURTEEN_LANE_E_BUNDLE_READER_PATH,
        role: "Fourteen-lane bounded stage-E reader",
        grain: "public D-to-E boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: FOURTEEN_LANE_E_REVIEW_PATH,
        role: "Fourteen-lane bounded stage-E role review",
        grain: "eight-role E closure decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_E_PORTFOLIO_JSON_PATH,
        role: "Fifteen-lane bounded stage-E portfolio closure",
        grain: "one reviewed E row per canonical lane",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_E_PORTFOLIO_SCHEMA_PATH,
        role: "Fifteen-lane bounded stage-E portfolio schema",
        grain: "portfolio E completion audit",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_E_PORTFOLIO_READER_PATH,
        role: "Fifteen-lane bounded stage-E portfolio reader",
        grain: "public portfolio E boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_E_PORTFOLIO_REVIEW_PATH,
        role: "Fifteen-lane bounded stage-E portfolio role review",
        grain: "eight-role portfolio E decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: LANE_F_CONTRACT_JSON_PATH,
        role: "Canonical lane-F public-release gate contract",
        grain: "ten-gate output-ready release rule",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: LANE_F_CONTRACT_SCHEMA_PATH,
        role: "Canonical lane-F public-release gate schema",
        grain: "stage-F start and output boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: LANE_F_CONTRACT_READER_PATH,
        role: "Canonical lane-F public-release gate reader",
        grain: "public stage-F boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_F_READINESS_JSON_PATH,
        role: "Fifteen-lane stage-F start-readiness audit",
        grain: "one blocked F-start row per canonical lane",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_F_READINESS_SCHEMA_PATH,
        role: "Fifteen-lane stage-F start-readiness schema",
        grain: "portfolio F-start audit contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_F_READINESS_READER_PATH,
        role: "Fifteen-lane stage-F start-readiness reader",
        grain: "public blocked-start boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_JSON_PATH,
        role: "Fifteen-lane two-level F advancement queue",
        grain: "candidate-input and E-rerun work per lane",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_SCHEMA_PATH,
        role: "Fifteen-lane two-level F advancement queue schema",
        grain: "portfolio work-sequencing contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_READER_PATH,
        role: "Fifteen-lane two-level F advancement queue reader",
        grain: "public advancement sequence",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_REVIEW_PATH,
        role: "Fifteen-lane two-level F advancement role review",
        grain: "eight-role sequencing decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_LEVEL_1_CORE_LESSONS_JSON_PATH,
        role: "TRN Level-1 CORE lessons audit",
        grain: "existing CORE coverage and reusable candidate-release gaps",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_LEVEL_1_CORE_LESSONS_SCHEMA_PATH,
        role: "TRN Level-1 CORE lessons audit schema",
        grain: "candidate-profile discovery boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_LEVEL_1_CORE_LESSONS_READER_PATH,
        role: "TRN Level-1 CORE lessons reader",
        grain: "public CORE-M recommendation boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_LEVEL_1_CORE_LESSONS_REVIEW_PATH,
        role: "TRN Level-1 CORE lessons role review",
        grain: "eight-role CORE-M discovery recommendation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_M_CONTRACT_JSON_PATH,
        role: "CORE-M candidate dossier and typed release contract",
        grain: "candidate-profile and output-eligibility interfaces",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_M_CONTRACT_SCHEMA_PATH,
        role: "CORE-M candidate dossier and typed release schema",
        grain: "profile, financing, gate, and output contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_M_CONTRACT_READER_PATH,
        role: "CORE-M candidate dossier and typed release reader",
        grain: "public shared-interface boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_M_CONTRACT_REVIEW_PATH,
        role: "CORE-M candidate dossier and typed release role review",
        grain: "eight-role shared-interface decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_M_CLOSURE_JSON_PATH,
        role: "CORE-M candidate dossier and typed release closure",
        grain: "validated shared-interface completion",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_LEVEL_1_DOSSIER_JSON_PATH,
        role: "TRN Level-1 H.R. 2247 candidate dossier",
        grain: "CORE-M cost-only candidate selection",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_LEVEL_1_DOSSIER_SCHEMA_PATH,
        role: "TRN Level-1 H.R. 2247 candidate dossier schema",
        grain: "profile-specific selection contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_LEVEL_1_DOSSIER_READER_PATH,
        role: "TRN Level-1 H.R. 2247 candidate dossier reader",
        grain: "public cost-only selection boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_LEVEL_1_DOSSIER_REVIEW_PATH,
        role: "TRN Level-1 H.R. 2247 candidate dossier role review",
        grain: "eight-role profile-specific selection decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_LEVEL_2_E_RERUN_JSON_PATH,
        role: "TRN Level-2 H.R. 2247 output-ready E rerun",
        grain: "typed cost-only E closure and F-start decision",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_LEVEL_2_E_RERUN_SCHEMA_PATH,
        role: "TRN Level-2 H.R. 2247 output-ready E rerun schema",
        grain: "profile-specific output-admission contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_LEVEL_2_E_RERUN_READER_PATH,
        role: "TRN Level-2 H.R. 2247 output-ready E rerun reader",
        grain: "public cost-only output boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_LEVEL_2_E_RERUN_REVIEW_PATH,
        role: "TRN Level-2 H.R. 2247 output-ready E rerun role review",
        grain: "eight-role output admission decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_N_CONTRACT_JSON_PATH,
        role: "CORE-N typed public-release surface contract",
        grain: "five-surface shared release interface",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_N_CONTRACT_SCHEMA_PATH,
        role: "CORE-N typed public-release surface schema",
        grain: "profile-to-surface contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_N_CONTRACT_READER_PATH,
        role: "CORE-N typed public-release surface reader",
        grain: "public cross-surface boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_N_CONTRACT_REVIEW_PATH,
        role: "CORE-N typed public-release surface role review",
        grain: "eight-role shared release decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: CORE_N_CLOSURE_JSON_PATH,
        role: "CORE-N typed public-release surface closure",
        grain: "validated shared release completion",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_F_COST_NOTE_JSON_PATH,
        role: "TRN-F H.R. 2247 cost note",
        grain: "typed public cost-only release",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_F_COST_NOTE_SCHEMA_PATH,
        role: "TRN-F H.R. 2247 cost-note schema",
        grain: "cost-note release contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_F_COST_NOTE_READER_PATH,
        role: "TRN-F H.R. 2247 cost-note reader",
        grain: "public cost-note boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: TRN_F_COST_NOTE_REVIEW_PATH,
        role: "TRN-F H.R. 2247 cost-note role review",
        grain: "eight-role public cost-note decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_START_JSON_PATH,
        role: "REV Level-1 individual-income rate-candidate start",
        grain: "real instrument context and matched-base blockers",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_START_SCHEMA_PATH,
        role: "REV Level-1 individual-income rate-candidate start schema",
        grain: "rate-bearing probe start contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_START_READER_PATH,
        role: "REV Level-1 individual-income rate-candidate start reader",
        grain: "public unmatched-base boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_START_REVIEW_PATH,
        role: "REV Level-1 individual-income rate-candidate start role review",
        grain: "eight-role rate-probe start decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_BASE_PERIMETER_JSON_PATH,
        role: "REV Level-1 individual-income legal/economic base perimeter",
        grain: "source-custodied definition-only base boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_BASE_PERIMETER_SCHEMA_PATH,
        role: "REV Level-1 legal/economic base perimeter schema",
        grain: "typed base separation contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_BASE_PERIMETER_READER_PATH,
        role: "REV Level-1 legal/economic base perimeter reader",
        grain: "public definition-only boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_BASE_PERIMETER_REVIEW_PATH,
        role: "REV Level-1 legal/economic base perimeter role review",
        grain: "eight-role definition decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_TIMING_BRIDGE_JSON_PATH,
        role: "REV Level-1 individual-income tax/fiscal timing bridge",
        grain: "bounded tax-year to fiscal-year reblock",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_TIMING_BRIDGE_SCHEMA_PATH,
        role: "REV Level-1 tax/fiscal timing bridge schema",
        grain: "cohort bridge diagnostic contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_TIMING_BRIDGE_READER_PATH,
        role: "REV Level-1 tax/fiscal timing bridge reader",
        grain: "public timing mismatch boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_TIMING_BRIDGE_REVIEW_PATH,
        role: "REV Level-1 tax/fiscal timing bridge role review",
        grain: "eight-role bounded reblock decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_RATE_LADDER_JSON_PATH,
        role: "REV Level-1 individual-income rate planning ladder",
        grain: "spending and rate sensitivity rows",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_RATE_LADDER_SCHEMA_PATH,
        role: "REV Level-1 rate planning ladder schema",
        grain: "planning-rate claim boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_RATE_LADDER_READER_PATH,
        role: "REV Level-1 rate planning ladder reader",
        grain: "public planning sensitivity boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_RATE_LADDER_REVIEW_PATH,
        role: "REV Level-1 rate planning ladder role review",
        grain: "eight-role planning-rate decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_POST_2025_PROXY_JSON_PATH,
        role: "REV Level-1 post-2025 rate-rescore proxy",
        grain: "post-law all-bracket and top-four schedules",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_POST_2025_PROXY_SCHEMA_PATH,
        role: "REV Level-1 post-2025 rate-rescore proxy schema",
        grain: "proxy calculation claim boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_POST_2025_PROXY_READER_PATH,
        role: "REV Level-1 post-2025 rate-rescore proxy reader",
        grain: "public numerical proxy boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: REV_LEVEL_1_POST_2025_PROXY_REVIEW_PATH,
        role: "REV Level-1 post-2025 rate-rescore proxy role review",
        grain: "eight-role numerical proxy decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: GOVINFO_HR2137_REPORTED_BILL_PDF_PATH,
        role: "Official H.R. 2137 reported bill source",
        grain: "GovInfo reported bill PDF",
        kind: "text",
        canonical: "source",
    },
    Artifact {
        path: GOVINFO_HR2137_REPORTED_BILL_HTML_PATH,
        role: "Official H.R. 2137 reported bill source",
        grain: "GovInfo reported bill HTML",
        kind: "text",
        canonical: "source",
    },
    Artifact {
        path: GOVINFO_HR2137_METADATA_PATH,
        role: "Official H.R. 2137 reported bill metadata",
        grain: "GovInfo custody and claim boundary",
        kind: "markdown",
        canonical: "source",
    },
    Artifact {
        path: MULTI_TRACK_FRONTIER_JSON_PATH,
        role: "Adaptive-rate multi-track frontier",
        grain: "parallel TRN-D HLT-A EDU-A execution",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: MULTI_TRACK_FRONTIER_SCHEMA_PATH,
        role: "Adaptive-rate multi-track frontier schema",
        grain: "parallel dependency contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: MULTI_TRACK_FRONTIER_READER_PATH,
        role: "Adaptive-rate multi-track frontier reader",
        grain: "public parallel execution boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HR2247_BILL_SOURCE_PATH,
        role: "Official H.R. 2247 operative-text source capture",
        grain: "verbatim GPO legislative excerpt",
        kind: "text",
        canonical: "source",
    },
    Artifact {
        path: HR2247_SCORE_SOURCE_PATH,
        role: "Official H.R. 2247 CBO score source capture",
        grain: "verbatim GPO House-report excerpt",
        kind: "text",
        canonical: "source",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/adaptive_rate_post_f_wave_roadmap.v1.draft.json",
        role: "Adaptive-rate post-F Wave G-L roadmap",
        grain: "six-wave dependency and blocker allocation plan",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/adaptive_rate_post_f_wave_roadmap.schema.md",
        role: "Adaptive-rate post-F Wave G-L roadmap schema",
        grain: "roadmap dependency and claim-boundary contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/adaptive-rate-post-f-wave-roadmap.md",
        role: "Adaptive-rate post-F Wave G-L roadmap reader",
        grain: "public six-wave implementation sequence",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/wave_g_official_current_law_solver_spine_contract.v1.draft.json",
        role: "Wave G official current-law solver spine contract",
        grain: "FY2025-FY2035 federal topline admission contract",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/wave_g_official_current_law_solver_spine_contract.schema.md",
        role: "Wave G official current-law solver spine contract schema",
        grain: "eight-gate current-law spine completion boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/wave-g-official-current-law-solver-spine-contract.md",
        role: "Wave G official current-law solver spine contract reader",
        grain: "public Wave G definition and non-claim boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/solver_rate_wave_f_readiness.v1.draft.json",
        role: "Solver and rate Wave F readiness",
        grain: "calibration completion and substantive solver/rate blocker audit",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/solver-rate-wave-f-readiness.md",
        role: "Solver and rate Wave F readiness reader",
        grain: "public Wave F calibration completion boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/current-versus-benchmark-scoreboard.md",
        role: "Current-versus-benchmark public scoreboard",
        grain: "public comparison packet",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/health_cost_decomposition.v1.draft.json",
        role: "Health cost diagnostic decomposition",
        grain: "cross-country price, volume, administration, context, and outcome signals",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/health-cost-decomposition.md",
        role: "Public health cost decomposition card",
        grain: "public diagnostic depth card",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/health_service_price_volume_bridge.cy2024.v1.draft.json",
        role: "Health service price-volume bridge",
        grain: "CY2024 service-category spending, price, and residual non-price growth",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/health-service-price-volume-bridge.md",
        role: "Public health service price-volume bridge",
        grain: "public service-category diagnostic card",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/health_category_benchmark_ladder.v1.draft.json",
        role: "Health category benchmark ladder",
        grain: "hospital, physician, and retail-drug benchmark readiness",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/health-category-benchmark-ladder.md",
        role: "Public health category benchmark ladder",
        grain: "public category benchmark-readiness card",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/health_target_admissibility.v1.draft.json",
        role: "Health target admissibility gate",
        grain: "hospital and professional Medicare-relative adequacy and access gate",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/health-target-admissibility.md",
        role: "Public health target admissibility card",
        grain: "public adequacy and access decision card",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/health_medicare_relative_scenarios.v1.draft.json",
        role: "Health Medicare-relative scenario paths",
        grain: "hospital and professional low, central, and high policy scenarios",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/health-medicare-relative-scenarios.md",
        role: "Public health Medicare-relative scenario card",
        grain: "public rate-path and scoring-gate card",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/health_commercial_sample_sensitivity.v1.draft.json",
        role: "Health commercial sample scenario sensitivity",
        grain: "matched analytical-volume Medicare-relative sensitivity",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/health-commercial-sample-sensitivity.md",
        role: "Public health commercial sample sensitivity card",
        grain: "public matched-sample arithmetic and boundary card",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/health_national_phi_sensitivity.v1.draft.json",
        role: "Health national private-insurance payer sensitivity",
        grain: "CY2024 national payer-payment scenario sensitivity",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/health-national-phi-sensitivity.md",
        role: "Public health national private-insurance sensitivity card",
        grain: "public national payer sensitivity and boundary card",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/fiscal_path_scenarios.v1.draft.json",
        role: "Fiscal primary-balance path scenarios",
        grain: "CBO 2036 baseline and adjustment equivalents",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/fiscal-path-scenarios.md",
        role: "Public fiscal path scenario card",
        grain: "public primary-balance and debt-boundary card",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/fiscal_debt_dynamics_2026_2036.v1.draft.json",
        role: "Fiscal annual debt dynamics scenarios",
        grain: "CBO 2026-2036 baseline and first-order debt paths",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/fiscal-debt-dynamics.md",
        role: "Public fiscal debt dynamics card",
        grain: "annual baseline, scenario results, and scoring boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/fiscal_policy_scale_baskets.v1.draft.json",
        role: "Fiscal policy scale baskets",
        grain: "CBO option magnitudes compared with TAXLANE adjustment paths",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/fiscal-policy-scale-baskets.md",
        role: "Public fiscal policy scale card",
        grain: "policy magnitude, arithmetic baskets, and non-additivity boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/fiscal_policy_distribution_screen.v1.draft.json",
        role: "Fiscal policy distribution screen",
        grain: "incidence channels, exposed groups, and protection gates",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/fiscal-policy-distribution-screen.md",
        role: "Public fiscal policy distribution card",
        grain: "burden channels and joint distribution-score boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "experiments/README.md",
        role: "Experiment family index",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "experiments/annual-budget-ballot/README.md",
        role: "Annual budget ballot experiment method",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "experiments/annual-budget-ballot/ballot.schema.md",
        role: "Annual budget ballot contract",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "experiments/annual-budget-ballot/config.v1.json",
        role: "Annual budget ballot synthetic configuration",
        grain: "experiment configuration",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "experiments/annual-budget-ballot/simulate.py",
        role: "Annual budget ballot simulation runner",
        grain: "reproducible experiment script",
        kind: "text",
        canonical: "supporting",
    },
    Artifact {
        path: "experiments/annual-budget-ballot/outputs/synthetic-run.v1.json",
        role: "Annual budget ballot synthetic output",
        grain: "state and national allocation vectors",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "experiments/annual-budget-ballot/outputs/synthetic-run.v1.md",
        role: "Annual budget ballot synthetic reader",
        grain: "public synthetic experiment summary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "experiments/annual-budget-ballot/config.v2.json",
        role: "Diverse annual budget ballot configuration",
        grain: "personality, noise, polarization, and uncertainty assumptions",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "experiments/annual-budget-ballot/simulate_v2.py",
        role: "Diverse annual budget ballot runner",
        grain: "reproducible diversity stress-test script",
        kind: "text",
        canonical: "supporting",
    },
    Artifact {
        path: "experiments/annual-budget-ballot/outputs/diverse-run.v2.json",
        role: "Diverse annual budget ballot output",
        grain: "state, national, and uncertainty allocation vectors",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "experiments/annual-budget-ballot/outputs/diverse-run.v2.md",
        role: "Diverse annual budget ballot reader",
        grain: "public diversity stress-test summary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/veterans_depth_card.fy2025.v1.draft.json",
        role: "Veterans FY2025 component depth card",
        grain: "federal function and subfunction components",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/veterans-depth-card.md",
        role: "Public veterans breadth/depth card",
        grain: "public fiscal depth card",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/transportation_depth_card.fy2025.v1.draft.json",
        role: "Transportation FY2025 component depth card",
        grain: "federal function and subfunction components",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/transportation-depth-card.md",
        role: "Public transportation breadth/depth card",
        grain: "public fiscal depth card",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/education_depth_card.fy2025.v1.draft.json",
        role: "Education-work-social-services FY2025 depth card",
        grain: "federal function and net subfunction components",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/education-depth-card.md",
        role: "Public education-work-social-services depth card",
        grain: "public fiscal depth card",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/higher_education_account_bridge.fy2025.v1.draft.json",
        role: "Higher-education FY2025 account bridge",
        grain: "federal account entries within subfunction 502",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/higher-education-account-bridge.md",
        role: "Public higher-education account bridge",
        grain: "public fiscal account reconciliation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/pell_short_training_impact_evidence.2012-2021.v1.draft.json",
        role: "Pell short-training randomized impact evidence",
        grain: "experimental Pell offer, education, and labor outcome contrasts",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/pell-short-training-impact-evidence.md",
        role: "Public Pell short-training impact evidence",
        grain: "public experimental Pell offer impact evidence",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/fsa_title_iv_student_access_baseline.fy2024.v1.draft.json",
        role: "FY2024 FSA Title IV student access baseline",
        grain: "administrative program disbursement and qualifier-bounded access measures",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/fsa-title-iv-student-access-baseline.md",
        role: "Public FSA Title IV student access baseline",
        grain: "public administrative program access and disbursement context",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/pell_bachelor_recipient_outcome_baseline.bb2016-2020.v1.draft.json",
        role: "Pell bachelor-recipient longitudinal outcome baseline",
        grain: "historical bachelor-completer outcomes by lifetime Pell receipt",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/pell-bachelor-recipient-outcome-baseline.md",
        role: "Public Pell bachelor-recipient outcome baseline",
        grain: "public completion-conditioned longitudinal outcome context",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/bps_first_time_student_longitudinal_bridge.ay2019-2022.v1.draft.json",
        role: "BPS first-time-student longitudinal bridge",
        grain: "current first-time-student three-year attainment and persistence context",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/bps-first-time-student-longitudinal-bridge.md",
        role: "Public BPS first-time-student longitudinal bridge",
        grain: "public early longitudinal attainment and persistence context",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/pell_current_entrant_persistence_baseline.bps2020-2022.v1.draft.json",
        role: "Pell current-entrant persistence baseline",
        grain: "entry-year Pell receipt by three-year attainment and persistence",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/pell-current-entrant-persistence-baseline.md",
        role: "Public Pell current-entrant persistence baseline",
        grain: "public descriptive Pell receipt-group outcome context",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/pell_current_entrant_persistence_significance_screen.bps2020-2022.v1.draft.json",
        role: "Pell current-entrant persistence significance screen",
        grain: "independent-estimates receipt-group outcome comparisons",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/pell-current-entrant-persistence-significance-screen.md",
        role: "Public Pell current-entrant persistence significance screen",
        grain: "public independent-estimates test context and boundaries",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/federal_crop_insurance_payment_integrity_bridge.fy2024-q4-2025.v1.draft.json",
        role: "Federal Crop Insurance payment-integrity bridge",
        grain: "FY2024 annual, scorecard, and RMA review-period reconciliation",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/federal-crop-insurance-payment-integrity-bridge.md",
        role: "Public Federal Crop Insurance payment-integrity bridge",
        grain: "public payment-type, period, and claim-boundary context",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/federal_crop_insurance_root_cause_definition_bridge.fy2024.v1.draft.json",
        role: "Federal Crop Insurance root-cause definition bridge",
        grain: "FY2024 FCIC data-access root-cause definitions",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/federal-crop-insurance-root-cause-definition-bridge.md",
        role: "Public Federal Crop Insurance root-cause definition bridge",
        grain: "FY2024 internal methodology closure and claim boundaries",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/federal_crop_insurance_payment_universe_bridge.fy2024.v1.draft.json",
        role: "Federal Crop Insurance payment-universe bridge",
        grain: "FY2024 FCIC included payment-category universe",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/federal-crop-insurance-payment-universe-bridge.md",
        role: "Public Federal Crop Insurance payment-universe bridge",
        grain: "FY2024 internal methodology closure and claim boundaries",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/federal_crop_insurance_sample_design_component_bridge.fy2024.v1.draft.json",
        role: "Federal Crop Insurance sample-design component bridge",
        grain: "FY2024 FCIC disclosed sampling governance and design attributes",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/federal-crop-insurance-sample-design-component-bridge.md",
        role: "Public Federal Crop Insurance sample-design component bridge",
        grain: "FY2024 narrow component closure and full-field boundaries",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/federal_crop_insurance_historical_sampling_method_bridge.fy2020.v1.draft.json",
        role: "Federal Crop Insurance historical sampling-method bridge",
        grain: "FY2020/RY2018 disclosed selection method and current-continuity boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/federal-crop-insurance-historical-sampling-method-bridge.md",
        role: "Public Federal Crop Insurance historical sampling-method bridge",
        grain: "historical component closure and current-field boundaries",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/federal_crop_insurance_public_methodology_evidence_ceiling.fy2025.v1.draft.json",
        role: "Federal Crop Insurance public methodology evidence ceiling",
        grain: "secure-plan boundary and FY2025 public-description continuity",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/federal-crop-insurance-public-methodology-evidence-ceiling.md",
        role: "Public Federal Crop Insurance methodology evidence ceiling",
        grain: "zero-closure access boundary and current-field residuals",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/federal_crop_insurance_recovery_lineage_boundary_bridge.fy2024.v1.draft.json",
        role: "Federal Crop Insurance recovery-lineage boundary bridge",
        grain: "FY2024 sampled-case disposition and amount-class non-additivity",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/federal-crop-insurance-recovery-lineage-boundary-bridge.md",
        role: "Public Federal Crop Insurance recovery-lineage boundary bridge",
        grain: "same-period disposition component and recovery residuals",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/federal_crop_insurance_appeal_collectibility_governance_bridge.fy2024.v1.draft.json",
        role: "Federal Crop Insurance appeal and collectibility governance bridge",
        grain: "FY2024 Final Finding dispute and contractual remedy states",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/federal-crop-insurance-appeal-collectibility-governance-bridge.md",
        role: "Public Federal Crop Insurance appeal and collectibility governance bridge",
        grain: "post-Finding state transitions and collectibility boundaries",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/federal_crop_insurance_public_cohort_outcome_evidence_ceiling.fy2024.v1.draft.json",
        role: "Federal Crop Insurance public cohort-outcome evidence ceiling",
        grain: "post-FY2024 cohort transition and unavailable outcome lineage",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/federal-crop-insurance-public-cohort-outcome-evidence-ceiling.md",
        role: "Public Federal Crop Insurance cohort-outcome evidence ceiling",
        grain: "later-cohort reporting boundary and authorized-export next gate",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/federal_crop_insurance_cohort_disposition_request_specification.fy2024.v1.draft.json",
        role: "Federal Crop Insurance cohort-disposition request specification",
        grain: "existing-records request fields, privacy boundary, and submission gate",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/federal-crop-insurance-cohort-disposition-request-specification.md",
        role: "Public Federal Crop Insurance cohort-disposition request specification",
        grain: "request rationale, privacy boundary, and unsent status",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/requests/federal-crop-insurance-fy2024-cohort-disposition-foia-request.md",
        role: "Unsent Federal Crop Insurance FY2024 cohort-disposition FOIA request",
        grain: "owner-completed request text with fee and identity placeholders",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/federal_crop_insurance_foia_response_intake_contract.v1.draft.json",
        role: "Federal Crop Insurance FOIA preflight and response-intake contract",
        grain: "owner submission gate, administrative lifecycle, and evidence firewall",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/templates/federal_crop_insurance_foia_response_intake.v1.template.json",
        role: "Federal Crop Insurance blank FOIA response-intake template",
        grain: "request custody, agency correspondence, production, and appeal state",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/requests/federal-crop-insurance-foia-submission-preflight.md",
        role: "Federal Crop Insurance FOIA owner submission preflight",
        grain: "identity, fee, scope, privacy, authorization, and outbound custody gates",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/federal-crop-insurance-foia-preflight-response-intake.md",
        role: "Public Federal Crop Insurance FOIA preflight and response-intake reader",
        grain: "administrative state machine and interpretation boundaries",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/medicare_part_d_payment_type_composition_bridge.fy2024.v1.draft.json",
        role: "Medicare Part D payment-type composition bridge",
        grain: "FY2024 same-period exact category reconciliation",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/medicare-part-d-payment-type-composition-bridge.md",
        role: "Public Medicare Part D payment-type composition bridge",
        grain: "category reconciliation and debt/recovery claim firewall",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/medicare_part_d_sponsor_documentation_dependency_evidence_ceiling.fy2024.v1.draft.json",
        role: "Medicare Part D sponsor-documentation dependency evidence ceiling",
        grain: "FY2024 scorecard correction and methodology-field reframing boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/medicare-part-d-sponsor-documentation-dependency-evidence-ceiling.md",
        role: "Public Medicare Part D sponsor-documentation dependency evidence ceiling",
        grain: "scorecard reconciliation, field reframing, and claim firewall",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/medicare_part_d_sample_design_evidence_ceiling.fy2024.v1.draft.json",
        role: "Medicare Part D sample-design evidence ceiling",
        grain: "FY2024 sampled-unit support and unreproducible-design boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/medicare-part-d-sample-design-evidence-ceiling.md",
        role: "Public Medicare Part D sample-design evidence ceiling",
        grain: "supported sampling components, exact residuals, and claim firewall",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/medicare_part_d_estimation_method_evidence_ceiling.fy2024.v1.draft.json",
        role: "Medicare Part D estimation-method evidence ceiling",
        grain: "FY2024 web-verified process observations, custody blocker, and estimator residuals",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/medicare-part-d-estimation-method-evidence-ceiling.md",
        role: "Public Medicare Part D estimation-method evidence ceiling",
        grain: "custody retry contract, estimator residuals, and claim firewall",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/medicare_part_d_missing_document_exclusion_treatment_bridge.fy2024.v1.draft.json",
        role: "Medicare Part D missing-document exclusion-treatment component bridge",
        grain: "CY2022 missing-document review, fail treatment, cure boundary, and historical contrast",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/medicare-part-d-missing-document-exclusion-treatment-bridge.md",
        role: "Public Medicare Part D missing-document exclusion-treatment component bridge",
        grain: "bounded component closure, full-field residuals, and claim firewall",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/medicare_part_d_payment_universe_measurement_object_denominator_bridge.fy2024.v1.draft.json",
        role: "Medicare Part D payment-universe measurement-object and denominator bridge",
        grain: "CY2022 PDE/GDC measurement object, FY2024 published denominator, and full-universe boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/medicare-part-d-payment-universe-measurement-object-denominator-bridge.md",
        role: "Public Medicare Part D payment-universe measurement-object and denominator bridge",
        grain: "bounded component closure, denominator reconciliation, residuals, and claim firewall",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/medicare_part_d_audit_closeout_recovery_process_bridge.q4-2025.v1.draft.json",
        role: "Medicare Part D audit-closeout recovery-process bridge",
        grain: "Q4 2025 issued and planned national-audit PDE-deletion pathway and recoverability boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/medicare-part-d-audit-closeout-recovery-process-bridge.md",
        role: "Public Medicare Part D audit-closeout recovery-process bridge",
        grain: "current-process component, period and amount firewall, residuals, and claim boundaries",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/medicare_part_d_published_uncertainty_output_bridge.fy2024.v1.draft.json",
        role: "Medicare Part D published uncertainty-output bridge",
        grain: "FY2024 confidence interval, annual margin-of-error output, and estimator boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/medicare-part-d-published-uncertainty-output-bridge.md",
        role: "Public Medicare Part D published uncertainty-output bridge",
        grain: "published bounds, margin-of-error boundary, estimator residuals, and claim firewall",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/medicare_part_d_reconciliation_pde_adjustment_documentation_bridge.cy2022.v1.draft.json",
        role: "Medicare Part D reconciliation-PDE adjustment-documentation bridge",
        grain: "CY2022 two-track post-reconciliation adjustment documentation treatment and payment-universe boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/medicare-part-d-reconciliation-pde-adjustment-documentation-bridge.md",
        role: "Public Medicare Part D reconciliation-PDE adjustment-documentation bridge",
        grain: "two-track documentation component, prior-pulse overlap, residuals, and claim firewall",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/medicare_part_d_sampling_estimation_plan_access_evidence_ceiling.fy2024.v1.draft.json",
        role: "Medicare Part D sampling-and-estimation-plan access evidence ceiling",
        grain: "FY2024 governmentwide secure-MAX location, reviewed public inventory, exact acquisition target, and zero-closure boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/medicare-part-d-sampling-estimation-plan-access-evidence-ceiling.md",
        role: "Public Medicare Part D sampling-and-estimation-plan access evidence ceiling",
        grain: "controlled plan location, public evidence ceiling, acquisition target, and claim firewall",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/medicare_part_d_sampling_estimation_methodology_plan_request_specification.fy2024.v1.draft.json",
        role: "Medicare Part D sampling-and-estimation methodology-plan request specification",
        grain: "FY2024 existing-records target, privacy boundary, preflight, and no-outbound gate",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/medicare-part-d-sampling-estimation-methodology-plan-request-specification.md",
        role: "Public Medicare Part D methodology-plan request-specification reader",
        grain: "unsent target, filing route, custody boundary, and zero-closure firewall",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/requests/medicare-part-d-fy2024-sampling-estimation-methodology-plan-foia-request.md",
        role: "Unsent Medicare Part D FY2024 methodology-plan FOIA request",
        grain: "existing-record request language, exclusions, owner placeholders, and internal preflight",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/medicare_part_d_sampling_estimation_methodology_plan_foia_response_intake_contract.fy2024.v1.draft.json",
        role: "Medicare Part D methodology-plan FOIA preflight and response-intake contract",
        grain: "hard owner gate, closed-world administrative lifecycle, and evidence firewall",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/templates/medicare_part_d_sampling_estimation_methodology_plan_foia_response_intake.v1.template.json",
        role: "Blank Medicare Part D methodology-plan FOIA response-intake template",
        grain: "draft-only null state, empty event history, and closed review gates",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/requests/medicare-part-d-fy2024-sampling-estimation-methodology-plan-foia-submission-preflight.md",
        role: "Medicare Part D methodology-plan FOIA owner submission preflight",
        grain: "identity, fee, scope, channel, freeze, checksum, and stop-condition checklist",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/medicare-part-d-sampling-estimation-methodology-plan-foia-preflight-response-intake.md",
        role: "Public Medicare Part D methodology-plan FOIA preflight and response-intake reader",
        grain: "administrative timing, state transitions, blank intake, and no-evidence boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/medicare_part_d_sponsor_documentation_dependency_bridge.fy2024.v1.draft.json",
        role: "Medicare Part D sponsor-documentation dependency bridge",
        grain: "CY2022 submission, correction, fail-treatment, and successor-responsibility closure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/medicare-part-d-sponsor-documentation-dependency-bridge.md",
        role: "Public Medicare Part D sponsor-documentation dependency bridge",
        grain: "same-period documentation treatment and claim firewall",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/va_pltss_payment_type_composition_bridge.fy2025.v1.draft.json",
        role: "VA PLTSS payment-type composition bridge",
        grain: "FY2024 same-period composition with later FY2025 taxonomy corroboration",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/va-pltss-payment-type-composition-bridge.md",
        role: "Public VA PLTSS payment-type composition bridge",
        grain: "category reconciliation, period boundary, and recoverability firewall",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/va_pltss_documentation_recoverability_boundary.fy2025.v1.draft.json",
        role: "VA PLTSS documentation and recoverability policy boundary",
        grain: "current VA classification rules and PLTSS-specific mapping gap",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/va-pltss-documentation-recoverability-boundary.md",
        role: "Public VA PLTSS documentation and recoverability policy boundary",
        grain: "classification, certified-return, and program disposition firewall",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: VA_PLTSS_OIG_VERIFICATION_JSON_PATH,
        role: "VA PLTSS independent PIIA verification record",
        grain: "FY2024 statistical review and public recovery-lineage boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: VA_PLTSS_OIG_VERIFICATION_READER_PATH,
        role: "Public VA PLTSS independent verification reader",
        grain: "methodology verification and transaction-lineage firewall",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: VA_PLTSS_SAME_COHORT_LINEAGE_CEILING_JSON_PATH,
        role: "VA PLTSS same-cohort debt and collection lineage evidence ceiling",
        grain: "FY2024 estimate, Q4 2025 recovery rows, bounded custody, and zero closures",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: VA_PLTSS_SAME_COHORT_LINEAGE_CEILING_READER_PATH,
        role: "Public VA PLTSS same-cohort lineage evidence-ceiling reader",
        grain: "source roles, negative-evidence boundary, and claim firewall",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: PAYMENT_INTEGRITY_BOUNDED_EXAMPLES_JSON_PATH,
        role: "Payment-integrity bounded factual examples surface",
        grain: "FY2024 headline, four program cards, and seven source-labeled examples",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: PAYMENT_INTEGRITY_BOUNDED_EXAMPLES_SCHEMA_PATH,
        role: "Payment-integrity bounded factual examples schema",
        grain: "presentation contract, precision rules, and claim gates",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: PAYMENT_INTEGRITY_BOUNDED_EXAMPLES_READER_PATH,
        role: "Public payment-integrity bounded factual examples reader",
        grain: "government-wide headline and four evidence-bounded program cards",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: PAYMENT_INTEGRITY_BOUNDED_EXAMPLES_REVIEW_PATH,
        role: "Five-lens payment-integrity bounded examples review",
        grain: "AI-simulated source, accounting, beneficiary, taxpayer, and skeptic review",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/k12_federalism_finance_bridge.fy2024.v1.draft.json",
        role: "K-12 FY2024 federalism finance bridge",
        grain: "national public school-system revenue and expenditure",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/k12-federalism-finance-bridge.md",
        role: "Public K-12 federalism finance bridge",
        grain: "public school-system fiscal scope",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/k12_outcome_baseline.naep2024-acgr2021-22.v1.draft.json",
        role: "K-12 national outcome baseline",
        grain: "public-school achievement and completion distributions",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/k12-outcome-baseline.md",
        role: "Public K-12 outcome baseline",
        grain: "public achievement and completion baseline",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/k12_pisa2022_peer_comparison.v1.draft.json",
        role: "K-12 PISA 2022 peer comparison",
        grain: "matched international student outcome comparison",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/k12-pisa-2022-peer-comparison.md",
        role: "Public K-12 PISA peer comparison",
        grain: "public matched outcome comparison",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/k12_oecd_resource_comparison.eag2025-data2022.v1.draft.json",
        role: "K-12 OECD resource comparison",
        grain: "matched international education-resource comparison",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/k12-oecd-resource-comparison.md",
        role: "Public K-12 OECD resource comparison",
        grain: "public matched resource comparison",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/census_cps_education_access_transition_baseline.oct2024.v1.draft.json",
        role: "CPS October 2024 education access and transition baseline",
        grain: "national age enrollment and recent-graduate transition estimates",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/census-cps-education-access-transition-baseline.md",
        role: "Public CPS education access and transition baseline",
        grain: "public age enrollment and recent-graduate transition context",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/wioa_national_outcome_baseline.py2024.v1.draft.json",
        role: "WIOA PY2024 national outcome baseline",
        grain: "national program participant and exit-cohort outcomes",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/wioa-national-outcome-baseline.md",
        role: "Public WIOA national outcome baseline",
        grain: "public descriptive worker-program outcome baseline",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/bls_cps_worker_baseline.cy2024.v1.draft.json",
        role: "BLS CPS CY2024 worker baseline",
        grain: "population employment and full-time worker earnings by education",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/bls-cps-worker-baseline.md",
        role: "Public BLS CPS worker baseline",
        grain: "public population employment and earnings context",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/training_employment_account_bridge.fy2025.v1.draft.json",
        role: "Training and employment FY2025 account bridge",
        grain: "federal account reconciliation and program budget-activity mapping",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/training-employment-account-bridge.md",
        role: "Public training and employment account bridge",
        grain: "public fiscal account and outcome-allocation boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/wia_gold_standard_impact_evidence.2011-2013.v1.draft.json",
        role: "WIA Gold Standard randomized impact evidence",
        grain: "historical randomized service-access contrasts",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/wia-gold-standard-impact-evidence.md",
        role: "Public WIA Gold Standard impact evidence",
        grain: "public historical randomized workforce evidence",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/disaster_depth_card.fy2025.v1.draft.json",
        role: "Disaster FY2025 subfunction depth card",
        grain: "federal disaster subfunction and evidence boundaries",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/disaster-depth-card.md",
        role: "Public disaster-resilience depth card",
        grain: "public fiscal depth card",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/justice_depth_card.fy2025.v1.draft.json",
        role: "Justice FY2025 function depth card",
        grain: "federal function and subfunction components",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/justice-depth-card.md",
        role: "Public justice breadth/depth card",
        grain: "public fiscal depth card",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/science_energy_environment_depth_card.fy2025.v1.draft.json",
        role: "Science-energy-environment composed depth card",
        grain: "three federal functions",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/science-energy-environment-depth-card.md",
        role: "Public science-energy-environment depth card",
        grain: "public fiscal depth card",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/agriculture_depth_card.fy2025.v1.draft.json",
        role: "Agriculture FY2025 depth card",
        grain: "federal function components",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/agriculture-depth-card.md",
        role: "Public agriculture depth card",
        grain: "public fiscal depth card",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/international_affairs_depth_card.fy2025.v1.draft.json",
        role: "International affairs FY2025 depth card",
        grain: "federal function components",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/international-affairs-depth-card.md",
        role: "Public international affairs depth card",
        grain: "public fiscal depth card",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/international_financial_programs_account_bridge.fy2025.v1.draft.json",
        role: "International financial programs FY2025 account bridge",
        grain: "federal account entries within subfunction 155",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/international-financial-programs-account-bridge.md",
        role: "Public international financial programs account bridge",
        grain: "public fiscal account reconciliation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/headline_basis_crosswalk/headline_basis_crosswalk.v1.draft.jsonl",
        role: "Headline measure basis and incompatibility crosswalk",
        grain: "headline measure definition",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/headline_basis_crosswalk/headline_basis_crosswalk.schema.md",
        role: "Headline basis crosswalk schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/headline_basis_crosswalk/README.md",
        role: "Headline basis crosswalk method note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/headline-number-selection-guide.md",
        role: "Public headline-number selection guide",
        grain: "public comparison guide",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/efficiency_pressure.fy2025.v1.draft.jsonl",
        role: "Efficiency pressure question-routing rows",
        grain: "FY2025 pressure surface",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/README.md",
        role: "Efficiency pressure method note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/efficiency_pressure.schema.md",
        role: "Efficiency pressure row schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/cost_down_backlog.fy2025.v1.draft.jsonl",
        role: "Cost-down backlog work-item rows",
        grain: "FY2025 cost-down lever",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/cost_down_backlog.schema.md",
        role: "Cost-down backlog row schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/cost_down_source_packets.fy2025.v1.draft.jsonl",
        role: "Cost-down source packet rows",
        grain: "cost-down source packet",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/cost_down_source_packets.schema.md",
        role: "Cost-down source packet schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/cost_down_evidence_queue.fy2025.v1.draft.jsonl",
        role: "Cost-down evidence queue rows",
        grain: "cost-down extraction queue",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/cost_down_evidence_queue.schema.md",
        role: "Cost-down evidence queue schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/cost_down_first_pass_rollup.v1.draft.jsonl",
        role: "Cost-down first-pass rollup",
        grain: "cost-down queue row by evidence status",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/cost_down_first_pass_rollup.schema.md",
        role: "Cost-down first-pass rollup schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/cost_down_scoring_readiness.v1.draft.jsonl",
        role: "Cost-down scoring readiness",
        grain: "ranked cost-down lever by scoring readiness",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/cost_down_scoring_readiness.schema.md",
        role: "Cost-down scoring readiness schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_eligibility_first_pass.jsonl",
        role: "Payment integrity eligibility first-pass extract",
        grain: "PaymentAccuracy homepage agency trend row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_eligibility_first_pass.schema.md",
        role: "Payment integrity eligibility first-pass extract schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_scorecards_q4_2025_first_pass.jsonl",
        role: "Payment integrity scorecard first-pass extract",
        grain: "PaymentAccuracy Q4 2025 scorecard row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_scorecards_q4_2025_first_pass.schema.md",
        role: "Payment integrity scorecard first-pass extract schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_gates_q4_2025.jsonl",
        role: "Payment integrity program review gates",
        grain: "PaymentAccuracy scorecard program review gate",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_gates_q4_2025.schema.md",
        role: "Payment integrity program review gates schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_tasks_q4_2025.jsonl",
        role: "Payment integrity program review tasks",
        grain: "PaymentAccuracy scorecard program review task",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_tasks_q4_2025.schema.md",
        role: "Payment integrity program review tasks schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_status_q4_2025.jsonl",
        role: "Payment integrity program review status",
        grain: "PaymentAccuracy scorecard program review status",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_status_q4_2025.schema.md",
        role: "Payment integrity program review status schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_plans_q4_2025.jsonl",
        role: "Payment integrity methodology plans",
        grain: "PaymentAccuracy program methodology plan",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_plans_q4_2025.schema.md",
        role: "Payment integrity methodology plans schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_fields_q4_2025.jsonl",
        role: "Payment integrity methodology field checklist",
        grain: "PaymentAccuracy program methodology field",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_fields_q4_2025.schema.md",
        role: "Payment integrity methodology field checklist schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_source_targets_q4_2025.jsonl",
        role: "Payment integrity methodology source targets",
        grain: "PaymentAccuracy methodology source target",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_source_targets_q4_2025.schema.md",
        role: "Payment integrity methodology source targets schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_queries_q4_2025.jsonl",
        role: "Payment integrity methodology queries",
        grain: "PaymentAccuracy methodology query row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_queries_q4_2025.schema.md",
        role: "Payment integrity methodology queries schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_query_runs_q4_2025.jsonl",
        role: "Payment integrity methodology query runs",
        grain: "PaymentAccuracy methodology query run row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_query_runs_q4_2025.schema.md",
        role: "Payment integrity methodology query runs schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_results_q4_2025.jsonl",
        role: "Payment integrity methodology results",
        grain: "PaymentAccuracy methodology result row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_results_q4_2025.schema.md",
        role: "Payment integrity methodology results schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_result_review_readiness_q4_2025.jsonl",
        role: "Payment integrity methodology result review readiness",
        grain: "PaymentAccuracy methodology result-review-readiness row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_result_review_readiness_q4_2025.schema.md",
        role: "Payment integrity methodology result review readiness schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_field_reviews_q4_2025.jsonl",
        role: "Payment integrity methodology field reviews",
        grain: "PaymentAccuracy methodology field-review row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_field_reviews_q4_2025.schema.md",
        role: "Payment integrity methodology field reviews schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_gap_followups_q4_2025.jsonl",
        role: "Payment integrity methodology gap followups",
        grain: "PaymentAccuracy methodology gap-followup row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_gap_followups_q4_2025.schema.md",
        role: "Payment integrity methodology gap followups schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_gap_source_captures_q4_2025.jsonl",
        role: "Payment integrity methodology gap source captures",
        grain: "PaymentAccuracy methodology gap source-capture row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_gap_source_captures_q4_2025.schema.md",
        role: "Payment integrity methodology gap source captures schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_source_capture_rollup_q4_2025.jsonl",
        role: "Payment integrity methodology source capture rollup",
        grain: "PaymentAccuracy methodology source-capture rollup row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_source_capture_rollup_q4_2025.schema.md",
        role: "Payment integrity methodology source capture rollup schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_readiness_q4_2025.jsonl",
        role: "Payment integrity methodology closure readiness",
        grain: "PaymentAccuracy methodology closure-readiness row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_readiness_q4_2025.schema.md",
        role: "Payment integrity methodology closure readiness schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_decisions_q4_2025.jsonl",
        role: "Payment integrity methodology closure decisions",
        grain: "PaymentAccuracy methodology closure-decision row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_decisions_q4_2025.schema.md",
        role: "Payment integrity methodology closure decisions schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_residual_source_gaps_q4_2025.jsonl",
        role: "Payment integrity methodology residual source gaps",
        grain: "PaymentAccuracy methodology residual source-gap row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_residual_source_gaps_q4_2025.schema.md",
        role: "Payment integrity methodology residual source gaps schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_coverage_q4_2025.jsonl",
        role: "Payment integrity methodology closure coverage",
        grain: "PaymentAccuracy methodology closure-coverage row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_coverage_q4_2025.schema.md",
        role: "Payment integrity methodology closure coverage schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_scoring_gate_q4_2025.jsonl",
        role: "Payment integrity methodology scoring gate",
        grain: "PaymentAccuracy methodology scoring-gate row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_scoring_gate_q4_2025.schema.md",
        role: "Payment integrity methodology scoring gate schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_program_rollup_q4_2025.jsonl",
        role: "Payment integrity methodology program rollup",
        grain: "PaymentAccuracy methodology program-rollup row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_program_rollup_q4_2025.schema.md",
        role: "Payment integrity methodology program rollup schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_open_program_status_q4_2025.jsonl",
        role: "Payment integrity methodology open program status",
        grain: "PaymentAccuracy methodology open-program status row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_open_program_status_q4_2025.schema.md",
        role: "Payment integrity methodology open program status schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_residual_gap_priority_q4_2025.jsonl",
        role: "Payment integrity methodology residual gap priority",
        grain: "PaymentAccuracy methodology residual-gap priority row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_residual_gap_priority_q4_2025.schema.md",
        role: "Payment integrity methodology residual gap priority schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_priority_source_work_q4_2025.jsonl",
        role: "Payment integrity methodology priority source work",
        grain: "PaymentAccuracy methodology priority source-work row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_priority_source_work_q4_2025.schema.md",
        role: "Payment integrity methodology priority source work schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_priority_reviewer_actions_q4_2025.jsonl",
        role: "Payment integrity methodology priority reviewer actions",
        grain: "PaymentAccuracy methodology priority reviewer-action row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_priority_reviewer_actions_q4_2025.schema.md",
        role: "Payment integrity methodology priority reviewer actions schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_field_updates_q4_2025.jsonl",
        role: "Payment integrity methodology field updates",
        grain: "PaymentAccuracy methodology field-update row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_field_updates_q4_2025.schema.md",
        role: "Payment integrity methodology field updates schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_queries_q4_2025.jsonl",
        role: "Payment integrity methodology follow-up source queries",
        grain: "PaymentAccuracy methodology follow-up source-query row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_queries_q4_2025.schema.md",
        role: "Payment integrity methodology follow-up source queries schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_query_runs_q4_2025.jsonl",
        role: "Payment integrity methodology follow-up source query runs",
        grain: "PaymentAccuracy methodology follow-up source-query-run row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_query_runs_q4_2025.schema.md",
        role: "Payment integrity methodology follow-up source query runs schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_captures_q4_2025.jsonl",
        role: "Payment integrity methodology follow-up source captures",
        grain: "PaymentAccuracy methodology follow-up source-capture row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_captures_q4_2025.schema.md",
        role: "Payment integrity methodology follow-up source captures schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_capture_rollup_q4_2025.jsonl",
        role: "Payment integrity methodology follow-up source capture rollup",
        grain: "PaymentAccuracy methodology follow-up source-capture rollup row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_capture_rollup_q4_2025.schema.md",
        role: "Payment integrity methodology follow-up source capture rollup schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_boundary_decisions_q4_2025.jsonl",
        role: "Payment integrity methodology follow-up boundary decisions",
        grain: "PaymentAccuracy methodology follow-up boundary-decision row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_boundary_decisions_q4_2025.schema.md",
        role: "Payment integrity methodology follow-up boundary decisions schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_boundary_readiness_q4_2025.jsonl",
        role: "Payment integrity methodology follow-up boundary readiness",
        grain: "PaymentAccuracy methodology follow-up boundary-readiness row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_boundary_readiness_q4_2025.schema.md",
        role: "Payment integrity methodology follow-up boundary readiness schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_narrow_closure_candidates_q4_2025.jsonl",
        role: "Payment integrity methodology narrow closure candidates",
        grain: "PaymentAccuracy methodology narrow closure-candidate row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_narrow_closure_candidates_q4_2025.schema.md",
        role: "Payment integrity methodology narrow closure candidates schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_narrow_closure_decisions_q4_2025.jsonl",
        role: "Payment integrity methodology narrow closure decisions",
        grain: "PaymentAccuracy methodology narrow closure-decision row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_narrow_closure_decisions_q4_2025.schema.md",
        role: "Payment integrity methodology narrow closure decisions schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_open_program_component_progress_q4_2025.jsonl",
        role: "Payment integrity methodology open-program component progress",
        grain: "PaymentAccuracy methodology open-program component-progress row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_open_program_component_progress_q4_2025.schema.md",
        role: "Payment integrity methodology open-program component progress schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_requirements_q4_2025.jsonl",
        role: "Payment integrity methodology component gate requirements",
        grain: "PaymentAccuracy methodology component gate-requirement row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_requirements_q4_2025.schema.md",
        role: "Payment integrity methodology component gate requirements schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_targets_q4_2025.jsonl",
        role: "Payment integrity methodology component gate source targets",
        grain: "PaymentAccuracy methodology component gate source-target row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_targets_q4_2025.schema.md",
        role: "Payment integrity methodology component gate source targets schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_queries_q4_2025.jsonl",
        role: "Payment integrity methodology component gate source queries",
        grain: "PaymentAccuracy methodology component gate source-query row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_queries_q4_2025.schema.md",
        role: "Payment integrity methodology component gate source queries schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_query_runs_q4_2025.jsonl",
        role: "Payment integrity methodology component gate source query runs",
        grain: "PaymentAccuracy methodology component gate source-query-run row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_query_runs_q4_2025.schema.md",
        role: "Payment integrity methodology component gate source query runs schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_captures_q4_2025.jsonl",
        role: "Payment integrity methodology component gate source captures",
        grain: "PaymentAccuracy methodology component gate source-capture row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_captures_q4_2025.schema.md",
        role: "Payment integrity methodology component gate source captures schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_capture_rollups_q4_2025.jsonl",
        role: "Payment integrity methodology component gate source capture rollups",
        grain: "PaymentAccuracy methodology component gate source-capture rollup row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_capture_rollups_q4_2025.schema.md",
        role: "Payment integrity methodology component gate source capture rollups schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_boundary_decisions_q4_2025.jsonl",
        role: "Payment integrity methodology component gate boundary decisions",
        grain: "PaymentAccuracy methodology component gate boundary-decision row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_boundary_decisions_q4_2025.schema.md",
        role: "Payment integrity methodology component gate boundary decisions schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_boundary_readiness_q4_2025.jsonl",
        role: "Payment integrity methodology component gate boundary readiness",
        grain: "PaymentAccuracy methodology component gate boundary-readiness row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_boundary_readiness_q4_2025.schema.md",
        role: "Payment integrity methodology component gate boundary readiness schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_narrow_candidates_q4_2025.jsonl",
        role: "Payment integrity methodology component gate narrow candidates",
        grain: "PaymentAccuracy methodology component gate narrow-candidate row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_narrow_candidates_q4_2025.schema.md",
        role: "Payment integrity methodology component gate narrow candidates schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_narrow_decisions_q4_2025.jsonl",
        role: "Payment integrity methodology component gate narrow decisions",
        grain: "PaymentAccuracy methodology component gate narrow-decision row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_narrow_decisions_q4_2025.schema.md",
        role: "Payment integrity methodology component gate narrow decisions schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_q4_2025.jsonl",
        role: "Payment integrity methodology component gate progress",
        grain: "PaymentAccuracy methodology component gate progress row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_q4_2025.schema.md",
        role: "Payment integrity methodology component gate progress schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_requirements_q4_2025.jsonl",
        role: "Payment integrity methodology component gate progress requirements",
        grain: "PaymentAccuracy methodology component gate progress requirement row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_requirements_q4_2025.schema.md",
        role: "Payment integrity methodology component gate progress requirements schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_targets_q4_2025.jsonl",
        role: "Payment integrity methodology component gate progress source targets",
        grain: "PaymentAccuracy methodology component gate progress source-target row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_targets_q4_2025.schema.md",
        role: "Payment integrity methodology component gate progress source targets schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_queries_q4_2025.jsonl",
        role: "Payment integrity methodology component gate progress source queries",
        grain: "PaymentAccuracy methodology component gate progress source-query row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_queries_q4_2025.schema.md",
        role: "Payment integrity methodology component gate progress source queries schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_query_runs_q4_2025.jsonl",
        role: "Payment integrity methodology component gate progress source query runs",
        grain: "PaymentAccuracy methodology component gate progress source-query-run row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_query_runs_q4_2025.schema.md",
        role: "Payment integrity methodology component gate progress source query runs schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_next_program_selection_q4_2025.jsonl",
        role: "Payment integrity next program selection",
        grain: "PaymentAccuracy next program selection row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_next_program_selection_q4_2025.schema.md",
        role: "Payment integrity next program selection schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_claims_timeliness_first_pass.jsonl",
        role: "Payment integrity claims timeliness first-pass extract",
        grain: "claims timeliness probe row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/payment_integrity_claims_timeliness_first_pass.schema.md",
        role: "Payment integrity claims timeliness first-pass extract schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/debt_maturity_risk_first_pass.jsonl",
        role: "Debt maturity risk first-pass extract",
        grain: "Treasury Fiscal Data probe row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/debt_maturity_risk_first_pass.schema.md",
        role: "Debt maturity risk first-pass extract schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/debt_primary_balance_first_pass.jsonl",
        role: "Debt primary balance first-pass extract",
        grain: "FY2025 fiscal-balance probe row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/debt_primary_balance_first_pass.schema.md",
        role: "Debt primary balance first-pass extract schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/disaster_supplemental_tracking_first_pass.jsonl",
        role: "Disaster supplemental tracking first-pass extract",
        grain: "FEMA declaration-area probe row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/disaster_supplemental_tracking_first_pass.schema.md",
        role: "Disaster supplemental tracking first-pass extract schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/disaster_mitigation_first_pass.jsonl",
        role: "Disaster mitigation first-pass extract",
        grain: "FEMA HMA project probe row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/disaster_mitigation_first_pass.schema.md",
        role: "Disaster mitigation first-pass extract schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/defense_audit_control_first_pass.jsonl",
        role: "Defense audit control first-pass extract",
        grain: "DoD OIG audit-control probe row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/defense_audit_control_first_pass.schema.md",
        role: "Defense audit control first-pass extract schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/defense_procurement_control_first_pass.jsonl",
        role: "Defense procurement control first-pass extract",
        grain: "GAO weapon-systems procurement-control probe row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/defense_procurement_control_first_pass.schema.md",
        role: "Defense procurement control first-pass extract schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/health_price_discipline_first_pass.jsonl",
        role: "Health price discipline first-pass extract",
        grain: "health price-discipline probe row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/health_price_discipline_first_pass.schema.md",
        role: "Health price discipline first-pass extract schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/health_admin_simplification_first_pass.jsonl",
        role: "Health administrative simplification first-pass extract",
        grain: "health administrative workflow probe row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/efficiency_pressure/extracts/health_admin_simplification_first_pass.schema.md",
        role: "Health administrative simplification first-pass extract schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/research/2026-06-28-efficiency-pressure-framework.md",
        role: "Efficiency pressure framework note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/cost-down-backlog.md",
        role: "Cost-down backlog reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/cost-down-evidence-queue.md",
        role: "Cost-down evidence queue reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/payment-integrity-first-pass-extract.md",
        role: "Payment integrity first-pass extract reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/payment-integrity-scorecard-extract.md",
        role: "Payment integrity scorecard extract reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/payment-integrity-program-review-gates.md",
        role: "Payment integrity program review gate reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/payment-integrity-program-review-tasks.md",
        role: "Payment integrity program review task reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/payment-integrity-program-review-status.md",
        role: "Payment integrity program review status reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/payment-integrity-methodology-plans.md",
        role: "Payment integrity methodology plan reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/payment-integrity-methodology-fields.md",
        role: "Payment integrity methodology field checklist reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/payment-integrity-methodology-source-targets.md",
        role: "Payment integrity methodology source target reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/payment-integrity-methodology-queries.md",
        role: "Payment integrity methodology query reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/payment-integrity-methodology-query-runs.md",
        role: "Payment integrity methodology query run reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/payment-integrity-methodology-results.md",
        role: "Payment integrity methodology result reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/payment-integrity-claims-timeliness-extract.md",
        role: "Payment integrity claims timeliness extract reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/debt-maturity-risk-extract.md",
        role: "Debt maturity risk extract reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/debt-primary-balance-extract.md",
        role: "Debt primary balance extract reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/disaster-supplemental-tracking-extract.md",
        role: "Disaster supplemental tracking extract reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/health-price-discipline-source-packet.md",
        role: "Health price discipline source packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/health-administrative-simplification-source-packet.md",
        role: "Health administrative simplification source packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/debt-primary-balance-source-packet.md",
        role: "Debt primary balance source packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/debt-maturity-risk-source-packet.md",
        role: "Debt maturity risk source packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/defense-procurement-control-source-packet.md",
        role: "Defense procurement control source packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/defense-audit-control-source-packet.md",
        role: "Defense audit control source packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/disaster-mitigation-source-packet.md",
        role: "Disaster mitigation source packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/disaster-supplemental-tracking-source-packet.md",
        role: "Disaster supplemental tracking source packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/payment-integrity-eligibility-source-packet.md",
        role: "Payment integrity eligibility source packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/payment-integrity-claims-timeliness-source-packet.md",
        role: "Payment integrity claims timeliness source packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/contribution_alignment/contribution_alignment.fy2025.v1.draft.jsonl",
        role: "Contribution-benefit alignment rows",
        grain: "FY2025 lane alignment surface",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/contribution_alignment/medicare_source_boundary.fy2025.draft.jsonl",
        role: "Medicare OMB source-boundary check",
        grain: "FY2025 Medicare source boundary",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/contribution_alignment/medicare_part_financing.cy2025.cms-trustees-2026.draft.jsonl",
        role: "Medicare Trustees part-financing split",
        grain: "CY2025 Medicare part financing",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/contribution_alignment/README.md",
        role: "Contribution alignment method note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/contribution_alignment/contribution_alignment.schema.md",
        role: "Contribution alignment row schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/research/2026-06-28-contribution-benefit-alignment.md",
        role: "Contribution-benefit alignment framework note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/research/2026-06-28-medicare-source-boundary.md",
        role: "Medicare source-boundary note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/research/2026-06-29-medicare-part-financing.md",
        role: "Medicare part-financing note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/denominator_requirements/denominator_requirements.v1.draft.jsonl",
        role: "Per-person display denominator requirements",
        grain: "denominator display basis",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/denominator_requirements/denominator_values.ty2022.irs-soi-1304.draft.jsonl",
        role: "Sourced tax-return denominator values",
        grain: "tax-year denominator value",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/denominator_requirements/denominator_values.cy2025.cms-medicare-trustees-2026.draft.jsonl",
        role: "Sourced Medicare enrollment denominator values",
        grain: "calendar-year Medicare denominator value",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/denominator_requirements/denominator_values.cy2025.census.draft.jsonl",
        role: "Sourced Census civic denominator values",
        grain: "calendar-year civic denominator value",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/denominator_requirements/denominator_values.cy2025.ssa-trustees-2026.draft.jsonl",
        role: "Sourced Social Security denominator values",
        grain: "calendar-year Social Security denominator value",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/denominator_requirements/per_unit_display_readiness.v1.draft.jsonl",
        role: "Per-unit display readiness rows",
        grain: "per-unit display claim",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/denominator_requirements/per_unit_receipt_cards.v1.draft.jsonl",
        role: "Per-unit receipt card rows",
        grain: "per-unit public card",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/denominator_requirements/README.md",
        role: "Denominator requirements method note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/denominator_requirements/denominator_requirements.schema.md",
        role: "Denominator requirements schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/research/2026-06-28-denominator-source-ladder.md",
        role: "Denominator source ladder",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/research/2026-06-29-medicare-denominators.md",
        role: "Medicare denominator values note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/research/2026-06-29-civic-denominators.md",
        role: "Civic denominator values note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/research/2026-06-29-social-security-denominators.md",
        role: "Social Security denominator values note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/denominator_requirements/per-unit-display-readiness.md",
        role: "Per-unit display readiness dashboard",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/per-unit-receipt-cards.md",
        role: "Per-unit receipt card reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/honest-federal-tax-receipt.md",
        role: "Flagship honest federal tax receipt",
        grain: "public receipt prototype",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/aligned-contribution-receipt.md",
        role: "Aligned contribution receipt reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_subfunction_model/source-profile.md",
        role: "Subfunction source coverage and reconciliation sample",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_subfunction_model/reconciliation-review.md",
        role: "Subfunction generated-row reconciliation review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_model/README.md",
        role: "Model method and schema note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_model/source-profile.md",
        role: "Source coverage and reconciliation sample",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/README.md",
        role: "Derived data index",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_model/reconciliation-review.md",
        role: "Generated-row reconciliation review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_model/decade-summary.md",
        role: "Human-readable decade summary",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/modeled-income-tax-outlays.md",
        role: "Reader-facing packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/modeled-income-tax-subfunction-outlays.md",
        role: "Reader-facing subfunction drilldown packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-22-subfunction-reader-role-review.md",
        role: "Subfunction reader role review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-07-14-payment-integrity-fy2024-annual-extraction-role-review.md",
        role: "FY2024 PaymentAccuracy annual-extraction role review",
        grain: "source custody, 68/54/59 row scope, reconciliation, and bounded-use decision",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/research/2026-06-22-subfunction-deficit-context-note.md",
        role: "Subfunction deficit context note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/data/README.md",
        role: "Data documentation index",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/data/dictionary.md",
        role: "Data dictionary",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/data/accountability-evidence-schema.md",
        role: "Accountability evidence schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/research/2026-06-23-accountability-evidence-boundary.md",
        role: "Accountability evidence boundary note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/charts/README.md",
        role: "Chart catalog",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/charts/income-tax-outlay-subfunction-model/README.md",
        role: "Subfunction chart set handoff note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/charts/income-tax-outlay-model/README.md",
        role: "Broad chart set handoff note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json",
        role: "Annual allocation chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json",
        role: "Decade allocation chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json",
        role: "Annual financing context chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json",
        role: "Decade financing context chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "docs/charts/income-tax-outlay-subfunction-model/fy2025-top-subfunctions.vl.json",
        role: "FY2025 top subfunction allocation chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "docs/charts/income-tax-outlay-subfunction-model/selected-subfunction-trends.vl.json",
        role: "Selected subfunction trend chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "docs/charts/income-tax-outlay-subfunction-model/decade-top-subfunctions.vl.json",
        role: "Decade top subfunction allocation chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "data/derived/taxpayer_receipt_model/taxpayer_receipt_model.placeholder-1000.fy2025.omb-fy2027-v1.draft.json",
        role: "Placeholder visibility receipt scenario",
        grain: "scenario",
        kind: "json",
        canonical: "yes",
    },
    Artifact {
        path: "data/derived/accountability_evidence/accountability_evidence.omb-fy2027-v1.2026-06-23.draft.jsonl",
        role: "Draft accountability evidence records",
        grain: "evidence record",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/README.md",
        role: "Accountability evidence dataset method note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/readiness-report.md",
        role: "Accountability evidence readiness report",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/action-queue.md",
        role: "Accountability evidence action queue",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-packet.md",
        role: "Accountability performance demand packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/accountability-work-items.jsonl",
        role: "Accountability machine-readable work items",
        grain: "work item",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/claim-guard-report.md",
        role: "Accountability claim guard report",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/public-questions.md",
        role: "Accountability public-safe questions",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-checklist.md",
        role: "Accountability performance demand checklist",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-checklist.jsonl",
        role: "Accountability performance demand checklist rows",
        grain: "demand checklist row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-claim-gates.json",
        role: "Accountability performance demand claim gates",
        grain: "claim gate summary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-dashboard.md",
        role: "Accountability performance demand dashboard",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-brief.md",
        role: "Accountability performance demand brief",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-letter.md",
        role: "Accountability performance demand letter template",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-rubric.md",
        role: "Accountability performance demand response rubric",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-followup.md",
        role: "Accountability performance demand follow-up template",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-log.md",
        role: "Accountability performance demand response log",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-log.jsonl",
        role: "Accountability performance demand response log rows",
        grain: "response log row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-log.schema.md",
        role: "Accountability performance demand response log schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-status.json",
        role: "Accountability performance demand response status",
        grain: "response status summary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-dashboard.md",
        role: "Accountability performance demand response dashboard",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-handoff.md",
        role: "Accountability performance demand response handoff",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-intake.md",
        role: "Accountability performance demand response intake template",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-intake.schema.md",
        role: "Accountability performance demand response intake schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-intake.example.jsonl",
        role: "Accountability performance demand response intake example rows",
        grain: "response intake row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_JSONL_PATH,
        role: "External accountability claim quarantine rows",
        grain: "external claim amount atom",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_SCHEMA_PATH,
        role: "External accountability claim intake schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_READER_PATH,
        role: "External accountability claim intake internal reader",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_REVIEW_PATH,
        role: "External accountability claim intake role review",
        grain: "review",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: HOUSE_SHIRLEY_TESTIMONY_METADATA_PATH,
        role: "House Shirley testimony source metadata",
        grain: "source custody metadata",
        kind: "markdown",
        canonical: "source",
    },
    Artifact {
        path: HOUSE_SHIRLEY_TESTIMONY_REVIEW_PATH,
        role: "House testimony Quality Learing Center claim atom role review",
        grain: "review",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: MN_DCYF_CCAP_PROVIDER_METADATA_PATH,
        role: "Minnesota DCYF CCAP provider table source metadata",
        grain: "source custody metadata",
        kind: "markdown",
        canonical: "source",
    },
    Artifact {
        path: MN_DCYF_CCAP_PROVIDER_REVIEW_PATH,
        role: "Minnesota DCYF Quality Learning Center payment context role review",
        grain: "review",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: MN_DCYF_CCAP_PROVIDER_CLOSURE_REVIEW_PATH,
        role: "Minnesota DCYF Quality Learning Center license closure context role review",
        grain: "review",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: QUALITY_LEARNING_CENTER_CY2025_PERIOD_CORRECTION_REVIEW_PATH,
        role: "Quality Learning Center CY2025 period correction role review",
        grain: "review",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: MN_DCYF_DATA_REQUESTS_METADATA_PATH,
        role: "Minnesota DCYF data-request route source metadata",
        grain: "source custody metadata",
        kind: "markdown",
        canonical: "source",
    },
    Artifact {
        path: MN_DCYF_PUBLIC_DATA_GUIDE_METADATA_PATH,
        role: "Minnesota DCYF public-data guide source metadata",
        grain: "source custody metadata",
        kind: "markdown",
        canonical: "source",
    },
    Artifact {
        path: MN_STAT_13_03_METADATA_PATH,
        role: "Minnesota Statutes section 13.03 source metadata",
        grain: "source custody metadata",
        kind: "markdown",
        canonical: "source",
    },
    Artifact {
        path: MN_STAT_142E_02_METADATA_PATH,
        role: "Minnesota Statutes section 142E.02 source metadata",
        grain: "source custody metadata",
        kind: "markdown",
        canonical: "source",
    },
    Artifact {
        path: MN_CCAP_CY2025_REQUEST_SPEC_PATH,
        role: "Minnesota CCAP Quality Learning Center CY2025 existing-records request specification",
        grain: "request specification",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: MN_CCAP_CY2025_REQUEST_READER_PATH,
        role: "Minnesota CCAP CY2025 existing-records request internal reader",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: MN_CCAP_CY2025_REQUEST_TEMPLATE_PATH,
        role: "Minnesota CCAP CY2025 draft unsent data request",
        grain: "request template",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: MN_CCAP_CY2025_REQUEST_REVIEW_PATH,
        role: "Minnesota CCAP CY2025 existing-records request role review",
        grain: "review",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-log.applied-example.jsonl",
        role: "Accountability performance demand response log applied example rows",
        grain: "response log row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-status.applied-example.json",
        role: "Accountability performance demand response applied status",
        grain: "response status summary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-dashboard.applied-example.md",
        role: "Accountability performance demand response applied dashboard",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-handoff.applied-example.md",
        role: "Accountability performance demand response applied handoff",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-applied-example.schema.md",
        role: "Accountability performance demand response applied example schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-delta.applied-example.md",
        role: "Accountability performance demand response applied delta",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-delta.applied-example.jsonl",
        role: "Accountability performance demand response applied delta rows",
        grain: "response delta row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-delta.applied-example.schema.md",
        role: "Accountability performance demand response applied delta schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-bundle.applied-example.md",
        role: "Accountability performance demand response applied bundle index",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-bundle.applied-example.json",
        role: "Accountability performance demand response applied bundle manifest",
        grain: "bundle manifest",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-response-bundle.applied-example.schema.md",
        role: "Accountability performance demand response applied bundle manifest schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-checklist.schema.md",
        role: "Accountability performance demand checklist schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/artifact-map.md",
        role: "Accountability artifact map",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/accountability-public-brief.md",
        role: "Reader-facing accountability brief",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/README.md",
        role: "Reading packet index",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/placeholder-visibility-receipt.md",
        role: "Placeholder receipt reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/placeholder-receipt-display-packet.md",
        role: "Placeholder receipt static display packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-placeholder-display-packet-role-review.md",
        role: "Placeholder receipt display packet role review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/design/placeholder-receipt-placement-spec.md",
        role: "Placeholder receipt static placement spec",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/design/README.md",
        role: "Design handoff index",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-placeholder-placement-spec-role-review.md",
        role: "Placeholder receipt placement spec role review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/design/placeholder-receipt-mockup-review-checklist.md",
        role: "Placeholder receipt mockup review checklist",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-placeholder-mockup-checklist-role-review.md",
        role: "Placeholder receipt mockup checklist role review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-evidence-role-review.md",
        role: "Accountability evidence role review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-evidence-source-custody-review.md",
        role: "Accountability evidence source-custody review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-validator-hardening-review.md",
        role: "Accountability validator hardening review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-readiness-classification-review.md",
        role: "Accountability readiness classification review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-readiness-report-review.md",
        role: "Accountability readiness report review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-evidence-only-record-review.md",
        role: "Accountability evidence-only record review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-next-action-report-review.md",
        role: "Accountability next-action report review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-action-queue-review.md",
        role: "Accountability action queue review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-performance-demand-packet-review.md",
        role: "Accountability performance demand packet review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-core-workflow-review.md",
        role: "Accountability core workflow review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-work-items-review.md",
        role: "Accountability work items review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-claim-guard-report-review.md",
        role: "Accountability claim guard report review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-public-questions-review.md",
        role: "Accountability public questions review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-public-brief-review.md",
        role: "Accountability public brief review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-brief-discovery-review.md",
        role: "Accountability brief discovery review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-artifact-map-review.md",
        role: "Accountability artifact map review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-performance-demand-checklist-review.md",
        role: "Accountability performance demand checklist review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-performance-demand-checklist-jsonl-review.md",
        role: "Accountability performance demand checklist JSONL review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-demand-checklist-core-contract-review.md",
        role: "Accountability demand checklist core contract review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-demand-checklist-jsonl-read-validation-review.md",
        role: "Accountability demand checklist JSONL read validation review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-demand-checklist-schema-review.md",
        role: "Accountability demand checklist schema review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-performance-demand-claim-gates-review.md",
        role: "Accountability performance demand claim gates review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-performance-demand-dashboard-review.md",
        role: "Accountability performance demand dashboard review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-rust-core-crate-architecture-review.md",
        role: "Rust core crate architecture review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/MISSION.md",
        role: "VTRACE mission",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/REQUIREMENTS.md",
        role: "VTRACE requirements",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/SPECIFICATION_BASELINE.md",
        role: "VTRACE specification baseline",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/TRACE.md",
        role: "VTRACE trace matrix",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/WORK_PACKAGES.md",
        role: "VTRACE work packages",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/VERIFICATION.md",
        role: "VTRACE verification plan",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/VALIDATION.md",
        role: "VTRACE validation scenarios",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/EVIDENCE.md",
        role: "VTRACE evidence ledger",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/REVIEW.md",
        role: "VTRACE adoption review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/charts/taxpayer-receipt-model/README.md",
        role: "Taxpayer receipt chart set handoff note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/charts/taxpayer-receipt-model/placeholder-lane-bars.vl.json",
        role: "Placeholder receipt lane bar chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "docs/charts/taxpayer-receipt-model/placeholder-financing-context.vl.json",
        role: "Placeholder receipt financing context chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "Cargo.toml",
        role: "Rust workspace manifest",
        grain: "tooling",
        kind: "toml",
        canonical: "supporting",
    },
    Artifact {
        path: "Cargo.lock",
        role: "Rust dependency lockfile",
        grain: "tooling",
        kind: "toml",
        canonical: "supporting",
    },
    Artifact {
        path: "crates/taxlane-core/Cargo.toml",
        role: "Rust Taxlane core crate manifest",
        grain: "tooling",
        kind: "toml",
        canonical: "supporting",
    },
    Artifact {
        path: "crates/taxlane-core/src/lib.rs",
        role: "Rust Taxlane core domain library",
        grain: "library",
        kind: "rust",
        canonical: "supporting",
    },
    Artifact {
        path: "crates/taxlane-net-interest/Cargo.toml",
        role: "Rust Taxlane net-interest feature crate manifest",
        grain: "tooling",
        kind: "toml",
        canonical: "supporting",
    },
    Artifact {
        path: "crates/taxlane-net-interest/src/lib.rs",
        role: "Rust Taxlane signed marginal-rollover engine",
        grain: "library",
        kind: "rust",
        canonical: "supporting",
    },
    Artifact {
        path: "tools/taxlane/Cargo.toml",
        role: "Rust Taxlane tools crate manifest",
        grain: "tooling",
        kind: "toml",
        canonical: "supporting",
    },
    Artifact {
        path: "tools/taxlane/src/main.rs",
        role: "Rust validation and manifest command implementation",
        grain: "script",
        kind: "rust",
        canonical: "supporting",
    },
    Artifact {
        path: NET_BASELINE_COMPATIBILITY_AUDIT_JSON_PATH,
        role: "NET current-law baseline compatibility audit",
        grain: "formula-input dispositions and zero-policy reconciliation",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: NET_BASELINE_COMPATIBILITY_AUDIT_SCHEMA_PATH,
        role: "NET baseline compatibility audit schema",
        grain: "input-admission and claim-boundary contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_BASELINE_COMPATIBILITY_AUDIT_READER_PATH,
        role: "NET baseline compatibility audit reader",
        grain: "public accounting advance and remaining gaps",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_BASELINE_COMPATIBILITY_AUDIT_REVIEW_PATH,
        role: "NET baseline compatibility role review",
        grain: "source and accounting fixed point",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_OMB_GROSS_TO_NET_BRIDGE_JSON_PATH,
        role: "NET OMB gross-to-net interest bridge",
        grain: "five signed interest components and vintage boundary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: NET_OMB_GROSS_TO_NET_BRIDGE_SCHEMA_PATH,
        role: "NET OMB gross-to-net interest bridge schema",
        grain: "component identity and no-stitch contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_OMB_GROSS_TO_NET_BRIDGE_READER_PATH,
        role: "NET OMB gross-to-net interest bridge reader",
        grain: "public gross, receipts, offsets, and net explanation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_NEW_BORROWING_TIMING_JSON_PATH,
        role: "NET new-borrowing timing convention",
        grain: "midpoint timing and current-year interest circularity",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: NET_NEW_BORROWING_TIMING_SCHEMA_PATH,
        role: "NET new-borrowing timing convention schema",
        grain: "timing rails, precision, and blocked feedback contract",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_NEW_BORROWING_TIMING_READER_PATH,
        role: "NET new-borrowing timing convention reader",
        grain: "public timing method and mechanical example boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_ACCOUNTING_BRIDGE_REVIEW_PATH,
        role: "NET accounting bridge role review",
        grain: "source, accounting, and fiscal fixed point",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_CBO_AVERAGE_RATE_FEEDBACK_JSON_PATH,
        role: "NET CBO average-rate feedback engine",
        grain: "matching-vintage rate path and reduced-form debt-service feedback",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: NET_CBO_AVERAGE_RATE_FEEDBACK_SCHEMA_PATH,
        role: "NET CBO average-rate feedback schema",
        grain: "delta-model identity, fixtures, and full-stock boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_CBO_AVERAGE_RATE_FEEDBACK_READER_PATH,
        role: "NET CBO average-rate feedback reader",
        grain: "public method, mechanical result, and claim boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_CBO_AVERAGE_RATE_FEEDBACK_REVIEW_PATH,
        role: "NET CBO average-rate feedback role review",
        grain: "source, accounting, service, and fiscal fixed point",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_PUBLIC_MATURITY_ENVELOPE_JSON_PATH,
        role: "NET MSPD public-maturity envelope",
        grain: "fiscal-year existing-stock runoff and non-pro-rata holder bounds",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: NET_PUBLIC_MATURITY_ENVELOPE_SCHEMA_PATH,
        role: "NET MSPD public-maturity envelope schema",
        grain: "source replay, interval arithmetic, and non-additivity",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_PUBLIC_MATURITY_ENVELOPE_READER_PATH,
        role: "NET MSPD public-maturity envelope reader",
        grain: "runoff results, holder uncertainty, and rollover boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_PUBLIC_MATURITY_ENVELOPE_REVIEW_PATH,
        role: "NET MSPD public-maturity envelope role review",
        grain: "source, accounting, service, and fiscal fixed point",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_EMPIRICAL_ROLLOVER_JSON_PATH,
        role: "NET MSPD empirical marginal-rollover convention",
        grain: "source-replayed term mix and signed refinancing fixture",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: NET_EMPIRICAL_ROLLOVER_SCHEMA_PATH,
        role: "NET MSPD empirical marginal-rollover schema",
        grain: "inclusive extraction, term mix, and model boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_EMPIRICAL_ROLLOVER_READER_PATH,
        role: "NET MSPD empirical marginal-rollover reader",
        grain: "issuance mix, fixture meaning, and claim boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: NET_EMPIRICAL_ROLLOVER_REVIEW_PATH,
        role: "NET MSPD empirical marginal-rollover role review",
        grain: "source, accounting, service, and fiscal fixed point",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: FIFTEEN_LANE_CANDIDATE_FRONTIER_JSON_PATH,
        role: "Fifteen-lane candidate execution frontier",
        grain: "one evidence-closure disposition per fiscal lane",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/fifteen_lane_candidate_execution_frontier.schema.md",
        role: "Fifteen-lane candidate execution-frontier schema",
        grain: "priority, execution-class, and authority boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/fifteen-lane-candidate-execution-frontier.md",
        role: "Fifteen-lane candidate execution-frontier reader",
        grain: "all-lane selection result and remaining blockers",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-08-01-fifteen-lane-candidate-execution-frontier-role-review.md",
        role: "Fifteen-lane candidate execution-frontier role review",
        grain: "finance, service, distribution, and scope fixed point",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: PAY_FULL_DMF_EVIDENCE_CEILING_JSON_PATH,
        role: "PAY full-DMF public-evidence ceiling",
        grain: "nine-gate public-source closure disposition",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/breadth_benchmark_matrix/pay_full_dmf_public_evidence_ceiling.schema.md",
        role: "PAY full-DMF public-evidence ceiling schema",
        grain: "search-exhaustion and owner-data boundary",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/pay-full-dmf-public-evidence-ceiling.md",
        role: "PAY full-DMF public-evidence ceiling reader",
        grain: "public closure result and reopen triggers",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-08-01-pay-full-dmf-public-evidence-ceiling-role-review.md",
        role: "PAY full-DMF public-evidence ceiling role review",
        grain: "source, finance, rights, privacy, and scope fixed point",
        kind: "markdown",
        canonical: "supporting",
    },
];

pub(crate) const RECEIPT_SHARE_CATEGORIES: &[ReceiptShareCategory] = &[
    ReceiptShareCategory {
        column: "B",
        receipt_category: "individual-income-taxes",
        source_receipt_label: "Individual Income Taxes",
        allocation_status: "general_receipt",
        notes: "Share of total receipts; AP13 general-fund concept supports ordinary individual income taxes as general receipts absent a cited legal dedication.",
    },
    ReceiptShareCategory {
        column: "C",
        receipt_category: "corporation-income-taxes",
        source_receipt_label: "Corporation Income Taxes",
        allocation_status: "unknown",
        notes: "Share of total receipts; allocation label remains unknown pending source-specific review.",
    },
    ReceiptShareCategory {
        column: "D",
        receipt_category: "social-insurance-and-retirement-receipts",
        source_receipt_label: "Social Insurance and Retirement Receipts Total",
        allocation_status: "unknown",
        notes: "Share of total receipts; social-insurance receipts remain separate from individual income taxes and require subcomponent review for allocation.",
    },
    ReceiptShareCategory {
        column: "G",
        receipt_category: "excise-taxes",
        source_receipt_label: "Excise Taxes",
        allocation_status: "unknown",
        notes: "Share of total receipts; excise taxes can include general and dedicated treatment.",
    },
    ReceiptShareCategory {
        column: "H",
        receipt_category: "other-receipts",
        source_receipt_label: "Other",
        allocation_status: "unknown",
        notes: "Share of total receipts; other receipts are heterogeneous and need source-specific review.",
    },
    ReceiptShareCategory {
        column: "I",
        receipt_category: "total-receipts",
        source_receipt_label: "Total Receipts",
        allocation_status: "mixed",
        notes: "Total receipts combine categories with different budget treatment and are not a legal allocation category.",
    },
];

pub(crate) const TABLE_3_2_NATIONAL_DEFENSE_LINES: &[Table32NationalDefenseLine] = &[
    Table32NationalDefenseLine {
        source_row: 13,
        subfunction_code: Some("051"),
        subfunction_label: Some("Department of Defense-Military"),
        source_label: "051 Subtotal, Department of Defense-Military",
        notes: "Subfunction total; lower component rows under 051 are not emitted in this proof slice.",
    },
    Table32NationalDefenseLine {
        source_row: 14,
        subfunction_code: Some("053"),
        subfunction_label: Some("Atomic energy defense activities"),
        source_label: "053 Atomic energy defense activities",
        notes: "National Defense subfunction total from Table 3.2.",
    },
    Table32NationalDefenseLine {
        source_row: 15,
        subfunction_code: Some("054"),
        subfunction_label: Some("Defense-related activities"),
        source_label: "054 Defense-related activities",
        notes: "National Defense subfunction total from Table 3.2.",
    },
    Table32NationalDefenseLine {
        source_row: 16,
        subfunction_code: None,
        subfunction_label: None,
        source_label: "Total, National Defense",
        notes: "Parent function total reconciled to OMB Historical Table 3.1 National Defense.",
    },
];

pub(crate) const PROGRAM_LANE_RATE_MODEL_DIR: &str = "data/derived/program_lane_rate_model";
