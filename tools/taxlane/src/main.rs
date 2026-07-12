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
    DisasterMitigationProjectProbeRecord, EfficiencyPressureRecord, HeadlineBasisRecord,
    HealthAdminSimplificationProbeRecord, HealthPriceDisciplineProbeRecord,
    PERFORMANCE_DEMAND_RESPONSE_INTAKE_USE_RULE, PUBLIC_CLAIM_ALLOWED_LABEL,
    PUBLIC_CLAIM_BLOCKED_LABEL, PaymentIntegrityClaimsTimelinessProbeRecord,
    PaymentIntegrityMethodologyClosureCoverageRecord,
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

const CHART_SPECS: &[&str] = &[
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

const MANIFEST_PATH: &str = "data/derived/income_tax_outlay_model/MANIFEST.md";
const ANNUAL_JSONL_PATH: &str = "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl";
const DECADE_JSONL_PATH: &str = "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-summary.jsonl";
const ANNUAL_CSV_PATH: &str = "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv";
const DECADE_CSV_PATH: &str = "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-wide.csv";
const DECADE_MD_PATH: &str = "data/derived/income_tax_outlay_model/decade-summary.md";
const SOURCE_PROFILE_PATH: &str = "data/derived/income_tax_outlay_model/source-profile.md";
const SUBFUNCTION_MODEL_JSONL_PATH: &str = "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.draft.jsonl";
const SUBFUNCTION_ANNUAL_CSV_PATH: &str = "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.annual-long.csv";
const SUBFUNCTION_DECADE_CSV_PATH: &str = "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.decade-long.csv";
const SUBFUNCTION_FY2025_TOP_CSV_PATH: &str = "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.fy2025-top-subfunctions.csv";
const SUBFUNCTION_MODEL_PROFILE_PATH: &str =
    "data/derived/income_tax_outlay_subfunction_model/source-profile.md";
const SUBFUNCTION_MODEL_README_PATH: &str =
    "data/derived/income_tax_outlay_subfunction_model/README.md";
const PLACEHOLDER_RECEIPT_JSON_PATH: &str = "data/derived/taxpayer_receipt_model/taxpayer_receipt_model.placeholder-1000.fy2025.omb-fy2027-v1.draft.json";
const PLACEHOLDER_RECEIPT_LANE_BARS_SPEC_PATH: &str =
    "docs/charts/taxpayer-receipt-model/placeholder-lane-bars.vl.json";
const PLACEHOLDER_RECEIPT_FINANCING_CONTEXT_SPEC_PATH: &str =
    "docs/charts/taxpayer-receipt-model/placeholder-financing-context.vl.json";
const ACCOUNTABILITY_EVIDENCE_JSONL_PATH: &str = "data/derived/accountability_evidence/accountability_evidence.omb-fy2027-v1.2026-06-23.draft.jsonl";
const ACCOUNTABILITY_READINESS_REPORT_PATH: &str =
    "data/derived/accountability_evidence/readiness-report.md";
const ACCOUNTABILITY_ACTION_QUEUE_PATH: &str =
    "data/derived/accountability_evidence/action-queue.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_PACKET_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-packet.md";
const ACCOUNTABILITY_WORK_ITEMS_JSONL_PATH: &str =
    "data/derived/accountability_evidence/accountability-work-items.jsonl";
const ACCOUNTABILITY_CLAIM_GUARD_REPORT_PATH: &str =
    "data/derived/accountability_evidence/claim-guard-report.md";
const ACCOUNTABILITY_PUBLIC_QUESTIONS_PATH: &str =
    "data/derived/accountability_evidence/public-questions.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-checklist.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_JSONL_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-checklist.jsonl";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_CLAIM_GATES_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-claim-gates.json";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_DASHBOARD_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-dashboard.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_BRIEF_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-brief.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_LETTER_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-letter.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_RUBRIC_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-rubric.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_FOLLOWUP_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-followup.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-log.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_JSONL_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-log.jsonl";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_SCHEMA_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-log.schema.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-status.json";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DASHBOARD_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-dashboard.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_HANDOFF_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-handoff.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_INTAKE_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-intake.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_INTAKE_SCHEMA_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-intake.schema.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_INTAKE_EXAMPLE_JSONL_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-intake.example.jsonl";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_APPLIED_EXAMPLE_JSONL_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-log.applied-example.jsonl";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_APPLIED_EXAMPLE_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-status.applied-example.json";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DASHBOARD_APPLIED_EXAMPLE_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-dashboard.applied-example.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_HANDOFF_APPLIED_EXAMPLE_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-handoff.applied-example.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_APPLIED_EXAMPLE_SCHEMA_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-applied-example.schema.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-delta.applied-example.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_JSONL_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-delta.applied-example.jsonl";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_SCHEMA_PATH: &str = "data/derived/accountability_evidence/performance-demand-response-delta.applied-example.schema.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-bundle.applied-example.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_JSON_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-response-bundle.applied-example.json";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_SCHEMA_PATH: &str = "data/derived/accountability_evidence/performance-demand-response-bundle.applied-example.schema.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_SCHEMA_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-checklist.schema.md";
const SPEND_CATEGORY_MAP_JSONL_PATH: &str =
    "data/derived/spend_category_map/spend_category_map.fy2025.omb-fy2027-v1.draft.jsonl";
const SPEND_CATEGORY_MAP_README_PATH: &str = "data/derived/spend_category_map/README.md";
const SPEND_CATEGORY_MAP_SCHEMA_PATH: &str =
    "data/derived/spend_category_map/spend_category_map.schema.md";
const SPEND_CATEGORY_MAP_HANDOFF_PATH: &str =
    "data/derived/spend_category_map/accountability-question-handoff.md";
const SPEND_CATEGORY_MAP_DASHBOARD_PATH: &str =
    "data/derived/spend_category_map/spend-category-dashboard.md";
const BREADTH_BENCHMARK_JSONL_PATH: &str =
    "data/derived/breadth_benchmark_matrix/breadth_benchmark_matrix.v1.draft.jsonl";
const BREADTH_BENCHMARK_README_PATH: &str = "data/derived/breadth_benchmark_matrix/README.md";
const BREADTH_BENCHMARK_SCHEMA_PATH: &str =
    "data/derived/breadth_benchmark_matrix/breadth_benchmark_matrix.schema.md";
const BREADTH_BENCHMARK_SCOREBOARD_PATH: &str =
    "docs/reading/current-versus-benchmark-scoreboard.md";
const HEALTH_COST_DECOMPOSITION_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_cost_decomposition.v1.draft.json";
const HEALTH_COST_DECOMPOSITION_READER_PATH: &str = "docs/reading/health-cost-decomposition.md";
const HEALTH_SERVICE_BRIDGE_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_service_price_volume_bridge.cy2024.v1.draft.json";
const HEALTH_SERVICE_BRIDGE_READER_PATH: &str =
    "docs/reading/health-service-price-volume-bridge.md";
const HEALTH_CATEGORY_BENCHMARK_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_category_benchmark_ladder.v1.draft.json";
const HEALTH_CATEGORY_BENCHMARK_READER_PATH: &str =
    "docs/reading/health-category-benchmark-ladder.md";
const HEALTH_TARGET_ADMISSIBILITY_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_target_admissibility.v1.draft.json";
const HEALTH_TARGET_ADMISSIBILITY_READER_PATH: &str = "docs/reading/health-target-admissibility.md";
const HEALTH_SCENARIOS_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_medicare_relative_scenarios.v1.draft.json";
const HEALTH_SCENARIOS_READER_PATH: &str = "docs/reading/health-medicare-relative-scenarios.md";
const HEALTH_SAMPLE_SENSITIVITY_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/health_commercial_sample_sensitivity.v1.draft.json";
const HEALTH_SAMPLE_SENSITIVITY_READER_PATH: &str =
    "docs/reading/health-commercial-sample-sensitivity.md";
const VETERANS_DEPTH_CARD_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/veterans_depth_card.fy2025.v1.draft.json";
const VETERANS_DEPTH_CARD_READER_PATH: &str = "docs/reading/veterans-depth-card.md";
const TRANSPORTATION_DEPTH_CARD_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/transportation_depth_card.fy2025.v1.draft.json";
const TRANSPORTATION_DEPTH_CARD_READER_PATH: &str = "docs/reading/transportation-depth-card.md";
const EDUCATION_DEPTH_CARD_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/education_depth_card.fy2025.v1.draft.json";
const EDUCATION_DEPTH_CARD_READER_PATH: &str = "docs/reading/education-depth-card.md";
const DISASTER_DEPTH_CARD_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/disaster_depth_card.fy2025.v1.draft.json";
const DISASTER_DEPTH_CARD_READER_PATH: &str = "docs/reading/disaster-depth-card.md";
const JUSTICE_DEPTH_CARD_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/justice_depth_card.fy2025.v1.draft.json";
const JUSTICE_DEPTH_CARD_READER_PATH: &str = "docs/reading/justice-depth-card.md";
const SCIENCE_DEPTH_CARD_JSON_PATH: &str = "data/derived/breadth_benchmark_matrix/science_energy_environment_depth_card.fy2025.v1.draft.json";
const SCIENCE_DEPTH_CARD_READER_PATH: &str =
    "docs/reading/science-energy-environment-depth-card.md";
const AGRICULTURE_DEPTH_CARD_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/agriculture_depth_card.fy2025.v1.draft.json";
const AGRICULTURE_DEPTH_CARD_READER_PATH: &str = "docs/reading/agriculture-depth-card.md";
const INTERNATIONAL_DEPTH_CARD_JSON_PATH: &str =
    "data/derived/breadth_benchmark_matrix/international_affairs_depth_card.fy2025.v1.draft.json";
const INTERNATIONAL_DEPTH_CARD_READER_PATH: &str =
    "docs/reading/international-affairs-depth-card.md";
const HEADLINE_BASIS_JSONL_PATH: &str =
    "data/derived/headline_basis_crosswalk/headline_basis_crosswalk.v1.draft.jsonl";
const HEADLINE_BASIS_README_PATH: &str = "data/derived/headline_basis_crosswalk/README.md";
const HEADLINE_BASIS_SCHEMA_PATH: &str =
    "data/derived/headline_basis_crosswalk/headline_basis_crosswalk.schema.md";
const HEADLINE_BASIS_GUIDE_PATH: &str = "docs/reading/headline-number-selection-guide.md";
const EFFICIENCY_PRESSURE_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/efficiency_pressure.fy2025.v1.draft.jsonl";
const EFFICIENCY_PRESSURE_README_PATH: &str = "data/derived/efficiency_pressure/README.md";
const EFFICIENCY_PRESSURE_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/efficiency_pressure.schema.md";
const COST_DOWN_BACKLOG_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_backlog.fy2025.v1.draft.jsonl";
const COST_DOWN_BACKLOG_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_backlog.schema.md";
const COST_DOWN_BACKLOG_READER_PATH: &str = "docs/reading/cost-down-backlog.md";
const COST_DOWN_SOURCE_PACKETS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_source_packets.fy2025.v1.draft.jsonl";
const COST_DOWN_SOURCE_PACKETS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_source_packets.schema.md";
const COST_DOWN_EVIDENCE_QUEUE_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_evidence_queue.fy2025.v1.draft.jsonl";
const COST_DOWN_EVIDENCE_QUEUE_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_evidence_queue.schema.md";
const COST_DOWN_EVIDENCE_QUEUE_READER_PATH: &str = "docs/reading/cost-down-evidence-queue.md";
const COST_DOWN_FIRST_PASS_ROLLUP_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_first_pass_rollup.v1.draft.jsonl";
const COST_DOWN_FIRST_PASS_ROLLUP_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_first_pass_rollup.schema.md";
const COST_DOWN_FIRST_PASS_ROLLUP_READER_PATH: &str = "docs/reading/cost-down-first-pass-rollup.md";
const COST_DOWN_SCORING_READINESS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_scoring_readiness.v1.draft.jsonl";
const COST_DOWN_SCORING_READINESS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/cost_down_scoring_readiness.schema.md";
const COST_DOWN_SCORING_READINESS_READER_PATH: &str = "docs/reading/cost-down-scoring-readiness.md";
const PAYMENT_INTEGRITY_ELIGIBILITY_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/payment_integrity_eligibility_first_pass.jsonl";
const PAYMENT_INTEGRITY_ELIGIBILITY_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/payment_integrity_eligibility_first_pass.schema.md";
const PAYMENT_INTEGRITY_FIRST_PASS_READER_PATH: &str =
    "docs/reading/payment-integrity-first-pass-extract.md";
const PAYMENT_INTEGRITY_SCORECARDS_Q4_2025_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_scorecards_q4_2025_first_pass.jsonl";
const PAYMENT_INTEGRITY_SCORECARDS_Q4_2025_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_scorecards_q4_2025_first_pass.schema.md";
const PAYMENT_INTEGRITY_SCORECARD_READER_PATH: &str =
    "docs/reading/payment-integrity-scorecard-extract.md";
const PAYMENT_INTEGRITY_PROGRAM_REVIEW_GATE_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_gates_q4_2025.jsonl";
const PAYMENT_INTEGRITY_PROGRAM_REVIEW_GATE_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_gates_q4_2025.schema.md";
const PAYMENT_INTEGRITY_PROGRAM_REVIEW_GATE_READER_PATH: &str =
    "docs/reading/payment-integrity-program-review-gates.md";
const PAYMENT_INTEGRITY_PROGRAM_REVIEW_TASKS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_tasks_q4_2025.jsonl";
const PAYMENT_INTEGRITY_PROGRAM_REVIEW_TASKS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_tasks_q4_2025.schema.md";
const PAYMENT_INTEGRITY_PROGRAM_REVIEW_TASKS_READER_PATH: &str =
    "docs/reading/payment-integrity-program-review-tasks.md";
const PAYMENT_INTEGRITY_PROGRAM_REVIEW_STATUS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_status_q4_2025.jsonl";
const PAYMENT_INTEGRITY_PROGRAM_REVIEW_STATUS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_program_review_status_q4_2025.schema.md";
const PAYMENT_INTEGRITY_PROGRAM_REVIEW_STATUS_READER_PATH: &str =
    "docs/reading/payment-integrity-program-review-status.md";
const PAYMENT_INTEGRITY_METHODOLOGY_PLANS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_plans_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_PLANS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_plans_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_PLANS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-plans.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FIELDS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_fields_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_FIELDS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_fields_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FIELDS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-fields.md";
const PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_TARGETS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_source_targets_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_TARGETS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_source_targets_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_TARGETS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-source-targets.md";
const PAYMENT_INTEGRITY_METHODOLOGY_QUERIES_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_queries_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_QUERIES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_queries_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_QUERIES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-queries.md";
const PAYMENT_INTEGRITY_METHODOLOGY_QUERY_RUNS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_query_runs_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_QUERY_RUNS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_query_runs_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_QUERY_RUNS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-query-runs.md";
const PAYMENT_INTEGRITY_METHODOLOGY_RESULTS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_results_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_RESULTS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_results_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_RESULTS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-results.md";
const PAYMENT_INTEGRITY_METHODOLOGY_RESULT_REVIEW_READINESS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_result_review_readiness_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_RESULT_REVIEW_READINESS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_result_review_readiness_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_RESULT_REVIEW_READINESS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-result-review-readiness.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FIELD_REVIEWS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_field_reviews_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_FIELD_REVIEWS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_field_reviews_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FIELD_REVIEWS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-field-reviews.md";
const PAYMENT_INTEGRITY_METHODOLOGY_GAP_FOLLOWUPS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_gap_followups_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_GAP_FOLLOWUPS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_gap_followups_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_GAP_FOLLOWUPS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-gap-followups.md";
const PAYMENT_INTEGRITY_METHODOLOGY_GAP_SOURCE_CAPTURES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_gap_source_captures_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_GAP_SOURCE_CAPTURES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_gap_source_captures_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_GAP_SOURCE_CAPTURES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-gap-source-captures.md";
const PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_CAPTURE_ROLLUP_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_source_capture_rollup_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_CAPTURE_ROLLUP_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_source_capture_rollup_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_CAPTURE_ROLLUP_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-source-capture-rollup.md";
const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_READINESS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_readiness_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_READINESS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_readiness_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_READINESS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-closure-readiness.md";
const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_DECISIONS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_decisions_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_DECISIONS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_decisions_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_DECISIONS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-closure-decisions.md";
const PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_SOURCE_GAPS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_residual_source_gaps_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_SOURCE_GAPS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_residual_source_gaps_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_SOURCE_GAPS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-residual-source-gaps.md";
const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_COVERAGE_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_coverage_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_COVERAGE_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_coverage_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_COVERAGE_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-closure-coverage.md";
const PAYMENT_INTEGRITY_METHODOLOGY_SCORING_GATE_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_scoring_gate_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_SCORING_GATE_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_scoring_gate_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_SCORING_GATE_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-scoring-gate.md";
const PAYMENT_INTEGRITY_METHODOLOGY_PROGRAM_ROLLUP_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_program_rollup_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_PROGRAM_ROLLUP_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_program_rollup_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_PROGRAM_ROLLUP_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-program-rollup.md";
const PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_STATUS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_open_program_status_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_STATUS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_open_program_status_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_STATUS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-open-program-status.md";
const PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_GAP_PRIORITY_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_residual_gap_priority_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_GAP_PRIORITY_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_residual_gap_priority_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_GAP_PRIORITY_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-residual-gap-priority.md";
const PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_SOURCE_WORK_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_priority_source_work_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_SOURCE_WORK_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_priority_source_work_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_SOURCE_WORK_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-priority-source-work.md";
const PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_REVIEWER_ACTIONS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_priority_reviewer_actions_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_REVIEWER_ACTIONS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_priority_reviewer_actions_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_REVIEWER_ACTIONS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-priority-reviewer-actions.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FIELD_UPDATES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_field_updates_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_FIELD_UPDATES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_field_updates_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FIELD_UPDATES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-field-updates.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERIES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_queries_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERIES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_queries_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERIES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-followup-source-queries.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERY_RUNS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_query_runs_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERY_RUNS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_query_runs_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERY_RUNS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-followup-source-query-runs.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_captures_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_captures_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-followup-source-captures.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURE_ROLLUP_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_capture_rollup_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURE_ROLLUP_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_capture_rollup_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURE_ROLLUP_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-followup-source-capture-rollup.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_DECISIONS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_boundary_decisions_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_DECISIONS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_boundary_decisions_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_DECISIONS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-followup-boundary-decisions.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_READINESS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_boundary_readiness_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_READINESS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_boundary_readiness_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_READINESS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-followup-boundary-readiness.md";
const PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_CANDIDATES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_narrow_closure_candidates_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_CANDIDATES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_narrow_closure_candidates_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_CANDIDATES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-narrow-closure-candidates.md";
const PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_DECISIONS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_narrow_closure_decisions_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_DECISIONS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_narrow_closure_decisions_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_DECISIONS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-narrow-closure-decisions.md";
const PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_COMPONENT_PROGRESS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_open_program_component_progress_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_COMPONENT_PROGRESS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_open_program_component_progress_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_COMPONENT_PROGRESS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-open-program-component-progress.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_REQUIREMENTS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_requirements_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_REQUIREMENTS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_requirements_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_REQUIREMENTS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-requirements.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_TARGETS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_targets_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_TARGETS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_targets_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_TARGETS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-source-targets.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERIES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_queries_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERIES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_queries_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERIES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-source-queries.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERY_RUNS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_query_runs_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERY_RUNS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_query_runs_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERY_RUNS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-source-query-runs.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_captures_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_captures_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-source-captures.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURE_ROLLUPS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_capture_rollups_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURE_ROLLUPS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_source_capture_rollups_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURE_ROLLUPS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-source-capture-rollups.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_DECISIONS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_boundary_decisions_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_DECISIONS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_boundary_decisions_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_DECISIONS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-boundary-decisions.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_READINESS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_boundary_readiness_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_READINESS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_boundary_readiness_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_READINESS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-boundary-readiness.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_CANDIDATES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_narrow_candidates_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_CANDIDATES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_narrow_candidates_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_CANDIDATES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-narrow-candidates.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_DECISIONS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_narrow_decisions_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_DECISIONS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_narrow_decisions_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_DECISIONS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-narrow-decisions.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-progress.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_REQUIREMENTS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_requirements_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_REQUIREMENTS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_requirements_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_REQUIREMENTS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-progress-requirements.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_TARGETS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_targets_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_TARGETS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_targets_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_TARGETS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-progress-source-targets.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERIES_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_queries_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERIES_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_queries_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERIES_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-progress-source-queries.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERY_RUNS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_query_runs_q4_2025.jsonl";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERY_RUNS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_methodology_component_gate_progress_source_query_runs_q4_2025.schema.md";
const PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERY_RUNS_READER_PATH: &str =
    "docs/reading/payment-integrity-methodology-component-gate-progress-source-query-runs.md";
const PAYMENT_INTEGRITY_NEXT_PROGRAM_SELECTION_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_next_program_selection_q4_2025.jsonl";
const PAYMENT_INTEGRITY_NEXT_PROGRAM_SELECTION_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_next_program_selection_q4_2025.schema.md";
const PAYMENT_INTEGRITY_NEXT_PROGRAM_SELECTION_READER_PATH: &str =
    "docs/reading/payment-integrity-next-program-selection.md";
const PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_JSONL_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_claims_timeliness_first_pass.jsonl";
const PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_SCHEMA_PATH: &str = "data/derived/efficiency_pressure/extracts/payment_integrity_claims_timeliness_first_pass.schema.md";
const PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_READER_PATH: &str =
    "docs/reading/payment-integrity-claims-timeliness-extract.md";
const DEBT_MATURITY_RISK_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/debt_maturity_risk_first_pass.jsonl";
const DEBT_MATURITY_RISK_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/debt_maturity_risk_first_pass.schema.md";
const DEBT_MATURITY_RISK_EXTRACT_READER_PATH: &str = "docs/reading/debt-maturity-risk-extract.md";
const DEBT_PRIMARY_BALANCE_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/debt_primary_balance_first_pass.jsonl";
const DEBT_PRIMARY_BALANCE_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/debt_primary_balance_first_pass.schema.md";
const DEBT_PRIMARY_BALANCE_EXTRACT_READER_PATH: &str =
    "docs/reading/debt-primary-balance-extract.md";
const DISASTER_SUPPLEMENTAL_TRACKING_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/disaster_supplemental_tracking_first_pass.jsonl";
const DISASTER_SUPPLEMENTAL_TRACKING_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/disaster_supplemental_tracking_first_pass.schema.md";
const DISASTER_SUPPLEMENTAL_TRACKING_EXTRACT_READER_PATH: &str =
    "docs/reading/disaster-supplemental-tracking-extract.md";
const DISASTER_MITIGATION_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/disaster_mitigation_first_pass.jsonl";
const DISASTER_MITIGATION_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/disaster_mitigation_first_pass.schema.md";
const DISASTER_MITIGATION_EXTRACT_READER_PATH: &str = "docs/reading/disaster-mitigation-extract.md";
const DEFENSE_AUDIT_CONTROL_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/defense_audit_control_first_pass.jsonl";
const DEFENSE_AUDIT_CONTROL_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/defense_audit_control_first_pass.schema.md";
const DEFENSE_AUDIT_CONTROL_EXTRACT_READER_PATH: &str =
    "docs/reading/defense-audit-control-extract.md";
const DEFENSE_PROCUREMENT_CONTROL_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/defense_procurement_control_first_pass.jsonl";
const DEFENSE_PROCUREMENT_CONTROL_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/defense_procurement_control_first_pass.schema.md";
const DEFENSE_PROCUREMENT_CONTROL_EXTRACT_READER_PATH: &str =
    "docs/reading/defense-procurement-control-extract.md";
const HEALTH_PRICE_DISCIPLINE_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/health_price_discipline_first_pass.jsonl";
const HEALTH_PRICE_DISCIPLINE_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/health_price_discipline_first_pass.schema.md";
const HEALTH_PRICE_DISCIPLINE_EXTRACT_READER_PATH: &str =
    "docs/reading/health-price-discipline-extract.md";
const HEALTH_ADMIN_SIMPLIFICATION_FIRST_PASS_JSONL_PATH: &str =
    "data/derived/efficiency_pressure/extracts/health_admin_simplification_first_pass.jsonl";
const HEALTH_ADMIN_SIMPLIFICATION_FIRST_PASS_SCHEMA_PATH: &str =
    "data/derived/efficiency_pressure/extracts/health_admin_simplification_first_pass.schema.md";
const HEALTH_ADMIN_SIMPLIFICATION_EXTRACT_READER_PATH: &str =
    "docs/reading/health-administrative-simplification-extract.md";
const HEALTH_PRICE_DISCIPLINE_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/health-price-discipline-source-packet.md";
const HEALTH_ADMIN_SIMPLIFICATION_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/health-administrative-simplification-source-packet.md";
const DEBT_PRIMARY_BALANCE_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/debt-primary-balance-source-packet.md";
const DEBT_MATURITY_RISK_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/debt-maturity-risk-source-packet.md";
const DEFENSE_PROCUREMENT_CONTROL_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/defense-procurement-control-source-packet.md";
const DEFENSE_AUDIT_CONTROL_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/defense-audit-control-source-packet.md";
const DISASTER_MITIGATION_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/disaster-mitigation-source-packet.md";
const DISASTER_SUPPLEMENTAL_TRACKING_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/disaster-supplemental-tracking-source-packet.md";
const PAYMENT_INTEGRITY_ELIGIBILITY_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/payment-integrity-eligibility-source-packet.md";
const PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_SOURCE_PACKET_READER_PATH: &str =
    "docs/reading/payment-integrity-claims-timeliness-source-packet.md";
const EFFICIENCY_PRESSURE_RESEARCH_PATH: &str =
    "docs/research/2026-06-28-efficiency-pressure-framework.md";
const PER_UNIT_DISPLAY_READINESS_JSONL_PATH: &str =
    "data/derived/denominator_requirements/per_unit_display_readiness.v1.draft.jsonl";
const PER_UNIT_RECEIPT_CARDS_JSONL_PATH: &str =
    "data/derived/denominator_requirements/per_unit_receipt_cards.v1.draft.jsonl";
const PER_UNIT_DISPLAY_READINESS_DASHBOARD_PATH: &str =
    "data/derived/denominator_requirements/per-unit-display-readiness.md";
const PER_UNIT_RECEIPT_CARDS_READER_PATH: &str = "docs/reading/per-unit-receipt-cards.md";
const ACCOUNTABILITY_ARTIFACT_MAP_PATH: &str =
    "data/derived/accountability_evidence/artifact-map.md";
const ACCOUNTABILITY_PUBLIC_BRIEF_PATH: &str = "docs/reading/accountability-public-brief.md";
const README_PATH: &str = "README.md";
const READING_INDEX_PATH: &str = "docs/reading/README.md";
const SOURCE_VERSION_LEDGER_PATH: &str = "docs/sources/source-version-ledger.md";
const OBSERVED_DATE: &str = "2026-06-21";
const MODEL_ID: &str = "individual-income-tax-proportional-outlays-v1";
const SUBFUNCTION_MODEL_ID: &str = "individual-income-tax-proportional-subfunction-outlays-v1";
const TABLE_1_1_PATH: &str = "data/raw/omb/SRC-OMB-HIST-1-1-FY2027/2026-06-21/hist01z1_fy2027.xlsx";
const TABLE_2_1_PATH: &str = "data/raw/omb/SRC-OMB-HIST-2-1-FY2027/2026-06-21/hist02z1_fy2027.xlsx";
const TABLE_2_2_PATH: &str = "data/raw/omb/SRC-OMB-HIST-2-2-FY2027/2026-06-21/hist02z2_fy2027.xlsx";
const TABLE_3_1_PATH: &str = "data/raw/omb/SRC-OMB-HIST-3-1-FY2027/2026-06-21/hist03z1_fy2027.xlsx";
const TABLE_3_2_PATH: &str = "data/raw/omb/SRC-OMB-HIST-3-2-FY2027/2026-06-21/hist03z2_fy2027.xlsx";
const RECEIPT_SHARE_JSONL_PATH: &str =
    "data/extracted/receipt_source/receipt_source.SRC-OMB-HIST-2-2-FY2027.2026-06-21.draft.jsonl";
const RECEIPT_SHARE_PROFILE_PATH: &str = "data/extracted/receipt_source/table-2-2-profile.md";
const OUTLAY_FUNCTION_3_1_JSONL_PATH: &str =
    "data/extracted/outlay_function/outlay_function.SRC-OMB-HIST-3-1-FY2027.2026-06-21.draft.jsonl";
const OUTLAY_FUNCTION_3_1_PROFILE_PATH: &str =
    "data/extracted/outlay_function/table-3-1-profile.md";
const OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_JSONL_PATH: &str = "data/extracted/outlay_function/outlay_function.SRC-OMB-HIST-3-2-FY2027.2026-06-21.national-defense.draft.jsonl";
const OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_PROFILE_PATH: &str =
    "data/extracted/outlay_function/table-3-2-national-defense-profile.md";
const OUTLAY_FUNCTION_3_2_JSONL_PATH: &str =
    "data/extracted/outlay_function/outlay_function.SRC-OMB-HIST-3-2-FY2027.2026-06-21.draft.jsonl";
const OUTLAY_FUNCTION_3_2_PROFILE_PATH: &str =
    "data/extracted/outlay_function/table-3-2-profile.md";
const TABLE_6_1_PATH: &str = "data/raw/omb/SRC-OMB-HIST-6-1-FY2027/2026-06-24/hist06z1_fy2027.xlsx";
const OUTLAY_COMPOSITION_6_1_NATIONAL_DEFENSE_JSONL_PATH: &str = "data/extracted/outlay_composition/outlay_composition.SRC-OMB-HIST-6-1-FY2027.2026-06-24.national-defense-gdp.draft.jsonl";
const OUTLAY_COMPOSITION_6_1_NATIONAL_DEFENSE_PROFILE_PATH: &str =
    "data/extracted/outlay_composition/table-6-1-national-defense-gdp-profile.md";
const OBSERVED_DATE_6_1: &str = "2026-06-24";
const SOURCE_IDS: &[&str] = &[
    "SRC-OMB-HIST-1-1-FY2027",
    "SRC-OMB-HIST-2-1-FY2027",
    "SRC-OMB-HIST-3-1-FY2027",
];

const BROAD_CATEGORIES: &[(&str, &str, i64)] = &[
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

const ANNUAL_HEADERS: &[&str] = &[
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

const DECADE_HEADERS: &[&str] = &[
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

const CATEGORY_FIELDS: &[(&str, &str)] = &[
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

const SUBFUNCTION_ANNUAL_HEADERS: &[&str] = &[
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

const SUBFUNCTION_TOP_HEADERS: &[&str] = &[
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

const SUBFUNCTION_DECADE_HEADERS: &[&str] = &[
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

#[derive(Clone, Copy)]
struct Artifact {
    path: &'static str,
    role: &'static str,
    grain: &'static str,
    kind: &'static str,
    canonical: &'static str,
}

impl Artifact {
    fn metadata(&self) -> ArtifactMetadata<'_> {
        ArtifactMetadata {
            path: self.path,
            role: self.role,
            grain: self.grain,
            kind: self.kind,
            canonical: self.canonical,
        }
    }
}

const ARTIFACTS: &[Artifact] = &[
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
];

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.as_slice() {
        [area, command] if area == "income-tax-outlay" && command == "validate" => {
            run_income_tax_outlay_validation()
        }
        [area, command, flag]
            if area == "income-tax-outlay" && command == "model" && flag == "--check" =>
        {
            run_model_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "model" => run_model_write(),
        [area, command, flag]
            if area == "income-tax-outlay"
                && command == "subfunction-model"
                && flag == "--check" =>
        {
            run_subfunction_model_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "subfunction-model" => {
            run_subfunction_model_write()
        }
        [area, command, flag]
            if area == "income-tax-outlay"
                && command == "subfunction-export"
                && flag == "--check" =>
        {
            run_subfunction_export_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "subfunction-export" => {
            run_subfunction_export_write()
        }
        [area, command, flag]
            if area == "income-tax-outlay" && command == "summary" && flag == "--check" =>
        {
            run_summary_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "summary" => {
            run_summary_write()
        }
        [area, command, flag]
            if area == "income-tax-outlay" && command == "export" && flag == "--check" =>
        {
            run_export_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "export" => run_export_write(),
        [area, command, flag]
            if area == "income-tax-outlay" && command == "manifest" && flag == "--check" =>
        {
            run_manifest_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "manifest" => {
            run_manifest_write()
        }
        [area, command, flag]
            if area == "receipt-source" && command == "table-2-2" && flag == "--check" =>
        {
            run_table_2_2_check()
        }
        [area, command] if area == "receipt-source" && command == "table-2-2" => {
            run_table_2_2_write()
        }
        [area, command, flag]
            if area == "outlay-function" && command == "table-3-1" && flag == "--check" =>
        {
            run_table_3_1_check()
        }
        [area, command] if area == "outlay-function" && command == "table-3-1" => {
            run_table_3_1_write()
        }
        [area, command, flag]
            if area == "outlay-function"
                && command == "table-3-2-national-defense"
                && flag == "--check" =>
        {
            run_table_3_2_national_defense_check()
        }
        [area, command] if area == "outlay-function" && command == "table-3-2-national-defense" => {
            run_table_3_2_national_defense_write()
        }
        [area, command, flag]
            if area == "outlay-composition"
                && command == "table-6-1-national-defense"
                && flag == "--check" =>
        {
            run_table_6_1_national_defense_check()
        }
        [area, command]
            if area == "outlay-composition" && command == "table-6-1-national-defense" =>
        {
            run_table_6_1_national_defense_write()
        }
        [area, command, flag]
            if area == "outlay-function" && command == "table-3-2" && flag == "--check" =>
        {
            run_table_3_2_check()
        }
        [area, command] if area == "outlay-function" && command == "table-3-2" => {
            run_table_3_2_write()
        }
        _ => {
            eprintln!(
                "usage: taxlane-tools income-tax-outlay <validate|model [--check]|subfunction-model [--check]|subfunction-export [--check]|summary [--check]|export [--check]|manifest [--check]>\n       taxlane-tools receipt-source table-2-2 [--check]\n       taxlane-tools outlay-function table-3-1 [--check]\n       taxlane-tools outlay-function table-3-2-national-defense [--check]\n       taxlane-tools outlay-function table-3-2 [--check]\n       taxlane-tools outlay-composition table-6-1-national-defense [--check]"
            );
            ExitCode::from(2)
        }
    }
}

fn run_income_tax_outlay_validation() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };

    if let Err(err) = build_annual_model(&root, true) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = build_decade_summary(&root, true) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = export_chart_views(&root, true) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = export_subfunction_chart_views(&root, true) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_manifest(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_accountability_evidence_records(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_program_lane_records(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_spend_category_map(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_breadth_benchmark_matrix(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_headline_basis_crosswalk(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_efficiency_pressure_records(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_per_unit_display_records(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_readiness_report(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_action_queue(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_packet(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_work_items(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_claim_guard_report(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_public_questions(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_public_brief(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_public_brief_discovery(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_artifact_map(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_checklist(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_checklist_jsonl(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_claim_gates(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_dashboard(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_brief(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_letter(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_rubric(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_followup(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_log(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_log_jsonl(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_log_schema(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_status(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_dashboard(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_handoff(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_intake(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_intake_schema(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_intake_example_jsonl(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) =
        check_accountability_performance_demand_response_log_applied_example_jsonl(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_status_applied_example(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) =
        check_accountability_performance_demand_response_dashboard_applied_example(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) =
        check_accountability_performance_demand_response_handoff_applied_example(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_applied_example_schema(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_delta_applied_example(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) =
        check_accountability_performance_demand_response_delta_applied_example_jsonl(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) =
        check_accountability_performance_demand_response_delta_applied_example_schema(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_bundle_applied_example(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) =
        check_accountability_performance_demand_response_bundle_applied_example_json(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) =
        check_accountability_performance_demand_response_bundle_applied_example_schema(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    for spec in CHART_SPECS {
        if let Err(err) = parse_json(&root.join(spec)) {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
        println!("validated JSON spec: {spec}");
    }

    if let Err(err) = validate_placeholder_receipt_chart_sync(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    println!(
        "validated income-tax outlay model checks and {} chart specs",
        CHART_SPECS.len()
    );
    ExitCode::SUCCESS
}

fn run_model_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_annual_model(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_model_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_annual_model(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_subfunction_model_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_subfunction_model(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_subfunction_model_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_subfunction_model(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_subfunction_export_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match export_subfunction_chart_views(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_subfunction_export_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match export_subfunction_chart_views(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_summary_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_decade_summary(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_summary_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_decade_summary(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_export_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match export_chart_views(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_export_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match export_chart_views(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_manifest_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match check_manifest(&root) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_manifest_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_manifest(&root).and_then(|manifest| {
        fs::write(root.join(MANIFEST_PATH), manifest)
            .map_err(|err| format!("failed to write {MANIFEST_PATH}: {err}"))
    }) {
        Ok(()) => {
            println!("wrote {MANIFEST_PATH}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_table_2_2_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_receipt_share_table_2_2(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_table_2_2_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_receipt_share_table_2_2(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_table_3_1_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_1(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_table_3_1_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_1(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_table_3_2_national_defense_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_2_national_defense(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_table_3_2_national_defense_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_2_national_defense(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_table_6_1_national_defense_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_composition_table_6_1_national_defense(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_table_6_1_national_defense_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_composition_table_6_1_national_defense(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_table_3_2_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_2(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_table_3_2_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_2(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn repo_root() -> Result<PathBuf, String> {
    env::current_dir().map_err(|err| format!("failed to get current directory: {err}"))
}

fn parse_json(path: &Path) -> Result<(), String> {
    let file = File::open(path).map_err(|err| format!("failed to open {:?}: {err}", path))?;
    serde_json::from_reader::<_, serde_json::Value>(file)
        .map_err(|err| format!("failed to parse {:?}: {err}", path))?;
    Ok(())
}

fn read_json(path: &Path) -> Result<serde_json::Value, String> {
    let file = File::open(path).map_err(|err| format!("failed to open {:?}: {err}", path))?;
    serde_json::from_reader::<_, serde_json::Value>(file)
        .map_err(|err| format!("failed to parse {:?}: {err}", path))
}

fn validate_placeholder_receipt_chart_sync(root: &Path) -> Result<(), String> {
    let receipt = read_json(&root.join(PLACEHOLDER_RECEIPT_JSON_PATH))?;
    let lane_spec = read_json(&root.join(PLACEHOLDER_RECEIPT_LANE_BARS_SPEC_PATH))?;
    let context_spec = read_json(&root.join(PLACEHOLDER_RECEIPT_FINANCING_CONTEXT_SPEC_PATH))?;

    let receipt_lanes = receipt
        .get("lane_allocations")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "placeholder receipt missing lane_allocations".to_string())?;
    let chart_lanes = lane_spec
        .pointer("/data/values")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "placeholder lane chart missing data.values".to_string())?;

    if receipt_lanes.len() != chart_lanes.len() {
        return Err(format!(
            "placeholder lane chart has {} rows but receipt has {} rows",
            chart_lanes.len(),
            receipt_lanes.len()
        ));
    }

    let mut chart_by_label = BTreeMap::new();
    for row in chart_lanes {
        let lane = row
            .get("lane")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "placeholder lane chart row missing lane".to_string())?;
        chart_by_label.insert(lane.to_string(), row);
    }

    for lane in receipt_lanes {
        let label = lane
            .get("public_label")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "placeholder receipt lane missing public_label".to_string())?;
        let chart = chart_by_label
            .get(label)
            .ok_or_else(|| format!("placeholder lane chart missing lane {label}"))?;
        assert_number_close(
            chart,
            "amount",
            number_field(lane, "placeholder_allocation_amount_rounded_usd")?,
            0.000001,
            &format!("placeholder lane chart amount for {label}"),
        )?;
        assert_number_close(
            chart,
            "share",
            number_field(lane, "allocation_share_percent")?,
            0.000001,
            &format!("placeholder lane chart share for {label}"),
        )?;
        let expected_treatment = chart_treatment_for_lane(lane)?;
        let actual_treatment = chart
            .get("treatment")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| format!("placeholder lane chart missing treatment for {label}"))?;
        if actual_treatment != expected_treatment {
            return Err(format!(
                "placeholder lane chart treatment for {label}: expected {expected_treatment:?}, found {actual_treatment:?}"
            ));
        }
    }

    let context_rows = context_spec
        .pointer("/data/values")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "placeholder financing context chart missing data.values".to_string())?;
    let mut context_by_measure = BTreeMap::new();
    for row in context_rows {
        let measure = row
            .get("measure")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "placeholder financing context row missing measure".to_string())?;
        context_by_measure.insert(measure.to_string(), row);
    }
    let borrowed = context_by_measure
        .get("Borrowed share of outlays")
        .ok_or_else(|| "placeholder financing context missing borrowed share".to_string())?;
    assert_number_close(
        borrowed,
        "percent",
        number_field(&receipt, "borrowed_share_percent_of_outlays")?,
        0.000001,
        "placeholder financing context borrowed share",
    )?;
    let coverage = context_by_measure
        .get("Individual income-tax coverage of outlays")
        .ok_or_else(|| "placeholder financing context missing income-tax coverage".to_string())?;
    assert_number_close(
        coverage,
        "percent",
        number_field(&receipt, "income_tax_coverage_percent_of_outlays")?,
        0.000001,
        "placeholder financing context income-tax coverage",
    )?;

    println!("validated placeholder receipt chart sync");
    Ok(())
}

fn chart_treatment_for_lane(lane: &serde_json::Value) -> Result<&'static str, String> {
    match string_field(lane, "display_treatment")?.as_str() {
        "modeled_lane" => Ok("Modeled lane"),
        "dedicated_financing_caveat_required" => Ok("Dedicated-financing caveat"),
        "display_separately" => match string_field(lane, "spending_control")?.as_str() {
            "net-interest" => Ok("Financing cost"),
            "offsetting" => Ok("Offset"),
            other => Err(format!(
                "unknown display_separately spending_control {other:?}"
            )),
        },
        "negative_or_offset_sensitive_lane" => Ok("Offset-sensitive adjustment"),
        other => Err(format!("unknown display_treatment {other:?}")),
    }
}

fn assert_number_close(
    row: &serde_json::Value,
    field: &str,
    expected: f64,
    tolerance: f64,
    label: &str,
) -> Result<(), String> {
    let actual = number_field(row, field)?;
    if (actual - expected).abs() > tolerance {
        return Err(format!("{label}: expected {expected}, found {actual}"));
    }
    Ok(())
}

fn build_receipt_share_table_2_2(root: &Path, check_only: bool) -> Result<(), String> {
    let rows = build_receipt_share_rows(root)?;
    validate_receipt_share_rows(&rows)?;
    let jsonl = receipt_share_jsonl(&rows);
    let markdown = receipt_share_profile_markdown(&rows)?;

    if check_only {
        compare_text(
            root,
            RECEIPT_SHARE_JSONL_PATH,
            &jsonl,
            "Table 2.2 receipt share JSONL",
        )?;
        compare_text(
            root,
            RECEIPT_SHARE_PROFILE_PATH,
            &markdown,
            "Table 2.2 receipt share profile",
        )?;
    } else {
        fs::write(root.join(RECEIPT_SHARE_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {RECEIPT_SHARE_JSONL_PATH}: {err}"))?;
        fs::write(root.join(RECEIPT_SHARE_PROFILE_PATH), markdown)
            .map_err(|err| format!("failed to write {RECEIPT_SHARE_PROFILE_PATH}: {err}"))?;
    }

    let first_year = rows
        .first()
        .ok_or_else(|| "no Table 2.2 rows".to_string())?
        .fiscal_year;
    let last_year = rows
        .last()
        .ok_or_else(|| "no Table 2.2 rows".to_string())?
        .fiscal_year;
    println!(
        "validated {} Table 2.2 receipt share rows for {}-{}",
        rows.len(),
        first_year,
        last_year
    );
    Ok(())
}

fn build_outlay_function_table_3_1(root: &Path, check_only: bool) -> Result<(), String> {
    let (rows, profile) = build_outlay_function_3_1_rows(root)?;
    validate_outlay_function_3_1_rows(&rows, &profile)?;
    let jsonl = outlay_function_3_1_jsonl(&rows);
    let markdown = outlay_function_3_1_profile_markdown(&profile);

    if check_only {
        compare_text(
            root,
            OUTLAY_FUNCTION_3_1_JSONL_PATH,
            &jsonl,
            "Table 3.1 outlay function JSONL",
        )?;
        compare_text(
            root,
            OUTLAY_FUNCTION_3_1_PROFILE_PATH,
            &markdown,
            "Table 3.1 outlay function profile",
        )?;
    } else {
        fs::write(root.join(OUTLAY_FUNCTION_3_1_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {OUTLAY_FUNCTION_3_1_JSONL_PATH}: {err}"))?;
        fs::write(root.join(OUTLAY_FUNCTION_3_1_PROFILE_PATH), markdown)
            .map_err(|err| format!("failed to write {OUTLAY_FUNCTION_3_1_PROFILE_PATH}: {err}"))?;
    }

    println!(
        "validated {} Table 3.1 outlay function rows for {}-{}",
        rows.len(),
        profile.first_year,
        profile.last_year
    );
    Ok(())
}

fn build_outlay_function_table_3_2_national_defense(
    root: &Path,
    check_only: bool,
) -> Result<(), String> {
    let (rows, profile) = build_table_3_2_national_defense_rows(root)?;
    validate_table_3_2_national_defense_rows(&rows, &profile)?;
    let jsonl = table_3_2_national_defense_jsonl(&rows);
    let markdown = table_3_2_national_defense_profile_markdown(&profile);

    if check_only {
        compare_text(
            root,
            OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_JSONL_PATH,
            &jsonl,
            "Table 3.2 National Defense JSONL",
        )?;
        compare_text(
            root,
            OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_PROFILE_PATH,
            &markdown,
            "Table 3.2 National Defense profile",
        )?;
    } else {
        fs::write(
            root.join(OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_JSONL_PATH),
            jsonl,
        )
        .map_err(|err| {
            format!("failed to write {OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_JSONL_PATH}: {err}")
        })?;
        fs::write(
            root.join(OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_PROFILE_PATH),
            markdown,
        )
        .map_err(|err| {
            format!("failed to write {OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_PROFILE_PATH}: {err}")
        })?;
    }

    println!(
        "validated {} Table 3.2 National Defense rows for {}-{}",
        rows.len(),
        profile.first_year,
        profile.last_year
    );
    Ok(())
}

fn build_outlay_function_table_3_2(root: &Path, check_only: bool) -> Result<(), String> {
    let (rows, profile) = build_table_3_2_rows(root)?;
    validate_table_3_2_rows(&profile)?;
    let jsonl = table_3_2_jsonl(&rows);
    let markdown = table_3_2_profile_markdown(&profile);

    if check_only {
        compare_text(
            root,
            OUTLAY_FUNCTION_3_2_JSONL_PATH,
            &jsonl,
            "Table 3.2 JSONL",
        )?;
        compare_text(
            root,
            OUTLAY_FUNCTION_3_2_PROFILE_PATH,
            &markdown,
            "Table 3.2 profile",
        )?;
    } else {
        fs::write(root.join(OUTLAY_FUNCTION_3_2_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {OUTLAY_FUNCTION_3_2_JSONL_PATH}: {err}"))?;
        fs::write(root.join(OUTLAY_FUNCTION_3_2_PROFILE_PATH), markdown)
            .map_err(|err| format!("failed to write {OUTLAY_FUNCTION_3_2_PROFILE_PATH}: {err}"))?;
    }

    println!(
        "validated {} Table 3.2 rows for {}-{}",
        rows.len(),
        profile.first_year,
        profile.last_year
    );
    Ok(())
}

#[derive(Clone, Copy)]
struct ReceiptShareCategory {
    column: &'static str,
    receipt_category: &'static str,
    source_receipt_label: &'static str,
    allocation_status: &'static str,
    notes: &'static str,
}

const RECEIPT_SHARE_CATEGORIES: &[ReceiptShareCategory] = &[
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

#[derive(Clone)]
struct ReceiptShareRow {
    fiscal_year: i64,
    source_row: i64,
    source_column: &'static str,
    receipt_category: &'static str,
    source_receipt_label: &'static str,
    percent: f64,
    actual_or_projection: &'static str,
    allocation_status: &'static str,
    notes: &'static str,
}

#[derive(Clone)]
struct OutlayFunctionRow {
    fiscal_year: i64,
    source_column: String,
    function_code: String,
    function_label: String,
    source_row: i64,
    amount: f64,
    actual_or_projection: &'static str,
    offsetting_treatment: &'static str,
    notes: &'static str,
    include_table_1_1_source: bool,
    table_1_1_row: Option<i64>,
}

struct OutlayFunctionCheck {
    year: i64,
    table_1_1_outlays: f64,
    table_3_1_total: f64,
    broad_category_total: f64,
    total_difference: f64,
    broad_category_difference: f64,
}

struct OutlayFunctionProfile {
    first_year: i64,
    last_year: i64,
    year_count: usize,
    record_count: usize,
    checks: Vec<OutlayFunctionCheck>,
}

#[derive(Clone, Copy)]
struct Table32NationalDefenseLine {
    source_row: i64,
    subfunction_code: Option<&'static str>,
    subfunction_label: Option<&'static str>,
    source_label: &'static str,
    notes: &'static str,
}

const TABLE_3_2_NATIONAL_DEFENSE_LINES: &[Table32NationalDefenseLine] = &[
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

#[derive(Clone)]
struct Table32OutlayFunctionRow {
    fiscal_year: i64,
    source_column: String,
    source_row: i64,
    function_code: &'static str,
    function_label: &'static str,
    subfunction_code: Option<&'static str>,
    subfunction_label: Option<&'static str>,
    source_label: &'static str,
    amount: f64,
    notes: &'static str,
}

struct Table32NationalDefenseCheck {
    year: i64,
    table_3_1_national_defense: f64,
    table_3_2_national_defense: f64,
    subfunction_total: f64,
    table_3_1_difference: f64,
    subfunction_difference: f64,
}

struct Table32NationalDefenseProfile {
    first_year: i64,
    last_year: i64,
    year_count: usize,
    record_count: usize,
    checks: Vec<Table32NationalDefenseCheck>,
}

#[derive(Clone)]
enum Table32LineKind {
    Subfunction,
    FunctionTotal,
    GrandTotal,
}

#[derive(Clone)]
struct Table32Line {
    source_row: i64,
    function_code: String,
    function_label: String,
    subfunction_code: Option<String>,
    subfunction_label: Option<String>,
    source_label: String,
    kind: Table32LineKind,
}

#[derive(Clone)]
struct Table32Row {
    fiscal_year: i64,
    source_column: String,
    source_row: i64,
    function_code: String,
    function_label: String,
    subfunction_code: Option<String>,
    subfunction_label: Option<String>,
    source_label: String,
    amount: f64,
    kind: Table32LineKind,
}

struct Table32FunctionCheck {
    year: i64,
    function_code: String,
    function_label: String,
    function_total: f64,
    subfunction_total: f64,
    difference: f64,
}

struct Table32GrandCheck {
    year: i64,
    table_3_1_total_outlays: f64,
    table_3_2_total_outlays: f64,
    function_total_sum: f64,
    table_3_1_difference: f64,
    function_total_difference: f64,
}

struct Table32Profile {
    first_year: i64,
    last_year: i64,
    year_count: usize,
    record_count: usize,
    line_count: usize,
    subfunction_line_count: usize,
    function_total_line_count: usize,
    function_count: usize,
    grand_checks: Vec<Table32GrandCheck>,
    function_checks: Vec<Table32FunctionCheck>,
}

#[derive(Clone)]
struct SubfunctionModelRow {
    fiscal_year: i64,
    source_column: String,
    source_row: i64,
    function_code: String,
    function_label: String,
    subfunction_code: String,
    subfunction_label: String,
    subfunction_outlays_amount: f64,
    subfunction_total_outlays_amount: f64,
    total_outlays_amount: f64,
    individual_income_tax_receipts_amount: f64,
    outlay_share_percent: f64,
    allocation_share_percent: f64,
    modeled_income_tax_allocation_amount: f64,
}

struct SubfunctionModelCheck {
    year: i64,
    table_3_2_total_outlays: f64,
    subfunction_total: f64,
    individual_income_tax: f64,
    modeled_sum: f64,
    subfunction_total_difference: f64,
}

struct SubfunctionModelProfile {
    first_year: i64,
    last_year: i64,
    year_count: usize,
    record_count: usize,
    subfunction_count: usize,
    checks: Vec<SubfunctionModelCheck>,
}

fn build_receipt_share_rows(root: &Path) -> Result<Vec<ReceiptShareRow>, String> {
    let sheet = read_sheet(&root.join(TABLE_2_2_PATH))?;
    let mut rows = Vec::new();

    for (row_num, cells) in &sheet {
        let Some(year_label) = table_2_2_year_label(cells.get("A")) else {
            continue;
        };
        let Some((year, actual_or_projection)) = parse_table_2_2_year(&year_label) else {
            continue;
        };

        for category in RECEIPT_SHARE_CATEGORIES {
            let Some(percent) = number_cell(cells.get(category.column)) else {
                return Err(format!(
                    "Table 2.2 row {row_num} missing percent in column {}",
                    category.column
                ));
            };
            rows.push(ReceiptShareRow {
                fiscal_year: year,
                source_row: *row_num,
                source_column: category.column,
                receipt_category: category.receipt_category,
                source_receipt_label: category.source_receipt_label,
                percent: round6(percent),
                actual_or_projection,
                allocation_status: category.allocation_status,
                notes: category.notes,
            });
        }
    }

    rows.sort_by_key(|row| {
        (
            row.fiscal_year,
            receipt_share_sort_key(row.receipt_category),
        )
    });
    Ok(rows)
}

fn build_outlay_function_3_1_rows(
    root: &Path,
) -> Result<(Vec<OutlayFunctionRow>, OutlayFunctionProfile), String> {
    let t11 = parse_table_1_1(&read_sheet(&root.join(TABLE_1_1_PATH))?);
    let sheet = read_sheet(&root.join(TABLE_3_1_PATH))?;
    let (years_31, t31) = parse_table_3_1(&sheet)?;
    let columns_by_year = table_3_1_year_columns(&sheet)?;
    let years: Vec<i64> = years_31
        .into_iter()
        .filter(|year| (1940..=2025).contains(year))
        .collect();

    let mut rows = Vec::new();
    let mut checks = Vec::new();
    let mut errors = Vec::new();

    for year in &years {
        let Some(table_11) = t11.get(year) else {
            errors.push(format!("{year}: missing Table 1.1 row"));
            continue;
        };
        let Some(source_column) = columns_by_year.get(year) else {
            errors.push(format!("{year}: missing Table 3.1 source column"));
            continue;
        };
        let Some(total_outlays_31) = t31
            .get("total-federal-outlays")
            .and_then(|values| values.get(year))
            .copied()
        else {
            errors.push(format!("{year}: missing Table 3.1 total outlays"));
            continue;
        };

        let broad_category_total: f64 = BROAD_CATEGORIES
            .iter()
            .map(|(key, _, _)| {
                t31.get(*key)
                    .and_then(|values| values.get(year))
                    .copied()
                    .ok_or_else(|| format!("{year}: missing Table 3.1 category {key}"))
            })
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum();
        let total_difference = total_outlays_31 - table_11.total_outlays;
        let broad_category_difference = broad_category_total - total_outlays_31;
        if total_difference.abs() > 0.5 {
            errors.push(format!(
                "{year}: Table 3.1 total {total_outlays_31} does not reconcile to Table 1.1 total {}",
                table_11.total_outlays
            ));
        }
        if broad_category_difference.abs() > 2.0 {
            errors.push(format!(
                "{year}: Table 3.1 broad category total {broad_category_total} does not reconcile to total {total_outlays_31}"
            ));
        }

        for (key, label, source_row) in BROAD_CATEGORIES {
            let amount = t31
                .get(*key)
                .and_then(|values| values.get(year))
                .copied()
                .ok_or_else(|| format!("{year}: missing Table 3.1 category {key}"))?;
            rows.push(OutlayFunctionRow {
                fiscal_year: *year,
                source_column: source_column.clone(),
                function_code: (*key).to_string(),
                function_label: (*label).to_string(),
                source_row: *source_row,
                amount: round6(amount),
                actual_or_projection: "actual",
                offsetting_treatment: if *key == "undistributed-offsetting-receipts" {
                    "undistributed-offsetting-receipts"
                } else {
                    "net"
                },
                notes: outlay_function_notes(key),
                include_table_1_1_source: false,
                table_1_1_row: None,
            });
        }
        rows.push(OutlayFunctionRow {
            fiscal_year: *year,
            source_column: source_column.clone(),
            function_code: "total-federal-outlays".to_string(),
            function_label: "Total, Federal outlays".to_string(),
            source_row: 35,
            amount: round6(total_outlays_31),
            actual_or_projection: "actual",
            offsetting_treatment: "net",
            notes: "Total federal outlays reconciled to OMB Historical Table 1.1 total outlays within displayed precision.",
            include_table_1_1_source: true,
            table_1_1_row: Some(table_11.row),
        });

        checks.push(OutlayFunctionCheck {
            year: *year,
            table_1_1_outlays: table_11.total_outlays,
            table_3_1_total: total_outlays_31,
            broad_category_total,
            total_difference,
            broad_category_difference,
        });
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years
        .first()
        .ok_or_else(|| "no Table 3.1 years".to_string())?;
    let last_year = *years
        .last()
        .ok_or_else(|| "no Table 3.1 years".to_string())?;
    let profile = OutlayFunctionProfile {
        first_year,
        last_year,
        year_count: years.len(),
        record_count: rows.len(),
        checks,
    };
    Ok((rows, profile))
}

fn validate_outlay_function_3_1_rows(
    rows: &[OutlayFunctionRow],
    profile: &OutlayFunctionProfile,
) -> Result<(), String> {
    let expected_rows = profile.year_count * (BROAD_CATEGORIES.len() + 1);
    if rows.len() != expected_rows {
        return Err(format!(
            "expected {expected_rows} Table 3.1 outlay function rows, found {}",
            rows.len()
        ));
    }
    for check in &profile.checks {
        if check.total_difference.abs() > 0.5 {
            return Err(format!(
                "{}: Table 3.1/Table 1.1 total difference {}",
                check.year, check.total_difference
            ));
        }
        if check.broad_category_difference.abs() > 2.0 {
            return Err(format!(
                "{}: broad category total difference {}",
                check.year, check.broad_category_difference
            ));
        }
    }
    Ok(())
}

fn outlay_function_notes(key: &str) -> &'static str {
    match key {
        "net-interest" => "Net interest is kept visible as its own outlay function.",
        "undistributed-offsetting-receipts" => {
            "Undistributed offsetting receipts are kept visible and negative as reported by OMB."
        }
        _ => "Broad Table 3.1 outlay function; no lane allocation applied yet.",
    }
}

fn outlay_function_3_1_jsonl(rows: &[OutlayFunctionRow]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        let source_ids = if row.include_table_1_1_source {
            "\"SRC-OMB-HIST-3-1-FY2027\",\"SRC-OMB-HIST-1-1-FY2027\""
        } else {
            "\"SRC-OMB-HIST-3-1-FY2027\""
        };
        let reconciliation = row.table_1_1_row.map_or_else(String::new, |table_1_1_row| {
            format!("; reconciled to Table 1.1 row {table_1_1_row}")
        });
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"outlay_function\",\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[{}],\"source_table\":{},\"source_row_ref\":{},\"superfunction\":null,\"function_code\":{},\"function_label\":{},\"subfunction_code\":null,\"subfunction_label\":null,\"measure\":\"outlays\",\"amount\":{},\"percent\":null,\"amount_units\":\"millions_usd\",\"actual_or_projection\":{},\"offsetting_treatment\":{},\"status\":\"draft-extracted\",\"observed_date\":{},\"notes\":{}}}",
            json_string(&format!(
                "outlay-function:{}:{}:total:outlays",
                row.fiscal_year, row.function_code
            )),
            row.fiscal_year,
            source_ids,
            json_string("OMB Historical Table 3.1 FY2027"),
            json_string(&format!(
                "Table!A{}:{}{}; {}{}",
                row.source_row,
                row.source_column,
                row.source_row,
                row.function_label,
                reconciliation
            )),
            json_string(&row.function_code),
            json_string(&row.function_label),
            json_amount(row.amount),
            json_string(row.actual_or_projection),
            json_string(row.offsetting_treatment),
            json_string(OBSERVED_DATE),
            json_string(row.notes),
        ));
    }
    lines.join("\n") + "\n"
}

fn outlay_function_3_1_profile_markdown(profile: &OutlayFunctionProfile) -> String {
    let sample_years = [1940, 1950, 1980, 2000, 2025];
    let mut lines = vec![
        "# Table 3.1 Outlay Function Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        "- Outlay source: `SRC-OMB-HIST-3-1-FY2027`".to_string(),
        "- Reconciliation source: `SRC-OMB-HIST-1-1-FY2027`".to_string(),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        format!("- Record count: {}", profile.record_count),
        "- Actual/projection treatment: actual years only; FY2026-FY2031 are excluded.".to_string(),
        String::new(),
        "## Extracted Rows".to_string(),
        String::new(),
        "| Function code | OMB label | Table 3.1 row |".to_string(),
        "|---|---|---:|".to_string(),
    ];
    for (key, label, row_num) in BROAD_CATEGORIES {
        lines.push(format!("| `{key}` | {label} | {row_num} |"));
    }
    lines.push("| `total-federal-outlays` | Total, Federal outlays | 35 |".to_string());
    lines.extend([
        String::new(),
        "## Reconciliation Sample".to_string(),
        String::new(),
        "Amounts are in millions of dollars. Broad category total is the sum of the six visible Table 3.1 rows above.".to_string(),
        String::new(),
        "| Fiscal year | Table 1.1 outlays | Table 3.1 total | Broad category total | Table total diff | Broad category diff |".to_string(),
        "|---:|---:|---:|---:|---:|---:|".to_string(),
    ]);
    for check in profile
        .checks
        .iter()
        .filter(|check| sample_years.contains(&check.year))
    {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            check.year,
            comma_number(check.table_1_1_outlays, 0),
            comma_number(check.table_3_1_total, 0),
            comma_number(check.broad_category_total, 0),
            comma_number(check.total_difference, 0),
            comma_number(check.broad_category_difference, 0),
        ));
    }
    lines.extend([
        String::new(),
        "## Extraction Decisions".to_string(),
        String::new(),
        "- Net interest is extracted as its own visible outlay function.".to_string(),
        "- Undistributed offsetting receipts are extracted as negative amounts with `offsetting_treatment = \"undistributed-offsetting-receipts\"`.".to_string(),
        "- Function codes are TAXLANE slugs because Table 3.1 uses labels, not OMB numeric function codes.".to_string(),
        "- No public lane allocation should use these draft rows.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

fn build_table_3_2_national_defense_rows(
    root: &Path,
) -> Result<(Vec<Table32OutlayFunctionRow>, Table32NationalDefenseProfile), String> {
    let sheet_31 = read_sheet(&root.join(TABLE_3_1_PATH))?;
    let (_, t31) = parse_table_3_1(&sheet_31)?;
    let sheet_32 = read_sheet(&root.join(TABLE_3_2_PATH))?;
    let columns_by_year = table_3_2_year_columns(&sheet_32)?;
    validate_table_3_2_national_defense_labels(&sheet_32)?;

    let years: Vec<i64> = columns_by_year
        .keys()
        .copied()
        .filter(|year| (1962..=2025).contains(year))
        .collect();
    let mut rows = Vec::new();
    let mut checks = Vec::new();
    let mut errors = Vec::new();

    for year in &years {
        let Some(column) = columns_by_year.get(year) else {
            errors.push(format!("{year}: missing Table 3.2 source column"));
            continue;
        };
        let Some(table_3_1_national_defense) = t31
            .get("national-defense")
            .and_then(|values| values.get(year))
            .copied()
        else {
            errors.push(format!("{year}: missing Table 3.1 National Defense"));
            continue;
        };

        let mut subfunction_total = 0.0;
        let mut parent_total = None;
        for line in TABLE_3_2_NATIONAL_DEFENSE_LINES {
            let amount = table_3_2_number(&sheet_32, line.source_row, column)?;
            if line.subfunction_code.is_some() {
                subfunction_total += amount;
            } else {
                parent_total = Some(amount);
            }
            rows.push(Table32OutlayFunctionRow {
                fiscal_year: *year,
                source_column: column.clone(),
                source_row: line.source_row,
                function_code: "050",
                function_label: "National Defense",
                subfunction_code: line.subfunction_code,
                subfunction_label: line.subfunction_label,
                source_label: line.source_label,
                amount: round6(amount),
                notes: line.notes,
            });
        }

        let Some(table_3_2_national_defense) = parent_total else {
            errors.push(format!("{year}: missing Table 3.2 National Defense total"));
            continue;
        };
        let table_3_1_difference = table_3_2_national_defense - table_3_1_national_defense;
        let subfunction_difference = subfunction_total - table_3_2_national_defense;
        if table_3_1_difference.abs() > 0.5 {
            errors.push(format!(
                "{year}: Table 3.2 National Defense {table_3_2_national_defense} does not reconcile to Table 3.1 {table_3_1_national_defense}"
            ));
        }
        if subfunction_difference.abs() > 2.0 {
            errors.push(format!(
                "{year}: Table 3.2 National Defense subfunctions {subfunction_total} do not reconcile to total {table_3_2_national_defense}"
            ));
        }
        checks.push(Table32NationalDefenseCheck {
            year: *year,
            table_3_1_national_defense,
            table_3_2_national_defense,
            subfunction_total,
            table_3_1_difference,
            subfunction_difference,
        });
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years
        .first()
        .ok_or_else(|| "no Table 3.2 National Defense years".to_string())?;
    let last_year = *years
        .last()
        .ok_or_else(|| "no Table 3.2 National Defense years".to_string())?;
    let profile = Table32NationalDefenseProfile {
        first_year,
        last_year,
        year_count: years.len(),
        record_count: rows.len(),
        checks,
    };
    Ok((rows, profile))
}

fn validate_table_3_2_national_defense_rows(
    rows: &[Table32OutlayFunctionRow],
    profile: &Table32NationalDefenseProfile,
) -> Result<(), String> {
    let expected_rows = profile.year_count * TABLE_3_2_NATIONAL_DEFENSE_LINES.len();
    if rows.len() != expected_rows {
        return Err(format!(
            "expected {expected_rows} Table 3.2 National Defense rows, found {}",
            rows.len()
        ));
    }
    for check in &profile.checks {
        if check.table_3_1_difference.abs() > 0.5 {
            return Err(format!(
                "{}: Table 3.2/Table 3.1 National Defense difference {}",
                check.year, check.table_3_1_difference
            ));
        }
        if check.subfunction_difference.abs() > 2.0 {
            return Err(format!(
                "{}: National Defense subfunction difference {}",
                check.year, check.subfunction_difference
            ));
        }
    }
    Ok(())
}

fn validate_table_3_2_national_defense_labels(
    sheet: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<(), String> {
    for line in TABLE_3_2_NATIONAL_DEFENSE_LINES {
        let label = sheet
            .get(&line.source_row)
            .and_then(|row| text_cell(row.get("A")))
            .ok_or_else(|| format!("missing Table 3.2 row {} label", line.source_row))?;
        if label != line.source_label {
            return Err(format!(
                "Unexpected Table 3.2 row {}: {label:?}",
                line.source_row
            ));
        }
    }
    Ok(())
}

fn table_3_2_number(
    sheet: &BTreeMap<i64, BTreeMap<String, CellValue>>,
    row_num: i64,
    column: &str,
) -> Result<f64, String> {
    sheet
        .get(&row_num)
        .and_then(|row| number_cell(row.get(column)))
        .ok_or_else(|| format!("missing Table 3.2 amount at {column}{row_num}"))
}

fn table_3_2_national_defense_jsonl(rows: &[Table32OutlayFunctionRow]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        let subfunction_id = row.subfunction_code.unwrap_or("total");
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"outlay_function\",\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[\"SRC-OMB-HIST-3-2-FY2027\"],\"source_table\":{},\"source_row_ref\":{},\"superfunction\":null,\"function_code\":{},\"function_label\":{},\"subfunction_code\":{},\"subfunction_label\":{},\"measure\":\"outlays\",\"amount\":{},\"percent\":null,\"amount_units\":\"millions_usd\",\"actual_or_projection\":\"actual\",\"offsetting_treatment\":\"net\",\"status\":\"draft-extracted\",\"observed_date\":{},\"notes\":{}}}",
            json_string(&format!(
                "outlay-function:{}:{}:{}:outlays",
                row.fiscal_year, row.function_code, subfunction_id
            )),
            row.fiscal_year,
            json_string("OMB Historical Table 3.2 FY2027"),
            json_string(&format!(
                "Table!A{}:{}{}; {}",
                row.source_row, row.source_column, row.source_row, row.source_label
            )),
            json_string(row.function_code),
            json_string(row.function_label),
            json_option_string(row.subfunction_code),
            json_option_string(row.subfunction_label),
            json_amount(row.amount),
            json_string(OBSERVED_DATE),
            json_string(row.notes),
        ));
    }
    lines.join("\n") + "\n"
}

fn table_3_2_national_defense_profile_markdown(profile: &Table32NationalDefenseProfile) -> String {
    let sample_years = [1962, 1970, 1980, 2000, 2025];
    let mut lines = vec![
        "# Table 3.2 National Defense Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        "- Outlay source: `SRC-OMB-HIST-3-2-FY2027`".to_string(),
        "- Reconciliation source: `SRC-OMB-HIST-3-1-FY2027`".to_string(),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        format!("- Record count: {}", profile.record_count),
        "- Actual/projection treatment: actual years only; TQ and FY2026-FY2031 estimates are excluded.".to_string(),
        String::new(),
        "## Extracted Rows".to_string(),
        String::new(),
        "| Function code | Subfunction code | Source label | Table 3.2 row |".to_string(),
        "|---|---|---|---:|".to_string(),
    ];
    for line in TABLE_3_2_NATIONAL_DEFENSE_LINES {
        lines.push(format!(
            "| `050` | {} | {} | {} |",
            line.subfunction_code
                .map(|code| format!("`{code}`"))
                .unwrap_or_else(|| "`null`".to_string()),
            line.source_label,
            line.source_row
        ));
    }
    lines.extend([
        String::new(),
        "## Reconciliation Sample".to_string(),
        String::new(),
        "Amounts are in millions of dollars. Subfunction total is rows 13, 14, and 15.".to_string(),
        String::new(),
        "| Fiscal year | Table 3.1 National Defense | Table 3.2 National Defense | Subfunction total | Table 3.1 diff | Subfunction diff |".to_string(),
        "|---:|---:|---:|---:|---:|---:|".to_string(),
    ]);
    for check in profile
        .checks
        .iter()
        .filter(|check| sample_years.contains(&check.year))
    {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            check.year,
            comma_number(check.table_3_1_national_defense, 0),
            comma_number(check.table_3_2_national_defense, 0),
            comma_number(check.subfunction_total, 0),
            comma_number(check.table_3_1_difference, 0),
            comma_number(check.subfunction_difference, 0),
        ));
    }
    lines.extend([
        String::new(),
        "## Extraction Decisions".to_string(),
        String::new(),
        "- This is a proof slice for function `050 National Defense`, not the full Table 3.2 extraction.".to_string(),
        "- Rows 6-12 are lower component rows inside subfunction `051`; this proof emits row 13 as the subfunction total instead.".to_string(),
        "- Parent total row 16 is emitted with `subfunction_code = null` so it can reconcile to Table 3.1.".to_string(),
        "- No public lane allocation should use these draft rows.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

struct Table61NationalDefenseRow {
    fiscal_year: i64,
    source_column: String,
    source_row: i64,
    percent_of_gdp: f64,
}

struct Table61NationalDefenseProfile {
    first_year: i64,
    last_year: i64,
    year_count: usize,
    samples: Vec<(i64, f64, f64)>,
}

fn table_6_1_year_columns(
    rows: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<BTreeMap<i64, String>, String> {
    for cells in rows.values() {
        let mut columns = BTreeMap::new();
        for (column, value) in cells {
            let year = match value {
                CellValue::Number(number) if number.fract() == 0.0 => Some(*number as i64),
                CellValue::Text(text) => text.trim().parse::<i64>().ok(),
                _ => None,
            };
            if let Some(year) = year.filter(|year| (1940..=2031).contains(year)) {
                columns.insert(year, column.clone());
            }
        }
        if columns.contains_key(&1940) && columns.contains_key(&2025) {
            return Ok(columns);
        }
    }
    Err("missing Table 6.1 year header row (1940..2025)".to_string())
}

fn table_6_1_section_row(
    rows: &BTreeMap<i64, BTreeMap<String, CellValue>>,
    label: &str,
) -> Result<i64, String> {
    rows.iter()
        .find(|(_, cells)| text_cell(cells.get("A")).as_deref() == Some(label))
        .map(|(row_num, _)| *row_num)
        .ok_or_else(|| format!("missing Table 6.1 section {label:?}"))
}

fn table_6_1_label_row_between(
    rows: &BTreeMap<i64, BTreeMap<String, CellValue>>,
    label: &str,
    after_row: i64,
    before_row: i64,
) -> Result<i64, String> {
    rows.iter()
        .filter(|(row_num, _)| **row_num > after_row && **row_num < before_row)
        .find(|(_, cells)| text_cell(cells.get("A")).as_deref() == Some(label))
        .map(|(row_num, _)| *row_num)
        .ok_or_else(|| {
            format!("missing Table 6.1 row {label:?} between rows {after_row} and {before_row}")
        })
}

fn build_table_6_1_national_defense_rows(
    root: &Path,
) -> Result<
    (
        Vec<Table61NationalDefenseRow>,
        Table61NationalDefenseProfile,
    ),
    String,
> {
    let sheet = read_sheet(&root.join(TABLE_6_1_PATH))?;
    let columns_by_year = table_6_1_year_columns(&sheet)?;
    let gdp_section = table_6_1_section_row(&sheet, "As percentages of GDP:")?;
    let outlays_section = table_6_1_section_row(&sheet, "As percentages of outlays:")?;
    if outlays_section <= gdp_section {
        return Err("Table 6.1 section order unexpected".to_string());
    }
    let defense_row =
        table_6_1_label_row_between(&sheet, "National defense (1)", gdp_section, outlays_section)?;
    let total_row =
        table_6_1_label_row_between(&sheet, "Total outlays", gdp_section, outlays_section)?;

    let years: Vec<i64> = columns_by_year
        .keys()
        .copied()
        .filter(|year| (1940..=2025).contains(year))
        .collect();
    let sample_years = [1944, 1953, 1968, 1979, 1986, 2000, 2010, 2025];
    let mut rows = Vec::new();
    let mut samples = Vec::new();
    let mut errors = Vec::new();

    for year in &years {
        let Some(column) = columns_by_year.get(year) else {
            errors.push(format!("{year}: missing Table 6.1 source column"));
            continue;
        };
        let Some(percent) = number_cell(sheet.get(&defense_row).and_then(|row| row.get(column)))
        else {
            errors.push(format!(
                "{year}: missing national-defense %GDP at {column}{defense_row}"
            ));
            continue;
        };
        if !(0.0..=50.0).contains(&percent) {
            errors.push(format!(
                "{year}: implausible national-defense %GDP {percent}"
            ));
        }
        rows.push(Table61NationalDefenseRow {
            fiscal_year: *year,
            source_column: column.clone(),
            source_row: defense_row,
            percent_of_gdp: round6(percent),
        });
        if sample_years.contains(year) {
            let total =
                number_cell(sheet.get(&total_row).and_then(|row| row.get(column))).unwrap_or(0.0);
            samples.push((*year, round6(percent), round6(total)));
        }
    }

    for (year, low, high) in [(1953_i64, 13.0_f64, 14.5_f64), (2025, 2.5, 3.5)] {
        match rows.iter().find(|row| row.fiscal_year == year) {
            Some(row) if (low..=high).contains(&row.percent_of_gdp) => {}
            Some(row) => errors.push(format!(
                "{year}: national-defense %GDP {} outside expected [{low}, {high}]",
                row.percent_of_gdp
            )),
            None => errors.push(format!("missing anchor year {year}")),
        }
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years
        .first()
        .ok_or_else(|| "no Table 6.1 years".to_string())?;
    let last_year = *years
        .last()
        .ok_or_else(|| "no Table 6.1 years".to_string())?;
    let profile = Table61NationalDefenseProfile {
        first_year,
        last_year,
        year_count: years.len(),
        samples,
    };
    Ok((rows, profile))
}

fn validate_table_6_1_national_defense_rows(
    rows: &[Table61NationalDefenseRow],
    profile: &Table61NationalDefenseProfile,
) -> Result<(), String> {
    if rows.len() != profile.year_count {
        return Err(format!(
            "expected {} Table 6.1 National Defense rows, found {}",
            profile.year_count,
            rows.len()
        ));
    }
    for row in rows {
        if !(0.0..=50.0).contains(&row.percent_of_gdp) {
            return Err(format!(
                "{}: implausible national-defense %GDP {}",
                row.fiscal_year, row.percent_of_gdp
            ));
        }
    }
    Ok(())
}

fn table_6_1_national_defense_jsonl(rows: &[Table61NationalDefenseRow]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"outlay_composition\",\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[\"SRC-OMB-HIST-6-1-FY2027\"],\"source_table\":{},\"source_row_ref\":{},\"function_code\":\"050\",\"function_label\":\"National Defense\",\"measure\":\"percent_of_gdp\",\"percent\":{},\"amount\":null,\"amount_units\":\"percent_of_gdp\",\"actual_or_projection\":\"actual\",\"status\":\"draft-extracted\",\"observed_date\":{},\"notes\":{}}}",
            json_string(&format!(
                "outlay-composition:{}:050:percent-of-gdp",
                row.fiscal_year
            )),
            row.fiscal_year,
            json_string("OMB Historical Table 6.1 FY2027"),
            json_string(&format!(
                "Table!{}{}; National defense (1) (As percentages of GDP)",
                row.source_column, row.source_row
            )),
            json_amount(row.percent_of_gdp),
            json_string(OBSERVED_DATE_6_1),
            json_string(
                "National defense outlays as a percentage of GDP (OMB budget-function 050 basis); actual years only."
            ),
        ));
    }
    lines.join("\n") + "\n"
}

fn table_6_1_national_defense_profile_markdown(profile: &Table61NationalDefenseProfile) -> String {
    let mut lines = vec![
        "# Table 6.1 National Defense (% of GDP) Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        "- Source: `SRC-OMB-HIST-6-1-FY2027` (Composition of Outlays).".to_string(),
        "- Series: national-defense outlays as a percentage of GDP (OMB budget-function 050 basis).".to_string(),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        "- Actual/projection treatment: actual years only; FY2026-FY2031 estimates are excluded.".to_string(),
        String::new(),
        "## Sample Years".to_string(),
        String::new(),
        "| Fiscal year | National defense, % of GDP | Total outlays, % of GDP |".to_string(),
        "|---:|---:|---:|".to_string(),
    ];
    for (year, defense, total) in &profile.samples {
        lines.push(format!(
            "| {} | {} | {} |",
            year,
            comma_number(*defense, 1),
            comma_number(*total, 1)
        ));
    }
    lines.extend([
        String::new(),
        "## Extraction Decisions".to_string(),
        String::new(),
        "- This is the national-defense (function 050) row of OMB Table 6.1's \"As percentages of GDP\" section.".to_string(),
        "- It is the OMB budget-function basis, not the SIPRI/NATO definition; the two series are not merged.".to_string(),
        "- Values are OMB-reported to one decimal place.".to_string(),
        "- No public lane allocation should use these draft rows.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

fn build_outlay_composition_table_6_1_national_defense(
    root: &Path,
    check_only: bool,
) -> Result<(), String> {
    let (rows, profile) = build_table_6_1_national_defense_rows(root)?;
    validate_table_6_1_national_defense_rows(&rows, &profile)?;
    let jsonl = table_6_1_national_defense_jsonl(&rows);
    let markdown = table_6_1_national_defense_profile_markdown(&profile);

    if check_only {
        compare_text(
            root,
            OUTLAY_COMPOSITION_6_1_NATIONAL_DEFENSE_JSONL_PATH,
            &jsonl,
            "Table 6.1 National Defense %GDP JSONL",
        )?;
        compare_text(
            root,
            OUTLAY_COMPOSITION_6_1_NATIONAL_DEFENSE_PROFILE_PATH,
            &markdown,
            "Table 6.1 National Defense %GDP profile",
        )?;
    } else {
        fs::write(
            root.join(OUTLAY_COMPOSITION_6_1_NATIONAL_DEFENSE_JSONL_PATH),
            jsonl,
        )
        .map_err(|err| {
            format!("failed to write {OUTLAY_COMPOSITION_6_1_NATIONAL_DEFENSE_JSONL_PATH}: {err}")
        })?;
        fs::write(
            root.join(OUTLAY_COMPOSITION_6_1_NATIONAL_DEFENSE_PROFILE_PATH),
            markdown,
        )
        .map_err(|err| {
            format!("failed to write {OUTLAY_COMPOSITION_6_1_NATIONAL_DEFENSE_PROFILE_PATH}: {err}")
        })?;
    }

    println!(
        "validated {} Table 6.1 National Defense %GDP rows for {}-{}",
        rows.len(),
        profile.first_year,
        profile.last_year
    );
    Ok(())
}

fn build_table_3_2_rows(root: &Path) -> Result<(Vec<Table32Row>, Table32Profile), String> {
    let sheet_31 = read_sheet(&root.join(TABLE_3_1_PATH))?;
    let (_, t31) = parse_table_3_1(&sheet_31)?;
    let sheet_32 = read_sheet(&root.join(TABLE_3_2_PATH))?;
    let columns_by_year = table_3_2_year_columns(&sheet_32)?;
    let lines = parse_table_3_2_lines(&sheet_32)?;
    let years: Vec<i64> = columns_by_year
        .keys()
        .copied()
        .filter(|year| (1962..=2025).contains(year))
        .collect();

    let mut rows = Vec::new();
    let mut errors = Vec::new();
    let mut grand_checks = Vec::new();
    let mut function_checks = Vec::new();

    for year in &years {
        let Some(column) = columns_by_year.get(year) else {
            errors.push(format!("{year}: missing Table 3.2 source column"));
            continue;
        };
        let Some(table_3_1_total_outlays) = t31
            .get("total-federal-outlays")
            .and_then(|values| values.get(year))
            .copied()
        else {
            errors.push(format!("{year}: missing Table 3.1 total outlays"));
            continue;
        };

        let mut subfunction_totals: BTreeMap<String, f64> = BTreeMap::new();
        let mut explicit_function_totals: BTreeMap<String, (String, f64)> = BTreeMap::new();
        let mut table_3_2_total_outlays = None;

        for line in &lines {
            let Some(amount) = table_3_2_optional_number(&sheet_32, line.source_row, column) else {
                continue;
            };
            match line.kind {
                Table32LineKind::Subfunction => {
                    *subfunction_totals
                        .entry(line.function_code.clone())
                        .or_insert(0.0) += amount;
                }
                Table32LineKind::FunctionTotal => {
                    explicit_function_totals.insert(
                        line.function_code.clone(),
                        (line.function_label.clone(), amount),
                    );
                }
                Table32LineKind::GrandTotal => {
                    table_3_2_total_outlays = Some(amount);
                }
            }
            rows.push(Table32Row {
                fiscal_year: *year,
                source_column: column.clone(),
                source_row: line.source_row,
                function_code: line.function_code.clone(),
                function_label: line.function_label.clone(),
                subfunction_code: line.subfunction_code.clone(),
                subfunction_label: line.subfunction_label.clone(),
                source_label: line.source_label.clone(),
                amount: round6(amount),
                kind: line.kind.clone(),
            });
        }

        let Some(table_3_2_total_outlays) = table_3_2_total_outlays else {
            errors.push(format!("{year}: missing Table 3.2 total outlays"));
            continue;
        };
        let mut function_total_sum = 0.0;
        for (function_code, subfunction_total) in &subfunction_totals {
            if let Some((function_label, function_total)) =
                explicit_function_totals.get(function_code)
            {
                let difference = subfunction_total - function_total;
                if difference.abs() > 2.0 {
                    errors.push(format!(
                        "{year}: Table 3.2 function {function_code} subfunctions {subfunction_total} do not reconcile to total {function_total}"
                    ));
                }
                function_total_sum += function_total;
                function_checks.push(Table32FunctionCheck {
                    year: *year,
                    function_code: function_code.clone(),
                    function_label: function_label.clone(),
                    function_total: *function_total,
                    subfunction_total: *subfunction_total,
                    difference,
                });
            } else {
                function_total_sum += subfunction_total;
            }
        }
        let table_3_1_difference = table_3_2_total_outlays - table_3_1_total_outlays;
        let function_total_difference = function_total_sum - table_3_2_total_outlays;
        if table_3_1_difference.abs() > 0.5 {
            errors.push(format!(
                "{year}: Table 3.2 total {table_3_2_total_outlays} does not reconcile to Table 3.1 total {table_3_1_total_outlays}"
            ));
        }
        if function_total_difference.abs() > 5.0 {
            errors.push(format!(
                "{year}: Table 3.2 function totals {function_total_sum} do not reconcile to total outlays {table_3_2_total_outlays}"
            ));
        }
        grand_checks.push(Table32GrandCheck {
            year: *year,
            table_3_1_total_outlays,
            table_3_2_total_outlays,
            function_total_sum,
            table_3_1_difference,
            function_total_difference,
        });
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years
        .first()
        .ok_or_else(|| "no Table 3.2 years".to_string())?;
    let last_year = *years
        .last()
        .ok_or_else(|| "no Table 3.2 years".to_string())?;
    let subfunction_line_count = lines
        .iter()
        .filter(|line| matches!(line.kind, Table32LineKind::Subfunction))
        .count();
    let function_total_line_count = lines
        .iter()
        .filter(|line| matches!(line.kind, Table32LineKind::FunctionTotal))
        .count();
    let function_count = lines
        .iter()
        .filter(|line| !matches!(line.kind, Table32LineKind::GrandTotal))
        .map(|line| line.function_code.clone())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let profile = Table32Profile {
        first_year,
        last_year,
        year_count: years.len(),
        record_count: rows.len(),
        line_count: lines.len(),
        subfunction_line_count,
        function_total_line_count,
        function_count,
        grand_checks,
        function_checks,
    };
    Ok((rows, profile))
}

fn parse_table_3_2_lines(
    sheet: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<Vec<Table32Line>, String> {
    let mut lines = Vec::new();
    let mut current_function: Option<(String, String)> = None;
    for (row_num, cells) in sheet {
        if *row_num < 4 {
            continue;
        }
        let Some(label) = text_cell(cells.get("A")) else {
            continue;
        };
        if let Some((code, function_label)) = parse_table_3_2_function_header(&label) {
            if is_table_3_2_function_code(&code) {
                current_function = Some((code, function_label));
            }
            continue;
        }
        if label.starts_with('(')
            || label == "On-budget unless otherwise stated"
            || label == "N/A = Not available"
        {
            continue;
        }
        if label == "Total outlays" {
            lines.push(Table32Line {
                source_row: *row_num,
                function_code: "total-federal-outlays".to_string(),
                function_label: "Total outlays".to_string(),
                subfunction_code: None,
                subfunction_label: None,
                source_label: label,
                kind: Table32LineKind::GrandTotal,
            });
            continue;
        }
        if let Some(total_label) = label.strip_prefix("Total, ") {
            let Some((function_code, function_label)) = current_function.clone() else {
                return Err(format!("Table 3.2 row {row_num} total without function"));
            };
            if total_label != function_label {
                return Err(format!(
                    "Table 3.2 row {row_num} total {total_label:?} does not match current function {function_label:?}"
                ));
            }
            lines.push(Table32Line {
                source_row: *row_num,
                function_code,
                function_label,
                subfunction_code: None,
                subfunction_label: None,
                source_label: label,
                kind: Table32LineKind::FunctionTotal,
            });
            continue;
        }
        if let Some((subfunction_code, mut subfunction_label)) = parse_table_3_2_coded_label(&label)
        {
            let Some((function_code, function_label)) = current_function.clone() else {
                return Err(format!(
                    "Table 3.2 row {row_num} subfunction without function"
                ));
            };
            if let Some(subtotal_label) = subfunction_label.strip_prefix("Subtotal, ") {
                subfunction_label = subtotal_label.to_string();
            }
            lines.push(Table32Line {
                source_row: *row_num,
                function_code,
                function_label,
                subfunction_code: Some(subfunction_code),
                subfunction_label: Some(subfunction_label),
                source_label: label,
                kind: Table32LineKind::Subfunction,
            });
        }
    }
    Ok(lines)
}

fn parse_table_3_2_function_header(label: &str) -> Option<(String, String)> {
    let label = label.strip_suffix(':')?;
    parse_table_3_2_coded_label(label)
}

fn is_table_3_2_function_code(code: &str) -> bool {
    matches!(
        code,
        "050"
            | "150"
            | "250"
            | "270"
            | "300"
            | "350"
            | "370"
            | "400"
            | "450"
            | "500"
            | "550"
            | "570"
            | "600"
            | "650"
            | "700"
            | "750"
            | "800"
            | "900"
            | "920"
            | "950"
    )
}

fn parse_table_3_2_coded_label(label: &str) -> Option<(String, String)> {
    let (code, rest) = label.split_once(' ')?;
    if code.len() == 3 && code.chars().all(|char| char.is_ascii_digit()) {
        Some((code.to_string(), rest.trim().to_string()))
    } else {
        None
    }
}

fn table_3_2_optional_number(
    sheet: &BTreeMap<i64, BTreeMap<String, CellValue>>,
    row_num: i64,
    column: &str,
) -> Option<f64> {
    sheet
        .get(&row_num)
        .and_then(|row| number_cell(row.get(column)))
}

fn validate_table_3_2_rows(profile: &Table32Profile) -> Result<(), String> {
    for check in &profile.grand_checks {
        if check.table_3_1_difference.abs() > 0.5 {
            return Err(format!(
                "{}: Table 3.2/Table 3.1 total difference {}",
                check.year, check.table_3_1_difference
            ));
        }
        if check.function_total_difference.abs() > 5.0 {
            return Err(format!(
                "{}: Table 3.2 function total difference {}",
                check.year, check.function_total_difference
            ));
        }
    }
    for check in &profile.function_checks {
        if check.difference.abs() > 2.0 {
            return Err(format!(
                "{} {}: Table 3.2 function difference {}",
                check.year, check.function_code, check.difference
            ));
        }
    }
    Ok(())
}

fn table_3_2_jsonl(rows: &[Table32Row]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        let subfunction_id = row.subfunction_code.as_deref().unwrap_or("total");
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"outlay_function\",\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[\"SRC-OMB-HIST-3-2-FY2027\"],\"source_table\":{},\"source_row_ref\":{},\"superfunction\":null,\"function_code\":{},\"function_label\":{},\"subfunction_code\":{},\"subfunction_label\":{},\"measure\":\"outlays\",\"amount\":{},\"percent\":null,\"amount_units\":\"millions_usd\",\"actual_or_projection\":\"actual\",\"offsetting_treatment\":{},\"status\":\"draft-extracted\",\"observed_date\":{},\"notes\":{}}}",
            json_string(&format!(
                "outlay-function:{}:{}:{}:outlays",
                row.fiscal_year, row.function_code, subfunction_id
            )),
            row.fiscal_year,
            json_string("OMB Historical Table 3.2 FY2027"),
            json_string(&format!(
                "Table!A{}:{}{}; {}",
                row.source_row, row.source_column, row.source_row, row.source_label
            )),
            json_string(&row.function_code),
            json_string(&row.function_label),
            json_owned_option_string(row.subfunction_code.as_ref()),
            json_owned_option_string(row.subfunction_label.as_ref()),
            json_amount(row.amount),
            json_string(table_3_2_offsetting_treatment(row)),
            json_string(OBSERVED_DATE),
            json_string(table_3_2_notes(row)),
        ));
    }
    lines.join("\n") + "\n"
}

fn table_3_2_offsetting_treatment(row: &Table32Row) -> &'static str {
    if row.function_code == "950" {
        "undistributed-offsetting-receipts"
    } else if row.subfunction_code.as_deref() == Some("809") {
        "offsetting-receipts"
    } else {
        "net"
    }
}

fn table_3_2_notes(row: &Table32Row) -> &'static str {
    match row.kind {
        Table32LineKind::Subfunction => {
            "Table 3.2 subfunction row; lower component rows and parenthetical on/off-budget splits are not emitted."
        }
        Table32LineKind::FunctionTotal => {
            "Table 3.2 parent function total used for subfunction reconciliation."
        }
        Table32LineKind::GrandTotal => {
            "Table 3.2 total outlays reconciled to OMB Historical Table 3.1 total outlays."
        }
    }
}

fn table_3_2_profile_markdown(profile: &Table32Profile) -> String {
    let sample_years = [1962, 1970, 1980, 2000, 2025];
    let mut lines = vec![
        "# Table 3.2 Outlay Function Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        "- Outlay source: `SRC-OMB-HIST-3-2-FY2027`".to_string(),
        "- Reconciliation source: `SRC-OMB-HIST-3-1-FY2027`".to_string(),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        format!("- Record count: {}", profile.record_count),
        format!("- Source lines emitted: {}", profile.line_count),
        format!("- Function count: {}", profile.function_count),
        format!("- Subfunction lines: {}", profile.subfunction_line_count),
        format!(
            "- Explicit function-total lines: {}",
            profile.function_total_line_count
        ),
        "- Actual/projection treatment: actual years only; TQ and FY2026-FY2031 estimates are excluded.".to_string(),
        String::new(),
        "## Reconciliation Sample".to_string(),
        String::new(),
        "Amounts are in millions of dollars. Function total sum uses explicit parent totals when Table 3.2 provides them, otherwise the emitted subfunction total.".to_string(),
        String::new(),
        "| Fiscal year | Table 3.1 total outlays | Table 3.2 total outlays | Function total sum | Table 3.1 diff | Function total diff |".to_string(),
        "|---:|---:|---:|---:|---:|---:|".to_string(),
    ];
    for check in profile
        .grand_checks
        .iter()
        .filter(|check| sample_years.contains(&check.year))
    {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            check.year,
            comma_number(check.table_3_1_total_outlays, 0),
            comma_number(check.table_3_2_total_outlays, 0),
            comma_number(check.function_total_sum, 0),
            comma_number(check.table_3_1_difference, 0),
            comma_number(check.function_total_difference, 0),
        ));
    }
    if let Some(check) = profile.function_checks.iter().max_by(|left, right| {
        left.difference
            .abs()
            .partial_cmp(&right.difference.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    }) {
        lines.extend([
            String::new(),
            "## Function Reconciliation Note".to_string(),
            String::new(),
            format!(
                "Largest displayed-source function subtotal difference: FY{} `{}` {} has subfunction total {} versus parent total {}, difference {}.",
                check.year,
                check.function_code,
                check.function_label,
                comma_number(check.subfunction_total, 0),
                comma_number(check.function_total, 0),
                comma_number(check.difference, 0),
            ),
        ]);
    }
    lines.extend([
        String::new(),
        "## Extraction Decisions".to_string(),
        String::new(),
        "- Emit three-digit coded subfunction rows and explicit parent `Total, ...` rows.".to_string(),
        "- Emit `Total outlays` as a grand-total record for annual reconciliation.".to_string(),
        "- Skip lower component rows without OMB subfunction codes, including parenthetical on/off-budget splits.".to_string(),
        "- Keep TQ and FY2026-FY2031 estimate columns out of this actual-year draft.".to_string(),
        "- No public lane allocation should use these draft rows.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

fn build_subfunction_model(root: &Path, check_only: bool) -> Result<(), String> {
    let (records, profile) = build_subfunction_model_records(root)?;
    validate_subfunction_model_records(&records, &profile)?;
    let jsonl = subfunction_model_jsonl(&records);
    let profile_markdown = subfunction_model_profile_markdown(&profile);
    let readme = subfunction_model_readme_markdown();

    if check_only {
        compare_text(
            root,
            SUBFUNCTION_MODEL_JSONL_PATH,
            &jsonl,
            "subfunction model JSONL",
        )?;
        compare_text(
            root,
            SUBFUNCTION_MODEL_PROFILE_PATH,
            &profile_markdown,
            "subfunction model profile",
        )?;
        compare_text(
            root,
            SUBFUNCTION_MODEL_README_PATH,
            &readme,
            "subfunction model README",
        )?;
    } else {
        fs::create_dir_all(root.join("data/derived/income_tax_outlay_subfunction_model"))
            .map_err(|err| format!("failed to create subfunction model directory: {err}"))?;
        fs::write(root.join(SUBFUNCTION_MODEL_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {SUBFUNCTION_MODEL_JSONL_PATH}: {err}"))?;
        fs::write(root.join(SUBFUNCTION_MODEL_PROFILE_PATH), profile_markdown)
            .map_err(|err| format!("failed to write {SUBFUNCTION_MODEL_PROFILE_PATH}: {err}"))?;
        fs::write(root.join(SUBFUNCTION_MODEL_README_PATH), readme)
            .map_err(|err| format!("failed to write {SUBFUNCTION_MODEL_README_PATH}: {err}"))?;
    }

    println!(
        "validated {} subfunction model rows for {}-{}",
        records.len(),
        profile.first_year,
        profile.last_year
    );
    Ok(())
}

fn build_subfunction_model_records(
    root: &Path,
) -> Result<(Vec<SubfunctionModelRow>, SubfunctionModelProfile), String> {
    let t21 = parse_table_2_1(&read_sheet(&root.join(TABLE_2_1_PATH))?);
    let sheet_32 = read_sheet(&root.join(TABLE_3_2_PATH))?;
    let columns_by_year = table_3_2_year_columns(&sheet_32)?;
    let lines = parse_table_3_2_lines(&sheet_32)?;
    let subfunction_lines: Vec<Table32Line> = lines
        .into_iter()
        .filter(|line| matches!(line.kind, Table32LineKind::Subfunction))
        .collect();
    let years: Vec<i64> = columns_by_year
        .keys()
        .copied()
        .filter(|year| (1962..=2025).contains(year))
        .collect();

    let mut records = Vec::new();
    let mut checks = Vec::new();
    let mut errors = Vec::new();

    for year in &years {
        let Some(column) = columns_by_year.get(year) else {
            errors.push(format!("{year}: missing Table 3.2 source column"));
            continue;
        };
        let Some(table_21) = t21.get(year) else {
            errors.push(format!("{year}: missing Table 2.1 row"));
            continue;
        };
        let total_outlays = table_3_2_optional_number(&sheet_32, 140, column)
            .ok_or_else(|| format!("{year}: missing Table 3.2 total outlays"))?;
        let mut subfunction_total = 0.0;
        let mut year_values = Vec::new();
        for line in &subfunction_lines {
            let Some(amount) = table_3_2_optional_number(&sheet_32, line.source_row, column) else {
                continue;
            };
            subfunction_total += amount;
            year_values.push((line, amount));
        }
        let subfunction_total_difference = subfunction_total - total_outlays;
        if subfunction_total_difference.abs() > 10.0 {
            errors.push(format!(
                "{year}: Table 3.2 subfunction total {subfunction_total} does not reconcile to total outlays {total_outlays}"
            ));
        }
        let income_tax = table_21.individual_income_tax;
        let mut modeled_sum = 0.0;
        for (line, amount) in year_values {
            let modeled_amount = income_tax * amount / subfunction_total;
            modeled_sum += modeled_amount;
            records.push(SubfunctionModelRow {
                fiscal_year: *year,
                source_column: column.clone(),
                source_row: line.source_row,
                function_code: line.function_code.clone(),
                function_label: line.function_label.clone(),
                subfunction_code: line
                    .subfunction_code
                    .clone()
                    .ok_or_else(|| "missing subfunction code".to_string())?,
                subfunction_label: line
                    .subfunction_label
                    .clone()
                    .ok_or_else(|| "missing subfunction label".to_string())?,
                subfunction_outlays_amount: round6(amount),
                subfunction_total_outlays_amount: round6(subfunction_total),
                total_outlays_amount: round6(total_outlays),
                individual_income_tax_receipts_amount: round6(income_tax),
                outlay_share_percent: round9(amount / total_outlays * 100.0),
                allocation_share_percent: round9(amount / subfunction_total * 100.0),
                modeled_income_tax_allocation_amount: round6(modeled_amount),
            });
        }
        if (modeled_sum - income_tax).abs() > 0.0005 {
            errors.push(format!(
                "{year}: subfunction modeled sum {modeled_sum} does not match individual income-tax receipts {income_tax}"
            ));
        }
        checks.push(SubfunctionModelCheck {
            year: *year,
            table_3_2_total_outlays: total_outlays,
            subfunction_total,
            individual_income_tax: income_tax,
            modeled_sum,
            subfunction_total_difference,
        });
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years
        .first()
        .ok_or_else(|| "no subfunction model years".to_string())?;
    let last_year = *years
        .last()
        .ok_or_else(|| "no subfunction model years".to_string())?;
    let subfunction_count = subfunction_lines.len();
    let profile = SubfunctionModelProfile {
        first_year,
        last_year,
        year_count: years.len(),
        record_count: records.len(),
        subfunction_count,
        checks,
    };
    Ok((records, profile))
}

fn validate_subfunction_model_records(
    records: &[SubfunctionModelRow],
    profile: &SubfunctionModelProfile,
) -> Result<(), String> {
    if records.is_empty() {
        return Err("no subfunction model rows".to_string());
    }
    for check in &profile.checks {
        if check.subfunction_total_difference.abs() > 10.0 {
            return Err(format!(
                "{}: subfunction total difference {}",
                check.year, check.subfunction_total_difference
            ));
        }
        if (check.modeled_sum - check.individual_income_tax).abs() > 0.0005 {
            return Err(format!(
                "{}: modeled sum {} does not equal income tax {}",
                check.year, check.modeled_sum, check.individual_income_tax
            ));
        }
    }
    Ok(())
}

fn subfunction_model_jsonl(records: &[SubfunctionModelRow]) -> String {
    let mut lines = Vec::new();
    for row in records {
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"income_tax_outlay_subfunction_model\",\"model_id\":{},\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[\"SRC-OMB-HIST-2-1-FY2027\",\"SRC-OMB-HIST-3-2-FY2027\"],\"source_table_refs\":{{\"tax_receipts\":\"OMB Historical Table 2.1 FY2027\",\"outlay_subfunction\":{}}},\"tax_source\":\"individual-income-taxes\",\"allocation_method\":\"proportional_outlay_share\",\"legal_allocation_status\":\"modeled_not_legal_dedication\",\"function_code\":{},\"function_label\":{},\"subfunction_code\":{},\"subfunction_label\":{},\"subfunction_outlays_amount\":{},\"total_outlays_amount\":{},\"subfunction_total_outlays_amount\":{},\"individual_income_tax_receipts_amount\":{},\"outlay_share_percent\":{},\"allocation_share_percent\":{},\"modeled_income_tax_allocation_amount\":{},\"actual_or_projection\":\"actual\",\"status\":\"draft\",\"observed_date\":{},\"notes\":\"Modeled allocation of ordinary individual income-tax receipts by Table 3.2 subfunction outlay share; not legal dedication or program tracing.\"}}",
            json_string(&format!(
                "income-tax-outlay-subfunction-model:{}:{}:{}",
                row.fiscal_year, row.function_code, row.subfunction_code
            )),
            json_string(SUBFUNCTION_MODEL_ID),
            row.fiscal_year,
            json_string(&format!(
                "OMB Historical Table 3.2 FY2027 row {}, column {}",
                row.source_row, row.source_column
            )),
            json_string(&row.function_code),
            json_string(&row.function_label),
            json_string(&row.subfunction_code),
            json_string(&row.subfunction_label),
            json_amount(row.subfunction_outlays_amount),
            json_amount(row.total_outlays_amount),
            json_amount(row.subfunction_total_outlays_amount),
            json_amount(row.individual_income_tax_receipts_amount),
            decimal_string(row.outlay_share_percent, 9),
            decimal_string(row.allocation_share_percent, 9),
            decimal_string(row.modeled_income_tax_allocation_amount, 6),
            json_string(OBSERVED_DATE),
        ));
    }
    lines.join("\n") + "\n"
}

fn subfunction_model_profile_markdown(profile: &SubfunctionModelProfile) -> String {
    let sample_years = [1962, 1970, 1980, 2000, 2025];
    let mut lines = vec![
        "# Income-Tax Outlay Subfunction Model Source Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        format!("- Model ID: `{SUBFUNCTION_MODEL_ID}`"),
        "- Tax receipt source: `SRC-OMB-HIST-2-1-FY2027`".to_string(),
        "- Outlay source: `SRC-OMB-HIST-3-2-FY2027`".to_string(),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        format!("- Subfunction count: {}", profile.subfunction_count),
        format!("- Record count: {}", profile.record_count),
        "- Actual/projection treatment: actual years only; TQ and FY2026-FY2031 estimates are excluded.".to_string(),
        String::new(),
        "## Reconciliation Sample".to_string(),
        String::new(),
        "Amounts are in millions of dollars. `Subfunction total` is the denominator used for modeled allocation.".to_string(),
        String::new(),
        "| Fiscal year | Table 3.2 total outlays | Subfunction total | Income tax receipts | Modeled sum | Subfunction diff |".to_string(),
        "|---:|---:|---:|---:|---:|---:|".to_string(),
    ];
    for check in profile
        .checks
        .iter()
        .filter(|check| sample_years.contains(&check.year))
    {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            check.year,
            comma_number(check.table_3_2_total_outlays, 0),
            comma_number(check.subfunction_total, 0),
            comma_number(check.individual_income_tax, 0),
            comma_number(check.modeled_sum, 3),
            comma_number(check.subfunction_total_difference, 0),
        ));
    }
    lines.extend([
        String::new(),
        "## Model Caveat".to_string(),
        String::new(),
        "This is a visibility model. It allocates ordinary individual income-tax receipts by reported Table 3.2 subfunction outlay shares. It is not a legal dedication, appropriation rule, or program-financing claim.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

fn subfunction_model_readme_markdown() -> String {
    [
        "# Individual Income-Tax Outlay Subfunction Model",
        "",
        "## Purpose",
        "",
        "This derived model estimates, by fiscal year and OMB Table 3.2 subfunction, how ordinary individual income-tax receipts would be allocated if allocated in proportion to that year's reported subfunction outlays.",
        "",
        "This is a visibility model. It is not a legal dedication, appropriation rule, or program-financing claim.",
        "",
        "## Model ID",
        "",
        "`individual-income-tax-proportional-subfunction-outlays-v1`",
        "",
        "## Inputs",
        "",
        "| Source ID | Role |",
        "|---|---|",
        "| `SRC-OMB-HIST-2-1-FY2027` | Individual income-tax receipt amount by fiscal year. |",
        "| `SRC-OMB-HIST-3-2-FY2027` | Function and subfunction outlays by fiscal year. |",
        "",
        "## Coverage",
        "",
        "The first draft model covers fiscal years 1962-2025, the overlap between Table 3.2 actual-year subfunction rows and Table 2.1 individual income-tax receipt rows.",
        "",
        "## Artifacts",
        "",
        "| Artifact | Role |",
        "|---|---|",
        "| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.draft.jsonl` | Canonical annual modeled allocation rows by Table 3.2 subfunction. |",
        "| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.annual-long.csv` | Chart-ready long CSV view with one row per fiscal year and subfunction. |",
        "| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.decade-long.csv` | Chart-ready decade rollup by subfunction. |",
        "| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.fy2025-top-subfunctions.csv` | Chart-ready FY2025 ranked view for the largest modeled subfunction allocations. |",
        "",
        "## Method",
        "",
        "For each fiscal year and emitted Table 3.2 subfunction:",
        "",
        "```text",
        "outlay_share_percent = subfunction_outlays / total_federal_outlays * 100",
        "allocation_share_percent = subfunction_outlays / sum_of_subfunction_outlays * 100",
        "modeled_income_tax_allocation = individual_income_tax_receipts",
        "                                * subfunction_outlays",
        "                                / sum_of_subfunction_outlays",
        "```",
        "",
        "The allocation denominator uses the emitted subfunction rows so modeled rows sum back to individual income-tax receipts. Small differences from displayed total outlays are source rounding.",
        "",
        "## Decade Rollup Caveat",
        "",
        "The decade-long CSV sums modeled allocation dollars within each decade and then calculates each subfunction's share of that decade total. It is not an average of annual percentages or annual ranks.",
        "",
        "The 1960s bucket is partial because subfunction actual-year coverage starts in FY1962. The 2020s bucket is partial because the actual-year model currently ends in FY2025.",
        "",
        "## Regeneration",
        "",
        "```powershell",
        "cargo run -p taxlane-tools -- income-tax-outlay subfunction-model",
        "cargo run -p taxlane-tools -- income-tax-outlay subfunction-model --check",
        "cargo run -p taxlane-tools -- income-tax-outlay subfunction-export",
        "cargo run -p taxlane-tools -- income-tax-outlay subfunction-export --check",
        "```",
        "",
        "## Validation Command",
        "",
        "```powershell",
        "cargo run -p taxlane-tools -- income-tax-outlay validate",
        "```",
        "",
    ]
    .join("\n")
}

fn table_2_2_year_label(value: Option<&CellValue>) -> Option<String> {
    text_cell(value).or_else(|| int_cell(value).map(|year| year.to_string()))
}

fn parse_table_2_2_year(label: &str) -> Option<(i64, &'static str)> {
    let trimmed = label.trim();
    if trimmed == "TQ" {
        return None;
    }
    if let Some(year) = trimmed.strip_suffix(" estimate") {
        return year.parse::<i64>().ok().map(|year| (year, "estimate"));
    }
    trimmed.parse::<i64>().ok().map(|year| {
        let status = if year <= 2025 { "actual" } else { "estimate" };
        (year, status)
    })
}

fn receipt_share_sort_key(category: &str) -> usize {
    RECEIPT_SHARE_CATEGORIES
        .iter()
        .position(|candidate| candidate.receipt_category == category)
        .unwrap_or(usize::MAX)
}

fn validate_receipt_share_rows(rows: &[ReceiptShareRow]) -> Result<(), String> {
    if rows.len() != 588 {
        return Err(format!(
            "expected 588 Table 2.2 receipt share rows, found {}",
            rows.len()
        ));
    }

    let mut by_year: BTreeMap<i64, Vec<&ReceiptShareRow>> = BTreeMap::new();
    for row in rows {
        if !(0.0..=100.0).contains(&row.percent) {
            return Err(format!(
                "{} {} percent out of range: {}",
                row.fiscal_year, row.receipt_category, row.percent
            ));
        }
        by_year.entry(row.fiscal_year).or_default().push(row);
    }

    for (year, year_rows) in by_year {
        if year_rows.len() != RECEIPT_SHARE_CATEGORIES.len() {
            return Err(format!(
                "{year}: expected {} share rows, found {}",
                RECEIPT_SHARE_CATEGORIES.len(),
                year_rows.len()
            ));
        }
        let category_sum: f64 = year_rows
            .iter()
            .filter(|row| row.receipt_category != "total-receipts")
            .map(|row| row.percent)
            .sum();
        if (category_sum - 100.0).abs() > 0.25 {
            return Err(format!(
                "{year}: receipt-source shares sum to {category_sum}"
            ));
        }
        let total = year_rows
            .iter()
            .find(|row| row.receipt_category == "total-receipts")
            .map(|row| row.percent)
            .ok_or_else(|| format!("{year}: missing total receipts share"))?;
        if (total - 100.0).abs() > 0.000001 {
            return Err(format!("{year}: total receipts share is {total}"));
        }
    }
    Ok(())
}

fn receipt_share_jsonl(rows: &[ReceiptShareRow]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        let mut source_ids = vec!["SRC-OMB-HIST-2-2-FY2027"];
        if row.receipt_category == "individual-income-taxes" {
            source_ids.push("SRC-OMB-AP-13-FUNDS-FY2027");
        }
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"receipt_source\",\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[{}],\"source_table\":\"OMB Historical Table 2.2 FY2027\",\"source_row_ref\":{},\"receipt_category\":{},\"source_receipt_label\":{},\"measure\":\"share_of_total\",\"amount\":null,\"percent\":{},\"amount_units\":\"percent\",\"actual_or_projection\":{},\"fund_group_link\":null,\"allocation_status\":{},\"status\":\"draft\",\"observed_date\":{},\"notes\":{}}}",
            json_string(&format!(
                "receipt:{}:{}:share-of-total",
                row.fiscal_year, row.receipt_category
            )),
            row.fiscal_year,
            source_ids
                .iter()
                .map(|source| json_string(source))
                .collect::<Vec<_>>()
                .join(","),
            json_string(&format!(
                "Table!A{}:{}{}; column {} {}",
                row.source_row,
                row.source_column,
                row.source_row,
                row.source_column,
                row.source_receipt_label
            )),
            json_string(row.receipt_category),
            json_string(row.source_receipt_label),
            decimal_string(row.percent, 6),
            json_string(row.actual_or_projection),
            json_string(row.allocation_status),
            json_string(OBSERVED_DATE),
            json_string(row.notes),
        ));
    }
    lines.join("\n") + "\n"
}

fn receipt_share_profile_markdown(rows: &[ReceiptShareRow]) -> Result<String, String> {
    let first_year = rows
        .first()
        .ok_or_else(|| "no Table 2.2 rows".to_string())?
        .fiscal_year;
    let last_year = rows
        .last()
        .ok_or_else(|| "no Table 2.2 rows".to_string())?
        .fiscal_year;
    let year_count = rows.len() / RECEIPT_SHARE_CATEGORIES.len();
    let estimate_count = rows
        .iter()
        .filter(|row| row.actual_or_projection == "estimate")
        .map(|row| row.fiscal_year)
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let sample_years = [1934, 1940, 1980, 2000, 2025, 2031];
    let mut by_year: BTreeMap<i64, BTreeMap<&str, f64>> = BTreeMap::new();
    for row in rows {
        by_year
            .entry(row.fiscal_year)
            .or_default()
            .insert(row.receipt_category, row.percent);
    }

    let mut lines = vec![
        "# OMB Table 2.2 Receipt Share Profile".to_string(),
        String::new(),
        "## Source".to_string(),
        String::new(),
        "- Source ID: `SRC-OMB-HIST-2-2-FY2027`".to_string(),
        "- Raw artifact: `data/raw/omb/SRC-OMB-HIST-2-2-FY2027/2026-06-21/hist02z2_fy2027.xlsx`"
            .to_string(),
        "- Table title: `Table 2.2 - PERCENTAGE COMPOSITION OF RECEIPTS BY SOURCE: 1934 - 2031`"
            .to_string(),
        String::new(),
        "## Coverage".to_string(),
        String::new(),
        format!("- Fiscal years emitted: {first_year}-{last_year}"),
        format!("- Year count: {year_count}"),
        format!("- Estimate years: {estimate_count}"),
        format!("- Record count: {}", rows.len()),
        String::new(),
        "## Extracted Columns".to_string(),
        String::new(),
        "| Column | Receipt category | Source label |".to_string(),
        "|---|---|---|".to_string(),
    ];
    for category in RECEIPT_SHARE_CATEGORIES {
        lines.push(format!(
            "| {} | `{}` | {} |",
            category.column, category.receipt_category, category.source_receipt_label
        ));
    }
    lines.extend([
        String::new(),
        "## Sample Shares".to_string(),
        String::new(),
        "Percentages are OMB-reported shares of total receipts.".to_string(),
        String::new(),
        "| Fiscal year | Individual income | Corporation income | Social insurance | Excise | Other | Total receipts |".to_string(),
        "|---:|---:|---:|---:|---:|---:|---:|".to_string(),
    ]);
    for year in sample_years {
        let categories = by_year
            .get(&year)
            .ok_or_else(|| format!("missing sample year {year}"))?;
        lines.push(format!(
            "| {year} | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% |",
            categories["individual-income-taxes"],
            categories["corporation-income-taxes"],
            categories["social-insurance-and-retirement-receipts"],
            categories["excise-taxes"],
            categories["other-receipts"],
            categories["total-receipts"],
        ));
    }
    lines.extend([
        String::new(),
        "## Extraction Decisions".to_string(),
        String::new(),
        "- Keep Table 2.2 percentage rows separate from Table 2.1 amount rows.".to_string(),
        "- Skip the transition-quarter `TQ` row because it is not a fiscal year.".to_string(),
        "- Preserve estimate years as `actual_or_projection = \"estimate\"`.".to_string(),
        "- Treat total receipts as `mixed` because it combines categories with different budget treatment.".to_string(),
        "- Keep non-individual receipt allocation labels as `unknown` pending narrower review.".to_string(),
        String::new(),
    ]);
    Ok(lines.join("\n"))
}

fn build_annual_model(root: &Path, check_only: bool) -> Result<(), String> {
    let (records, profile) = build_annual_records(root)?;
    let jsonl = annual_model_jsonl(&records);
    let markdown = source_profile_markdown(&profile);

    if check_only {
        compare_text(root, ANNUAL_JSONL_PATH, &jsonl, "annual model JSONL")?;
        compare_text(root, SOURCE_PROFILE_PATH, &markdown, "source profile")?;
    } else {
        fs::write(root.join(ANNUAL_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {ANNUAL_JSONL_PATH}: {err}"))?;
        fs::write(root.join(SOURCE_PROFILE_PATH), markdown)
            .map_err(|err| format!("failed to write {SOURCE_PROFILE_PATH}: {err}"))?;
    }

    println!(
        "validated {} rows for {}-{}",
        profile.record_count, profile.first_year, profile.last_year
    );
    Ok(())
}

#[derive(Clone)]
enum CellValue {
    Number(f64),
    Text(String),
}

#[derive(Clone)]
struct Table11Row {
    row: i64,
    total_receipts: f64,
    total_outlays: f64,
    surplus_or_deficit: f64,
}

#[derive(Clone)]
struct Table21Row {
    row: i64,
    individual_income_tax: f64,
}

#[derive(Clone)]
struct AnnualRecord {
    fiscal_year: i64,
    category_key: &'static str,
    category_label: &'static str,
    table_11_row: i64,
    table_21_row: i64,
    table_31_row: i64,
    category_outlays_amount: f64,
    total_outlays_amount: f64,
    category_total_outlays_amount: f64,
    individual_income_tax_receipts_amount: f64,
    outlay_share_percent: f64,
    allocation_share_percent: f64,
    modeled_income_tax_allocation_amount: f64,
    total_receipts_amount: f64,
    surplus_or_deficit_amount: f64,
    deficit_gap_amount: f64,
    borrowed_share_percent_of_outlays: f64,
    income_tax_coverage_percent_of_outlays: f64,
    category_total_reconciliation_difference_amount: f64,
}

struct AnnualCheck {
    year: i64,
    table_1_1_outlays: f64,
    table_3_1_outlays: f64,
    category_total: f64,
    income_tax: f64,
    modeled_sum: f64,
    deficit_gap: f64,
}

struct AnnualProfile {
    year_count: usize,
    first_year: i64,
    last_year: i64,
    record_count: usize,
    annual_checks: Vec<AnnualCheck>,
}

fn build_annual_records(root: &Path) -> Result<(Vec<AnnualRecord>, AnnualProfile), String> {
    let t11 = parse_table_1_1(&read_sheet(&root.join(TABLE_1_1_PATH))?);
    let t21 = parse_table_2_1(&read_sheet(&root.join(TABLE_2_1_PATH))?);
    let (years_31, t31) = parse_table_3_1(&read_sheet(&root.join(TABLE_3_1_PATH))?)?;
    let years: Vec<i64> = years_31
        .into_iter()
        .filter(|year| (1940..=2025).contains(year))
        .collect();

    let mut records = Vec::new();
    let mut errors = Vec::new();
    let mut annual_checks = Vec::new();

    for year in &years {
        let Some(table_11) = t11.get(year) else {
            errors.push(format!("{year}: missing Table 1.1 row"));
            continue;
        };
        let Some(table_21) = t21.get(year) else {
            errors.push(format!("{year}: missing Table 2.1 row"));
            continue;
        };
        let Some(total_outlays_31) = t31
            .get("total-federal-outlays")
            .and_then(|values| values.get(year))
            .copied()
        else {
            errors.push(format!("{year}: missing Table 3.1 total outlays"));
            continue;
        };

        if (table_11.total_outlays - total_outlays_31).abs() > 0.5 {
            errors.push(format!(
                "{year}: Table 3.1 total {total_outlays_31} does not reconcile to Table 1.1 total {}",
                table_11.total_outlays
            ));
        }

        let category_total: f64 = BROAD_CATEGORIES
            .iter()
            .map(|(key, _, _)| {
                t31.get(*key)
                    .and_then(|values| values.get(year))
                    .copied()
                    .ok_or_else(|| format!("{year}: missing Table 3.1 category {key}"))
            })
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum();
        let category_total_difference = category_total - total_outlays_31;
        if (category_total - total_outlays_31).abs() > 2.0 {
            errors.push(format!(
                "{year}: category total {category_total} does not reconcile to Table 3.1 total {total_outlays_31}"
            ));
        }

        let income_tax = table_21.individual_income_tax;
        let total_receipts = table_11.total_receipts;
        let surplus_or_deficit = table_11.surplus_or_deficit;
        let deficit_gap = (total_outlays_31 - total_receipts).max(0.0);
        let borrowed_share = deficit_gap / total_outlays_31 * 100.0;
        let income_tax_coverage = income_tax / total_outlays_31 * 100.0;
        let mut modeled_sum = 0.0;

        for (key, label, table_row) in BROAD_CATEGORIES {
            let category_outlays = t31
                .get(*key)
                .and_then(|values| values.get(year))
                .copied()
                .ok_or_else(|| format!("{year}: missing Table 3.1 category {key}"))?;
            let outlay_share = category_outlays / total_outlays_31 * 100.0;
            let allocation_share = category_outlays / category_total * 100.0;
            let modeled_amount = income_tax * category_outlays / category_total;
            modeled_sum += modeled_amount;
            records.push(AnnualRecord {
                fiscal_year: *year,
                category_key: key,
                category_label: label,
                table_11_row: table_11.row,
                table_21_row: table_21.row,
                table_31_row: *table_row,
                category_outlays_amount: round6(category_outlays),
                total_outlays_amount: round6(total_outlays_31),
                category_total_outlays_amount: round6(category_total),
                individual_income_tax_receipts_amount: round6(income_tax),
                outlay_share_percent: round9(outlay_share),
                allocation_share_percent: round9(allocation_share),
                modeled_income_tax_allocation_amount: round6(modeled_amount),
                total_receipts_amount: round6(total_receipts),
                surplus_or_deficit_amount: round6(surplus_or_deficit),
                deficit_gap_amount: round6(deficit_gap),
                borrowed_share_percent_of_outlays: round9(borrowed_share),
                income_tax_coverage_percent_of_outlays: round9(income_tax_coverage),
                category_total_reconciliation_difference_amount: round6(category_total_difference),
            });
        }

        if (modeled_sum - income_tax).abs() > 0.0005 {
            errors.push(format!(
                "{year}: modeled allocation sum {modeled_sum} does not match individual income-tax receipts {income_tax}"
            ));
        }
        annual_checks.push(AnnualCheck {
            year: *year,
            table_1_1_outlays: table_11.total_outlays,
            table_3_1_outlays: total_outlays_31,
            category_total,
            income_tax,
            modeled_sum,
            deficit_gap,
        });
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years.first().ok_or_else(|| "no annual years".to_string())?;
    let last_year = *years.last().ok_or_else(|| "no annual years".to_string())?;
    let profile = AnnualProfile {
        year_count: years.len(),
        first_year,
        last_year,
        record_count: records.len(),
        annual_checks,
    };
    Ok((records, profile))
}

fn read_sheet(path: &Path) -> Result<BTreeMap<i64, BTreeMap<String, CellValue>>, String> {
    let file = File::open(path).map_err(|err| format!("failed to open {:?}: {err}", path))?;
    let mut archive =
        ZipArchive::new(file).map_err(|err| format!("failed to read XLSX {:?}: {err}", path))?;
    let shared = read_shared_strings(&mut archive)?;
    let mut sheet_xml = String::new();
    archive
        .by_name("xl/worksheets/sheet1.xml")
        .map_err(|err| format!("failed to read sheet1.xml from {:?}: {err}", path))?
        .read_to_string(&mut sheet_xml)
        .map_err(|err| format!("failed to decode sheet1.xml from {:?}: {err}", path))?;
    let doc = Document::parse(&sheet_xml)
        .map_err(|err| format!("failed to parse sheet1.xml from {:?}: {err}", path))?;
    let mut rows = BTreeMap::new();
    for row in doc.descendants().filter(|node| node.has_tag_name("row")) {
        let row_num = row
            .attribute("r")
            .and_then(|value| value.parse::<i64>().ok())
            .ok_or_else(|| format!("sheet row without numeric r in {:?}", path))?;
        let mut cells = BTreeMap::new();
        for cell in row.children().filter(|node| node.has_tag_name("c")) {
            let Some(reference) = cell.attribute("r") else {
                continue;
            };
            let column = cell_column(reference);
            if column.is_empty() {
                continue;
            }
            let cell_type = cell.attribute("t");
            let raw = cell
                .children()
                .find(|node| node.has_tag_name("v"))
                .and_then(|node| node.text());
            let value = match (cell_type, raw) {
                (Some("s"), Some(raw)) => shared
                    .get(raw.parse::<usize>().map_err(|err| {
                        format!("invalid shared string index {raw:?} in {:?}: {err}", path)
                    })?)
                    .cloned(),
                (Some("inlineStr"), _) => Some(
                    cell.descendants()
                        .filter(|node| node.has_tag_name("t"))
                        .filter_map(|node| node.text())
                        .collect::<String>(),
                ),
                (_, Some(raw)) => Some(raw.to_string()),
                _ => None,
            };
            if let Some(value) = value.and_then(|value| cell_value(&value)) {
                cells.insert(column, value);
            }
        }
        rows.insert(row_num, cells);
    }
    Ok(rows)
}

fn read_shared_strings<R: Read + std::io::Seek>(
    archive: &mut ZipArchive<R>,
) -> Result<Vec<String>, String> {
    let Ok(mut file) = archive.by_name("xl/sharedStrings.xml") else {
        return Ok(Vec::new());
    };
    let mut xml = String::new();
    file.read_to_string(&mut xml)
        .map_err(|err| format!("failed to decode sharedStrings.xml: {err}"))?;
    let doc =
        Document::parse(&xml).map_err(|err| format!("failed to parse sharedStrings.xml: {err}"))?;
    let strings = doc
        .descendants()
        .filter(|node| node.has_tag_name("si"))
        .map(|si| {
            si.descendants()
                .filter(|node| node.has_tag_name("t"))
                .filter_map(|node| node.text())
                .collect::<String>()
        })
        .collect();
    Ok(strings)
}

fn cell_column(reference: &str) -> String {
    reference
        .chars()
        .take_while(|char| char.is_ascii_alphabetic())
        .collect()
}

fn cell_value(raw: &str) -> Option<CellValue> {
    let value = raw.trim();
    if value.is_empty() || value == ".........." {
        return None;
    }
    if value == "-*" {
        return Some(CellValue::Number(0.0));
    }
    match value.parse::<f64>() {
        Ok(number) => Some(CellValue::Number(number)),
        Err(_) => Some(CellValue::Text(value.to_string())),
    }
}

fn parse_table_1_1(rows: &BTreeMap<i64, BTreeMap<String, CellValue>>) -> BTreeMap<i64, Table11Row> {
    let mut output = BTreeMap::new();
    for (row_num, cells) in rows {
        let Some(year) = int_cell(cells.get("A")) else {
            continue;
        };
        let (Some(receipts), Some(outlays), Some(surplus_or_deficit)) = (
            number_cell(cells.get("B")),
            number_cell(cells.get("C")),
            number_cell(cells.get("D")),
        ) else {
            continue;
        };
        output.insert(
            year,
            Table11Row {
                row: *row_num,
                total_receipts: receipts,
                total_outlays: outlays,
                surplus_or_deficit,
            },
        );
    }
    output
}

fn parse_table_2_1(rows: &BTreeMap<i64, BTreeMap<String, CellValue>>) -> BTreeMap<i64, Table21Row> {
    let mut output = BTreeMap::new();
    for (row_num, cells) in rows {
        let (Some(year), Some(amount)) = (int_cell(cells.get("A")), number_cell(cells.get("B")))
        else {
            continue;
        };
        output.insert(
            year,
            Table21Row {
                row: *row_num,
                individual_income_tax: amount,
            },
        );
    }
    output
}

fn parse_table_3_1(
    rows: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<(Vec<i64>, BTreeMap<String, BTreeMap<i64, f64>>), String> {
    let header = rows
        .get(&2)
        .ok_or_else(|| "missing Table 3.1 header row 2".to_string())?;
    let mut years_by_col = BTreeMap::new();
    for (column, value) in header {
        if let Some(year) = int_cell(Some(value)) {
            years_by_col.insert(column.clone(), year);
        }
    }

    let mut categories = BTreeMap::new();
    let mut table_rows: Vec<(&str, &str, i64)> = BROAD_CATEGORIES.to_vec();
    table_rows.push(("total-federal-outlays", "Total, Federal outlays", 35));
    for (key, label, row_num) in table_rows {
        let cells = rows
            .get(&row_num)
            .ok_or_else(|| format!("missing Table 3.1 row {row_num}"))?;
        if text_cell(cells.get("A")).as_deref() != Some(label) {
            return Err(format!(
                "Unexpected Table 3.1 row {row_num}: {:?}",
                text_cell(cells.get("A"))
            ));
        }
        let mut values = BTreeMap::new();
        for (column, year) in &years_by_col {
            if let Some(value) = number_cell(cells.get(column)) {
                values.insert(*year, value);
            }
        }
        categories.insert(key.to_string(), values);
    }
    let mut years = years_by_col.values().copied().collect::<Vec<_>>();
    years.sort_unstable();
    Ok((years, categories))
}

fn table_3_1_year_columns(
    rows: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<BTreeMap<i64, String>, String> {
    let header = rows
        .get(&2)
        .ok_or_else(|| "missing Table 3.1 header row 2".to_string())?;
    let mut columns = BTreeMap::new();
    for (column, value) in header {
        if let Some(year) = int_cell(Some(value)) {
            columns.insert(year, column.clone());
        }
    }
    Ok(columns)
}

fn table_3_2_year_columns(
    rows: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<BTreeMap<i64, String>, String> {
    let header = rows
        .get(&3)
        .ok_or_else(|| "missing Table 3.2 header row 3".to_string())?;
    let mut columns = BTreeMap::new();
    for (column, value) in header {
        let year = match value {
            CellValue::Number(number) if number.fract() == 0.0 => Some(*number as i64),
            CellValue::Text(text) => parse_table_3_2_year(text),
            _ => None,
        };
        if let Some(year) = year {
            columns.insert(year, column.clone());
        }
    }
    Ok(columns)
}

fn parse_table_3_2_year(label: &str) -> Option<i64> {
    let trimmed = label.trim();
    if trimmed == "TQ" {
        return None;
    }
    trimmed
        .strip_suffix(" estimate")
        .unwrap_or(trimmed)
        .parse::<i64>()
        .ok()
}

fn int_cell(value: Option<&CellValue>) -> Option<i64> {
    match value {
        Some(CellValue::Number(number)) if number.fract() == 0.0 => Some(*number as i64),
        _ => None,
    }
}

fn number_cell(value: Option<&CellValue>) -> Option<f64> {
    match value {
        Some(CellValue::Number(number)) => Some(*number),
        _ => None,
    }
}

fn text_cell(value: Option<&CellValue>) -> Option<String> {
    match value {
        Some(CellValue::Text(text)) => Some(text.clone()),
        _ => None,
    }
}

fn annual_model_jsonl(records: &[AnnualRecord]) -> String {
    let mut lines = Vec::new();
    for record in records {
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"income_tax_outlay_model\",\"model_id\":{},\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[{}],\"source_table_refs\":{{\"fiscal_spine\":{},\"tax_receipts\":{},\"outlay_category\":{},\"outlay_total\":\"OMB Historical Table 3.1 FY2027 row 35\"}},\"tax_source\":\"individual-income-taxes\",\"allocation_method\":\"proportional_outlay_share\",\"legal_allocation_status\":\"modeled_not_legal_dedication\",\"category_key\":{},\"category_label\":{},\"category_outlays_amount\":{},\"total_outlays_amount\":{},\"category_total_outlays_amount\":{},\"individual_income_tax_receipts_amount\":{},\"outlay_share_percent\":{},\"allocation_share_percent\":{},\"modeled_income_tax_allocation_amount\":{},\"total_receipts_amount\":{},\"surplus_or_deficit_amount\":{},\"deficit_gap_amount\":{},\"borrowed_share_percent_of_outlays\":{},\"income_tax_coverage_percent_of_outlays\":{},\"category_total_reconciliation_difference_amount\":{},\"actual_or_projection\":\"actual\",\"status\":\"draft\",\"observed_date\":{},\"notes\":\"Modeled allocation of ordinary individual income-tax receipts by broad Table 3.1 outlay share, normalized over displayed broad-category rows to handle source rounding; not legal dedication or program tracing.\"}}",
            json_string(&format!("income-tax-outlay-model:{}:{}", record.fiscal_year, record.category_key)),
            json_string(MODEL_ID),
            record.fiscal_year,
            SOURCE_IDS.iter().map(|source| json_string(source)).collect::<Vec<_>>().join(","),
            json_string(&format!("OMB Historical Table 1.1 FY2027 row {}", record.table_11_row)),
            json_string(&format!("OMB Historical Table 2.1 FY2027 row {}, column B", record.table_21_row)),
            json_string(&format!("OMB Historical Table 3.1 FY2027 row {}", record.table_31_row)),
            json_string(record.category_key),
            json_string(record.category_label),
            decimal_string(record.category_outlays_amount, 6),
            decimal_string(record.total_outlays_amount, 6),
            decimal_string(record.category_total_outlays_amount, 6),
            decimal_string(record.individual_income_tax_receipts_amount, 6),
            decimal_string(record.outlay_share_percent, 9),
            decimal_string(record.allocation_share_percent, 9),
            decimal_string(record.modeled_income_tax_allocation_amount, 6),
            decimal_string(record.total_receipts_amount, 6),
            decimal_string(record.surplus_or_deficit_amount, 6),
            annual_deficit_gap_string(record.deficit_gap_amount),
            decimal_string(record.borrowed_share_percent_of_outlays, 9),
            decimal_string(record.income_tax_coverage_percent_of_outlays, 9),
            decimal_string(record.category_total_reconciliation_difference_amount, 6),
            json_string(OBSERVED_DATE),
        ));
    }
    lines.join("\n") + "\n"
}

fn source_profile_markdown(profile: &AnnualProfile) -> String {
    let sample_years = [1940, 1950, 1960, 1970, 1980, 1990, 2000, 2010, 2020, 2025];
    let mut lines = vec![
        "# Income-Tax Outlay Model Source Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        format!("- Model ID: `{MODEL_ID}`"),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        format!("- Record count: {}", profile.record_count),
        "- Actual/projection treatment: actual years only; FY2026-FY2031 are excluded.".to_string(),
        String::new(),
        "## Source Roles".to_string(),
        String::new(),
        "| Source ID | Use |".to_string(),
        "|---|---|".to_string(),
        "| `SRC-OMB-HIST-1-1-FY2027` | Total receipts, total outlays, and surplus/deficit. |"
            .to_string(),
        "| `SRC-OMB-HIST-2-1-FY2027` | Individual income-tax receipts. |".to_string(),
        "| `SRC-OMB-HIST-3-1-FY2027` | Broad outlay categories and total federal outlays. |"
            .to_string(),
        String::new(),
        "## Broad Categories".to_string(),
        String::new(),
        "| Category key | OMB label | Table 3.1 row |".to_string(),
        "|---|---|---:|".to_string(),
    ];
    for (key, label, row_num) in BROAD_CATEGORIES {
        lines.push(format!("| `{key}` | {label} | {row_num} |"));
    }
    lines.extend([
        String::new(),
        "## Reconciliation Sample".to_string(),
        String::new(),
        "All amounts are in millions of dollars. `Modeled sum` is the sum of".to_string(),
        "the six category allocation rows for the fiscal year.".to_string(),
        String::new(),
        "| Fiscal year | Table 1.1 outlays | Table 3.1 outlays | Category total | Income tax receipts | Modeled sum | Deficit gap |".to_string(),
        "|---:|---:|---:|---:|---:|---:|---:|".to_string(),
    ]);
    for row in profile
        .annual_checks
        .iter()
        .filter(|row| sample_years.contains(&row.year))
    {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} | {} |",
            row.year,
            comma_number(row.table_1_1_outlays, 0),
            comma_number(row.table_3_1_outlays, 0),
            comma_number(row.category_total, 0),
            comma_number(row.income_tax, 0),
            comma_number(row.modeled_sum, 3),
            comma_number(row.deficit_gap, 0),
        ));
    }
    lines.extend([
        String::new(),
        "## Model Caveat".to_string(),
        String::new(),
        "These rows allocate individual income-tax receipts by reported outlay".to_string(),
        "share, normalized over the displayed broad-category rows when source".to_string(),
        "rounding creates a small difference from the displayed total. They do".to_string(),
        "not claim that income-tax dollars were legally dedicated to the listed".to_string(),
        "outlay categories.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

fn build_decade_summary(root: &Path, check_only: bool) -> Result<(), String> {
    let rows = build_decade_summary_rows(root)?;
    validate_decade_summary_rows(&rows)?;
    let jsonl = decade_summary_jsonl(&rows);
    let markdown = decade_summary_markdown(&rows)?;

    if check_only {
        compare_text(root, DECADE_JSONL_PATH, &jsonl, "decade JSONL")?;
        compare_text(root, DECADE_MD_PATH, &markdown, "decade Markdown")?;
    } else {
        fs::write(root.join(DECADE_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {DECADE_JSONL_PATH}: {err}"))?;
        fs::write(root.join(DECADE_MD_PATH), markdown)
            .map_err(|err| format!("failed to write {DECADE_MD_PATH}: {err}"))?;
    }
    println!("validated {} decade summary rows", rows.len());
    Ok(())
}

#[derive(Clone)]
struct DecadeSummaryRow {
    decade: String,
    start_fiscal_year: i64,
    end_fiscal_year: i64,
    year_count: usize,
    coverage_note: &'static str,
    category_key: String,
    category_label: String,
    cumulative_modeled_income_tax_allocation_amount: f64,
    cumulative_individual_income_tax_receipts_amount: f64,
    category_percent_of_decade_income_tax: f64,
    cumulative_total_outlays_amount: f64,
    cumulative_total_receipts_amount: f64,
    cumulative_deficit_gap_amount: f64,
    borrowed_share_percent_of_outlays: f64,
    income_tax_coverage_percent_of_outlays: f64,
}

fn build_decade_summary_rows(root: &Path) -> Result<Vec<DecadeSummaryRow>, String> {
    let annual_rows = read_jsonl(root.join(ANNUAL_JSONL_PATH))?;
    let mut by_decade: BTreeMap<String, Vec<serde_json::Value>> = BTreeMap::new();
    for row in annual_rows {
        let year = int_field(&row, "fiscal_year")?;
        by_decade.entry(decade_label(year)).or_default().push(row);
    }

    let mut output = Vec::new();
    for (decade, decade_rows) in by_decade {
        let mut years: Vec<i64> = decade_rows
            .iter()
            .map(|row| int_field(row, "fiscal_year"))
            .collect::<Result<Vec<_>, _>>()?;
        years.sort_unstable();
        years.dedup();

        for year in &years {
            let count = decade_rows
                .iter()
                .filter(|row| int_field(row, "fiscal_year").ok() == Some(*year))
                .count();
            if count != CATEGORY_FIELDS.len() {
                return Err(format!(
                    "{decade}: expected six category rows for fiscal year {year}, found {count}"
                ));
            }
        }

        let anchors: Vec<&serde_json::Value> = decade_rows
            .iter()
            .filter(|row| {
                string_field(row, "category_key").ok().as_deref() == Some("national-defense")
            })
            .collect();
        let income_tax_total = sum_field(&anchors, "individual_income_tax_receipts_amount")?;
        let total_outlays = sum_field(&anchors, "total_outlays_amount")?;
        let total_receipts = sum_field(&anchors, "total_receipts_amount")?;
        let deficit_gap = sum_field(&anchors, "deficit_gap_amount")?;
        let borrowed_share = if total_outlays == 0.0 {
            0.0
        } else {
            deficit_gap / total_outlays * 100.0
        };
        let income_tax_coverage = if total_outlays == 0.0 {
            0.0
        } else {
            income_tax_total / total_outlays * 100.0
        };

        let mut percent_sum = 0.0;
        for (category_key, _) in CATEGORY_FIELDS {
            let category_rows: Vec<&serde_json::Value> = decade_rows
                .iter()
                .filter(|row| {
                    string_field(row, "category_key").ok().as_deref() == Some(*category_key)
                })
                .collect();
            if category_rows.len() != years.len() {
                return Err(format!("{decade}: missing {category_key} rows"));
            }
            let modeled_total = sum_field(&category_rows, "modeled_income_tax_allocation_amount")?;
            let category_percent = modeled_total / income_tax_total * 100.0;
            percent_sum += category_percent;
            output.push(DecadeSummaryRow {
                decade: decade.clone(),
                start_fiscal_year: *years.first().ok_or_else(|| format!("{decade}: no years"))?,
                end_fiscal_year: *years.last().ok_or_else(|| format!("{decade}: no years"))?,
                year_count: years.len(),
                coverage_note: if years.len() < 10 {
                    "partial_decade"
                } else {
                    "full_decade"
                },
                category_key: (*category_key).to_string(),
                category_label: string_field(category_rows[0], "category_label")?,
                cumulative_modeled_income_tax_allocation_amount: round6(modeled_total),
                cumulative_individual_income_tax_receipts_amount: round6(income_tax_total),
                category_percent_of_decade_income_tax: round9(category_percent),
                cumulative_total_outlays_amount: round6(total_outlays),
                cumulative_total_receipts_amount: round6(total_receipts),
                cumulative_deficit_gap_amount: round6(deficit_gap),
                borrowed_share_percent_of_outlays: round9(borrowed_share),
                income_tax_coverage_percent_of_outlays: round9(income_tax_coverage),
            });
        }
        if (percent_sum - 100.0).abs() > 0.00001 {
            return Err(format!(
                "{decade}: category percentages sum to {percent_sum}"
            ));
        }
    }
    Ok(output)
}

fn validate_decade_summary_rows(rows: &[DecadeSummaryRow]) -> Result<(), String> {
    if rows.len() != 54 {
        return Err(format!(
            "expected 54 decade summary rows, found {}",
            rows.len()
        ));
    }
    let mut by_decade: BTreeMap<&str, usize> = BTreeMap::new();
    for row in rows {
        *by_decade.entry(&row.decade).or_default() += 1;
    }
    for (decade, count) in by_decade {
        if count != CATEGORY_FIELDS.len() {
            return Err(format!(
                "{decade}: expected six category rows, found {count}"
            ));
        }
    }
    Ok(())
}

fn decade_summary_jsonl(rows: &[DecadeSummaryRow]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"income_tax_outlay_model_decade_summary\",\"source_record_family\":\"income_tax_outlay_model\",\"model_id\":\"individual-income-tax-proportional-outlays-v1\",\"decade\":{},\"start_fiscal_year\":{},\"end_fiscal_year\":{},\"year_count\":{},\"coverage_note\":{},\"category_key\":{},\"category_label\":{},\"cumulative_modeled_income_tax_allocation_amount\":{},\"cumulative_individual_income_tax_receipts_amount\":{},\"category_percent_of_decade_income_tax\":{},\"cumulative_total_outlays_amount\":{},\"cumulative_total_receipts_amount\":{},\"cumulative_deficit_gap_amount\":{},\"borrowed_share_percent_of_outlays\":{},\"income_tax_coverage_percent_of_outlays\":{},\"allocation_method\":\"proportional_outlay_share\",\"legal_allocation_status\":\"modeled_not_legal_dedication\",\"actual_or_projection\":\"actual\",\"status\":\"draft\",\"notes\":\"Decade summary derived from annual modeled allocation rows; not legal dedication or program tracing.\"}}",
            json_string(&format!("income-tax-outlay-model:{}:{}:decade-summary", row.decade, row.category_key)),
            json_string(&row.decade),
            row.start_fiscal_year,
            row.end_fiscal_year,
            row.year_count,
            json_string(row.coverage_note),
            json_string(&row.category_key),
            json_string(&row.category_label),
            decimal_string(row.cumulative_modeled_income_tax_allocation_amount, 6),
            decimal_string(row.cumulative_individual_income_tax_receipts_amount, 6),
            decimal_string(row.category_percent_of_decade_income_tax, 9),
            decimal_string(row.cumulative_total_outlays_amount, 6),
            decimal_string(row.cumulative_total_receipts_amount, 6),
            decimal_string(row.cumulative_deficit_gap_amount, 6),
            decimal_string(row.borrowed_share_percent_of_outlays, 9),
            decimal_string(row.income_tax_coverage_percent_of_outlays, 9),
        ));
    }
    lines.join("\n") + "\n"
}

fn decade_summary_markdown(rows: &[DecadeSummaryRow]) -> Result<String, String> {
    let mut by_decade: BTreeMap<&str, BTreeMap<&str, &DecadeSummaryRow>> = BTreeMap::new();
    for row in rows {
        by_decade
            .entry(&row.decade)
            .or_default()
            .insert(&row.category_key, row);
    }

    let mut lines = vec![
        "# Decade Summary: Modeled Income-Tax Outlay Allocation".to_string(),
        String::new(),
        "This table summarizes the annual draft model by decade. Category".to_string(),
        "percentages equal cumulative modeled category allocations divided by".to_string(),
        "cumulative individual income-tax receipts for the years in that decade.".to_string(),
        "The 2020s are partial because the current actual-year model ends in 2025.".to_string(),
        String::new(),
        "These are modeled allocations, not legal destinations for income-tax".to_string(),
        "receipts.".to_string(),
        String::new(),
        "| Decade | Years | National defense | Human resources | Physical resources | Net interest | Other functions | Offsetting receipts | Borrowed share of outlays | Income-tax coverage of outlays |".to_string(),
        "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|".to_string(),
    ];

    for (decade, categories) in by_decade {
        let first = categories
            .get("national-defense")
            .ok_or_else(|| format!("{decade}: missing national-defense row"))?;
        let values: Vec<f64> = CATEGORY_FIELDS
            .iter()
            .map(|(category, _)| {
                categories
                    .get(category)
                    .map(|row| row.category_percent_of_decade_income_tax)
                    .ok_or_else(|| format!("{decade}: missing {category} row"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        lines.push(format!(
            "| {} | {}-{} | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% |",
            decade,
            first.start_fiscal_year,
            first.end_fiscal_year,
            values[0],
            values[1],
            values[2],
            values[3],
            values[4],
            values[5],
            first.borrowed_share_percent_of_outlays,
            first.income_tax_coverage_percent_of_outlays
        ));
    }
    Ok(lines.join("\n") + "\n")
}

fn export_chart_views(root: &Path, check_only: bool) -> Result<(), String> {
    let annual = build_annual_csv_rows(root)?;
    let decade = build_decade_csv_rows(root)?;
    validate_csv_rows(&annual, "annual", 86)?;
    validate_csv_rows(&decade, "decade", 9)?;

    if check_only {
        compare_csv(root, ANNUAL_CSV_PATH, ANNUAL_HEADERS, &annual)?;
        compare_csv(root, DECADE_CSV_PATH, DECADE_HEADERS, &decade)?;
    } else {
        write_csv(root, ANNUAL_CSV_PATH, ANNUAL_HEADERS, &annual)?;
        write_csv(root, DECADE_CSV_PATH, DECADE_HEADERS, &decade)?;
    }

    println!(
        "validated {} annual rows and {} decade rows",
        annual.len(),
        decade.len()
    );
    Ok(())
}

fn export_subfunction_chart_views(root: &Path, check_only: bool) -> Result<(), String> {
    let annual = build_subfunction_annual_csv_rows(root)?;
    let decade = build_subfunction_decade_csv_rows(root)?;
    let top = build_subfunction_fy2025_top_csv_rows(root, 25)?;
    validate_subfunction_csv_rows(&annual, "subfunction annual", 4691)?;
    validate_subfunction_decade_csv_rows(&decade)?;
    validate_subfunction_csv_rows(&top, "subfunction FY2025 top", 25)?;

    if check_only {
        compare_csv(
            root,
            SUBFUNCTION_ANNUAL_CSV_PATH,
            SUBFUNCTION_ANNUAL_HEADERS,
            &annual,
        )?;
        compare_csv(
            root,
            SUBFUNCTION_DECADE_CSV_PATH,
            SUBFUNCTION_DECADE_HEADERS,
            &decade,
        )?;
        compare_csv(
            root,
            SUBFUNCTION_FY2025_TOP_CSV_PATH,
            SUBFUNCTION_TOP_HEADERS,
            &top,
        )?;
    } else {
        write_csv(
            root,
            SUBFUNCTION_ANNUAL_CSV_PATH,
            SUBFUNCTION_ANNUAL_HEADERS,
            &annual,
        )?;
        write_csv(
            root,
            SUBFUNCTION_DECADE_CSV_PATH,
            SUBFUNCTION_DECADE_HEADERS,
            &decade,
        )?;
        write_csv(
            root,
            SUBFUNCTION_FY2025_TOP_CSV_PATH,
            SUBFUNCTION_TOP_HEADERS,
            &top,
        )?;
    }

    println!(
        "validated {} subfunction annual rows, {} decade rows, and {} FY2025 top rows",
        annual.len(),
        decade.len(),
        top.len()
    );
    Ok(())
}

fn build_annual_csv_rows(root: &Path) -> Result<Vec<BTreeMap<String, String>>, String> {
    let rows = read_jsonl(root.join(ANNUAL_JSONL_PATH))?;
    let mut grouped: BTreeMap<i64, BTreeMap<String, serde_json::Value>> = BTreeMap::new();
    for row in rows {
        let year = int_field(&row, "fiscal_year")?;
        let category = string_field(&row, "category_key")?;
        grouped.entry(year).or_default().insert(category, row);
    }

    let mut output = Vec::new();
    for (year, categories) in grouped {
        let anchor = categories
            .get("national-defense")
            .ok_or_else(|| format!("{year}: missing national-defense row"))?;
        let mut row = BTreeMap::new();
        row.insert("fiscal_year".to_string(), year.to_string());
        row.insert("coverage_note".to_string(), "full_year".to_string());
        insert_json_number(
            &mut row,
            "individual_income_tax_receipts_millions",
            anchor,
            "individual_income_tax_receipts_amount",
        );
        insert_json_number(
            &mut row,
            "total_outlays_millions",
            anchor,
            "total_outlays_amount",
        );
        insert_json_number(
            &mut row,
            "total_receipts_millions",
            anchor,
            "total_receipts_amount",
        );
        insert_json_number(
            &mut row,
            "deficit_gap_millions",
            anchor,
            "deficit_gap_amount",
        );
        insert_number(
            &mut row,
            "borrowed_share_percent_of_outlays",
            number_field(anchor, "borrowed_share_percent_of_outlays")?,
        );
        insert_number(
            &mut row,
            "income_tax_coverage_percent_of_outlays",
            number_field(anchor, "income_tax_coverage_percent_of_outlays")?,
        );
        row.insert(
            "allocation_method".to_string(),
            string_field(anchor, "allocation_method")?,
        );
        row.insert(
            "legal_allocation_status".to_string(),
            string_field(anchor, "legal_allocation_status")?,
        );
        row.insert(
            "actual_or_projection".to_string(),
            string_field(anchor, "actual_or_projection")?,
        );

        let mut percent_sum = 0.0;
        for (category_key, field_name) in CATEGORY_FIELDS {
            let category = categories
                .get(*category_key)
                .ok_or_else(|| format!("{year}: missing {category_key} row"))?;
            let percent = number_field(category, "allocation_share_percent")?;
            insert_number(&mut row, field_name, percent);
            percent_sum += percent;
        }
        insert_number(&mut row, "category_percent_sum", round6(percent_sum));
        output.push(row);
    }
    Ok(output)
}

fn build_decade_csv_rows(root: &Path) -> Result<Vec<BTreeMap<String, String>>, String> {
    let rows = read_jsonl(root.join(DECADE_JSONL_PATH))?;
    let mut grouped: BTreeMap<String, BTreeMap<String, serde_json::Value>> = BTreeMap::new();
    for row in rows {
        let decade = string_field(&row, "decade")?;
        let category = string_field(&row, "category_key")?;
        grouped.entry(decade).or_default().insert(category, row);
    }

    let mut output = Vec::new();
    for (decade, categories) in grouped {
        let anchor = categories
            .get("national-defense")
            .ok_or_else(|| format!("{decade}: missing national-defense row"))?;
        let mut row = BTreeMap::new();
        row.insert("decade".to_string(), decade);
        row.insert(
            "start_fiscal_year".to_string(),
            int_field(anchor, "start_fiscal_year")?.to_string(),
        );
        row.insert(
            "end_fiscal_year".to_string(),
            int_field(anchor, "end_fiscal_year")?.to_string(),
        );
        row.insert(
            "year_count".to_string(),
            int_field(anchor, "year_count")?.to_string(),
        );
        row.insert(
            "coverage_note".to_string(),
            string_field(anchor, "coverage_note")?,
        );
        insert_json_number(
            &mut row,
            "cumulative_individual_income_tax_receipts_millions",
            anchor,
            "cumulative_individual_income_tax_receipts_amount",
        );
        insert_json_number(
            &mut row,
            "cumulative_total_outlays_millions",
            anchor,
            "cumulative_total_outlays_amount",
        );
        insert_json_number(
            &mut row,
            "cumulative_total_receipts_millions",
            anchor,
            "cumulative_total_receipts_amount",
        );
        insert_json_number(
            &mut row,
            "cumulative_deficit_gap_millions",
            anchor,
            "cumulative_deficit_gap_amount",
        );
        insert_number(
            &mut row,
            "borrowed_share_percent_of_outlays",
            number_field(anchor, "borrowed_share_percent_of_outlays")?,
        );
        insert_number(
            &mut row,
            "income_tax_coverage_percent_of_outlays",
            number_field(anchor, "income_tax_coverage_percent_of_outlays")?,
        );
        row.insert(
            "allocation_method".to_string(),
            string_field(anchor, "allocation_method")?,
        );
        row.insert(
            "legal_allocation_status".to_string(),
            string_field(anchor, "legal_allocation_status")?,
        );
        row.insert(
            "actual_or_projection".to_string(),
            string_field(anchor, "actual_or_projection")?,
        );

        let mut percent_sum = 0.0;
        for (category_key, field_name) in CATEGORY_FIELDS {
            let category = categories
                .get(*category_key)
                .ok_or_else(|| format!("missing {category_key} row"))?;
            let percent = number_field(category, "category_percent_of_decade_income_tax")?;
            insert_number(&mut row, field_name, percent);
            percent_sum += percent;
        }
        insert_number(&mut row, "category_percent_sum", round6(percent_sum));
        output.push(row);
    }
    Ok(output)
}

fn build_subfunction_annual_csv_rows(root: &Path) -> Result<Vec<BTreeMap<String, String>>, String> {
    let rows = read_jsonl(root.join(SUBFUNCTION_MODEL_JSONL_PATH))?;
    rows.iter().map(subfunction_annual_csv_row).collect()
}

fn build_subfunction_decade_csv_rows(root: &Path) -> Result<Vec<BTreeMap<String, String>>, String> {
    let rows = read_jsonl(root.join(SUBFUNCTION_MODEL_JSONL_PATH))?;
    let mut grouped: BTreeMap<String, BTreeMap<(String, String), SubfunctionDecadeRollup>> =
        BTreeMap::new();
    let mut decade_receipts: BTreeMap<String, BTreeMap<i64, f64>> = BTreeMap::new();

    for row in &rows {
        let year = int_field(row, "fiscal_year")?;
        let decade = decade_label(year);
        let income_tax = number_field(row, "individual_income_tax_receipts_amount")?;
        decade_receipts
            .entry(decade.clone())
            .or_default()
            .entry(year)
            .or_insert(income_tax);

        let function_code = string_field(row, "function_code")?;
        let subfunction_code = string_field(row, "subfunction_code")?;
        let rollup = grouped
            .entry(decade)
            .or_default()
            .entry((function_code, subfunction_code))
            .or_insert_with(|| SubfunctionDecadeRollup {
                function_code: string_field(row, "function_code").unwrap_or_default(),
                function_label: string_field(row, "function_label").unwrap_or_default(),
                subfunction_code: string_field(row, "subfunction_code").unwrap_or_default(),
                subfunction_label: string_field(row, "subfunction_label").unwrap_or_default(),
                subfunction_outlays: 0.0,
                modeled_allocation: 0.0,
            });
        rollup.subfunction_outlays += number_field(row, "subfunction_outlays_amount")?;
        rollup.modeled_allocation += number_field(row, "modeled_income_tax_allocation_amount")?;
    }

    let mut output = Vec::new();
    for (decade, mut subfunctions) in grouped {
        let receipts_by_year = decade_receipts
            .get(&decade)
            .ok_or_else(|| format!("{decade}: missing receipt denominator"))?;
        let start_year = *receipts_by_year
            .keys()
            .next()
            .ok_or_else(|| format!("{decade}: no years"))?;
        let end_year = *receipts_by_year
            .keys()
            .next_back()
            .ok_or_else(|| format!("{decade}: no years"))?;
        let year_count = receipts_by_year.len();
        let income_tax: f64 = receipts_by_year.values().sum();
        let coverage_note = if year_count == 10 {
            "full_decade"
        } else {
            "partial_decade"
        };

        let mut rows: Vec<SubfunctionDecadeRollup> =
            subfunctions.values_mut().map(|row| row.clone()).collect();
        rows.sort_by(|left, right| {
            right
                .modeled_allocation
                .partial_cmp(&left.modeled_allocation)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| left.subfunction_label.cmp(&right.subfunction_label))
        });

        for row in rows {
            let mut output_row = BTreeMap::new();
            output_row.insert("decade".to_string(), decade.clone());
            output_row.insert("start_fiscal_year".to_string(), start_year.to_string());
            output_row.insert("end_fiscal_year".to_string(), end_year.to_string());
            output_row.insert("year_count".to_string(), year_count.to_string());
            output_row.insert("coverage_note".to_string(), coverage_note.to_string());
            output_row.insert("function_code".to_string(), row.function_code);
            output_row.insert("function_label".to_string(), row.function_label);
            output_row.insert("subfunction_code".to_string(), row.subfunction_code);
            output_row.insert("subfunction_label".to_string(), row.subfunction_label);
            insert_rounded_number(
                &mut output_row,
                "cumulative_individual_income_tax_receipts_millions",
                income_tax,
                6,
            );
            insert_rounded_number(
                &mut output_row,
                "cumulative_subfunction_outlays_millions",
                row.subfunction_outlays,
                6,
            );
            insert_rounded_number(
                &mut output_row,
                "cumulative_modeled_income_tax_allocation_millions",
                row.modeled_allocation,
                6,
            );
            insert_number(
                &mut output_row,
                "decade_allocation_share_percent",
                round9(row.modeled_allocation / income_tax * 100.0),
            );
            output_row.insert(
                "allocation_method".to_string(),
                "proportional_outlay_share".to_string(),
            );
            output_row.insert(
                "legal_allocation_status".to_string(),
                "modeled_not_legal_dedication".to_string(),
            );
            output_row.insert("actual_or_projection".to_string(), "actual".to_string());
            output.push(output_row);
        }
    }
    Ok(output)
}

fn build_subfunction_fy2025_top_csv_rows(
    root: &Path,
    count: usize,
) -> Result<Vec<BTreeMap<String, String>>, String> {
    let mut rows: Vec<serde_json::Value> = read_jsonl(root.join(SUBFUNCTION_MODEL_JSONL_PATH))?
        .into_iter()
        .filter(|row| int_field(row, "fiscal_year") == Ok(2025))
        .collect();
    rows.sort_by(|left, right| {
        number_field(right, "modeled_income_tax_allocation_amount")
            .unwrap_or(0.0)
            .partial_cmp(&number_field(left, "modeled_income_tax_allocation_amount").unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    rows.iter()
        .take(count)
        .enumerate()
        .map(|(index, row)| subfunction_top_csv_row(index + 1, row))
        .collect()
}

#[derive(Clone)]
struct SubfunctionDecadeRollup {
    function_code: String,
    function_label: String,
    subfunction_code: String,
    subfunction_label: String,
    subfunction_outlays: f64,
    modeled_allocation: f64,
}

fn subfunction_annual_csv_row(row: &serde_json::Value) -> Result<BTreeMap<String, String>, String> {
    let mut output = BTreeMap::new();
    output.insert(
        "fiscal_year".to_string(),
        int_field(row, "fiscal_year")?.to_string(),
    );
    output.insert(
        "function_code".to_string(),
        string_field(row, "function_code")?,
    );
    output.insert(
        "function_label".to_string(),
        string_field(row, "function_label")?,
    );
    output.insert(
        "subfunction_code".to_string(),
        string_field(row, "subfunction_code")?,
    );
    output.insert(
        "subfunction_label".to_string(),
        string_field(row, "subfunction_label")?,
    );
    insert_json_number(
        &mut output,
        "individual_income_tax_receipts_millions",
        row,
        "individual_income_tax_receipts_amount",
    );
    insert_json_number(
        &mut output,
        "total_outlays_millions",
        row,
        "total_outlays_amount",
    );
    insert_json_number(
        &mut output,
        "subfunction_outlays_millions",
        row,
        "subfunction_outlays_amount",
    );
    insert_rounded_number(
        &mut output,
        "modeled_income_tax_allocation_millions",
        number_field(row, "modeled_income_tax_allocation_amount")?,
        6,
    );
    insert_number(
        &mut output,
        "allocation_share_percent",
        number_field(row, "allocation_share_percent")?,
    );
    insert_number(
        &mut output,
        "outlay_share_percent",
        number_field(row, "outlay_share_percent")?,
    );
    output.insert(
        "allocation_method".to_string(),
        string_field(row, "allocation_method")?,
    );
    output.insert(
        "legal_allocation_status".to_string(),
        string_field(row, "legal_allocation_status")?,
    );
    output.insert(
        "actual_or_projection".to_string(),
        string_field(row, "actual_or_projection")?,
    );
    Ok(output)
}

fn subfunction_top_csv_row(
    rank: usize,
    row: &serde_json::Value,
) -> Result<BTreeMap<String, String>, String> {
    let mut output = BTreeMap::new();
    output.insert("rank".to_string(), rank.to_string());
    output.insert(
        "fiscal_year".to_string(),
        int_field(row, "fiscal_year")?.to_string(),
    );
    output.insert(
        "function_code".to_string(),
        string_field(row, "function_code")?,
    );
    output.insert(
        "function_label".to_string(),
        string_field(row, "function_label")?,
    );
    output.insert(
        "subfunction_code".to_string(),
        string_field(row, "subfunction_code")?,
    );
    output.insert(
        "subfunction_label".to_string(),
        string_field(row, "subfunction_label")?,
    );
    insert_rounded_number(
        &mut output,
        "modeled_income_tax_allocation_millions",
        number_field(row, "modeled_income_tax_allocation_amount")?,
        6,
    );
    insert_number(
        &mut output,
        "allocation_share_percent",
        number_field(row, "allocation_share_percent")?,
    );
    insert_json_number(
        &mut output,
        "subfunction_outlays_millions",
        row,
        "subfunction_outlays_amount",
    );
    output.insert(
        "allocation_method".to_string(),
        string_field(row, "allocation_method")?,
    );
    output.insert(
        "legal_allocation_status".to_string(),
        string_field(row, "legal_allocation_status")?,
    );
    Ok(output)
}

fn validate_csv_rows(
    rows: &[BTreeMap<String, String>],
    label: &str,
    expected_count: usize,
) -> Result<(), String> {
    if rows.len() != expected_count {
        return Err(format!(
            "{label}: expected {expected_count} rows, found {}",
            rows.len()
        ));
    }
    for row in rows {
        let percent_sum = row
            .get("category_percent_sum")
            .ok_or_else(|| format!("{label}: missing category_percent_sum"))?
            .parse::<f64>()
            .map_err(|err| format!("{label}: invalid category_percent_sum: {err}"))?;
        if (percent_sum - 100.0).abs() > 0.00001 {
            return Err(format!("{label}: percent sum {percent_sum} for {row:?}"));
        }
        if row.get("legal_allocation_status").map(String::as_str)
            != Some("modeled_not_legal_dedication")
        {
            return Err(format!("{label}: missing modeled legal status for {row:?}"));
        }
        if row.get("actual_or_projection").map(String::as_str) != Some("actual") {
            return Err(format!("{label}: unexpected projection status for {row:?}"));
        }
    }
    Ok(())
}

fn validate_subfunction_csv_rows(
    rows: &[BTreeMap<String, String>],
    label: &str,
    expected_count: usize,
) -> Result<(), String> {
    if rows.len() != expected_count {
        return Err(format!(
            "{label}: expected {expected_count} rows, found {}",
            rows.len()
        ));
    }
    for row in rows {
        if row.get("legal_allocation_status").map(String::as_str)
            != Some("modeled_not_legal_dedication")
        {
            return Err(format!("{label}: missing modeled legal status for {row:?}"));
        }
        if row.get("allocation_method").map(String::as_str) != Some("proportional_outlay_share") {
            return Err(format!("{label}: unexpected allocation method for {row:?}"));
        }
    }
    Ok(())
}

fn validate_subfunction_decade_csv_rows(rows: &[BTreeMap<String, String>]) -> Result<(), String> {
    if rows.is_empty() {
        return Err("subfunction decade: no rows".to_string());
    }
    let mut percent_sums: BTreeMap<String, f64> = BTreeMap::new();
    for row in rows {
        if row.get("legal_allocation_status").map(String::as_str)
            != Some("modeled_not_legal_dedication")
        {
            return Err(format!(
                "subfunction decade: missing modeled legal status for {row:?}"
            ));
        }
        if row.get("allocation_method").map(String::as_str) != Some("proportional_outlay_share") {
            return Err(format!(
                "subfunction decade: unexpected allocation method for {row:?}"
            ));
        }
        let decade = row
            .get("decade")
            .ok_or_else(|| "subfunction decade: missing decade".to_string())?;
        let percent = row
            .get("decade_allocation_share_percent")
            .ok_or_else(|| "subfunction decade: missing percent".to_string())?
            .parse::<f64>()
            .map_err(|err| format!("subfunction decade: invalid percent: {err}"))?;
        *percent_sums.entry(decade.to_string()).or_default() += percent;
    }
    for (decade, percent_sum) in percent_sums {
        if (percent_sum - 100.0).abs() > 0.0001 {
            return Err(format!(
                "subfunction decade: {decade} percent sum {percent_sum}"
            ));
        }
    }
    Ok(())
}

fn write_csv(
    root: &Path,
    relative_path: &str,
    headers: &[&str],
    rows: &[BTreeMap<String, String>],
) -> Result<(), String> {
    let text = csv_text(headers, rows)?;
    fs::write(root.join(relative_path), text)
        .map_err(|err| format!("failed to write {relative_path}: {err}"))
}

fn compare_csv(
    root: &Path,
    relative_path: &str,
    headers: &[&str],
    rows: &[BTreeMap<String, String>],
) -> Result<(), String> {
    let expected = normalize_newlines(&csv_text(headers, rows)?);
    let current = fs::read_to_string(root.join(relative_path))
        .map_err(|err| format!("failed to read {relative_path}: {err}"))?;
    if normalize_newlines(&current) != expected {
        return Err(format!(
            "stale CSV export: run `cargo run -p taxlane-tools -- income-tax-outlay export`"
        ));
    }
    Ok(())
}

fn compare_text(
    root: &Path,
    relative_path: &str,
    expected: &str,
    label: &str,
) -> Result<(), String> {
    let current = fs::read_to_string(root.join(relative_path))
        .map_err(|err| format!("failed to read {relative_path}: {err}"))?;
    if normalize_newlines(&current) != normalize_newlines(expected) {
        return Err(format!("stale {label}"));
    }
    Ok(())
}

fn csv_text(headers: &[&str], rows: &[BTreeMap<String, String>]) -> Result<String, String> {
    if rows.is_empty() {
        return Err("no CSV rows".to_string());
    }
    let mut writer = csv::Writer::from_writer(Vec::new());
    writer
        .write_record(headers.iter().copied())
        .map_err(|err| format!("failed to write CSV header: {err}"))?;
    for row in rows {
        let values: Vec<&str> = headers
            .iter()
            .map(|header| row.get(*header).map(String::as_str).unwrap_or(""))
            .collect();
        writer
            .write_record(values)
            .map_err(|err| format!("failed to write CSV row: {err}"))?;
    }
    let bytes = writer
        .into_inner()
        .map_err(|err| format!("failed to finish CSV: {err}"))?;
    String::from_utf8(bytes).map_err(|err| format!("invalid UTF-8 CSV: {err}"))
}

fn read_jsonl(path: PathBuf) -> Result<Vec<serde_json::Value>, String> {
    let content =
        fs::read_to_string(&path).map_err(|err| format!("failed to read {:?}: {err}", path))?;
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<serde_json::Value>(line)
                .map_err(|err| format!("failed to parse JSONL {:?}: {err}", path))
        })
        .collect()
}

const PROGRAM_LANE_RATE_MODEL_DIR: &str = "data/derived/program_lane_rate_model";

/// Validate the program-lane reform record families: every record must keep the
/// `proposed_reform` allocation gate, carry id/family/status, cite only
/// ledger-backed sources, and (for the share models) reconcile to 100%.
fn validate_program_lane_records(root: &Path) -> Result<(), String> {
    let source_ledger = fs::read_to_string(root.join(SOURCE_VERSION_LEDGER_PATH))
        .map_err(|err| format!("failed to read {SOURCE_VERSION_LEDGER_PATH}: {err}"))?;
    let dir = root.join(PROGRAM_LANE_RATE_MODEL_DIR);
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .map_err(|err| format!("failed to read {:?}: {err}", dir))?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().map(|ext| ext == "jsonl").unwrap_or(false))
        .collect();
    files.sort();
    if files.is_empty() {
        return Err("program-lane: no JSONL records found".to_string());
    }

    let mut total_records = 0usize;
    for path in files {
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string();
        let records = read_jsonl(path.clone())?;
        if records.is_empty() {
            return Err(format!("program-lane: {file_name} has no records"));
        }

        let share_field: Option<&str> = if file_name.starts_with("program_lane_rate_model.") {
            Some("recommended_receipt_share_percent")
        } else if file_name.starts_with("income_tax_budget_allocation.") {
            Some("pct_of_income_tax_budget")
        } else {
            None
        };
        let mut share_sum = 0f64;

        for record in &records {
            let id = record
                .get("record_id")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            if id.is_empty() {
                return Err(format!(
                    "program-lane: {file_name} record missing record_id"
                ));
            }
            if record
                .get("record_family")
                .and_then(|value| value.as_str())
                .unwrap_or("")
                .is_empty()
            {
                return Err(format!("program-lane: {id} missing record_family"));
            }
            let method = record
                .get("allocation_method")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            if !method.contains("proposed_reform") {
                return Err(format!(
                    "program-lane: {id} allocation_method must contain proposed_reform (got {method:?})"
                ));
            }
            if record
                .get("status")
                .and_then(|value| value.as_str())
                .unwrap_or("")
                .is_empty()
            {
                return Err(format!("program-lane: {id} missing status"));
            }
            if let Some(source_ids) = record.get("source_ids").and_then(|value| value.as_array()) {
                for source_id in source_ids {
                    if let Some(source_id) = source_id.as_str() {
                        if !source_ledger.contains(&format!("`{source_id}`")) {
                            return Err(format!(
                                "program-lane: {id} source_id {source_id} is missing from {SOURCE_VERSION_LEDGER_PATH}"
                            ));
                        }
                    }
                }
            }
            if let Some(field) = share_field {
                if let Some(value) = record.get(field).and_then(|value| value.as_f64()) {
                    share_sum += value;
                }
            }
        }

        if let Some(field) = share_field {
            if (share_sum - 100.0).abs() > 0.1 {
                return Err(format!(
                    "program-lane: {file_name} {field} sums to {share_sum:.4}, expected 100"
                ));
            }
        }
        total_records += records.len();
    }

    println!("validated {total_records} program-lane records");
    Ok(())
}

fn validate_accountability_evidence_records(root: &Path) -> Result<(), String> {
    let source_ledger = fs::read_to_string(root.join(SOURCE_VERSION_LEDGER_PATH))
        .map_err(|err| format!("failed to read {SOURCE_VERSION_LEDGER_PATH}: {err}"))?;
    let records = read_accountability_evidence_records(root)?;
    if records.is_empty() {
        return Err("accountability evidence: no records".to_string());
    }

    for record in records {
        record
            .validate()
            .map_err(|err| format!("{}: {err}", record.record_id))?;
        for source_id in &record.source_ids {
            let ledger_token = format!("`{source_id}`");
            if !source_ledger.contains(&ledger_token) {
                return Err(format!(
                    "{}: source_id {source_id} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                    record.record_id
                ));
            }
        }
    }

    println!("validated accountability evidence records");
    Ok(())
}

fn validate_spend_category_map(root: &Path) -> Result<(), String> {
    let rows: Vec<SpendCategoryMapRecord> = read_jsonl(root.join(SPEND_CATEGORY_MAP_JSONL_PATH))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row).map_err(|err| {
                format!("{SPEND_CATEGORY_MAP_JSONL_PATH} row failed to parse: {err}")
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    if rows.len() != 15 {
        return Err(format!(
            "spend category map must contain 15 top FY2025 rows, got {}",
            rows.len()
        ));
    }

    for (index, row) in rows.iter().enumerate() {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        let expected_rank = (index + 1) as u16;
        if row.rank != expected_rank {
            return Err(format!(
                "spend category rows must be sorted by rank; expected {expected_rank}, got {}",
                row.rank
            ));
        }
        if row.record_id != format!("spendcat-fy2025-{expected_rank:03}") {
            return Err(format!(
                "spend category row {} has unexpected record_id {}",
                row.rank, row.record_id
            ));
        }
    }

    let total_share: f64 = rows
        .iter()
        .map(|row| row.share_of_total_outlays_percent)
        .sum();
    if total_share < 90.0 {
        return Err(format!(
            "top spend category rows should cover most FY2025 outlays; got {total_share:.2}%"
        ));
    }

    let index = fs::read_to_string(root.join("data/derived/README.md"))
        .map_err(|err| format!("failed to read data/derived/README.md: {err}"))?;
    if !index.contains("spend_category_map/") {
        return Err("data/derived/README.md must link spend_category_map/".to_string());
    }

    let reader = fs::read_to_string(root.join("docs/reading/where-federal-money-goes.md"))
        .map_err(|err| format!("failed to read docs/reading/where-federal-money-goes.md: {err}"))?;
    if !reader.contains(SPEND_CATEGORY_MAP_JSONL_PATH) {
        return Err("where-federal-money-goes.md must cite the spend category JSONL".to_string());
    }

    for path in [
        SPEND_CATEGORY_MAP_README_PATH,
        SPEND_CATEGORY_MAP_SCHEMA_PATH,
        SPEND_CATEGORY_MAP_HANDOFF_PATH,
        SPEND_CATEGORY_MAP_DASHBOARD_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing spend category support artifact: {path}"));
        }
    }

    let expected_dashboard = build_spend_category_dashboard(&rows)?;
    compare_text(
        root,
        SPEND_CATEGORY_MAP_DASHBOARD_PATH,
        &expected_dashboard,
        "spend category dashboard",
    )?;

    println!("validated {} spend category map rows", rows.len());
    Ok(())
}

fn validate_breadth_benchmark_matrix(root: &Path) -> Result<(), String> {
    let source_ledger = fs::read_to_string(root.join(SOURCE_VERSION_LEDGER_PATH))
        .map_err(|err| format!("failed to read {SOURCE_VERSION_LEDGER_PATH}: {err}"))?;
    let rows: Vec<BreadthBenchmarkRecord> = read_jsonl(root.join(BREADTH_BENCHMARK_JSONL_PATH))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row)
                .map_err(|err| format!("{BREADTH_BENCHMARK_JSONL_PATH} row failed to parse: {err}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    if rows.len() != 17 {
        return Err(format!(
            "breadth benchmark matrix must contain 17 initial rows, got {}",
            rows.len()
        ));
    }

    let mut ids = BTreeSet::new();
    let mut tiers = BTreeMap::new();
    let mut statuses = BTreeMap::new();
    let mut improper_payment_rows = 0usize;
    for row in &rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !ids.insert(row.record_id.clone()) {
            return Err(format!("duplicate breadth benchmark row {}", row.record_id));
        }
        *tiers.entry(row.depth_tier.as_str()).or_insert(0usize) += 1;
        *statuses
            .entry(row.coverage_status.as_str())
            .or_insert(0usize) += 1;
        if row.improper_payment_amount_millions.is_some() {
            improper_payment_rows += 1;
        }
        for source_id in &row.source_ids {
            if !source_ledger.contains(&format!("`{source_id}`")) {
                return Err(format!(
                    "{}: source_id {source_id} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                    row.record_id
                ));
            }
        }
    }

    for required in ["tier_1_full", "tier_2_card"] {
        if !tiers.contains_key(required) {
            return Err(format!("breadth benchmark matrix needs {required} rows"));
        }
    }
    for required in ["full_comparison", "topline_only"] {
        if !statuses.contains_key(required) {
            return Err(format!(
                "breadth benchmark matrix needs {required} coverage rows"
            ));
        }
    }
    if improper_payment_rows != 1 {
        return Err(format!(
            "breadth benchmark matrix must contain one scoped improper-payment topline, got {improper_payment_rows}"
        ));
    }

    for path in [
        BREADTH_BENCHMARK_README_PATH,
        BREADTH_BENCHMARK_SCHEMA_PATH,
        BREADTH_BENCHMARK_SCOREBOARD_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing breadth benchmark artifact: {path}"));
        }
    }
    let scoreboard = fs::read_to_string(root.join(BREADTH_BENCHMARK_SCOREBOARD_PATH))
        .map_err(|err| format!("failed to read {BREADTH_BENCHMARK_SCOREBOARD_PATH}: {err}"))?;
    if !scoreboard.contains(BREADTH_BENCHMARK_JSONL_PATH) {
        return Err(format!(
            "{BREADTH_BENCHMARK_SCOREBOARD_PATH} must cite {BREADTH_BENCHMARK_JSONL_PATH}"
        ));
    }
    for required_boundary in [
        "efficiency gap != improper payments",
        "!= fraud != recoverable savings",
    ] {
        if !scoreboard.contains(required_boundary) {
            return Err(format!(
                "{BREADTH_BENCHMARK_SCOREBOARD_PATH} must preserve boundary: {required_boundary}"
            ));
        }
    }

    let veterans_text = fs::read_to_string(root.join(VETERANS_DEPTH_CARD_JSON_PATH))
        .map_err(|err| format!("failed to read {VETERANS_DEPTH_CARD_JSON_PATH}: {err}"))?;
    let veterans_card: serde_json::Value = serde_json::from_str(&veterans_text)
        .map_err(|err| format!("failed to parse {VETERANS_DEPTH_CARD_JSON_PATH}: {err}"))?;
    let veterans_total = number_field(&veterans_card, "total_outlays_millions")?;
    let components = veterans_card
        .get("components")
        .and_then(|value| value.as_array())
        .ok_or_else(|| "veterans depth card needs components".to_string())?;
    if components.len() != 5 {
        return Err(format!(
            "veterans depth card must contain five subfunctions, got {}",
            components.len()
        ));
    }
    let component_total: f64 = components
        .iter()
        .map(|component| number_field(component, "outlays_millions"))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum();
    if (component_total - veterans_total).abs() > 0.001 || veterans_total != 377_163.0 {
        return Err(format!(
            "veterans depth card components do not reconcile: {component_total} vs {veterans_total}"
        ));
    }
    for blocked_field in ["fraud_status", "savings_status"] {
        let value = string_field(&veterans_card, blocked_field)?;
        if !value.contains("not_") && !value.contains("blocked") {
            return Err(format!("veterans depth card must block {blocked_field}"));
        }
    }
    let veterans_reader = fs::read_to_string(root.join(VETERANS_DEPTH_CARD_READER_PATH))
        .map_err(|err| format!("failed to read {VETERANS_DEPTH_CARD_READER_PATH}: {err}"))?;
    if !veterans_reader.contains(VETERANS_DEPTH_CARD_JSON_PATH) {
        return Err("veterans depth reader must cite its machine record".to_string());
    }
    let transportation_text = fs::read_to_string(root.join(TRANSPORTATION_DEPTH_CARD_JSON_PATH))
        .map_err(|err| format!("failed to read {TRANSPORTATION_DEPTH_CARD_JSON_PATH}: {err}"))?;
    let transportation: serde_json::Value = serde_json::from_str(&transportation_text)
        .map_err(|err| format!("failed to parse {TRANSPORTATION_DEPTH_CARD_JSON_PATH}: {err}"))?;
    let transportation_total = number_field(&transportation, "total_outlays_millions")?;
    let transportation_components = transportation
        .get("components")
        .and_then(|value| value.as_array())
        .ok_or_else(|| "transportation depth card needs components".to_string())?;
    let transportation_sum: f64 = transportation_components
        .iter()
        .map(|component| number_field(component, "outlays_millions"))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum();
    if transportation_components.len() != 4
        || (transportation_sum - transportation_total).abs() > 0.001
        || transportation_total != 145_320.0
    {
        return Err("transportation depth card does not reconcile to function 400".to_string());
    }
    let transportation_reader =
        fs::read_to_string(root.join(TRANSPORTATION_DEPTH_CARD_READER_PATH)).map_err(|err| {
            format!("failed to read {TRANSPORTATION_DEPTH_CARD_READER_PATH}: {err}")
        })?;
    if !transportation_reader.contains(TRANSPORTATION_DEPTH_CARD_JSON_PATH)
        || !transportation_reader.contains("not an under-spending, waste, fraud, or savings")
    {
        return Err(
            "transportation depth reader must cite the card and preserve claim boundaries"
                .to_string(),
        );
    }

    validate_education_depth_card(root)?;
    validate_disaster_depth_card(root)?;
    validate_justice_depth_card(root)?;
    validate_science_depth_card(root)?;
    validate_agriculture_depth_card(root)?;
    validate_international_depth_card(root)?;
    validate_health_cost_decomposition(root)?;
    validate_health_service_bridge(root)?;
    validate_health_category_benchmark_ladder(root)?;
    validate_health_target_admissibility(root)?;
    validate_health_scenarios(root)?;
    validate_health_sample_sensitivity(root)?;
    println!(
        "validated {} breadth benchmark rows across full comparisons and toplines with no open coverage gaps",
        rows.len()
    );
    Ok(())
}

fn validate_health_sample_sensitivity(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(HEALTH_SAMPLE_SENSITIVITY_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let categories = card
        .get("categories")
        .and_then(|v| v.as_array())
        .ok_or("health sample sensitivity categories")?;
    if categories.len() != 2 {
        return Err("health sample sensitivity needs two categories".to_string());
    }
    for category in categories {
        let commercial = number_field(category, "commercial_allowed_usd_billions")?;
        let medicare = number_field(category, "simulated_medicare_allowed_usd_billions")?;
        let scenarios = category
            .get("scenarios")
            .and_then(|v| v.as_array())
            .ok_or("health sample sensitivity scenarios")?;
        if scenarios.len() != 3 {
            return Err("health sample sensitivity needs three scenarios per category".to_string());
        }
        for scenario in scenarios {
            let target = number_field(scenario, "target_percent_medicare")? / 100.0;
            let change = number_field(scenario, "mechanical_sample_payment_change_usd_billions")?;
            let expected = medicare * target - commercial;
            if (change - expected).abs() > 0.001 {
                return Err("health sample dollar sensitivity does not reconcile".to_string());
            }
        }
    }
    let prohibited = card
        .get("prohibited_uses")
        .and_then(|v| v.as_array())
        .ok_or("health sample prohibited uses")?;
    if prohibited.len() < 5 || !string_field(&card, "net_savings_status")?.contains("blocked") {
        return Err("health sample sensitivity must block national and net claims".to_string());
    }
    let reader = fs::read_to_string(root.join(HEALTH_SAMPLE_SENSITIVITY_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_SAMPLE_SENSITIVITY_JSON_PATH,
        "sample sensitivity != national gross savings != net savings",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "health sample sensitivity reader missing {required}"
            ));
        }
    }
    Ok(())
}

fn validate_health_scenarios(root: &Path) -> Result<(), String> {
    let text =
        fs::read_to_string(root.join(HEALTH_SCENARIOS_JSON_PATH)).map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let categories = card
        .get("categories")
        .and_then(|v| v.as_array())
        .ok_or("health scenario categories")?;
    if categories.len() != 2 {
        return Err("health scenarios need hospital and professional categories".to_string());
    }
    for category in categories {
        let current = number_field(category, "current_reference_percent_medicare")?;
        let scenarios = category
            .get("scenarios")
            .and_then(|v| v.as_array())
            .ok_or("health category scenarios")?;
        if scenarios.len() != 3
            || !string_field(category, "dollar_effect_status")?.contains("blocked")
        {
            return Err(
                "health category must have three scenarios with dollars blocked".to_string(),
            );
        }
        for scenario in scenarios {
            let target = number_field(scenario, "target_percent_medicare")?;
            let change = number_field(scenario, "gross_rate_change_percent")?;
            let expected = (target / current - 1.0) * 100.0;
            if (change - expected).abs() > 0.001 {
                return Err("health scenario rate change does not reconcile".to_string());
            }
        }
    }
    let gates = card
        .get("shared_gates")
        .and_then(|v| v.as_array())
        .ok_or("health scenario shared gates")?;
    if gates.len() < 6
        || !string_field(&card, "gross_dollar_effect_status")?.contains("blocked")
        || !string_field(&card, "net_savings_status")?.contains("blocked")
    {
        return Err("health scenarios must preserve model gates and savings blocks".to_string());
    }
    let reader =
        fs::read_to_string(root.join(HEALTH_SCENARIOS_READER_PATH)).map_err(|e| e.to_string())?;
    for required in [
        HEALTH_SCENARIOS_JSON_PATH,
        "illustrative rate path != spending reduction != federal savings",
    ] {
        if !reader.contains(required) {
            return Err(format!("health scenario reader missing {required}"));
        }
    }
    Ok(())
}

fn validate_health_target_admissibility(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(HEALTH_TARGET_ADMISSIBILITY_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let anchors = card
        .get("anchors")
        .and_then(|v| v.as_array())
        .ok_or("health target admissibility anchors")?;
    if anchors.len() != 2 {
        return Err(
            "health target admissibility needs hospital and professional anchors".to_string(),
        );
    }
    for anchor in anchors {
        if string_field(anchor, "anchor_status")? != "conditional_scenario_anchor"
            || !string_field(anchor, "target_status")?.contains("blocked")
        {
            return Err(
                "health target anchor must be conditional with universal target blocked"
                    .to_string(),
            );
        }
    }
    let hospital = anchors
        .iter()
        .find(|v| string_field(v, "category").ok().as_deref() == Some("hospital care"))
        .ok_or("hospital admissibility anchor")?;
    if number_field(hospital, "aggregate_ffs_medicare_margin_percent")? != -12.1
        || number_field(hospital, "efficient_hospital_median_ffs_margin_percent")? != -1.0
        || number_field(hospital, "efficient_hospital_projected_2026_margin_percent")? != 1.0
    {
        return Err(
            "hospital adequacy margins must preserve aggregate and efficient-provider distinction"
                .to_string(),
        );
    }
    let rules = card
        .get("admissibility_rules")
        .and_then(|v| v.as_array())
        .ok_or("health target admissibility rules")?;
    if rules.len() < 6 || !string_field(&card, "savings_status")?.contains("blocked") {
        return Err(
            "health target admissibility must preserve floors and savings block".to_string(),
        );
    }
    let reader = fs::read_to_string(root.join(HEALTH_TARGET_ADMISSIBILITY_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_TARGET_ADMISSIBILITY_JSON_PATH,
        "credible anchor != universal target != gross reduction != net savings",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "health target admissibility reader missing {required}"
            ));
        }
    }
    Ok(())
}

fn validate_health_category_benchmark_ladder(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(HEALTH_CATEGORY_BENCHMARK_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let rows = card
        .get("categories")
        .and_then(|v| v.as_array())
        .ok_or("health category benchmark rows")?;
    if rows.len() != 3 {
        return Err("health category benchmark ladder must contain three rows".to_string());
    }
    let hospital = rows
        .iter()
        .find(|v| string_field(v, "category").ok().as_deref() == Some("hospital care"))
        .ok_or("hospital benchmark row")?;
    if number_field(hospital, "current_value")? != 253.0
        || !string_field(hospital, "scoring_status")?.contains("blocked")
    {
        return Err("hospital reference must be 253 percent with target blocked".to_string());
    }
    let professional = rows
        .iter()
        .find(|v| {
            string_field(v, "category").ok().as_deref() == Some("physician and clinical services")
        })
        .ok_or("professional benchmark row")?;
    if number_field(professional, "current_value")? != 139.0
        || number_field(professional, "state_average_low")? != 117.0
        || number_field(professional, "state_average_high")? != 243.0
        || string_field(professional, "comparison_grade")? != "B"
        || !string_field(professional, "scoring_status")?.contains("blocked")
    {
        return Err(
            "professional reference must preserve estimate, range, grade, and target block"
                .to_string(),
        );
    }
    let drugs = rows
        .iter()
        .find(|v| string_field(v, "category").ok().as_deref() == Some("retail prescription drugs"))
        .ok_or("drug benchmark row")?;
    let ratio = number_field(drugs, "current_value")? / number_field(drugs, "benchmark_value")?;
    if (ratio - number_field(drugs, "current_to_benchmark_ratio")?).abs() > 0.001
        || !string_field(drugs, "scoring_status")?.contains("blocked")
    {
        return Err(
            "drug spending comparison must reconcile with price target blocked".to_string(),
        );
    }
    let reader = fs::read_to_string(root.join(HEALTH_CATEGORY_BENCHMARK_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_CATEGORY_BENCHMARK_JSON_PATH,
        "reference price != expected price != addressable excess != savings",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "health category benchmark reader missing {required}"
            ));
        }
    }
    Ok(())
}

fn validate_health_service_bridge(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(HEALTH_SERVICE_BRIDGE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let categories = card
        .get("categories")
        .and_then(|v| v.as_array())
        .ok_or("health service bridge categories")?;
    if categories.len() != 3 {
        return Err("health service bridge must contain three categories".to_string());
    }
    let mut spending = 0.0;
    let mut share = 0.0;
    for category in categories {
        spending += number_field(category, "spending_usd_billions")?;
        share += number_field(category, "share_total_nhe_percent")?;
        let total = number_field(category, "expenditure_growth_percent")? / 100.0;
        let price = number_field(category, "price_growth_percent")? / 100.0;
        let residual = number_field(category, "implied_non_price_growth_percent")?;
        let expected = ((1.0 + total) / (1.0 + price) - 1.0) * 100.0;
        if (expected - residual).abs() > 0.001 {
            return Err("health service bridge residual does not reconcile".to_string());
        }
        if !string_field(category, "peer_price_benchmark_status")?.contains("blocked") {
            return Err("health service category peer benchmark must remain blocked".to_string());
        }
    }
    if (spending - number_field(&card, "covered_spending_usd_billions")?).abs() > 0.001
        || (share - number_field(&card, "covered_share_total_nhe_percent")?).abs() > 0.001
    {
        return Err("health service bridge totals do not reconcile".to_string());
    }
    let reader = fs::read_to_string(root.join(HEALTH_SERVICE_BRIDGE_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_SERVICE_BRIDGE_JSON_PATH,
        "growth decomposition != peer efficiency finding != fraud != savings",
    ] {
        if !reader.contains(required) {
            return Err(format!("health service bridge reader missing {required}"));
        }
    }
    Ok(())
}

fn validate_health_cost_decomposition(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(HEALTH_COST_DECOMPOSITION_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let headline = card
        .get("headline")
        .ok_or("health decomposition headline")?;
    let us = number_field(headline, "us_total_health_spending_percent_gdp")?;
    let peer = number_field(headline, "oecd_average_percent_gdp")?;
    let gap = number_field(headline, "observed_gap_percentage_points")?;
    if us != 17.2 || peer != 9.3 || (us - peer - gap).abs() > 0.0001 {
        return Err("health decomposition headline does not reconcile".to_string());
    }
    let signals = card
        .get("diagnostic_signals")
        .and_then(|v| v.as_array())
        .ok_or("health decomposition diagnostic signals")?;
    if signals.len() != 5 {
        return Err("health decomposition must contain five diagnostic signals".to_string());
    }
    for field in ["decomposition_status", "fraud_status", "savings_status"] {
        let value = string_field(&card, field)?;
        if !value.contains("not_") && !value.contains("blocked") && !value.contains("diagnostic") {
            return Err(format!(
                "health decomposition must preserve {field} boundary"
            ));
        }
    }
    let reader = fs::read_to_string(root.join(HEALTH_COST_DECOMPOSITION_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_COST_DECOMPOSITION_JSON_PATH,
        "observed spending gap != inefficiency != fraud != recoverable savings",
    ] {
        if !reader.contains(required) {
            return Err(format!("health decomposition reader missing {required}"));
        }
    }
    Ok(())
}

fn validate_education_depth_card(root: &Path) -> Result<(), String> {
    let text =
        fs::read_to_string(root.join(EDUCATION_DEPTH_CARD_JSON_PATH)).map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let total = number_field(&card, "total_outlays_millions")?;
    let parts = card
        .get("components")
        .and_then(|v| v.as_array())
        .ok_or("education components")?;
    let sum: f64 = parts
        .iter()
        .map(|v| number_field(v, "outlays_millions"))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    let higher = parts
        .iter()
        .find(|v| v.get("subfunction_code").and_then(|x| x.as_str()) == Some("502"))
        .ok_or("higher education component")?;
    if parts.len() != 6
        || total != 72_042.0
        || (sum - total).abs() > 0.001
        || number_field(higher, "outlays_millions")? != -35_005.0
        || higher.get("accounting_caveat").is_none()
    {
        return Err("education depth reconciliation failed".to_string());
    }
    let reader = fs::read_to_string(root.join(EDUCATION_DEPTH_CARD_READER_PATH))
        .map_err(|e| e.to_string())?;
    if !reader.contains(EDUCATION_DEPTH_CARD_JSON_PATH)
        || !reader.contains("does not mean government")
        || !reader.contains("negative education")
    {
        return Err("education depth caveat missing".to_string());
    }
    Ok(())
}

fn validate_disaster_depth_card(root: &Path) -> Result<(), String> {
    let text =
        fs::read_to_string(root.join(DISASTER_DEPTH_CARD_JSON_PATH)).map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    if number_field(&card, "outlays_millions")? != 62_768.0
        || string_field(&card, "scope_status")? != "subfunction_not_parent_function"
        || number_field(
            card.get("evidence_probes").ok_or("evidence probes")?,
            "declaration_rows",
        )? != 8.0
    {
        return Err("disaster depth card boundary failed".to_string());
    }
    let reader = fs::read_to_string(root.join(DISASTER_DEPTH_CARD_READER_PATH))
        .map_err(|e| e.to_string())?;
    if !reader.contains(DISASTER_DEPTH_CARD_JSON_PATH)
        || !reader.contains("Declarations are not spending")
        || !reader.contains("not realized savings")
    {
        return Err("disaster depth reader boundary failed".to_string());
    }
    Ok(())
}

fn validate_justice_depth_card(root: &Path) -> Result<(), String> {
    let text =
        fs::read_to_string(root.join(JUSTICE_DEPTH_CARD_JSON_PATH)).map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let parts = card
        .get("components")
        .and_then(|v| v.as_array())
        .ok_or("justice components")?;
    let sum: f64 = parts
        .iter()
        .map(|v| number_field(v, "outlays_millions"))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    if parts.len() != 4
        || sum != 83_146.0
        || number_field(&card, "total_outlays_millions")? != 83_146.0
    {
        return Err("justice depth reconciliation failed".to_string());
    }
    let reader =
        fs::read_to_string(root.join(JUSTICE_DEPTH_CARD_READER_PATH)).map_err(|e| e.to_string())?;
    if !reader.contains(JUSTICE_DEPTH_CARD_JSON_PATH)
        || !reader.contains("not the cost of the US justice system")
        || !reader.contains("due-process")
    {
        return Err("justice depth boundary failed".to_string());
    }
    Ok(())
}

fn validate_science_depth_card(root: &Path) -> Result<(), String> {
    let text =
        fs::read_to_string(root.join(SCIENCE_DEPTH_CARD_JSON_PATH)).map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let functions = card
        .get("functions")
        .and_then(|v| v.as_array())
        .ok_or("science functions")?;
    let sum: f64 = functions
        .iter()
        .map(|v| number_field(v, "outlays_millions"))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    if functions.len() != 3
        || sum != 152_565.0
        || string_field(&card, "composition_status")?
            != "orientation_subtotal_not_single_omb_function"
    {
        return Err("science-energy-environment boundary failed".to_string());
    }
    let reader =
        fs::read_to_string(root.join(SCIENCE_DEPTH_CARD_READER_PATH)).map_err(|e| e.to_string())?;
    if !reader.contains(SCIENCE_DEPTH_CARD_JSON_PATH) || !reader.contains("not an OMB function") {
        return Err("science depth reader boundary failed".to_string());
    }
    Ok(())
}

fn validate_agriculture_depth_card(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(AGRICULTURE_DEPTH_CARD_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let parts = card
        .get("components")
        .and_then(|v| v.as_array())
        .ok_or("agriculture components")?;
    let sum: f64 = parts
        .iter()
        .map(|v| number_field(v, "outlays_millions"))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    if parts.len() != 2
        || sum != 47_447.0
        || !string_field(&card, "scope_boundary")?.contains("not function 350")
    {
        return Err("agriculture depth boundary failed".to_string());
    }
    let reader = fs::read_to_string(root.join(AGRICULTURE_DEPTH_CARD_READER_PATH))
        .map_err(|e| e.to_string())?;
    if !reader.contains(AGRICULTURE_DEPTH_CARD_JSON_PATH) || !reader.contains("double count") {
        return Err("agriculture reader boundary failed".to_string());
    }
    Ok(())
}

fn validate_international_depth_card(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(INTERNATIONAL_DEPTH_CARD_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let parts = card
        .get("components")
        .and_then(|v| v.as_array())
        .ok_or("international components")?;
    let sum: f64 = parts
        .iter()
        .map(|v| number_field(v, "outlays_millions"))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    let financial = parts
        .iter()
        .find(|v| v.get("subfunction_code").and_then(|x| x.as_str()) == Some("155"))
        .ok_or("financial component")?;
    if parts.len() != 5
        || sum != 45_171.0
        || number_field(financial, "outlays_millions")? != -14_936.0
        || financial.get("accounting_caveat").is_none()
    {
        return Err("international depth boundary failed".to_string());
    }
    let reader = fs::read_to_string(root.join(INTERNATIONAL_DEPTH_CARD_READER_PATH))
        .map_err(|e| e.to_string())?;
    if !reader.contains(INTERNATIONAL_DEPTH_CARD_JSON_PATH)
        || !reader.contains("negative diplomacy")
    {
        return Err("international reader boundary failed".to_string());
    }
    Ok(())
}

fn validate_headline_basis_crosswalk(root: &Path) -> Result<(), String> {
    let source_ledger = fs::read_to_string(root.join(SOURCE_VERSION_LEDGER_PATH))
        .map_err(|err| format!("failed to read {SOURCE_VERSION_LEDGER_PATH}: {err}"))?;
    let rows: Vec<HeadlineBasisRecord> = read_jsonl(root.join(HEADLINE_BASIS_JSONL_PATH))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row)
                .map_err(|err| format!("{HEADLINE_BASIS_JSONL_PATH} row failed to parse: {err}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if rows.len() != 9 {
        return Err(format!(
            "headline basis crosswalk must contain 9 initial rows, got {}",
            rows.len()
        ));
    }
    let ids: BTreeSet<String> = rows.iter().map(|row| row.record_id.clone()).collect();
    if ids.len() != rows.len() {
        return Err("headline basis crosswalk contains duplicate record IDs".to_string());
    }
    let mut canonical_groups = BTreeSet::new();
    for row in &rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if row.headline_use == "canonical" {
            canonical_groups.insert(row.comparison_group.as_str());
        }
        for source_id in &row.source_ids {
            if !source_ledger.contains(&format!("`{source_id}`")) {
                return Err(format!(
                    "{}: source_id {source_id} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                    row.record_id
                ));
            }
        }
        for incompatible_id in &row.cannot_substitute_for {
            if !ids.contains(incompatible_id) {
                return Err(format!(
                    "{} references missing incompatible measure {incompatible_id}",
                    row.record_id
                ));
            }
            let other = rows
                .iter()
                .find(|candidate| candidate.record_id == *incompatible_id)
                .expect("ID membership checked");
            if !other.cannot_substitute_for.contains(&row.record_id) {
                return Err(format!(
                    "headline incompatibility must be reciprocal: {} -> {}",
                    row.record_id, incompatible_id
                ));
            }
        }
    }
    for group in ["interest", "defense", "health"] {
        if !canonical_groups.contains(group) {
            return Err(format!(
                "headline basis group {group} needs a canonical federal measure"
            ));
        }
    }
    for path in [
        HEADLINE_BASIS_README_PATH,
        HEADLINE_BASIS_SCHEMA_PATH,
        HEADLINE_BASIS_GUIDE_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing headline basis artifact: {path}"));
        }
    }
    let guide = fs::read_to_string(root.join(HEADLINE_BASIS_GUIDE_PATH))
        .map_err(|err| format!("failed to read {HEADLINE_BASIS_GUIDE_PATH}: {err}"))?;
    if !guide.contains(HEADLINE_BASIS_JSONL_PATH) || !guide.contains("not interchangeable") {
        return Err(
            "headline selection guide must cite the crosswalk and incompatibility rule".to_string(),
        );
    }
    println!("validated {} headline basis crosswalk rows", rows.len());
    Ok(())
}

fn validate_efficiency_pressure_records(root: &Path) -> Result<(), String> {
    let source_ledger = fs::read_to_string(root.join(SOURCE_VERSION_LEDGER_PATH))
        .map_err(|err| format!("failed to read {SOURCE_VERSION_LEDGER_PATH}: {err}"))?;
    let spend_rows: Vec<SpendCategoryMapRecord> =
        read_jsonl(root.join(SPEND_CATEGORY_MAP_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!("{SPEND_CATEGORY_MAP_JSONL_PATH} row failed to parse: {err}")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    let spend_ids: BTreeSet<String> = spend_rows.iter().map(|row| row.record_id.clone()).collect();
    let rows: Vec<EfficiencyPressureRecord> =
        read_jsonl(root.join(EFFICIENCY_PRESSURE_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!("{EFFICIENCY_PRESSURE_JSONL_PATH} row failed to parse: {err}")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

    if rows.len() != 5 {
        return Err(format!(
            "efficiency pressure must contain 5 FY2025 rows, got {}",
            rows.len()
        ));
    }

    let mut ids = BTreeSet::new();
    let mut level_counts = BTreeMap::new();
    for row in &rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate efficiency pressure row {}",
                row.record_id
            ));
        }
        *level_counts
            .entry(row.pressure_level.as_str())
            .or_insert(0usize) += 1;
        for related in &row.related_spend_categories {
            if related.starts_with("spendcat-") && !spend_ids.contains(related) {
                return Err(format!(
                    "{} references missing spend category {}",
                    row.record_id, related
                ));
            }
        }
    }

    for required_level in ["highest", "high", "watch"] {
        if !level_counts.contains_key(required_level) {
            return Err(format!(
                "efficiency pressure rows must include pressure_level {required_level}"
            ));
        }
    }

    let backlog_rows: Vec<CostDownBacklogRecord> =
        read_jsonl(root.join(COST_DOWN_BACKLOG_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!("{COST_DOWN_BACKLOG_JSONL_PATH} row failed to parse: {err}")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if backlog_rows.len() != 10 {
        return Err(format!(
            "cost-down backlog must contain 10 FY2025 rows, got {}",
            backlog_rows.len()
        ));
    }
    let mut backlog_ids = BTreeSet::new();
    let mut backlog_count_by_pressure = BTreeMap::new();
    for row in &backlog_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !backlog_ids.insert(row.record_id.clone()) {
            return Err(format!("duplicate cost-down backlog row {}", row.record_id));
        }
        if !ids.contains(&row.source_pressure_record_id) {
            return Err(format!(
                "{} references missing efficiency pressure row {}",
                row.record_id, row.source_pressure_record_id
            ));
        }
        *backlog_count_by_pressure
            .entry(row.source_pressure_record_id.as_str())
            .or_insert(0usize) += 1;
    }
    for pressure_id in &ids {
        let count = backlog_count_by_pressure
            .get(pressure_id.as_str())
            .copied()
            .unwrap_or(0);
        if count != 2 {
            return Err(format!(
                "{pressure_id} must have exactly 2 cost-down backlog rows, got {count}"
            ));
        }
    }

    let source_packet_rows: Vec<CostDownSourcePacketRecord> =
        read_jsonl(root.join(COST_DOWN_SOURCE_PACKETS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!("{COST_DOWN_SOURCE_PACKETS_JSONL_PATH} row failed to parse: {err}")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if source_packet_rows.len() != 10 {
        return Err(format!(
            "cost-down source packets must contain 10 current source packet rows, got {}",
            source_packet_rows.len()
        ));
    }
    let mut source_packet_ids = BTreeSet::new();
    let mut source_packet_by_id = BTreeMap::new();
    for row in &source_packet_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !source_packet_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate cost-down source packet row {}",
                row.record_id
            ));
        }
        source_packet_by_id.insert(row.record_id.clone(), row);
        if !backlog_ids.contains(&row.source_backlog_record_id) {
            return Err(format!(
                "{} references missing cost-down backlog row {}",
                row.record_id, row.source_backlog_record_id
            ));
        }
        if !ids.contains(&row.source_pressure_record_id) {
            return Err(format!(
                "{} references missing efficiency pressure row {}",
                row.record_id, row.source_pressure_record_id
            ));
        }
        for source_id in &row.source_ids {
            if !source_ledger.contains(&format!("`{source_id}`")) {
                return Err(format!(
                    "{}: source_id {source_id} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                    row.record_id
                ));
            }
        }
    }

    let evidence_queue_rows: Vec<CostDownEvidenceQueueRecord> =
        read_jsonl(root.join(COST_DOWN_EVIDENCE_QUEUE_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!("{COST_DOWN_EVIDENCE_QUEUE_JSONL_PATH} row failed to parse: {err}")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if evidence_queue_rows.len() != source_packet_rows.len() {
        return Err(format!(
            "cost-down evidence queue must cover every source packet row; got {} queue rows for {} packets",
            evidence_queue_rows.len(),
            source_packet_rows.len()
        ));
    }
    let mut evidence_queue_ids = BTreeSet::new();
    let mut queued_packet_ids = BTreeSet::new();
    for row in &evidence_queue_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !evidence_queue_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate cost-down evidence queue row {}",
                row.record_id
            ));
        }
        if !queued_packet_ids.insert(row.source_packet_record_id.clone()) {
            return Err(format!(
                "multiple evidence queue rows point to {}",
                row.source_packet_record_id
            ));
        }
        let packet = source_packet_by_id
            .get(&row.source_packet_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing source packet {}",
                    row.record_id, row.source_packet_record_id
                )
            })?;
        if row.source_backlog_record_id != packet.source_backlog_record_id {
            return Err(format!(
                "{} backlog {} does not match packet {} backlog {}",
                row.record_id,
                row.source_backlog_record_id,
                packet.record_id,
                packet.source_backlog_record_id
            ));
        }
        if row.source_pressure_record_id != packet.source_pressure_record_id {
            return Err(format!(
                "{} pressure {} does not match packet {} pressure {}",
                row.record_id,
                row.source_pressure_record_id,
                packet.record_id,
                packet.source_pressure_record_id
            ));
        }
        if row.lane_id != packet.lane_id {
            return Err(format!(
                "{} lane {} does not match packet {} lane {}",
                row.record_id, row.lane_id, packet.record_id, packet.lane_id
            ));
        }
        for source_id in &row.primary_source_ids {
            if !source_ledger.contains(&format!("`{source_id}`")) {
                return Err(format!(
                    "{}: primary_source_id {source_id} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                    row.record_id
                ));
            }
        }
    }
    if queued_packet_ids != source_packet_ids {
        return Err(
            "cost-down evidence queue must cover every source packet exactly once".to_string(),
        );
    }

    let payment_integrity_probe_rows: Vec<PaymentIntegrityPortalProbeRecord> =
        read_jsonl(root.join(PAYMENT_INTEGRITY_ELIGIBILITY_FIRST_PASS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_ELIGIBILITY_FIRST_PASS_JSONL_PATH} row failed to parse: {err}"
            )
        })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if payment_integrity_probe_rows.len() != 6 {
        return Err(format!(
            "payment integrity first-pass extract must contain 6 homepage agency rows, got {}",
            payment_integrity_probe_rows.len()
        ));
    }
    let mut payment_integrity_probe_ids = BTreeSet::new();
    let mut highest_count = 0usize;
    let mut lowest_count = 0usize;
    for row in &payment_integrity_probe_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !payment_integrity_probe_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity first-pass extract row {}",
                row.record_id
            ));
        }
        if !evidence_queue_ids.contains(&row.source_evidence_queue_record_id) {
            return Err(format!(
                "{} references missing evidence queue row {}",
                row.record_id, row.source_evidence_queue_record_id
            ));
        }
        if row.source_evidence_queue_record_id
            != "cost-down-evidence-queue:payment-integrity:eligibility-accuracy:v1"
        {
            return Err(format!(
                "{} must point to the payment-integrity eligibility evidence queue row",
                row.record_id
            ));
        }
        if !source_ledger.contains(&format!("`{}`", row.source_id)) {
            return Err(format!(
                "{}: source_id {} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                row.record_id, row.source_id
            ));
        }
        match row.row_kind.as_str() {
            "homepage_highest_performing_agency" => highest_count += 1,
            "homepage_lowest_performing_agency" => lowest_count += 1,
            _ => {}
        }
    }
    if highest_count != 3 || lowest_count != 3 {
        return Err(format!(
            "payment integrity first-pass extract must contain 3 highest and 3 lowest homepage agency rows, got {highest_count} and {lowest_count}"
        ));
    }

    let payment_integrity_scorecard_rows: Vec<PaymentIntegrityScorecardProbeRecord> =
        read_jsonl(root.join(PAYMENT_INTEGRITY_SCORECARDS_Q4_2025_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
            format!("{PAYMENT_INTEGRITY_SCORECARDS_Q4_2025_JSONL_PATH} row failed to parse: {err}")
        })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if payment_integrity_scorecard_rows.len() != 4 {
        return Err(format!(
            "payment integrity scorecard extract must contain 4 Q4 2025 scorecard rows, got {}",
            payment_integrity_scorecard_rows.len()
        ));
    }
    let mut payment_integrity_scorecard_ids = BTreeSet::new();
    for row in &payment_integrity_scorecard_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !payment_integrity_scorecard_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity scorecard extract row {}",
                row.record_id
            ));
        }
        if !evidence_queue_ids.contains(&row.source_evidence_queue_record_id) {
            return Err(format!(
                "{} references missing evidence queue row {}",
                row.record_id, row.source_evidence_queue_record_id
            ));
        }
        if row.source_evidence_queue_record_id
            != "cost-down-evidence-queue:payment-integrity:eligibility-accuracy:v1"
        {
            return Err(format!(
                "{} must point to the payment-integrity eligibility evidence queue row",
                row.record_id
            ));
        }
        if !source_ledger.contains(&format!("`{}`", row.source_id)) {
            return Err(format!(
                "{}: source_id {} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                row.record_id, row.source_id
            ));
        }
    }

    let payment_integrity_program_gate_rows: Vec<PaymentIntegrityProgramReviewGateRecord> =
        read_jsonl(root.join(PAYMENT_INTEGRITY_PROGRAM_REVIEW_GATE_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!(
                        "{PAYMENT_INTEGRITY_PROGRAM_REVIEW_GATE_JSONL_PATH} row failed to parse: {err}"
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if payment_integrity_program_gate_rows.len() != 4 {
        return Err(format!(
            "payment integrity program review gates must contain 4 rows, got {}",
            payment_integrity_program_gate_rows.len()
        ));
    }
    let mut payment_integrity_program_gate_ids = BTreeSet::new();
    let mut payment_integrity_program_gate_scorecard_ids = BTreeSet::new();
    for row in &payment_integrity_program_gate_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !payment_integrity_program_gate_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity program review gate row {}",
                row.record_id
            ));
        }
        if !payment_integrity_scorecard_ids.contains(&row.source_scorecard_record_id) {
            return Err(format!(
                "{} references missing scorecard row {}",
                row.record_id, row.source_scorecard_record_id
            ));
        }
        if !payment_integrity_program_gate_scorecard_ids
            .insert(row.source_scorecard_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity program review gate scorecard reference {}",
                row.source_scorecard_record_id
            ));
        }
        if row.source_readiness_record_id
            != "cost-down-scoring-readiness:payment-integrity:eligibility-accuracy:v1"
        {
            return Err(format!(
                "{} must point to the payment-integrity eligibility scoring-readiness row",
                row.record_id
            ));
        }
    }
    for scorecard_id in &payment_integrity_scorecard_ids {
        if !payment_integrity_program_gate_scorecard_ids.contains(scorecard_id) {
            return Err(format!(
                "payment integrity program review gates are missing scorecard row {scorecard_id}"
            ));
        }
    }

    let payment_integrity_program_task_rows: Vec<PaymentIntegrityProgramReviewTaskRecord> =
        read_jsonl(root.join(PAYMENT_INTEGRITY_PROGRAM_REVIEW_TASKS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!(
                        "{PAYMENT_INTEGRITY_PROGRAM_REVIEW_TASKS_JSONL_PATH} row failed to parse: {err}"
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if payment_integrity_program_task_rows.len() != 16 {
        return Err(format!(
            "payment integrity program review tasks must contain 16 rows, got {}",
            payment_integrity_program_task_rows.len()
        ));
    }
    let mut payment_integrity_program_task_ids = BTreeSet::new();
    let mut task_families_by_gate: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for row in &payment_integrity_program_task_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !payment_integrity_program_task_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity program review task row {}",
                row.record_id
            ));
        }
        if !payment_integrity_program_gate_ids.contains(&row.source_program_gate_record_id) {
            return Err(format!(
                "{} references missing program gate row {}",
                row.record_id, row.source_program_gate_record_id
            ));
        }
        if !payment_integrity_scorecard_ids.contains(&row.source_scorecard_record_id) {
            return Err(format!(
                "{} references missing scorecard row {}",
                row.record_id, row.source_scorecard_record_id
            ));
        }
        let families = task_families_by_gate
            .entry(row.source_program_gate_record_id.clone())
            .or_default();
        if !families.insert(row.evidence_family.clone()) {
            return Err(format!(
                "duplicate payment integrity program review task family {} for {}",
                row.evidence_family, row.source_program_gate_record_id
            ));
        }
    }
    let required_task_families = BTreeSet::from([
        "methodology".to_string(),
        "access_floor".to_string(),
        "corrective_action".to_string(),
        "confidence_limits".to_string(),
    ]);
    for gate_id in &payment_integrity_program_gate_ids {
        if task_families_by_gate.get(gate_id) != Some(&required_task_families) {
            return Err(format!(
                "payment integrity program review tasks must include all four evidence families for {gate_id}"
            ));
        }
    }

    let payment_integrity_program_status_rows: Vec<PaymentIntegrityProgramReviewStatusRecord> =
        read_jsonl(root.join(PAYMENT_INTEGRITY_PROGRAM_REVIEW_STATUS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!(
                        "{PAYMENT_INTEGRITY_PROGRAM_REVIEW_STATUS_JSONL_PATH} row failed to parse: {err}"
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if payment_integrity_program_status_rows.len() != 4 {
        return Err(format!(
            "payment integrity program review status must contain 4 rows, got {}",
            payment_integrity_program_status_rows.len()
        ));
    }
    let mut payment_integrity_program_status_ids = BTreeSet::new();
    let mut payment_integrity_program_status_gate_ids = BTreeSet::new();
    for row in &payment_integrity_program_status_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !payment_integrity_program_status_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity program review status row {}",
                row.record_id
            ));
        }
        if !payment_integrity_program_gate_ids.contains(&row.source_program_gate_record_id) {
            return Err(format!(
                "{} references missing program gate row {}",
                row.record_id, row.source_program_gate_record_id
            ));
        }
        if !payment_integrity_program_status_gate_ids
            .insert(row.source_program_gate_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity program review status gate reference {}",
                row.source_program_gate_record_id
            ));
        }
        let family_count = task_families_by_gate
            .get(&row.source_program_gate_record_id)
            .map(BTreeSet::len)
            .unwrap_or(0);
        if usize::from(row.total_required_task_count) != family_count {
            return Err(format!(
                "{} total_required_task_count does not match task families for {}",
                row.record_id, row.source_program_gate_record_id
            ));
        }
    }
    for gate_id in &payment_integrity_program_gate_ids {
        if !payment_integrity_program_status_gate_ids.contains(gate_id) {
            return Err(format!(
                "payment integrity program review status is missing gate row {gate_id}"
            ));
        }
    }

    let methodology_plan_rows: Vec<PaymentIntegrityMethodologyPlanRecord> =
        read_jsonl(root.join(PAYMENT_INTEGRITY_METHODOLOGY_PLANS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
            format!("{PAYMENT_INTEGRITY_METHODOLOGY_PLANS_JSONL_PATH} row failed to parse: {err}")
        })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if methodology_plan_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology plans must contain 4 rows, got {}",
            methodology_plan_rows.len()
        ));
    }
    let mut methodology_plan_ids = BTreeSet::new();
    let mut methodology_plan_status_ids = BTreeSet::new();
    let mut methodology_plan_priorities = BTreeSet::new();
    let methodology_task_ids: BTreeSet<_> = payment_integrity_program_task_rows
        .iter()
        .filter(|row| row.evidence_family == "methodology")
        .map(|row| row.record_id.clone())
        .collect();
    for row in &methodology_plan_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_plan_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology plan row {}",
                row.record_id
            ));
        }
        if !payment_integrity_program_status_ids.contains(&row.source_program_status_record_id) {
            return Err(format!(
                "{} references missing program status row {}",
                row.record_id, row.source_program_status_record_id
            ));
        }
        if !methodology_task_ids.contains(&row.source_methodology_task_record_id) {
            return Err(format!(
                "{} references missing methodology task row {}",
                row.record_id, row.source_methodology_task_record_id
            ));
        }
        if !methodology_plan_status_ids.insert(row.source_program_status_record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology plan status reference {}",
                row.source_program_status_record_id
            ));
        }
        if !methodology_plan_priorities.insert(row.extraction_priority) {
            return Err(format!(
                "duplicate payment integrity methodology plan extraction priority {}",
                row.extraction_priority
            ));
        }
    }
    for status_id in &payment_integrity_program_status_ids {
        if !methodology_plan_status_ids.contains(status_id) {
            return Err(format!(
                "payment integrity methodology plans are missing status row {status_id}"
            ));
        }
    }

    let methodology_field_rows: Vec<PaymentIntegrityMethodologyFieldRecord> =
        read_jsonl(root.join(PAYMENT_INTEGRITY_METHODOLOGY_FIELDS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
            format!("{PAYMENT_INTEGRITY_METHODOLOGY_FIELDS_JSONL_PATH} row failed to parse: {err}")
        })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if methodology_field_rows.len() != 32 {
        return Err(format!(
            "payment integrity methodology fields must contain 32 rows, got {}",
            methodology_field_rows.len()
        ));
    }
    let methodology_plan_fields: BTreeMap<_, BTreeSet<_>> = methodology_plan_rows
        .iter()
        .map(|row| {
            (
                row.record_id.clone(),
                row.required_methodology_fields
                    .iter()
                    .cloned()
                    .collect::<BTreeSet<_>>(),
            )
        })
        .collect();
    let mut methodology_field_ids = BTreeSet::new();
    let mut methodology_field_by_id = BTreeMap::new();
    let mut methodology_fields_by_plan: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for row in &methodology_field_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_field_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology field row {}",
                row.record_id
            ));
        }
        let Some(required_fields) =
            methodology_plan_fields.get(&row.source_methodology_plan_record_id)
        else {
            return Err(format!(
                "{} references missing methodology plan row {}",
                row.record_id, row.source_methodology_plan_record_id
            ));
        };
        if !required_fields.contains(&row.methodology_field) {
            return Err(format!(
                "{} field '{}' is not required by {}",
                row.record_id, row.methodology_field, row.source_methodology_plan_record_id
            ));
        }
        let fields = methodology_fields_by_plan
            .entry(row.source_methodology_plan_record_id.clone())
            .or_default();
        if !fields.insert(row.methodology_field.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology field '{}' for {}",
                row.methodology_field, row.source_methodology_plan_record_id
            ));
        }
        methodology_field_by_id.insert(row.record_id.clone(), row);
    }
    for (plan_id, required_fields) in &methodology_plan_fields {
        if methodology_fields_by_plan.get(plan_id) != Some(required_fields) {
            return Err(format!(
                "payment integrity methodology fields do not match required fields for {plan_id}"
            ));
        }
    }

    let methodology_source_target_rows: Vec<PaymentIntegrityMethodologySourceTargetRecord> =
        read_jsonl(root.join(PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_TARGETS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!(
                        "{PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_TARGETS_JSONL_PATH} row failed to parse: {err}"
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if methodology_source_target_rows.len() != 12 {
        return Err(format!(
            "payment integrity methodology source targets must contain 12 rows, got {}",
            methodology_source_target_rows.len()
        ));
    }
    let methodology_plan_targets: BTreeMap<_, BTreeSet<_>> = methodology_plan_rows
        .iter()
        .map(|row| {
            (
                row.record_id.clone(),
                row.source_discovery_targets
                    .iter()
                    .cloned()
                    .collect::<BTreeSet<_>>(),
            )
        })
        .collect();
    let mut methodology_source_target_ids = BTreeSet::new();
    let mut methodology_source_targets_by_plan: BTreeMap<String, BTreeSet<String>> =
        BTreeMap::new();
    let mut methodology_source_priorities_by_plan: BTreeMap<String, BTreeSet<u8>> = BTreeMap::new();
    for row in &methodology_source_target_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_source_target_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology source target row {}",
                row.record_id
            ));
        }
        let Some(required_targets) =
            methodology_plan_targets.get(&row.source_methodology_plan_record_id)
        else {
            return Err(format!(
                "{} references missing methodology plan row {}",
                row.record_id, row.source_methodology_plan_record_id
            ));
        };
        if !required_targets.contains(&row.source_target) {
            return Err(format!(
                "{} source target '{}' is not required by {}",
                row.record_id, row.source_target, row.source_methodology_plan_record_id
            ));
        }
        let targets = methodology_source_targets_by_plan
            .entry(row.source_methodology_plan_record_id.clone())
            .or_default();
        if !targets.insert(row.source_target.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology source target '{}' for {}",
                row.source_target, row.source_methodology_plan_record_id
            ));
        }
        let priorities = methodology_source_priorities_by_plan
            .entry(row.source_methodology_plan_record_id.clone())
            .or_default();
        if !priorities.insert(row.target_priority) {
            return Err(format!(
                "duplicate payment integrity methodology source target priority {} for {}",
                row.target_priority, row.source_methodology_plan_record_id
            ));
        }
    }
    for (plan_id, required_targets) in &methodology_plan_targets {
        if methodology_source_targets_by_plan.get(plan_id) != Some(required_targets) {
            return Err(format!(
                "payment integrity methodology source targets do not match required targets for {plan_id}"
            ));
        }
    }

    let methodology_query_rows: Vec<PaymentIntegrityMethodologyQueryRecord> =
        read_jsonl(root.join(PAYMENT_INTEGRITY_METHODOLOGY_QUERIES_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
            format!("{PAYMENT_INTEGRITY_METHODOLOGY_QUERIES_JSONL_PATH} row failed to parse: {err}")
        })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if methodology_query_rows.len() != 12 {
        return Err(format!(
            "payment integrity methodology queries must contain 12 rows, got {}",
            methodology_query_rows.len()
        ));
    }
    let mut methodology_query_ids = BTreeSet::new();
    let mut methodology_query_target_ids = BTreeSet::new();
    for row in &methodology_query_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_query_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology query row {}",
                row.record_id
            ));
        }
        if !methodology_source_target_ids.contains(&row.source_methodology_target_record_id) {
            return Err(format!(
                "{} references missing methodology source target row {}",
                row.record_id, row.source_methodology_target_record_id
            ));
        }
        if !methodology_query_target_ids.insert(row.source_methodology_target_record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology query target reference {}",
                row.source_methodology_target_record_id
            ));
        }
    }
    for target_id in &methodology_source_target_ids {
        if !methodology_query_target_ids.contains(target_id) {
            return Err(format!(
                "payment integrity methodology queries are missing source target row {target_id}"
            ));
        }
    }

    let methodology_query_run_rows: Vec<PaymentIntegrityMethodologyQueryRunRecord> =
        read_jsonl(root.join(PAYMENT_INTEGRITY_METHODOLOGY_QUERY_RUNS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_QUERY_RUNS_JSONL_PATH} row failed to parse: {err}"
            )
        })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if methodology_query_run_rows.len() != 12 {
        return Err(format!(
            "payment integrity methodology query runs must contain 12 rows, got {}",
            methodology_query_run_rows.len()
        ));
    }
    let mut methodology_query_run_ids = BTreeSet::new();
    let mut methodology_query_run_query_ids = BTreeSet::new();
    for row in &methodology_query_run_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_query_run_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology query run row {}",
                row.record_id
            ));
        }
        if !methodology_query_ids.contains(&row.source_methodology_query_record_id) {
            return Err(format!(
                "{} references missing methodology query row {}",
                row.record_id, row.source_methodology_query_record_id
            ));
        }
        if !methodology_query_run_query_ids.insert(row.source_methodology_query_record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology query run query reference {}",
                row.source_methodology_query_record_id
            ));
        }
    }
    for query_id in &methodology_query_ids {
        if !methodology_query_run_query_ids.contains(query_id) {
            return Err(format!(
                "payment integrity methodology query runs are missing query row {query_id}"
            ));
        }
    }

    let methodology_result_rows: Vec<PaymentIntegrityMethodologyResultRecord> =
        read_jsonl(root.join(PAYMENT_INTEGRITY_METHODOLOGY_RESULTS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
            format!("{PAYMENT_INTEGRITY_METHODOLOGY_RESULTS_JSONL_PATH} row failed to parse: {err}")
        })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if methodology_result_rows.len() != 10 {
        return Err(format!(
            "payment integrity methodology results must contain 10 captured result rows, got {}",
            methodology_result_rows.len()
        ));
    }
    let mut methodology_result_ids = BTreeSet::new();
    let mut va_pltss_methodology_result_ids = BTreeSet::new();
    let mut usda_crop_methodology_result_ids = BTreeSet::new();
    for row in &methodology_result_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_result_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology result row {}",
                row.record_id
            ));
        }
        if row.program_or_activity == "Purchased Long Term Services and Supports (PLTSS)" {
            va_pltss_methodology_result_ids.insert(row.record_id.clone());
        }
        if row.program_or_activity == "Federal Crop Insurance Program" {
            usda_crop_methodology_result_ids.insert(row.record_id.clone());
        }
        if !methodology_query_run_ids.contains(&row.source_methodology_query_run_record_id) {
            return Err(format!(
                "{} references missing methodology query-run row {}",
                row.record_id, row.source_methodology_query_run_record_id
            ));
        }
    }
    if va_pltss_methodology_result_ids.len() != 3 {
        return Err(format!(
            "expected 3 VA PLTSS methodology result rows, got {}",
            va_pltss_methodology_result_ids.len()
        ));
    }
    if usda_crop_methodology_result_ids.len() != 3 {
        return Err(format!(
            "expected 3 USDA Federal Crop Insurance methodology result rows, got {}",
            usda_crop_methodology_result_ids.len()
        ));
    }

    let methodology_result_review_readiness_rows:
        Vec<PaymentIntegrityMethodologyResultReviewReadinessRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_RESULT_REVIEW_READINESS_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_RESULT_REVIEW_READINESS_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_result_review_readiness_rows.len() != 2 {
        return Err(format!(
            "payment integrity methodology result review readiness must contain 2 rows, got {}",
            methodology_result_review_readiness_rows.len()
        ));
    }
    let va_pltss_plan_id = "payment-integrity-methodology-plan:va-pltss:q4-2025";
    let usda_crop_plan_id =
        "payment-integrity-methodology-plan:usda-federal-crop-insurance:q4-2025";
    let va_pltss_methodology_fields: BTreeSet<String> = methodology_field_rows
        .iter()
        .filter(|row| row.source_methodology_plan_record_id == va_pltss_plan_id)
        .map(|row| row.methodology_field.clone())
        .collect();
    if va_pltss_methodology_fields.len() != 8 {
        return Err(format!(
            "expected 8 VA PLTSS methodology fields for {va_pltss_plan_id}, got {}",
            va_pltss_methodology_fields.len()
        ));
    }
    let usda_crop_methodology_fields: BTreeSet<String> = methodology_field_rows
        .iter()
        .filter(|row| row.source_methodology_plan_record_id == usda_crop_plan_id)
        .map(|row| row.methodology_field.clone())
        .collect();
    if usda_crop_methodology_fields.len() != 8 {
        return Err(format!(
            "expected 8 USDA Federal Crop Insurance methodology fields for {usda_crop_plan_id}, got {}",
            usda_crop_methodology_fields.len()
        ));
    }
    let mut methodology_result_review_readiness_ids = BTreeSet::new();
    let mut methodology_result_review_readiness_programs = BTreeSet::new();
    for row in &methodology_result_review_readiness_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_result_review_readiness_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology result review readiness row {}",
                row.record_id
            ));
        }
        methodology_result_review_readiness_programs.insert(row.program_or_activity.clone());
        let source_ids: BTreeSet<String> = row
            .source_methodology_result_record_ids
            .iter()
            .cloned()
            .collect();
        let next_fields: BTreeSet<String> = row.next_methodology_fields.iter().cloned().collect();
        match (row.agency_code.as_str(), row.program_or_activity.as_str()) {
            ("VA", "Purchased Long Term Services and Supports (PLTSS)") => {
                if source_ids != va_pltss_methodology_result_ids {
                    return Err(format!(
                        "{} must exactly cover VA PLTSS methodology result rows",
                        row.record_id
                    ));
                }
                if next_fields != va_pltss_methodology_fields {
                    return Err(format!(
                        "{} must queue exactly the VA PLTSS methodology fields",
                        row.record_id
                    ));
                }
            }
            ("USDA", "Federal Crop Insurance Program") => {
                if source_ids != usda_crop_methodology_result_ids {
                    return Err(format!(
                        "{} must exactly cover USDA Federal Crop Insurance methodology result rows",
                        row.record_id
                    ));
                }
                if next_fields != usda_crop_methodology_fields {
                    return Err(format!(
                        "{} must queue exactly the USDA Federal Crop Insurance methodology fields",
                        row.record_id
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "{} must be a supported result-review-readiness row, got {} / {}",
                    row.record_id, row.agency_code, row.program_or_activity
                ));
            }
        }
    }
    let expected_methodology_result_review_readiness_programs = BTreeSet::from([
        "Federal Crop Insurance Program".to_string(),
        "Purchased Long Term Services and Supports (PLTSS)".to_string(),
    ]);
    if methodology_result_review_readiness_programs
        != expected_methodology_result_review_readiness_programs
    {
        return Err(
            "payment integrity methodology result review readiness must cover VA PLTSS and USDA Federal Crop Insurance"
                .to_string(),
        );
    }

    let methodology_field_review_rows: Vec<PaymentIntegrityMethodologyFieldReviewRecord> =
        read_jsonl(root.join(
            PAYMENT_INTEGRITY_METHODOLOGY_FIELD_REVIEWS_JSONL_PATH,
        ))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row).map_err(|err| {
                format!(
                    "{PAYMENT_INTEGRITY_METHODOLOGY_FIELD_REVIEWS_JSONL_PATH} row failed to parse: {err}"
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if methodology_field_review_rows.len() != 32 {
        return Err(format!(
            "payment integrity methodology field reviews must contain 32 Part D, Medicaid, VA PLTSS, and USDA Federal Crop Insurance rows, got {}",
            methodology_field_review_rows.len()
        ));
    }
    let cms_part_d_plan_id = "payment-integrity-methodology-plan:cms-part-d:q4-2025";
    let cms_medicaid_plan_id = "payment-integrity-methodology-plan:cms-medicaid:q4-2025";
    let cms_part_d_methodology_field_ids: BTreeSet<String> = methodology_field_rows
        .iter()
        .filter(|row| row.source_methodology_plan_record_id == cms_part_d_plan_id)
        .map(|row| row.record_id.clone())
        .collect();
    if cms_part_d_methodology_field_ids.len() != 8 {
        return Err(format!(
            "expected 8 CMS Part D methodology fields for {cms_part_d_plan_id}, got {}",
            cms_part_d_methodology_field_ids.len()
        ));
    }
    let cms_medicaid_methodology_field_ids: BTreeSet<String> = methodology_field_rows
        .iter()
        .filter(|row| row.source_methodology_plan_record_id == cms_medicaid_plan_id)
        .map(|row| row.record_id.clone())
        .collect();
    if cms_medicaid_methodology_field_ids.len() != 8 {
        return Err(format!(
            "expected 8 CMS Medicaid methodology fields for {cms_medicaid_plan_id}, got {}",
            cms_medicaid_methodology_field_ids.len()
        ));
    }
    let va_pltss_methodology_field_ids: BTreeSet<String> = methodology_field_rows
        .iter()
        .filter(|row| row.source_methodology_plan_record_id == va_pltss_plan_id)
        .map(|row| row.record_id.clone())
        .collect();
    if va_pltss_methodology_field_ids.len() != 8 {
        return Err(format!(
            "expected 8 VA PLTSS methodology fields for {va_pltss_plan_id}, got {}",
            va_pltss_methodology_field_ids.len()
        ));
    }
    let usda_crop_methodology_field_ids: BTreeSet<String> = methodology_field_rows
        .iter()
        .filter(|row| row.source_methodology_plan_record_id == usda_crop_plan_id)
        .map(|row| row.record_id.clone())
        .collect();
    if usda_crop_methodology_field_ids.len() != 8 {
        return Err(format!(
            "expected 8 USDA Federal Crop Insurance methodology fields for {usda_crop_plan_id}, got {}",
            usda_crop_methodology_field_ids.len()
        ));
    }
    let expected_reviewed_methodology_field_ids: BTreeSet<String> =
        cms_part_d_methodology_field_ids
            .union(&cms_medicaid_methodology_field_ids)
            .cloned()
            .collect::<BTreeSet<_>>()
            .union(&va_pltss_methodology_field_ids)
            .cloned()
            .collect::<BTreeSet<_>>()
            .union(&usda_crop_methodology_field_ids)
            .cloned()
            .collect();
    let mut methodology_field_review_ids = BTreeSet::new();
    let mut reviewed_methodology_field_ids = BTreeSet::new();
    let mut part_d_methodology_field_review_ids = BTreeSet::new();
    for row in &methodology_field_review_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_field_review_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology field review row {}",
                row.record_id
            ));
        }
        if !methodology_result_ids.contains(&row.source_methodology_result_record_id) {
            return Err(format!(
                "{} references missing methodology result row {}",
                row.record_id, row.source_methodology_result_record_id
            ));
        }
        if !methodology_field_ids.contains(&row.source_methodology_field_record_id) {
            return Err(format!(
                "{} references missing methodology field row {}",
                row.record_id, row.source_methodology_field_record_id
            ));
        }
        reviewed_methodology_field_ids.insert(row.source_methodology_field_record_id.clone());
        if cms_part_d_methodology_field_ids.contains(&row.source_methodology_field_record_id) {
            part_d_methodology_field_review_ids.insert(row.record_id.clone());
        }
    }
    if reviewed_methodology_field_ids != expected_reviewed_methodology_field_ids {
        return Err(
            "payment integrity methodology field reviews must exactly cover CMS Part D, Medicaid, VA PLTSS, and USDA Federal Crop Insurance methodology fields"
                .to_string(),
        );
    }

    let methodology_gap_followup_rows: Vec<PaymentIntegrityMethodologyGapFollowupRecord> =
        read_jsonl(root.join(
            PAYMENT_INTEGRITY_METHODOLOGY_GAP_FOLLOWUPS_JSONL_PATH,
        ))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row).map_err(|err| {
                format!(
                    "{PAYMENT_INTEGRITY_METHODOLOGY_GAP_FOLLOWUPS_JSONL_PATH} row failed to parse: {err}"
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if methodology_gap_followup_rows.len() != methodology_field_review_ids.len() {
        return Err(format!(
            "payment integrity methodology gap followups must match field-review count; got {} followups for {} reviews",
            methodology_gap_followup_rows.len(),
            methodology_field_review_ids.len()
        ));
    }
    let mut methodology_gap_followup_ids = BTreeSet::new();
    let mut methodology_gap_followup_review_ids = BTreeSet::new();
    let mut methodology_gap_followup_priorities_by_program: BTreeMap<String, BTreeSet<u8>> =
        BTreeMap::new();
    let mut part_d_methodology_gap_followup_ids = BTreeSet::new();
    for row in &methodology_gap_followup_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_gap_followup_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology gap followup row {}",
                row.record_id
            ));
        }
        if !methodology_field_review_ids.contains(&row.source_methodology_field_review_record_id) {
            return Err(format!(
                "{} references missing methodology field-review row {}",
                row.record_id, row.source_methodology_field_review_record_id
            ));
        }
        if !methodology_gap_followup_review_ids
            .insert(row.source_methodology_field_review_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology gap followup for review {}",
                row.source_methodology_field_review_record_id
            ));
        }
        if part_d_methodology_field_review_ids
            .contains(&row.source_methodology_field_review_record_id)
        {
            part_d_methodology_gap_followup_ids.insert(row.record_id.clone());
        }
        let priorities = methodology_gap_followup_priorities_by_program
            .entry(row.program_or_activity.clone())
            .or_default();
        if !priorities.insert(row.followup_priority) {
            return Err(format!(
                "duplicate payment integrity methodology gap followup priority {} for {}",
                row.followup_priority, row.program_or_activity
            ));
        }
    }
    if methodology_gap_followup_review_ids != methodology_field_review_ids {
        return Err(
            "payment integrity methodology gap followups must exactly cover field-review rows"
                .to_string(),
        );
    }
    let expected_gap_followup_priorities = (1..=8).collect::<BTreeSet<_>>();
    for (program, priorities) in &methodology_gap_followup_priorities_by_program {
        if priorities != &expected_gap_followup_priorities {
            return Err(format!(
                "payment integrity methodology gap followups must use priorities 1 through 8 for {program}"
            ));
        }
    }

    let methodology_gap_source_capture_rows:
        Vec<PaymentIntegrityMethodologyGapSourceCaptureRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_GAP_SOURCE_CAPTURES_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_GAP_SOURCE_CAPTURES_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_gap_source_capture_rows.len() != 32 {
        return Err(format!(
            "payment integrity methodology gap source captures must contain 32 Part D, Medicaid, VA PLTSS, and USDA Federal Crop Insurance rows, got {}",
            methodology_gap_source_capture_rows.len()
        ));
    }
    let mut methodology_gap_source_capture_ids = BTreeSet::new();
    let mut methodology_gap_source_capture_followup_ids = BTreeSet::new();
    let mut part_d_methodology_gap_source_capture_ids = BTreeSet::new();
    for row in &methodology_gap_source_capture_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_gap_source_capture_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology gap source capture row {}",
                row.record_id
            ));
        }
        if !methodology_gap_followup_ids.contains(&row.source_methodology_gap_followup_record_id) {
            return Err(format!(
                "{} references missing methodology gap-followup row {}",
                row.record_id, row.source_methodology_gap_followup_record_id
            ));
        }
        if part_d_methodology_gap_followup_ids
            .contains(&row.source_methodology_gap_followup_record_id)
        {
            part_d_methodology_gap_source_capture_ids.insert(row.record_id.clone());
        }
        if !methodology_gap_source_capture_followup_ids
            .insert(row.source_methodology_gap_followup_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology gap source capture for followup {}",
                row.source_methodology_gap_followup_record_id
            ));
        }
    }
    if methodology_gap_source_capture_followup_ids != methodology_gap_followup_ids {
        return Err(
            "payment integrity methodology gap source captures must exactly cover gap-followup rows"
                .to_string(),
        );
    }

    let methodology_source_capture_rollup_rows:
        Vec<PaymentIntegrityMethodologySourceCaptureRollupRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_CAPTURE_ROLLUP_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_CAPTURE_ROLLUP_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_source_capture_rollup_rows.len() != methodology_gap_source_capture_ids.len() {
        return Err(format!(
            "payment integrity methodology source capture rollups must match source-capture count; got {} rollups for {} captures",
            methodology_source_capture_rollup_rows.len(),
            methodology_gap_source_capture_ids.len()
        ));
    }
    let mut methodology_source_capture_rollup_ids = BTreeSet::new();
    let mut methodology_source_capture_rollup_capture_ids = BTreeSet::new();
    let mut methodology_source_capture_rollup_followup_ids = BTreeSet::new();
    let mut part_d_methodology_source_capture_rollup_ids = BTreeSet::new();
    for row in &methodology_source_capture_rollup_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_source_capture_rollup_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology source capture rollup row {}",
                row.record_id
            ));
        }
        if !methodology_gap_followup_ids.contains(&row.source_methodology_gap_followup_record_id) {
            return Err(format!(
                "{} references missing methodology gap-followup row {}",
                row.record_id, row.source_methodology_gap_followup_record_id
            ));
        }
        if !methodology_gap_source_capture_ids
            .contains(&row.source_methodology_gap_source_capture_record_id)
        {
            return Err(format!(
                "{} references missing methodology gap source-capture row {}",
                row.record_id, row.source_methodology_gap_source_capture_record_id
            ));
        }
        if part_d_methodology_gap_source_capture_ids
            .contains(&row.source_methodology_gap_source_capture_record_id)
        {
            part_d_methodology_source_capture_rollup_ids.insert(row.record_id.clone());
        }
        methodology_source_capture_rollup_followup_ids
            .insert(row.source_methodology_gap_followup_record_id.clone());
        if !methodology_source_capture_rollup_capture_ids
            .insert(row.source_methodology_gap_source_capture_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology source capture rollup for capture {}",
                row.source_methodology_gap_source_capture_record_id
            ));
        }
    }
    if methodology_source_capture_rollup_followup_ids != methodology_gap_followup_ids
        || methodology_source_capture_rollup_capture_ids != methodology_gap_source_capture_ids
    {
        return Err(
            "payment integrity methodology source capture rollups must exactly cover gap-followup and source-capture rows"
                .to_string(),
        );
    }

    let methodology_closure_readiness_rows:
        Vec<PaymentIntegrityMethodologyClosureReadinessRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_READINESS_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_READINESS_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_closure_readiness_rows.len() != methodology_source_capture_rollup_ids.len() {
        return Err(format!(
            "payment integrity methodology closure readiness rows must match source-capture rollup count; got {} readiness rows for {} rollups",
            methodology_closure_readiness_rows.len(),
            methodology_source_capture_rollup_ids.len()
        ));
    }
    let mut methodology_closure_readiness_ids = BTreeSet::new();
    let mut methodology_closure_readiness_rollup_ids = BTreeSet::new();
    let mut part_d_additional_source_readiness_ids = BTreeSet::new();
    let mut closure_review_candidate_count = 0;
    for row in &methodology_closure_readiness_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_closure_readiness_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology closure readiness row {}",
                row.record_id
            ));
        }
        if !methodology_source_capture_rollup_ids
            .contains(&row.source_methodology_source_capture_rollup_record_id)
        {
            return Err(format!(
                "{} references missing methodology source-capture rollup row {}",
                row.record_id, row.source_methodology_source_capture_rollup_record_id
            ));
        }
        if !methodology_closure_readiness_rollup_ids.insert(
            row.source_methodology_source_capture_rollup_record_id
                .clone(),
        ) {
            return Err(format!(
                "duplicate payment integrity methodology closure readiness row for rollup {}",
                row.source_methodology_source_capture_rollup_record_id
            ));
        }
        if row.closure_readiness_status == "closure_review_candidate" {
            closure_review_candidate_count += 1;
        }
        if part_d_methodology_source_capture_rollup_ids
            .contains(&row.source_methodology_source_capture_rollup_record_id)
            && row.closure_readiness_status == "additional_source_needed"
        {
            part_d_additional_source_readiness_ids.insert(row.record_id.clone());
        }
    }
    if methodology_closure_readiness_rollup_ids != methodology_source_capture_rollup_ids {
        return Err(
            "payment integrity methodology closure readiness rows must exactly cover source-capture rollup rows"
                .to_string(),
        );
    }
    if closure_review_candidate_count == 0 {
        return Err(
            "payment integrity methodology closure readiness must contain at least one closure-review candidate"
                .to_string(),
        );
    }

    let closure_review_candidate_ids = methodology_closure_readiness_rows
        .iter()
        .filter(|row| row.closure_readiness_status == "closure_review_candidate")
        .map(|row| row.record_id.clone())
        .collect::<BTreeSet<_>>();
    let methodology_closure_decision_rows:
        Vec<PaymentIntegrityMethodologyClosureDecisionRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_DECISIONS_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_DECISIONS_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_closure_decision_rows.len() != 2 {
        return Err(format!(
            "payment integrity methodology closure decisions must contain 2 internal closure rows, got {}",
            methodology_closure_decision_rows.len()
        ));
    }
    let mut methodology_closure_decision_ids = BTreeSet::new();
    let mut methodology_closure_decision_readiness_ids = BTreeSet::new();
    let mut part_d_methodology_closure_decision_ids = BTreeSet::new();
    let mut medicaid_methodology_closure_decision_ids = BTreeSet::new();
    for row in &methodology_closure_decision_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_closure_decision_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology closure decision row {}",
                row.record_id
            ));
        }
        if !closure_review_candidate_ids
            .contains(&row.source_methodology_closure_readiness_record_id)
        {
            return Err(format!(
                "{} must reference a closure-review candidate readiness row, got {}",
                row.record_id, row.source_methodology_closure_readiness_record_id
            ));
        }
        if row.methodology_field != "sample period" {
            return Err(format!(
                "{} closure decision must currently be scoped to sample period, got {}",
                row.record_id, row.methodology_field
            ));
        }
        if row.program_or_activity == "Medicare Prescription Drug Benefit (Part D)" {
            part_d_methodology_closure_decision_ids.insert(row.record_id.clone());
        } else if row.program_or_activity == "Medicaid" {
            medicaid_methodology_closure_decision_ids.insert(row.record_id.clone());
        }
        if !methodology_closure_decision_readiness_ids
            .insert(row.source_methodology_closure_readiness_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology closure decision for readiness row {}",
                row.source_methodology_closure_readiness_record_id
            ));
        }
    }

    let methodology_residual_source_gap_rows:
        Vec<PaymentIntegrityMethodologyResidualSourceGapRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_SOURCE_GAPS_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_SOURCE_GAPS_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    let all_additional_source_readiness_ids = methodology_closure_readiness_rows
        .iter()
        .filter(|row| row.closure_readiness_status == "additional_source_needed")
        .map(|row| row.record_id.clone())
        .collect::<BTreeSet<_>>();
    if methodology_residual_source_gap_rows.len() != all_additional_source_readiness_ids.len() {
        return Err(format!(
            "payment integrity methodology residual source gaps must match additional-source readiness count; got {} gaps for {} readiness rows",
            methodology_residual_source_gap_rows.len(),
            all_additional_source_readiness_ids.len()
        ));
    }
    let mut methodology_residual_source_gap_ids = BTreeSet::new();
    let mut methodology_residual_source_gap_readiness_ids = BTreeSet::new();
    let mut part_d_methodology_residual_source_gap_ids = BTreeSet::new();
    let mut medicaid_methodology_residual_source_gap_ids = BTreeSet::new();
    for row in &methodology_residual_source_gap_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_residual_source_gap_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology residual source gap row {}",
                row.record_id
            ));
        }
        if !all_additional_source_readiness_ids
            .contains(&row.source_methodology_closure_readiness_record_id)
        {
            return Err(format!(
                "{} must reference an additional-source-needed readiness row, got {}",
                row.record_id, row.source_methodology_closure_readiness_record_id
            ));
        }
        if part_d_additional_source_readiness_ids
            .contains(&row.source_methodology_closure_readiness_record_id)
        {
            part_d_methodology_residual_source_gap_ids.insert(row.record_id.clone());
        } else if row.program_or_activity == "Medicaid" {
            medicaid_methodology_residual_source_gap_ids.insert(row.record_id.clone());
        }
        if !methodology_residual_source_gap_readiness_ids
            .insert(row.source_methodology_closure_readiness_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology residual source gap for readiness row {}",
                row.source_methodology_closure_readiness_record_id
            ));
        }
    }
    if methodology_residual_source_gap_readiness_ids != all_additional_source_readiness_ids {
        return Err(
            "payment integrity methodology residual source gaps must exactly cover additional-source readiness rows"
                .to_string(),
        );
    }

    let methodology_closure_coverage_rows: Vec<PaymentIntegrityMethodologyClosureCoverageRecord> =
        read_jsonl(root.join(
            PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_COVERAGE_JSONL_PATH,
        ))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row).map_err(|err| {
                format!(
                    "{PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_COVERAGE_JSONL_PATH} row failed to parse: {err}"
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if methodology_closure_coverage_rows.len() != 2 {
        return Err(format!(
            "payment integrity methodology closure coverage must contain 2 rows, got {}",
            methodology_closure_coverage_rows.len()
        ));
    }
    let mut methodology_closure_coverage_ids = BTreeSet::new();
    let mut methodology_closure_coverage_programs = BTreeSet::new();
    let mut methodology_closure_coverage_by_id = BTreeMap::new();
    for row in &methodology_closure_coverage_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_closure_coverage_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology closure coverage row {}",
                row.record_id
            ));
        }
        if !methodology_closure_coverage_programs.insert(row.program_or_activity.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology closure coverage program {}",
                row.program_or_activity
            ));
        }
        if !methodology_closure_decision_ids
            .contains(&row.source_methodology_closure_decision_record_id)
        {
            return Err(format!(
                "{} references missing methodology closure-decision row {}",
                row.record_id, row.source_methodology_closure_decision_record_id
            ));
        }
        match row.program_or_activity.as_str() {
            "Medicare Prescription Drug Benefit (Part D)" => {
                if !part_d_methodology_closure_decision_ids
                    .contains(&row.source_methodology_closure_decision_record_id)
                    || row.closed_field_count as usize
                        != part_d_methodology_closure_decision_ids.len()
                    || row.open_field_count as usize
                        != part_d_methodology_residual_source_gap_ids.len()
                {
                    return Err(format!(
                        "{} closure coverage counts must match Part D closure decisions and residual gaps",
                        row.record_id
                    ));
                }
            }
            "Medicaid" => {
                if !medicaid_methodology_closure_decision_ids
                    .contains(&row.source_methodology_closure_decision_record_id)
                    || row.closed_field_count as usize
                        != medicaid_methodology_closure_decision_ids.len()
                    || row.open_field_count as usize
                        != medicaid_methodology_residual_source_gap_ids.len()
                {
                    return Err(format!(
                        "{} closure coverage counts must match Medicaid closure decisions and residual gaps",
                        row.record_id
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "{} closure coverage program is not supported: {}",
                    row.record_id, row.program_or_activity
                ));
            }
        }
        methodology_closure_coverage_by_id.insert(row.record_id.clone(), row);
    }

    let methodology_scoring_gate_rows: Vec<PaymentIntegrityMethodologyScoringGateRecord> =
        read_jsonl(root.join(PAYMENT_INTEGRITY_METHODOLOGY_SCORING_GATE_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!(
                        "{PAYMENT_INTEGRITY_METHODOLOGY_SCORING_GATE_JSONL_PATH} row failed to parse: {err}"
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if methodology_scoring_gate_rows.len() != 2 {
        return Err(format!(
            "payment integrity methodology scoring gate must contain 2 rows, got {}",
            methodology_scoring_gate_rows.len()
        ));
    }
    let mut methodology_scoring_gate_ids = BTreeSet::new();
    let mut methodology_scoring_gate_coverage_ids = BTreeSet::new();
    let mut methodology_scoring_gate_by_id = BTreeMap::new();
    for row in &methodology_scoring_gate_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_scoring_gate_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology scoring gate row {}",
                row.record_id
            ));
        }
        if !methodology_closure_coverage_ids
            .contains(&row.source_methodology_closure_coverage_record_id)
        {
            return Err(format!(
                "{} references missing methodology closure-coverage row {}",
                row.record_id, row.source_methodology_closure_coverage_record_id
            ));
        }
        if !methodology_scoring_gate_coverage_ids
            .insert(row.source_methodology_closure_coverage_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology scoring gate for closure coverage {}",
                row.source_methodology_closure_coverage_record_id
            ));
        }
        methodology_scoring_gate_by_id.insert(row.record_id.clone(), row);
    }
    if methodology_scoring_gate_coverage_ids != methodology_closure_coverage_ids {
        return Err(
            "payment integrity methodology scoring gates must exactly cover closure coverage rows"
                .to_string(),
        );
    }

    let methodology_program_rollup_rows: Vec<PaymentIntegrityMethodologyProgramRollupRecord> =
        read_jsonl(root.join(
            PAYMENT_INTEGRITY_METHODOLOGY_PROGRAM_ROLLUP_JSONL_PATH,
        ))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row).map_err(|err| {
                format!(
                    "{PAYMENT_INTEGRITY_METHODOLOGY_PROGRAM_ROLLUP_JSONL_PATH} row failed to parse: {err}"
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if methodology_program_rollup_rows.len() != 2 {
        return Err(format!(
            "payment integrity methodology program rollup must contain 2 rows, got {}",
            methodology_program_rollup_rows.len()
        ));
    }
    let mut methodology_program_rollup_ids = BTreeSet::new();
    let mut methodology_program_rollup_scoring_gate_ids = BTreeSet::new();
    let mut methodology_program_rollup_coverage_ids = BTreeSet::new();
    for row in &methodology_program_rollup_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_program_rollup_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology program rollup row {}",
                row.record_id
            ));
        }
        let scoring_gate = methodology_scoring_gate_by_id
            .get(&row.source_methodology_scoring_gate_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing methodology scoring-gate row {}",
                    row.record_id, row.source_methodology_scoring_gate_record_id
                )
            })?;
        let closure_coverage = methodology_closure_coverage_by_id
            .get(&row.source_methodology_closure_coverage_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing methodology closure-coverage row {}",
                    row.record_id, row.source_methodology_closure_coverage_record_id
                )
            })?;
        if scoring_gate.source_methodology_closure_coverage_record_id
            != row.source_methodology_closure_coverage_record_id
        {
            return Err(format!(
                "{} scoring gate and closure coverage references do not match",
                row.record_id
            ));
        }
        if row.agency_code != closure_coverage.agency_code
            || row.program_or_activity != closure_coverage.program_or_activity
            || row.agency_code != scoring_gate.agency_code
            || row.program_or_activity != scoring_gate.program_or_activity
        {
            return Err(format!(
                "{} program identity must match linked scoring-gate and closure-coverage rows",
                row.record_id
            ));
        }
        if row.total_methodology_fields != closure_coverage.total_methodology_fields
            || row.closed_field_count != closure_coverage.closed_field_count
            || row.open_field_count != closure_coverage.open_field_count
            || row.next_open_methodology_fields != closure_coverage.open_fields
            || row.scoring_gate_status != scoring_gate.gate_status
        {
            return Err(format!(
                "{} must mirror linked methodology closure coverage and scoring gate status",
                row.record_id
            ));
        }
        if !methodology_program_rollup_scoring_gate_ids
            .insert(row.source_methodology_scoring_gate_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology program rollup for scoring gate {}",
                row.source_methodology_scoring_gate_record_id
            ));
        }
        if !methodology_program_rollup_coverage_ids
            .insert(row.source_methodology_closure_coverage_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology program rollup for closure coverage {}",
                row.source_methodology_closure_coverage_record_id
            ));
        }
    }
    if methodology_program_rollup_scoring_gate_ids != methodology_scoring_gate_ids
        || methodology_program_rollup_coverage_ids != methodology_closure_coverage_ids
    {
        return Err(
            "payment integrity methodology program rollups must exactly cover scoring-gate and closure-coverage rows"
                .to_string(),
        );
    }

    let methodology_open_program_status_rows:
        Vec<PaymentIntegrityMethodologyOpenProgramStatusRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_STATUS_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_STATUS_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_open_program_status_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology open program status must contain 4 rows, got {}",
            methodology_open_program_status_rows.len()
        ));
    }
    let mut closure_decision_count_by_program: BTreeMap<String, u8> = BTreeMap::new();
    for row in &methodology_closure_decision_rows {
        *closure_decision_count_by_program
            .entry(row.program_or_activity.clone())
            .or_default() += 1;
    }
    let mut residual_gap_count_by_program: BTreeMap<String, u8> = BTreeMap::new();
    for row in &methodology_residual_source_gap_rows {
        *residual_gap_count_by_program
            .entry(row.program_or_activity.clone())
            .or_default() += 1;
    }
    let mut field_count_by_program: BTreeMap<String, u8> = BTreeMap::new();
    for row in &methodology_field_rows {
        *field_count_by_program
            .entry(row.program_or_activity.clone())
            .or_default() += 1;
    }
    let expected_open_program_status_programs = BTreeSet::from([
        "Federal Crop Insurance Program".to_string(),
        "Medicaid".to_string(),
        "Medicare Prescription Drug Benefit (Part D)".to_string(),
        "Purchased Long Term Services and Supports (PLTSS)".to_string(),
    ]);
    let mut methodology_open_program_status_ids = BTreeSet::new();
    let mut methodology_open_program_status_programs = BTreeSet::new();
    let mut methodology_open_program_status_by_id = BTreeMap::new();
    for row in &methodology_open_program_status_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_open_program_status_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology open program status row {}",
                row.record_id
            ));
        }
        if !methodology_open_program_status_programs.insert(row.program_or_activity.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology open program status program {}",
                row.program_or_activity
            ));
        }
        if !methodology_plan_ids.contains(&row.source_methodology_plan_record_id) {
            return Err(format!(
                "{} references missing methodology plan row {}",
                row.record_id, row.source_methodology_plan_record_id
            ));
        }
        let field_count = field_count_by_program
            .get(&row.program_or_activity)
            .copied()
            .unwrap_or_default();
        let closure_decision_count = closure_decision_count_by_program
            .get(&row.program_or_activity)
            .copied()
            .unwrap_or_default();
        let residual_gap_count = residual_gap_count_by_program
            .get(&row.program_or_activity)
            .copied()
            .unwrap_or_default();
        if row.total_methodology_fields != field_count
            || row.closure_decision_count != closure_decision_count
            || row.closed_field_count != closure_decision_count
            || row.residual_source_gap_count != residual_gap_count
            || row.open_field_count != residual_gap_count
        {
            return Err(format!(
                "{} counts must match methodology fields, closure decisions, and residual gaps",
                row.record_id
            ));
        }
        methodology_open_program_status_by_id.insert(row.record_id.clone(), row);
    }
    if methodology_open_program_status_programs != expected_open_program_status_programs {
        return Err(
            "payment integrity methodology open program status must cover Part D, Medicaid, VA PLTSS, and USDA Federal Crop Insurance"
                .to_string(),
        );
    }

    let methodology_residual_gap_priority_rows:
        Vec<PaymentIntegrityMethodologyResidualGapPriorityRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_GAP_PRIORITY_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_GAP_PRIORITY_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_residual_gap_priority_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology residual gap priority must contain 4 rows, got {}",
            methodology_residual_gap_priority_rows.len()
        ));
    }
    let methodology_residual_source_gap_by_id: BTreeMap<_, _> =
        methodology_residual_source_gap_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let expected_methodology_residual_gap_priorities = BTreeSet::from([1, 2, 3, 4]);
    let mut methodology_residual_gap_priority_ids = BTreeSet::new();
    let mut methodology_residual_gap_priority_programs = BTreeSet::new();
    let mut methodology_residual_gap_priority_ranks = BTreeSet::new();
    let mut methodology_residual_gap_priority_by_id = BTreeMap::new();
    for row in &methodology_residual_gap_priority_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_residual_gap_priority_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology residual gap priority row {}",
                row.record_id
            ));
        }
        if !methodology_residual_gap_priority_programs.insert(row.program_or_activity.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology residual gap priority program {}",
                row.program_or_activity
            ));
        }
        methodology_residual_gap_priority_ranks.insert(row.priority_rank);
        let open_status = methodology_open_program_status_by_id
            .get(&row.source_open_program_status_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing methodology open-program status row {}",
                    row.record_id, row.source_open_program_status_record_id
                )
            })?;
        let residual_gap = methodology_residual_source_gap_by_id
            .get(&row.source_residual_source_gap_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing methodology residual source-gap row {}",
                    row.record_id, row.source_residual_source_gap_record_id
                )
            })?;
        if row.agency_code != open_status.agency_code
            || row.agency_code != residual_gap.agency_code
            || row.program_or_activity != open_status.program_or_activity
            || row.program_or_activity != residual_gap.program_or_activity
        {
            return Err(format!(
                "{} agency/program must match open-program status and residual source-gap rows",
                row.record_id
            ));
        }
        if row.selected_methodology_field != residual_gap.methodology_field {
            return Err(format!(
                "{} selected methodology field must match residual source-gap methodology_field",
                row.record_id
            ));
        }
        if row.next_query_text != residual_gap.next_query_text {
            return Err(format!(
                "{} next_query_text must match residual source-gap next_query_text",
                row.record_id
            ));
        }
        methodology_residual_gap_priority_by_id.insert(row.record_id.clone(), row);
    }
    if methodology_residual_gap_priority_programs != expected_open_program_status_programs {
        return Err(
            "payment integrity methodology residual gap priority must cover the same four programs as open-program status"
                .to_string(),
        );
    }
    if methodology_residual_gap_priority_ranks != expected_methodology_residual_gap_priorities {
        return Err(
            "payment integrity methodology residual gap priority ranks must exactly cover 1 through 4"
                .to_string(),
        );
    }

    let methodology_priority_source_work_rows:
        Vec<PaymentIntegrityMethodologyPrioritySourceWorkRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_SOURCE_WORK_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_SOURCE_WORK_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_priority_source_work_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology priority source work must contain 4 rows, got {}",
            methodology_priority_source_work_rows.len()
        ));
    }
    let mut methodology_priority_source_work_ids = BTreeSet::new();
    let mut methodology_priority_source_work_priority_ids = BTreeSet::new();
    let mut methodology_priority_source_work_by_id = BTreeMap::new();
    for row in &methodology_priority_source_work_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_priority_source_work_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology priority source work row {}",
                row.record_id
            ));
        }
        if !methodology_priority_source_work_priority_ids
            .insert(row.source_residual_gap_priority_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology priority source work priority reference {}",
                row.source_residual_gap_priority_record_id
            ));
        }
        let priority = methodology_residual_gap_priority_by_id
            .get(&row.source_residual_gap_priority_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing methodology residual-gap priority row {}",
                    row.record_id, row.source_residual_gap_priority_record_id
                )
            })?;
        if row.agency_code != priority.agency_code
            || row.program_or_activity != priority.program_or_activity
            || row.priority_rank != priority.priority_rank
            || row.selected_methodology_field != priority.selected_methodology_field
        {
            return Err(format!(
                "{} must match agency, program, rank, and selected field from its residual-gap priority row",
                row.record_id
            ));
        }
        methodology_priority_source_work_by_id.insert(row.record_id.clone(), row);
    }
    if methodology_priority_source_work_priority_ids != methodology_residual_gap_priority_ids {
        return Err(
            "payment integrity methodology priority source work must exactly cover residual-gap priority rows"
                .to_string(),
        );
    }

    let methodology_priority_reviewer_action_rows:
        Vec<PaymentIntegrityMethodologyPriorityReviewerActionRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_REVIEWER_ACTIONS_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_REVIEWER_ACTIONS_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_priority_reviewer_action_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology priority reviewer actions must contain 4 rows, got {}",
            methodology_priority_reviewer_action_rows.len()
        ));
    }
    let mut methodology_priority_reviewer_action_ids = BTreeSet::new();
    let mut methodology_priority_reviewer_action_source_work_ids = BTreeSet::new();
    let mut methodology_priority_reviewer_action_reframing_programs = BTreeSet::new();
    let mut methodology_priority_reviewer_action_by_id = BTreeMap::new();
    for row in &methodology_priority_reviewer_action_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_priority_reviewer_action_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology priority reviewer action row {}",
                row.record_id
            ));
        }
        if !methodology_priority_reviewer_action_source_work_ids
            .insert(row.source_priority_source_work_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology priority reviewer action source-work reference {}",
                row.source_priority_source_work_record_id
            ));
        }
        let source_work = methodology_priority_source_work_by_id
            .get(&row.source_priority_source_work_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing methodology priority source-work row {}",
                    row.record_id, row.source_priority_source_work_record_id
                )
            })?;
        if row.agency_code != source_work.agency_code
            || row.program_or_activity != source_work.program_or_activity
            || row.priority_rank != source_work.priority_rank
            || row.selected_methodology_field != source_work.selected_methodology_field
        {
            return Err(format!(
                "{} must match agency, program, rank, and selected field from its priority source-work row",
                row.record_id
            ));
        }
        if row.field_reframing_allowed {
            methodology_priority_reviewer_action_reframing_programs
                .insert(row.program_or_activity.clone());
        }
        methodology_priority_reviewer_action_by_id.insert(row.record_id.clone(), row);
    }
    if methodology_priority_reviewer_action_source_work_ids != methodology_priority_source_work_ids
    {
        return Err(
            "payment integrity methodology priority reviewer actions must exactly cover priority source-work rows"
                .to_string(),
        );
    }
    if methodology_priority_reviewer_action_reframing_programs
        != BTreeSet::from(["Federal Crop Insurance Program".to_string()])
    {
        return Err(
            "payment integrity methodology priority reviewer actions may only allow current field reframing for USDA Federal Crop Insurance"
                .to_string(),
        );
    }

    let methodology_field_update_rows: Vec<PaymentIntegrityMethodologyFieldUpdateRecord> =
        read_jsonl(root.join(PAYMENT_INTEGRITY_METHODOLOGY_FIELD_UPDATES_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!(
                        "{PAYMENT_INTEGRITY_METHODOLOGY_FIELD_UPDATES_JSONL_PATH} row failed to parse: {err}"
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if methodology_field_update_rows.len() != 1 {
        return Err(format!(
            "payment integrity methodology field updates must contain 1 row, got {}",
            methodology_field_update_rows.len()
        ));
    }
    let mut methodology_field_update_ids = BTreeSet::new();
    let mut methodology_field_update_by_id = BTreeMap::new();
    for row in &methodology_field_update_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_field_update_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology field update row {}",
                row.record_id
            ));
        }
        let reviewer_action = methodology_priority_reviewer_action_by_id
            .get(&row.source_priority_reviewer_action_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing methodology priority reviewer-action row {}",
                    row.record_id, row.source_priority_reviewer_action_record_id
                )
            })?;
        let methodology_field = methodology_field_by_id
            .get(&row.source_methodology_field_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing methodology field row {}",
                    row.record_id, row.source_methodology_field_record_id
                )
            })?;
        if !reviewer_action.field_reframing_allowed
            || row.agency_code != reviewer_action.agency_code
            || row.program_or_activity != reviewer_action.program_or_activity
            || row.old_methodology_field != reviewer_action.selected_methodology_field
        {
            return Err(format!(
                "{} must match a field-reframing reviewer action",
                row.record_id
            ));
        }
        if row.agency_code != methodology_field.agency_code
            || row.program_or_activity != methodology_field.program_or_activity
            || row.old_methodology_field != methodology_field.methodology_field
            || row.old_required_source_target != methodology_field.required_source_target
            || row.old_completion_rule != methodology_field.completion_rule
        {
            return Err(format!(
                "{} old field values must match the source methodology-field row",
                row.record_id
            ));
        }
        if row.program_or_activity != "Federal Crop Insurance Program"
            || row.revised_methodology_field
                != "data-access outside-agency-control root-cause definition"
        {
            return Err(
                "payment integrity methodology field update currently supports only USDA FCIC root-cause reframing"
                    .to_string(),
            );
        }
        methodology_field_update_by_id.insert(row.record_id.clone(), row);
    }

    let methodology_followup_source_query_rows:
        Vec<PaymentIntegrityMethodologyFollowupSourceQueryRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERIES_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERIES_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_followup_source_query_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology follow-up source queries must contain 4 rows, got {}",
            methodology_followup_source_query_rows.len()
        ));
    }
    let mut methodology_followup_source_query_ids = BTreeSet::new();
    let mut methodology_followup_source_query_action_ids = BTreeSet::new();
    for row in &methodology_followup_source_query_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_followup_source_query_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology follow-up source query row {}",
                row.record_id
            ));
        }
        if !methodology_followup_source_query_action_ids
            .insert(row.source_priority_reviewer_action_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology follow-up source query reviewer-action reference {}",
                row.source_priority_reviewer_action_record_id
            ));
        }
        let reviewer_action = methodology_priority_reviewer_action_by_id
            .get(&row.source_priority_reviewer_action_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing methodology priority reviewer-action row {}",
                    row.record_id, row.source_priority_reviewer_action_record_id
                )
            })?;
        if row.agency_code != reviewer_action.agency_code
            || row.program_or_activity != reviewer_action.program_or_activity
            || row.priority_rank != reviewer_action.priority_rank
        {
            return Err(format!(
                "{} must match agency, program, and rank from its reviewer-action row",
                row.record_id
            ));
        }
        match &row.source_field_update_record_id {
            Some(field_update_id) => {
                let field_update = methodology_field_update_by_id
                    .get(field_update_id)
                    .ok_or_else(|| {
                        format!(
                            "{} references missing methodology field-update row {}",
                            row.record_id, field_update_id
                        )
                    })?;
                if row.program_or_activity != field_update.program_or_activity
                    || row
                        .query_objective
                        .to_ascii_lowercase()
                        .contains("agency-process-error")
                {
                    return Err(format!(
                        "{} field-update follow-up must use revised field framing",
                        row.record_id
                    ));
                }
            }
            None => {
                if reviewer_action.field_reframing_allowed {
                    return Err(format!(
                        "{} must reference the field-update row for a reframed field",
                        row.record_id
                    ));
                }
            }
        }
    }
    if methodology_followup_source_query_action_ids != methodology_priority_reviewer_action_ids {
        return Err(
            "payment integrity methodology follow-up source queries must exactly cover priority reviewer-action rows"
                .to_string(),
        );
    }

    let methodology_followup_source_query_by_id: BTreeMap<_, _> =
        methodology_followup_source_query_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_followup_source_query_run_rows:
        Vec<PaymentIntegrityMethodologyFollowupSourceQueryRunRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERY_RUNS_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERY_RUNS_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_followup_source_query_run_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology follow-up source query runs must contain 4 rows, got {}",
            methodology_followup_source_query_run_rows.len()
        ));
    }
    let mut methodology_followup_source_query_run_ids = BTreeSet::new();
    let mut methodology_followup_source_query_run_query_ids = BTreeSet::new();
    for row in &methodology_followup_source_query_run_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_followup_source_query_run_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology follow-up source query run row {}",
                row.record_id
            ));
        }
        if !methodology_followup_source_query_run_query_ids
            .insert(row.source_followup_source_query_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology follow-up source query run query reference {}",
                row.source_followup_source_query_record_id
            ));
        }
        let query = methodology_followup_source_query_by_id
            .get(&row.source_followup_source_query_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing methodology follow-up source-query row {}",
                    row.record_id, row.source_followup_source_query_record_id
                )
            })?;
        if row.agency_code != query.agency_code
            || row.program_or_activity != query.program_or_activity
            || row.priority_rank != query.priority_rank
            || row.planned_query_text != query.query_text
        {
            return Err(format!(
                "{} must match agency, program, rank, and query text from its follow-up source-query row",
                row.record_id
            ));
        }
    }
    if methodology_followup_source_query_run_query_ids != methodology_followup_source_query_ids {
        return Err(
            "payment integrity methodology follow-up source query runs must exactly cover follow-up source queries"
                .to_string(),
        );
    }

    let methodology_followup_source_capture_rows:
        Vec<PaymentIntegrityMethodologyFollowupSourceCaptureRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURES_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURES_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_followup_source_capture_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology follow-up source captures must contain 4 rows, got {}",
            methodology_followup_source_capture_rows.len()
        ));
    }
    let methodology_followup_source_query_run_by_id: BTreeMap<_, _> =
        methodology_followup_source_query_run_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let mut methodology_followup_source_capture_ids = BTreeSet::new();
    let mut methodology_followup_source_capture_run_ids = BTreeSet::new();
    for row in &methodology_followup_source_capture_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_followup_source_capture_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology follow-up source capture row {}",
                row.record_id
            ));
        }
        if !methodology_followup_source_capture_run_ids
            .insert(row.source_followup_source_query_run_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology follow-up source capture query-run reference {}",
                row.source_followup_source_query_run_record_id
            ));
        }
        let query_run = methodology_followup_source_query_run_by_id
            .get(&row.source_followup_source_query_run_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing methodology follow-up source-query-run row {}",
                    row.record_id, row.source_followup_source_query_run_record_id
                )
            })?;
        if row.agency_code != query_run.agency_code
            || row.program_or_activity != query_run.program_or_activity
            || row.priority_rank != query_run.priority_rank
        {
            return Err(format!(
                "{} must match agency, program, and rank from its follow-up source-query-run row",
                row.record_id
            ));
        }
    }
    if methodology_followup_source_capture_run_ids != methodology_followup_source_query_run_ids {
        return Err(
            "payment integrity methodology follow-up source captures must exactly cover follow-up source query runs"
                .to_string(),
        );
    }

    let methodology_followup_source_capture_by_id: BTreeMap<_, _> =
        methodology_followup_source_capture_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_followup_source_capture_rollup_rows:
        Vec<PaymentIntegrityMethodologyFollowupSourceCaptureRollupRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURE_ROLLUP_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURE_ROLLUP_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_followup_source_capture_rollup_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology follow-up source capture rollup must contain 4 rows, got {}",
            methodology_followup_source_capture_rollup_rows.len()
        ));
    }
    let mut methodology_followup_source_capture_rollup_ids = BTreeSet::new();
    let mut methodology_followup_source_capture_rollup_capture_ids = BTreeSet::new();
    for row in &methodology_followup_source_capture_rollup_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_followup_source_capture_rollup_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology follow-up source capture rollup row {}",
                row.record_id
            ));
        }
        if !methodology_followup_source_capture_rollup_capture_ids
            .insert(row.source_followup_source_capture_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology follow-up source capture rollup capture reference {}",
                row.source_followup_source_capture_record_id
            ));
        }
        let capture = methodology_followup_source_capture_by_id
            .get(&row.source_followup_source_capture_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing methodology follow-up source-capture row {}",
                    row.record_id, row.source_followup_source_capture_record_id
                )
            })?;
        if row.agency_code != capture.agency_code
            || row.program_or_activity != capture.program_or_activity
            || row.priority_rank != capture.priority_rank
        {
            return Err(format!(
                "{} must match agency, program, and rank from its follow-up source-capture row",
                row.record_id
            ));
        }
    }
    if methodology_followup_source_capture_rollup_capture_ids
        != methodology_followup_source_capture_ids
    {
        return Err(
            "payment integrity methodology follow-up source capture rollup must exactly cover follow-up source captures"
                .to_string(),
        );
    }

    let methodology_followup_source_capture_rollup_by_id: BTreeMap<_, _> =
        methodology_followup_source_capture_rollup_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_followup_boundary_decision_rows:
        Vec<PaymentIntegrityMethodologyFollowupBoundaryDecisionRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_DECISIONS_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_DECISIONS_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_followup_boundary_decision_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology follow-up boundary decisions must contain 4 rows, got {}",
            methodology_followup_boundary_decision_rows.len()
        ));
    }
    let mut methodology_followup_boundary_decision_ids = BTreeSet::new();
    let mut methodology_followup_boundary_decision_rollup_ids = BTreeSet::new();
    for row in &methodology_followup_boundary_decision_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_followup_boundary_decision_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology follow-up boundary decision row {}",
                row.record_id
            ));
        }
        if !methodology_followup_boundary_decision_rollup_ids
            .insert(row.source_followup_source_capture_rollup_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology follow-up boundary decision rollup reference {}",
                row.source_followup_source_capture_rollup_record_id
            ));
        }
        let rollup = methodology_followup_source_capture_rollup_by_id
            .get(&row.source_followup_source_capture_rollup_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing methodology follow-up source-capture rollup row {}",
                    row.record_id, row.source_followup_source_capture_rollup_record_id
                )
            })?;
        if row.agency_code != rollup.agency_code
            || row.program_or_activity != rollup.program_or_activity
            || row.priority_rank != rollup.priority_rank
        {
            return Err(format!(
                "{} must match agency, program, and rank from its follow-up source-capture rollup row",
                row.record_id
            ));
        }
    }
    if methodology_followup_boundary_decision_rollup_ids
        != methodology_followup_source_capture_rollup_ids
    {
        return Err(
            "payment integrity methodology follow-up boundary decisions must exactly cover follow-up source capture rollups"
                .to_string(),
        );
    }

    let methodology_followup_boundary_decision_by_id: BTreeMap<_, _> =
        methodology_followup_boundary_decision_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_followup_boundary_readiness_rows:
        Vec<PaymentIntegrityMethodologyFollowupBoundaryReadinessRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_READINESS_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_READINESS_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_followup_boundary_readiness_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology follow-up boundary readiness must contain 4 rows, got {}",
            methodology_followup_boundary_readiness_rows.len()
        ));
    }
    let mut methodology_followup_boundary_readiness_ids = BTreeSet::new();
    let mut methodology_followup_boundary_readiness_decision_ids = BTreeSet::new();
    for row in &methodology_followup_boundary_readiness_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_followup_boundary_readiness_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology follow-up boundary readiness row {}",
                row.record_id
            ));
        }
        if !methodology_followup_boundary_readiness_decision_ids
            .insert(row.source_followup_boundary_decision_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology follow-up boundary readiness decision reference {}",
                row.source_followup_boundary_decision_record_id
            ));
        }
        let decision = methodology_followup_boundary_decision_by_id
            .get(&row.source_followup_boundary_decision_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing methodology follow-up boundary-decision row {}",
                    row.record_id, row.source_followup_boundary_decision_record_id
                )
            })?;
        if row.agency_code != decision.agency_code
            || row.program_or_activity != decision.program_or_activity
            || row.priority_rank != decision.priority_rank
        {
            return Err(format!(
                "{} must match agency, program, and rank from its follow-up boundary-decision row",
                row.record_id
            ));
        }
    }
    if methodology_followup_boundary_readiness_decision_ids
        != methodology_followup_boundary_decision_ids
    {
        return Err(
            "payment integrity methodology follow-up boundary readiness must exactly cover follow-up boundary decisions"
                .to_string(),
        );
    }

    let narrow_ready_followup_boundary_readiness_ids: BTreeSet<_> =
        methodology_followup_boundary_readiness_rows
            .iter()
            .filter(|row| row.boundary_readiness_status == "narrow_internal_readiness_candidate")
            .map(|row| row.record_id.clone())
            .collect();
    let methodology_followup_boundary_readiness_by_id: BTreeMap<_, _> =
        methodology_followup_boundary_readiness_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_narrow_closure_candidate_rows:
        Vec<PaymentIntegrityMethodologyNarrowClosureCandidateRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_CANDIDATES_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_CANDIDATES_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_narrow_closure_candidate_rows.len() != 2 {
        return Err(format!(
            "payment integrity methodology narrow closure candidates must contain 2 rows, got {}",
            methodology_narrow_closure_candidate_rows.len()
        ));
    }
    let mut methodology_narrow_closure_candidate_ids = BTreeSet::new();
    let mut methodology_narrow_closure_candidate_readiness_ids = BTreeSet::new();
    for row in &methodology_narrow_closure_candidate_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_narrow_closure_candidate_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology narrow closure candidate row {}",
                row.record_id
            ));
        }
        if !methodology_narrow_closure_candidate_readiness_ids
            .insert(row.source_followup_boundary_readiness_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology narrow closure candidate readiness reference {}",
                row.source_followup_boundary_readiness_record_id
            ));
        }
        let readiness = methodology_followup_boundary_readiness_by_id
            .get(&row.source_followup_boundary_readiness_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing methodology follow-up boundary-readiness row {}",
                    row.record_id, row.source_followup_boundary_readiness_record_id
                )
            })?;
        if readiness.boundary_readiness_status != "narrow_internal_readiness_candidate"
            || row.agency_code != readiness.agency_code
            || row.program_or_activity != readiness.program_or_activity
            || row.priority_rank != readiness.priority_rank
        {
            return Err(format!(
                "{} must match a narrow internal boundary-readiness row",
                row.record_id
            ));
        }
    }
    if methodology_narrow_closure_candidate_readiness_ids
        != narrow_ready_followup_boundary_readiness_ids
    {
        return Err(
            "payment integrity methodology narrow closure candidates must exactly cover narrow-ready boundary-readiness rows"
                .to_string(),
        );
    }

    let methodology_narrow_closure_candidate_by_id: BTreeMap<_, _> =
        methodology_narrow_closure_candidate_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_narrow_closure_decision_rows:
        Vec<PaymentIntegrityMethodologyNarrowClosureDecisionRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_DECISIONS_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_DECISIONS_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_narrow_closure_decision_rows.len() != 2 {
        return Err(format!(
            "payment integrity methodology narrow closure decisions must contain 2 rows, got {}",
            methodology_narrow_closure_decision_rows.len()
        ));
    }
    let mut methodology_narrow_closure_decision_ids = BTreeSet::new();
    let mut methodology_narrow_closure_decision_candidate_ids = BTreeSet::new();
    for row in &methodology_narrow_closure_decision_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_narrow_closure_decision_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology narrow closure decision row {}",
                row.record_id
            ));
        }
        if !methodology_narrow_closure_decision_candidate_ids
            .insert(row.source_narrow_closure_candidate_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology narrow closure decision candidate reference {}",
                row.source_narrow_closure_candidate_record_id
            ));
        }
        let candidate = methodology_narrow_closure_candidate_by_id
            .get(&row.source_narrow_closure_candidate_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing methodology narrow closure-candidate row {}",
                    row.record_id, row.source_narrow_closure_candidate_record_id
                )
            })?;
        if row.agency_code != candidate.agency_code
            || row.program_or_activity != candidate.program_or_activity
            || row.priority_rank != candidate.priority_rank
        {
            return Err(format!(
                "{} must match agency, program, and rank from its narrow closure-candidate row",
                row.record_id
            ));
        }
    }
    if methodology_narrow_closure_decision_candidate_ids != methodology_narrow_closure_candidate_ids
    {
        return Err(
            "payment integrity methodology narrow closure decisions must exactly cover narrow closure candidates"
                .to_string(),
        );
    }

    let methodology_narrow_closure_decision_by_id: BTreeMap<_, _> =
        methodology_narrow_closure_decision_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_open_program_component_progress_rows:
        Vec<PaymentIntegrityMethodologyOpenProgramComponentProgressRecord> = read_jsonl(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_COMPONENT_PROGRESS_JSONL_PATH),
    )?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_COMPONENT_PROGRESS_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_open_program_component_progress_rows.len() != 2 {
        return Err(format!(
            "payment integrity methodology open-program component progress must contain 2 rows, got {}",
            methodology_open_program_component_progress_rows.len()
        ));
    }
    let mut methodology_open_program_component_progress_ids = BTreeSet::new();
    let mut methodology_open_program_component_progress_decision_ids = BTreeSet::new();
    for row in &methodology_open_program_component_progress_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_open_program_component_progress_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology open-program component progress row {}",
                row.record_id
            ));
        }
        if !methodology_open_program_component_progress_decision_ids
            .insert(row.source_narrow_closure_decision_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology open-program component progress decision reference {}",
                row.source_narrow_closure_decision_record_id
            ));
        }
        let open_status = methodology_open_program_status_by_id
            .get(&row.source_open_program_status_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing open-program status row {}",
                    row.record_id, row.source_open_program_status_record_id
                )
            })?;
        let narrow_decision = methodology_narrow_closure_decision_by_id
            .get(&row.source_narrow_closure_decision_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing narrow closure-decision row {}",
                    row.record_id, row.source_narrow_closure_decision_record_id
                )
            })?;
        if row.agency_code != open_status.agency_code
            || row.program_or_activity != open_status.program_or_activity
            || row.agency_code != narrow_decision.agency_code
            || row.program_or_activity != narrow_decision.program_or_activity
        {
            return Err(format!(
                "{} must match agency and program from both source rows",
                row.record_id
            ));
        }
        if row.closed_field_count_after_component_decision != open_status.closed_field_count
            || row.open_field_count_after_component_decision != open_status.open_field_count
            || row.total_methodology_fields != open_status.total_methodology_fields
        {
            return Err(format!(
                "{} must keep field counts unchanged from the open-program status row",
                row.record_id
            ));
        }
    }
    if methodology_open_program_component_progress_decision_ids
        != methodology_narrow_closure_decision_ids
    {
        return Err(
            "payment integrity methodology open-program component progress must exactly cover narrow closure decisions"
                .to_string(),
        );
    }

    let methodology_open_program_component_progress_by_id: BTreeMap<_, _> =
        methodology_open_program_component_progress_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_open_program_component_progress_ids: BTreeSet<_> =
        methodology_open_program_component_progress_rows
            .iter()
            .map(|row| row.record_id.clone())
            .collect();
    let methodology_component_gate_requirement_rows:
        Vec<PaymentIntegrityMethodologyComponentGateRequirementRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_REQUIREMENTS_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_REQUIREMENTS_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_component_gate_requirement_rows.len() != 2 {
        return Err(format!(
            "payment integrity methodology component gate requirements must contain 2 rows, got {}",
            methodology_component_gate_requirement_rows.len()
        ));
    }
    let mut methodology_component_gate_requirement_ids = BTreeSet::new();
    let mut methodology_component_gate_requirement_progress_ids = BTreeSet::new();
    for row in &methodology_component_gate_requirement_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_component_gate_requirement_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology component gate requirement row {}",
                row.record_id
            ));
        }
        if !methodology_component_gate_requirement_progress_ids
            .insert(row.source_component_progress_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology component gate requirement progress reference {}",
                row.source_component_progress_record_id
            ));
        }
        let progress = methodology_open_program_component_progress_by_id
            .get(&row.source_component_progress_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing component-progress row {}",
                    row.record_id, row.source_component_progress_record_id
                )
            })?;
        if row.agency_code != progress.agency_code
            || row.program_or_activity != progress.program_or_activity
        {
            return Err(format!(
                "{} must match agency and program from its component-progress row",
                row.record_id
            ));
        }
    }
    if methodology_component_gate_requirement_progress_ids
        != methodology_open_program_component_progress_ids
    {
        return Err(
            "payment integrity methodology component gate requirements must exactly cover component-progress rows"
                .to_string(),
        );
    }

    let methodology_component_gate_requirement_by_id: BTreeMap<_, _> =
        methodology_component_gate_requirement_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_component_gate_requirement_ids: BTreeSet<_> =
        methodology_component_gate_requirement_rows
            .iter()
            .map(|row| row.record_id.clone())
            .collect();
    let methodology_component_gate_source_target_rows:
        Vec<PaymentIntegrityMethodologyComponentGateSourceTargetRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_TARGETS_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_TARGETS_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_component_gate_source_target_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology component gate source targets must contain 4 rows, got {}",
            methodology_component_gate_source_target_rows.len()
        ));
    }
    let mut methodology_component_gate_source_target_ids = BTreeSet::new();
    let mut methodology_component_gate_source_target_requirement_ids = BTreeSet::new();
    let mut methodology_component_gate_source_target_priorities_by_requirement: BTreeMap<
        String,
        BTreeSet<u8>,
    > = BTreeMap::new();
    for row in &methodology_component_gate_source_target_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_component_gate_source_target_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology component gate source target row {}",
                row.record_id
            ));
        }
        methodology_component_gate_source_target_requirement_ids
            .insert(row.source_component_gate_requirement_record_id.clone());
        let requirement = methodology_component_gate_requirement_by_id
            .get(&row.source_component_gate_requirement_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing component gate requirement row {}",
                    row.record_id, row.source_component_gate_requirement_record_id
                )
            })?;
        if row.agency_code != requirement.agency_code
            || row.program_or_activity != requirement.program_or_activity
        {
            return Err(format!(
                "{} must match agency and program from its component gate requirement row",
                row.record_id
            ));
        }
        let priorities = methodology_component_gate_source_target_priorities_by_requirement
            .entry(row.source_component_gate_requirement_record_id.clone())
            .or_default();
        if !priorities.insert(row.source_target_priority) {
            return Err(format!(
                "duplicate payment integrity methodology component gate source target priority {} for {}",
                row.source_target_priority, row.source_component_gate_requirement_record_id
            ));
        }
    }
    if methodology_component_gate_source_target_requirement_ids
        != methodology_component_gate_requirement_ids
    {
        return Err(
            "payment integrity methodology component gate source targets must cover every component gate requirement"
                .to_string(),
        );
    }
    let required_component_source_target_priorities = BTreeSet::from([1_u8, 2_u8]);
    for requirement_id in &methodology_component_gate_requirement_ids {
        if methodology_component_gate_source_target_priorities_by_requirement.get(requirement_id)
            != Some(&required_component_source_target_priorities)
        {
            return Err(format!(
                "payment integrity methodology component gate source targets must have priorities 1 and 2 for {requirement_id}"
            ));
        }
    }

    let methodology_component_gate_source_target_by_id: BTreeMap<_, _> =
        methodology_component_gate_source_target_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_component_gate_source_target_ids: BTreeSet<_> =
        methodology_component_gate_source_target_rows
            .iter()
            .map(|row| row.record_id.clone())
            .collect();
    let methodology_component_gate_source_query_rows:
        Vec<PaymentIntegrityMethodologyComponentGateSourceQueryRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERIES_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERIES_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_component_gate_source_query_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology component gate source queries must contain 4 rows, got {}",
            methodology_component_gate_source_query_rows.len()
        ));
    }
    let mut methodology_component_gate_source_query_ids = BTreeSet::new();
    let mut methodology_component_gate_source_query_target_ids = BTreeSet::new();
    for row in &methodology_component_gate_source_query_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_component_gate_source_query_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology component gate source query row {}",
                row.record_id
            ));
        }
        if !methodology_component_gate_source_query_target_ids
            .insert(row.source_component_gate_source_target_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology component gate source query target reference {}",
                row.source_component_gate_source_target_record_id
            ));
        }
        let target = methodology_component_gate_source_target_by_id
            .get(&row.source_component_gate_source_target_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing component gate source-target row {}",
                    row.record_id, row.source_component_gate_source_target_record_id
                )
            })?;
        if row.agency_code != target.agency_code
            || row.program_or_activity != target.program_or_activity
            || row.source_target_priority != target.source_target_priority
        {
            return Err(format!(
                "{} must match agency, program, and priority from its component gate source-target row",
                row.record_id
            ));
        }
    }
    if methodology_component_gate_source_query_target_ids
        != methodology_component_gate_source_target_ids
    {
        return Err(
            "payment integrity methodology component gate source queries must exactly cover component gate source targets"
                .to_string(),
        );
    }

    let methodology_component_gate_source_query_by_id: BTreeMap<_, _> =
        methodology_component_gate_source_query_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_component_gate_source_query_ids: BTreeSet<_> =
        methodology_component_gate_source_query_rows
            .iter()
            .map(|row| row.record_id.clone())
            .collect();
    let methodology_component_gate_source_query_run_rows:
        Vec<PaymentIntegrityMethodologyComponentGateSourceQueryRunRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERY_RUNS_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERY_RUNS_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_component_gate_source_query_run_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology component gate source query runs must contain 4 rows, got {}",
            methodology_component_gate_source_query_run_rows.len()
        ));
    }
    let mut methodology_component_gate_source_query_run_ids = BTreeSet::new();
    let mut methodology_component_gate_source_query_run_query_ids = BTreeSet::new();
    for row in &methodology_component_gate_source_query_run_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_component_gate_source_query_run_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology component gate source query run row {}",
                row.record_id
            ));
        }
        if !methodology_component_gate_source_query_run_query_ids
            .insert(row.source_component_gate_source_query_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology component gate source query run query reference {}",
                row.source_component_gate_source_query_record_id
            ));
        }
        let query = methodology_component_gate_source_query_by_id
            .get(&row.source_component_gate_source_query_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing component gate source-query row {}",
                    row.record_id, row.source_component_gate_source_query_record_id
                )
            })?;
        if row.agency_code != query.agency_code
            || row.program_or_activity != query.program_or_activity
            || row.source_target_priority != query.source_target_priority
            || row.planned_query_text != query.query_text
            || row.required_capture_fields != query.expected_evidence
        {
            return Err(format!(
                "{} must match agency, program, priority, query text, and expected evidence from its component gate source-query row",
                row.record_id
            ));
        }
    }
    if methodology_component_gate_source_query_run_query_ids
        != methodology_component_gate_source_query_ids
    {
        return Err(
            "payment integrity methodology component gate source query runs must exactly cover component gate source queries"
                .to_string(),
        );
    }

    let methodology_component_gate_source_query_run_by_id: BTreeMap<_, _> =
        methodology_component_gate_source_query_run_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_component_gate_source_query_run_ids: BTreeSet<_> =
        methodology_component_gate_source_query_run_rows
            .iter()
            .map(|row| row.record_id.clone())
            .collect();
    let methodology_component_gate_source_capture_rows:
        Vec<PaymentIntegrityMethodologyComponentGateSourceCaptureRecord> = read_jsonl(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURES_JSONL_PATH,
    ))?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURES_JSONL_PATH} row failed to parse: {err}"
            )
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if methodology_component_gate_source_capture_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology component gate source captures must contain 4 rows, got {}",
            methodology_component_gate_source_capture_rows.len()
        ));
    }
    let mut methodology_component_gate_source_capture_ids = BTreeSet::new();
    let mut methodology_component_gate_source_capture_run_ids = BTreeSet::new();
    for row in &methodology_component_gate_source_capture_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_component_gate_source_capture_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology component gate source capture row {}",
                row.record_id
            ));
        }
        if !methodology_component_gate_source_capture_run_ids
            .insert(row.source_component_gate_source_query_run_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology component gate source capture query-run reference {}",
                row.source_component_gate_source_query_run_record_id
            ));
        }
        let query_run = methodology_component_gate_source_query_run_by_id
            .get(&row.source_component_gate_source_query_run_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing component gate source-query-run row {}",
                    row.record_id, row.source_component_gate_source_query_run_record_id
                )
            })?;
        if row.agency_code != query_run.agency_code
            || row.program_or_activity != query_run.program_or_activity
            || row.source_target_priority != query_run.source_target_priority
        {
            return Err(format!(
                "{} must match agency, program, and priority from its component gate source-query-run row",
                row.record_id
            ));
        }
    }
    if methodology_component_gate_source_capture_run_ids
        != methodology_component_gate_source_query_run_ids
    {
        return Err(
            "payment integrity methodology component gate source captures must exactly cover component gate source query runs"
                .to_string(),
        );
    }

    let methodology_component_gate_source_capture_by_id: BTreeMap<_, _> =
        methodology_component_gate_source_capture_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_component_gate_source_capture_ids: BTreeSet<_> =
        methodology_component_gate_source_capture_rows
            .iter()
            .map(|row| row.record_id.clone())
            .collect();
    let methodology_component_gate_source_capture_rollup_rows:
        Vec<PaymentIntegrityMethodologyComponentGateSourceCaptureRollupRecord> =
        read_jsonl(root.join(
            PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURE_ROLLUPS_JSONL_PATH,
        ))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row).map_err(|err| {
                format!(
                    "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURE_ROLLUPS_JSONL_PATH} row failed to parse: {err}"
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if methodology_component_gate_source_capture_rollup_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology component gate source capture rollups must contain 4 rows, got {}",
            methodology_component_gate_source_capture_rollup_rows.len()
        ));
    }
    let mut methodology_component_gate_source_capture_rollup_ids = BTreeSet::new();
    let mut methodology_component_gate_source_capture_rollup_capture_ids = BTreeSet::new();
    for row in &methodology_component_gate_source_capture_rollup_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_component_gate_source_capture_rollup_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology component gate source capture rollup row {}",
                row.record_id
            ));
        }
        if !methodology_component_gate_source_capture_rollup_capture_ids
            .insert(row.source_component_gate_source_capture_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology component gate source capture rollup capture reference {}",
                row.source_component_gate_source_capture_record_id
            ));
        }
        let capture = methodology_component_gate_source_capture_by_id
            .get(&row.source_component_gate_source_capture_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing component gate source-capture row {}",
                    row.record_id, row.source_component_gate_source_capture_record_id
                )
            })?;
        if row.agency_code != capture.agency_code
            || row.program_or_activity != capture.program_or_activity
            || row.source_target_priority != capture.source_target_priority
        {
            return Err(format!(
                "{} must match agency, program, and priority from its component gate source-capture row",
                row.record_id
            ));
        }
    }
    if methodology_component_gate_source_capture_rollup_capture_ids
        != methodology_component_gate_source_capture_ids
    {
        return Err(
            "payment integrity methodology component gate source capture rollups must exactly cover component gate source captures"
                .to_string(),
        );
    }

    let methodology_component_gate_source_capture_rollup_by_id: BTreeMap<_, _> =
        methodology_component_gate_source_capture_rollup_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_component_gate_source_capture_rollup_ids: BTreeSet<_> =
        methodology_component_gate_source_capture_rollup_rows
            .iter()
            .map(|row| row.record_id.clone())
            .collect();
    let methodology_component_gate_boundary_decision_rows:
        Vec<PaymentIntegrityMethodologyComponentGateBoundaryDecisionRecord> =
        read_jsonl(root.join(
            PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_DECISIONS_JSONL_PATH,
        ))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row).map_err(|err| {
                format!(
                    "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_DECISIONS_JSONL_PATH} row failed to parse: {err}"
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if methodology_component_gate_boundary_decision_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology component gate boundary decisions must contain 4 rows, got {}",
            methodology_component_gate_boundary_decision_rows.len()
        ));
    }
    let mut methodology_component_gate_boundary_decision_ids = BTreeSet::new();
    let mut methodology_component_gate_boundary_decision_rollup_ids = BTreeSet::new();
    for row in &methodology_component_gate_boundary_decision_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_component_gate_boundary_decision_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology component gate boundary decision row {}",
                row.record_id
            ));
        }
        if !methodology_component_gate_boundary_decision_rollup_ids.insert(
            row.source_component_gate_source_capture_rollup_record_id
                .clone(),
        ) {
            return Err(format!(
                "duplicate payment integrity methodology component gate boundary decision rollup reference {}",
                row.source_component_gate_source_capture_rollup_record_id
            ));
        }
        let rollup = methodology_component_gate_source_capture_rollup_by_id
            .get(&row.source_component_gate_source_capture_rollup_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing component gate source-capture-rollup row {}",
                    row.record_id, row.source_component_gate_source_capture_rollup_record_id
                )
            })?;
        if row.agency_code != rollup.agency_code
            || row.program_or_activity != rollup.program_or_activity
            || row.source_target_priority != rollup.source_target_priority
        {
            return Err(format!(
                "{} must match agency, program, and priority from its component gate source-capture-rollup row",
                row.record_id
            ));
        }
    }
    if methodology_component_gate_boundary_decision_rollup_ids
        != methodology_component_gate_source_capture_rollup_ids
    {
        return Err(
            "payment integrity methodology component gate boundary decisions must exactly cover component gate source capture rollups"
                .to_string(),
        );
    }

    let methodology_component_gate_boundary_decision_by_id: BTreeMap<_, _> =
        methodology_component_gate_boundary_decision_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_component_gate_boundary_decision_ids: BTreeSet<_> =
        methodology_component_gate_boundary_decision_rows
            .iter()
            .map(|row| row.record_id.clone())
            .collect();
    let methodology_component_gate_boundary_readiness_rows:
        Vec<PaymentIntegrityMethodologyComponentGateBoundaryReadinessRecord> =
        read_jsonl(root.join(
            PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_READINESS_JSONL_PATH,
        ))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row).map_err(|err| {
                format!(
                    "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_READINESS_JSONL_PATH} row failed to parse: {err}"
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if methodology_component_gate_boundary_readiness_rows.len() != 4 {
        return Err(format!(
            "payment integrity methodology component gate boundary readiness must contain 4 rows, got {}",
            methodology_component_gate_boundary_readiness_rows.len()
        ));
    }
    let mut methodology_component_gate_boundary_readiness_ids = BTreeSet::new();
    let mut methodology_component_gate_boundary_readiness_decision_ids = BTreeSet::new();
    for row in &methodology_component_gate_boundary_readiness_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_component_gate_boundary_readiness_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology component gate boundary readiness row {}",
                row.record_id
            ));
        }
        if !methodology_component_gate_boundary_readiness_decision_ids.insert(
            row.source_component_gate_boundary_decision_record_id
                .clone(),
        ) {
            return Err(format!(
                "duplicate payment integrity methodology component gate boundary readiness decision reference {}",
                row.source_component_gate_boundary_decision_record_id
            ));
        }
        let decision = methodology_component_gate_boundary_decision_by_id
            .get(&row.source_component_gate_boundary_decision_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing component gate boundary-decision row {}",
                    row.record_id, row.source_component_gate_boundary_decision_record_id
                )
            })?;
        if row.agency_code != decision.agency_code
            || row.program_or_activity != decision.program_or_activity
            || row.source_target_priority != decision.source_target_priority
        {
            return Err(format!(
                "{} must match agency, program, and priority from its component gate boundary-decision row",
                row.record_id
            ));
        }
    }
    if methodology_component_gate_boundary_readiness_decision_ids
        != methodology_component_gate_boundary_decision_ids
    {
        return Err(
            "payment integrity methodology component gate boundary readiness must exactly cover component gate boundary decisions"
                .to_string(),
        );
    }

    let methodology_component_gate_boundary_readiness_by_id: BTreeMap<_, _> =
        methodology_component_gate_boundary_readiness_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let narrow_ready_component_gate_boundary_readiness_ids: BTreeSet<_> =
        methodology_component_gate_boundary_readiness_rows
            .iter()
            .filter(|row| row.boundary_readiness_status == "narrow_internal_readiness_candidate")
            .map(|row| row.record_id.clone())
            .collect();
    let methodology_component_gate_narrow_candidate_rows:
        Vec<PaymentIntegrityMethodologyComponentGateNarrowCandidateRecord> =
        read_jsonl(root.join(
            PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_CANDIDATES_JSONL_PATH,
        ))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row).map_err(|err| {
                format!(
                    "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_CANDIDATES_JSONL_PATH} row failed to parse: {err}"
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if methodology_component_gate_narrow_candidate_rows.len() != 1 {
        return Err(format!(
            "payment integrity methodology component gate narrow candidates must contain 1 row, got {}",
            methodology_component_gate_narrow_candidate_rows.len()
        ));
    }
    let mut methodology_component_gate_narrow_candidate_ids = BTreeSet::new();
    let mut methodology_component_gate_narrow_candidate_readiness_ids = BTreeSet::new();
    for row in &methodology_component_gate_narrow_candidate_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_component_gate_narrow_candidate_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology component gate narrow candidate row {}",
                row.record_id
            ));
        }
        if !methodology_component_gate_narrow_candidate_readiness_ids.insert(
            row.source_component_gate_boundary_readiness_record_id
                .clone(),
        ) {
            return Err(format!(
                "duplicate payment integrity methodology component gate narrow candidate readiness reference {}",
                row.source_component_gate_boundary_readiness_record_id
            ));
        }
        let readiness = methodology_component_gate_boundary_readiness_by_id
            .get(&row.source_component_gate_boundary_readiness_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing component gate boundary-readiness row {}",
                    row.record_id, row.source_component_gate_boundary_readiness_record_id
                )
            })?;
        if readiness.boundary_readiness_status != "narrow_internal_readiness_candidate" {
            return Err(format!(
                "{} must reference a narrow_internal_readiness_candidate readiness row",
                row.record_id
            ));
        }
        if row.agency_code != readiness.agency_code
            || row.program_or_activity != readiness.program_or_activity
            || row.source_target_priority != readiness.source_target_priority
        {
            return Err(format!(
                "{} must match agency, program, and priority from its component gate boundary-readiness row",
                row.record_id
            ));
        }
    }
    if methodology_component_gate_narrow_candidate_readiness_ids
        != narrow_ready_component_gate_boundary_readiness_ids
    {
        return Err(
            "payment integrity methodology component gate narrow candidates must exactly cover narrow-ready boundary-readiness rows"
                .to_string(),
        );
    }

    let methodology_component_gate_narrow_candidate_by_id: BTreeMap<_, _> =
        methodology_component_gate_narrow_candidate_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_component_gate_narrow_decision_rows:
        Vec<PaymentIntegrityMethodologyComponentGateNarrowDecisionRecord> =
        read_jsonl(root.join(
            PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_DECISIONS_JSONL_PATH,
        ))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row).map_err(|err| {
                format!(
                    "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_DECISIONS_JSONL_PATH} row failed to parse: {err}"
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if methodology_component_gate_narrow_decision_rows.len() != 1 {
        return Err(format!(
            "payment integrity methodology component gate narrow decisions must contain 1 row, got {}",
            methodology_component_gate_narrow_decision_rows.len()
        ));
    }
    let mut methodology_component_gate_narrow_decision_ids = BTreeSet::new();
    let mut methodology_component_gate_narrow_decision_candidate_ids = BTreeSet::new();
    for row in &methodology_component_gate_narrow_decision_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_component_gate_narrow_decision_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology component gate narrow decision row {}",
                row.record_id
            ));
        }
        if !methodology_component_gate_narrow_decision_candidate_ids
            .insert(row.source_component_gate_narrow_candidate_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology component gate narrow decision candidate reference {}",
                row.source_component_gate_narrow_candidate_record_id
            ));
        }
        let candidate = methodology_component_gate_narrow_candidate_by_id
            .get(&row.source_component_gate_narrow_candidate_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing component gate narrow-candidate row {}",
                    row.record_id, row.source_component_gate_narrow_candidate_record_id
                )
            })?;
        if row.agency_code != candidate.agency_code
            || row.program_or_activity != candidate.program_or_activity
            || row.source_target_priority != candidate.source_target_priority
        {
            return Err(format!(
                "{} must match agency, program, and priority from its component gate narrow-candidate row",
                row.record_id
            ));
        }
    }
    if methodology_component_gate_narrow_decision_candidate_ids
        != methodology_component_gate_narrow_candidate_ids
    {
        return Err(
            "payment integrity methodology component gate narrow decisions must exactly cover component gate narrow candidates"
                .to_string(),
        );
    }

    let methodology_component_gate_narrow_decision_by_id: BTreeMap<_, _> =
        methodology_component_gate_narrow_decision_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_component_gate_progress_rows:
        Vec<PaymentIntegrityMethodologyComponentGateProgressRecord> = read_jsonl(root.join(
            PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_JSONL_PATH,
        ))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row).map_err(|err| {
                format!(
                    "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_JSONL_PATH} row failed to parse: {err}"
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if methodology_component_gate_progress_rows.len() != 1 {
        return Err(format!(
            "payment integrity methodology component gate progress must contain 1 row, got {}",
            methodology_component_gate_progress_rows.len()
        ));
    }
    let mut methodology_component_gate_progress_ids = BTreeSet::new();
    let mut methodology_component_gate_progress_decision_ids = BTreeSet::new();
    for row in &methodology_component_gate_progress_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_component_gate_progress_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology component gate progress row {}",
                row.record_id
            ));
        }
        if !methodology_component_gate_progress_decision_ids
            .insert(row.source_component_gate_narrow_decision_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology component gate progress decision reference {}",
                row.source_component_gate_narrow_decision_record_id
            ));
        }
        let open_status = methodology_open_program_status_by_id
            .get(&row.source_open_program_status_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing open-program status row {}",
                    row.record_id, row.source_open_program_status_record_id
                )
            })?;
        let decision = methodology_component_gate_narrow_decision_by_id
            .get(&row.source_component_gate_narrow_decision_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing component gate narrow-decision row {}",
                    row.record_id, row.source_component_gate_narrow_decision_record_id
                )
            })?;
        if row.agency_code != open_status.agency_code
            || row.program_or_activity != open_status.program_or_activity
            || row.agency_code != decision.agency_code
            || row.program_or_activity != decision.program_or_activity
        {
            return Err(format!(
                "{} must match agency and program from both source rows",
                row.record_id
            ));
        }
        if row.closed_field_count_after_component_decision != open_status.closed_field_count
            || row.open_field_count_after_component_decision != open_status.open_field_count
            || row.total_methodology_fields != open_status.total_methodology_fields
        {
            return Err(format!(
                "{} must keep field counts unchanged from the open-program status row",
                row.record_id
            ));
        }
    }
    if methodology_component_gate_progress_decision_ids
        != methodology_component_gate_narrow_decision_ids
    {
        return Err(
            "payment integrity methodology component gate progress must exactly cover component gate narrow decisions"
                .to_string(),
        );
    }

    let methodology_component_gate_progress_by_id: BTreeMap<_, _> =
        methodology_component_gate_progress_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_component_gate_progress_ids: BTreeSet<_> =
        methodology_component_gate_progress_rows
            .iter()
            .map(|row| row.record_id.clone())
            .collect();
    let methodology_component_gate_progress_requirement_rows:
        Vec<PaymentIntegrityMethodologyComponentGateProgressRequirementRecord> =
        read_jsonl(root.join(
            PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_REQUIREMENTS_JSONL_PATH,
        ))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row).map_err(|err| {
                format!(
                    "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_REQUIREMENTS_JSONL_PATH} row failed to parse: {err}"
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if methodology_component_gate_progress_requirement_rows.len() != 1 {
        return Err(format!(
            "payment integrity methodology component gate progress requirements must contain 1 row, got {}",
            methodology_component_gate_progress_requirement_rows.len()
        ));
    }
    let mut methodology_component_gate_progress_requirement_ids = BTreeSet::new();
    let mut methodology_component_gate_progress_requirement_progress_ids = BTreeSet::new();
    for row in &methodology_component_gate_progress_requirement_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_component_gate_progress_requirement_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology component gate progress requirement row {}",
                row.record_id
            ));
        }
        if !methodology_component_gate_progress_requirement_progress_ids
            .insert(row.source_component_gate_progress_record_id.clone())
        {
            return Err(format!(
                "duplicate payment integrity methodology component gate progress requirement progress reference {}",
                row.source_component_gate_progress_record_id
            ));
        }
        let progress = methodology_component_gate_progress_by_id
            .get(&row.source_component_gate_progress_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing component gate progress row {}",
                    row.record_id, row.source_component_gate_progress_record_id
                )
            })?;
        if row.agency_code != progress.agency_code
            || row.program_or_activity != progress.program_or_activity
        {
            return Err(format!(
                "{} must match agency and program from its component gate progress row",
                row.record_id
            ));
        }
    }
    if methodology_component_gate_progress_requirement_progress_ids
        != methodology_component_gate_progress_ids
    {
        return Err(
            "payment integrity methodology component gate progress requirements must exactly cover component gate progress rows"
                .to_string(),
        );
    }

    let methodology_component_gate_progress_requirement_by_id: BTreeMap<_, _> =
        methodology_component_gate_progress_requirement_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_component_gate_progress_requirement_ids: BTreeSet<_> =
        methodology_component_gate_progress_requirement_rows
            .iter()
            .map(|row| row.record_id.clone())
            .collect();
    let methodology_component_gate_progress_source_target_rows:
        Vec<PaymentIntegrityMethodologyComponentGateProgressSourceTargetRecord> =
        read_jsonl(root.join(
            PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_TARGETS_JSONL_PATH,
        ))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row).map_err(|err| {
                format!(
                    "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_TARGETS_JSONL_PATH} row failed to parse: {err}"
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if methodology_component_gate_progress_source_target_rows.len() != 2 {
        return Err(format!(
            "payment integrity methodology component gate progress source targets must contain 2 rows, got {}",
            methodology_component_gate_progress_source_target_rows.len()
        ));
    }
    let mut methodology_component_gate_progress_source_target_ids = BTreeSet::new();
    let mut methodology_component_gate_progress_source_target_requirement_ids = BTreeSet::new();
    let mut methodology_component_gate_progress_source_target_priorities_by_requirement: BTreeMap<
        String,
        BTreeSet<u8>,
    > = BTreeMap::new();
    for row in &methodology_component_gate_progress_source_target_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_component_gate_progress_source_target_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology component gate progress source target row {}",
                row.record_id
            ));
        }
        methodology_component_gate_progress_source_target_requirement_ids.insert(
            row.source_component_gate_progress_requirement_record_id
                .clone(),
        );
        let priorities =
            methodology_component_gate_progress_source_target_priorities_by_requirement
                .entry(
                    row.source_component_gate_progress_requirement_record_id
                        .clone(),
                )
                .or_default();
        if !priorities.insert(row.source_target_priority) {
            return Err(format!(
                "duplicate payment integrity methodology component gate progress source target priority {} for {}",
                row.source_target_priority,
                row.source_component_gate_progress_requirement_record_id
            ));
        }
        let requirement = methodology_component_gate_progress_requirement_by_id
            .get(&row.source_component_gate_progress_requirement_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing component gate progress requirement row {}",
                    row.record_id, row.source_component_gate_progress_requirement_record_id
                )
            })?;
        if row.agency_code != requirement.agency_code
            || row.program_or_activity != requirement.program_or_activity
        {
            return Err(format!(
                "{} must match agency and program from its component gate progress requirement row",
                row.record_id
            ));
        }
    }
    if methodology_component_gate_progress_source_target_requirement_ids
        != methodology_component_gate_progress_requirement_ids
    {
        return Err(
            "payment integrity methodology component gate progress source targets must cover every component gate progress requirement"
                .to_string(),
        );
    }
    for (requirement_id, priorities) in
        &methodology_component_gate_progress_source_target_priorities_by_requirement
    {
        if priorities != &BTreeSet::from([1, 2]) {
            return Err(format!(
                "payment integrity methodology component gate progress source targets for {requirement_id} must use priorities 1 and 2"
            ));
        }
    }

    let methodology_component_gate_progress_source_target_by_id: BTreeMap<_, _> =
        methodology_component_gate_progress_source_target_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_component_gate_progress_source_target_ids: BTreeSet<_> =
        methodology_component_gate_progress_source_target_rows
            .iter()
            .map(|row| row.record_id.clone())
            .collect();
    let methodology_component_gate_progress_source_query_rows:
        Vec<PaymentIntegrityMethodologyComponentGateProgressSourceQueryRecord> =
        read_jsonl(root.join(
            PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERIES_JSONL_PATH,
        ))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row).map_err(|err| {
                format!(
                    "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERIES_JSONL_PATH} row failed to parse: {err}"
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if methodology_component_gate_progress_source_query_rows.len() != 2 {
        return Err(format!(
            "payment integrity methodology component gate progress source queries must contain 2 rows, got {}",
            methodology_component_gate_progress_source_query_rows.len()
        ));
    }
    let mut methodology_component_gate_progress_source_query_ids = BTreeSet::new();
    let mut methodology_component_gate_progress_source_query_target_ids = BTreeSet::new();
    for row in &methodology_component_gate_progress_source_query_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_component_gate_progress_source_query_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology component gate progress source query row {}",
                row.record_id
            ));
        }
        if !methodology_component_gate_progress_source_query_target_ids.insert(
            row.source_component_gate_progress_source_target_record_id
                .clone(),
        ) {
            return Err(format!(
                "duplicate payment integrity methodology component gate progress source query target reference {}",
                row.source_component_gate_progress_source_target_record_id
            ));
        }
        let target = methodology_component_gate_progress_source_target_by_id
            .get(&row.source_component_gate_progress_source_target_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing component gate progress source-target row {}",
                    row.record_id, row.source_component_gate_progress_source_target_record_id
                )
            })?;
        if row.agency_code != target.agency_code
            || row.program_or_activity != target.program_or_activity
            || row.source_target_priority != target.source_target_priority
        {
            return Err(format!(
                "{} must match agency, program, and priority from its component gate progress source-target row",
                row.record_id
            ));
        }
    }
    if methodology_component_gate_progress_source_query_target_ids
        != methodology_component_gate_progress_source_target_ids
    {
        return Err(
            "payment integrity methodology component gate progress source queries must exactly cover component gate progress source targets"
                .to_string(),
        );
    }

    let methodology_component_gate_progress_source_query_by_id: BTreeMap<_, _> =
        methodology_component_gate_progress_source_query_rows
            .iter()
            .map(|row| (row.record_id.clone(), row))
            .collect();
    let methodology_component_gate_progress_source_query_ids: BTreeSet<_> =
        methodology_component_gate_progress_source_query_rows
            .iter()
            .map(|row| row.record_id.clone())
            .collect();
    let methodology_component_gate_progress_source_query_run_rows:
        Vec<PaymentIntegrityMethodologyComponentGateProgressSourceQueryRunRecord> =
        read_jsonl(root.join(
            PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERY_RUNS_JSONL_PATH,
        ))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row).map_err(|err| {
                format!(
                    "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERY_RUNS_JSONL_PATH} row failed to parse: {err}"
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if methodology_component_gate_progress_source_query_run_rows.len() != 2 {
        return Err(format!(
            "payment integrity methodology component gate progress source query runs must contain 2 rows, got {}",
            methodology_component_gate_progress_source_query_run_rows.len()
        ));
    }
    let mut methodology_component_gate_progress_source_query_run_ids = BTreeSet::new();
    let mut methodology_component_gate_progress_source_query_run_query_ids = BTreeSet::new();
    for row in &methodology_component_gate_progress_source_query_run_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !methodology_component_gate_progress_source_query_run_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity methodology component gate progress source query run row {}",
                row.record_id
            ));
        }
        if !methodology_component_gate_progress_source_query_run_query_ids.insert(
            row.source_component_gate_progress_source_query_record_id
                .clone(),
        ) {
            return Err(format!(
                "duplicate payment integrity methodology component gate progress source query run query reference {}",
                row.source_component_gate_progress_source_query_record_id
            ));
        }
        let query = methodology_component_gate_progress_source_query_by_id
            .get(&row.source_component_gate_progress_source_query_record_id)
            .ok_or_else(|| {
                format!(
                    "{} references missing component gate progress source-query row {}",
                    row.record_id, row.source_component_gate_progress_source_query_record_id
                )
            })?;
        if row.agency_code != query.agency_code
            || row.program_or_activity != query.program_or_activity
            || row.source_target_priority != query.source_target_priority
            || row.planned_query_text != query.query_text
            || row.required_capture_fields != query.expected_evidence
        {
            return Err(format!(
                "{} must match agency, program, priority, query text, and expected evidence from its component gate progress source-query row",
                row.record_id
            ));
        }
    }
    if methodology_component_gate_progress_source_query_run_query_ids
        != methodology_component_gate_progress_source_query_ids
    {
        return Err(
            "payment integrity methodology component gate progress source query runs must exactly cover component gate progress source queries"
                .to_string(),
        );
    }

    let next_program_selection_rows: Vec<PaymentIntegrityNextProgramSelectionRecord> =
        read_jsonl(root.join(PAYMENT_INTEGRITY_NEXT_PROGRAM_SELECTION_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
            format!(
                "{PAYMENT_INTEGRITY_NEXT_PROGRAM_SELECTION_JSONL_PATH} row failed to parse: {err}"
            )
        })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if next_program_selection_rows.len() != 3 {
        return Err(format!(
            "payment integrity next program selection must contain 3 rows, got {}",
            next_program_selection_rows.len()
        ));
    }
    let mut next_program_selection_ids = BTreeSet::new();
    let mut next_program_selection_keys = BTreeSet::new();
    for row in &next_program_selection_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !next_program_selection_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity next program selection row {}",
                row.record_id
            ));
        }
        next_program_selection_keys.insert(row.selected_program_key.clone());
        if !methodology_plan_rows
            .iter()
            .any(|plan| plan.program_or_activity == row.program_or_activity)
        {
            return Err(format!(
                "{} selects {}, but no methodology plan exists for that selected program",
                row.record_id, row.program_or_activity
            ));
        }
        if !row
            .official_source_urls
            .iter()
            .any(|url| url.contains("paymentaccuracy.gov"))
        {
            return Err(format!(
                "{} must include a PaymentAccuracy source URL",
                row.record_id
            ));
        }
        match row.agency_code.as_str() {
            "HHS" => {
                if !row
                    .official_source_urls
                    .iter()
                    .any(|url| url.contains("cms.gov"))
                {
                    return Err(format!("{} must include a CMS source URL", row.record_id));
                }
            }
            "VA" => {
                if !row
                    .official_source_urls
                    .iter()
                    .any(|url| url.contains("department.va.gov") || url.contains("www.va.gov"))
                {
                    return Err(format!("{} must include a VA source URL", row.record_id));
                }
            }
            "USDA" => {
                if !row
                    .official_source_urls
                    .iter()
                    .any(|url| url.contains("rma.usda.gov") || url.contains("usda.gov"))
                {
                    return Err(format!(
                        "{} must include a USDA/RMA source URL",
                        row.record_id
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "{} next-program selection agency is not supported: {}",
                    row.record_id, row.agency_code
                ));
            }
        }
        if row.starting_methodology_fields.len() < 6 {
            return Err(format!(
                "{} must list at least six starting methodology fields",
                row.record_id
            ));
        }
    }
    let expected_next_program_selection_keys = BTreeSet::from([
        "cms-medicaid".to_string(),
        "usda-federal-crop-insurance".to_string(),
        "va-pltss".to_string(),
    ]);
    if next_program_selection_keys != expected_next_program_selection_keys {
        return Err(
            "payment integrity next program selection rows must cover cms-medicaid, va-pltss, and usda-federal-crop-insurance"
                .to_string(),
        );
    }

    let payment_integrity_claims_rows: Vec<PaymentIntegrityClaimsTimelinessProbeRecord> =
        read_jsonl(root.join(PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!(
                        "{PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_JSONL_PATH} row failed to parse: {err}"
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if payment_integrity_claims_rows.len() != 4 {
        return Err(format!(
            "payment integrity claims-timeliness extract must contain 4 probe rows, got {}",
            payment_integrity_claims_rows.len()
        ));
    }
    let mut payment_integrity_claims_ids = BTreeSet::new();
    let mut claims_source_ids = BTreeSet::new();
    for row in &payment_integrity_claims_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !payment_integrity_claims_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate payment integrity claims-timeliness extract row {}",
                row.record_id
            ));
        }
        if !evidence_queue_ids.contains(&row.source_evidence_queue_record_id) {
            return Err(format!(
                "{} references missing evidence queue row {}",
                row.record_id, row.source_evidence_queue_record_id
            ));
        }
        if !source_ledger.contains(&format!("`{}`", row.source_id)) {
            return Err(format!(
                "{}: source_id {} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                row.record_id, row.source_id
            ));
        }
        claims_source_ids.insert(row.source_id.clone());
    }
    for required_source in ["SRC-SSA-PERFORMANCE", "SRC-VA-CLAIMS-DATA"] {
        if !claims_source_ids.contains(required_source) {
            return Err(format!(
                "payment integrity claims-timeliness extract must include {required_source}"
            ));
        }
    }

    let debt_maturity_rows: Vec<DebtMaturityRiskTreasuryProbeRecord> =
        read_jsonl(root.join(DEBT_MATURITY_RISK_FIRST_PASS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!("{DEBT_MATURITY_RISK_FIRST_PASS_JSONL_PATH} row failed to parse: {err}")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if debt_maturity_rows.len() != 9 {
        return Err(format!(
            "debt maturity risk first-pass extract must contain 9 Treasury probe rows, got {}",
            debt_maturity_rows.len()
        ));
    }
    let mut debt_maturity_ids = BTreeSet::new();
    let mut debt_source_ids = BTreeSet::new();
    let mut debt_stock_count = 0usize;
    let mut avg_rate_count = 0usize;
    for row in &debt_maturity_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !debt_maturity_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate debt maturity risk extract row {}",
                row.record_id
            ));
        }
        if !evidence_queue_ids.contains(&row.source_evidence_queue_record_id) {
            return Err(format!(
                "{} references missing evidence queue row {}",
                row.record_id, row.source_evidence_queue_record_id
            ));
        }
        if !source_ledger.contains(&format!("`{}`", row.source_id)) {
            return Err(format!(
                "{}: source_id {} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                row.record_id, row.source_id
            ));
        }
        debt_source_ids.insert(row.source_id.clone());
        match row.row_kind.as_str() {
            "debt_stock" => debt_stock_count += 1,
            "average_interest_rate" => avg_rate_count += 1,
            _ => {}
        }
    }
    for required_source in ["SRC-TREASURY-DEBT-PENNY", "SRC-TREASURY-AVG-INTEREST"] {
        if !debt_source_ids.contains(required_source) {
            return Err(format!(
                "debt maturity risk first-pass extract must include {required_source}"
            ));
        }
    }
    if debt_stock_count != 1 || avg_rate_count != 8 {
        return Err(format!(
            "debt maturity risk first-pass extract must include 1 debt-stock row and 8 average-rate rows, got {debt_stock_count} and {avg_rate_count}"
        ));
    }

    let debt_primary_balance_rows: Vec<DebtPrimaryBalanceFiscalProbeRecord> =
        read_jsonl(root.join(DEBT_PRIMARY_BALANCE_FIRST_PASS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!(
                        "{DEBT_PRIMARY_BALANCE_FIRST_PASS_JSONL_PATH} row failed to parse: {err}"
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if debt_primary_balance_rows.len() != 1 {
        return Err(format!(
            "debt primary balance first-pass extract must contain 1 fiscal-balance row, got {}",
            debt_primary_balance_rows.len()
        ));
    }
    let mut debt_primary_balance_ids = BTreeSet::new();
    for row in &debt_primary_balance_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !debt_primary_balance_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate debt primary balance extract row {}",
                row.record_id
            ));
        }
        if !evidence_queue_ids.contains(&row.source_evidence_queue_record_id) {
            return Err(format!(
                "{} references missing evidence queue row {}",
                row.record_id, row.source_evidence_queue_record_id
            ));
        }
        for source_id in &row.source_ids {
            if source_id.starts_with("SRC-") && !source_ledger.contains(&format!("`{source_id}`")) {
                return Err(format!(
                    "{}: source_id {source_id} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                    row.record_id
                ));
            }
        }
    }

    let disaster_declaration_rows: Vec<DisasterDeclarationProbeRecord> =
        read_jsonl(root.join(DISASTER_SUPPLEMENTAL_TRACKING_FIRST_PASS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
            format!(
                "{DISASTER_SUPPLEMENTAL_TRACKING_FIRST_PASS_JSONL_PATH} row failed to parse: {err}"
            )
        })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if disaster_declaration_rows.len() != 8 {
        return Err(format!(
            "disaster supplemental tracking first-pass extract must contain 8 FEMA declaration rows, got {}",
            disaster_declaration_rows.len()
        ));
    }
    let mut disaster_declaration_ids = BTreeSet::new();
    for row in &disaster_declaration_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !disaster_declaration_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate disaster supplemental tracking extract row {}",
                row.record_id
            ));
        }
        if !evidence_queue_ids.contains(&row.source_evidence_queue_record_id) {
            return Err(format!(
                "{} references missing evidence queue row {}",
                row.record_id, row.source_evidence_queue_record_id
            ));
        }
        if !source_ledger.contains(&format!("`{}`", row.source_id)) {
            return Err(format!(
                "{}: source_id {} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                row.record_id, row.source_id
            ));
        }
    }

    let disaster_mitigation_rows: Vec<DisasterMitigationProjectProbeRecord> =
        read_jsonl(root.join(DISASTER_MITIGATION_FIRST_PASS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!(
                        "{DISASTER_MITIGATION_FIRST_PASS_JSONL_PATH} row failed to parse: {err}"
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if disaster_mitigation_rows.len() != 5 {
        return Err(format!(
            "disaster mitigation first-pass extract must contain 5 FEMA HMA project rows, got {}",
            disaster_mitigation_rows.len()
        ));
    }
    let mut disaster_mitigation_ids = BTreeSet::new();
    for row in &disaster_mitigation_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !disaster_mitigation_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate disaster mitigation extract row {}",
                row.record_id
            ));
        }
        if !evidence_queue_ids.contains(&row.source_evidence_queue_record_id) {
            return Err(format!(
                "{} references missing evidence queue row {}",
                row.record_id, row.source_evidence_queue_record_id
            ));
        }
        if !source_ledger.contains(&format!("`{}`", row.source_id)) {
            return Err(format!(
                "{}: source_id {} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                row.record_id, row.source_id
            ));
        }
    }

    let defense_audit_control_rows: Vec<DefenseAuditControlProbeRecord> =
        read_jsonl(root.join(DEFENSE_AUDIT_CONTROL_FIRST_PASS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!(
                        "{DEFENSE_AUDIT_CONTROL_FIRST_PASS_JSONL_PATH} row failed to parse: {err}"
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if defense_audit_control_rows.len() != 6 {
        return Err(format!(
            "defense audit-control first-pass extract must contain 6 DoD OIG rows, got {}",
            defense_audit_control_rows.len()
        ));
    }
    let mut defense_audit_control_ids = BTreeSet::new();
    for row in &defense_audit_control_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !defense_audit_control_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate defense audit-control extract row {}",
                row.record_id
            ));
        }
        if !evidence_queue_ids.contains(&row.source_evidence_queue_record_id) {
            return Err(format!(
                "{} references missing evidence queue row {}",
                row.record_id, row.source_evidence_queue_record_id
            ));
        }
        if !source_ledger.contains(&format!("`{}`", row.source_id)) {
            return Err(format!(
                "{}: source_id {} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                row.record_id, row.source_id
            ));
        }
    }

    let defense_procurement_control_rows: Vec<DefenseProcurementControlProbeRecord> =
        read_jsonl(root.join(DEFENSE_PROCUREMENT_CONTROL_FIRST_PASS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
            format!(
                "{DEFENSE_PROCUREMENT_CONTROL_FIRST_PASS_JSONL_PATH} row failed to parse: {err}"
            )
        })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if defense_procurement_control_rows.len() != 6 {
        return Err(format!(
            "defense procurement-control first-pass extract must contain 6 GAO weapon-systems rows, got {}",
            defense_procurement_control_rows.len()
        ));
    }
    let mut defense_procurement_control_ids = BTreeSet::new();
    for row in &defense_procurement_control_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !defense_procurement_control_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate defense procurement-control extract row {}",
                row.record_id
            ));
        }
        if !evidence_queue_ids.contains(&row.source_evidence_queue_record_id) {
            return Err(format!(
                "{} references missing evidence queue row {}",
                row.record_id, row.source_evidence_queue_record_id
            ));
        }
        if !source_ledger.contains(&format!("`{}`", row.source_id)) {
            return Err(format!(
                "{}: source_id {} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                row.record_id, row.source_id
            ));
        }
    }

    let health_price_rows: Vec<HealthPriceDisciplineProbeRecord> =
        read_jsonl(root.join(HEALTH_PRICE_DISCIPLINE_FIRST_PASS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!(
                        "{HEALTH_PRICE_DISCIPLINE_FIRST_PASS_JSONL_PATH} row failed to parse: {err}"
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if health_price_rows.len() != 6 {
        return Err(format!(
            "health price-discipline first-pass extract must contain 6 rows, got {}",
            health_price_rows.len()
        ));
    }
    let mut health_price_ids = BTreeSet::new();
    for row in &health_price_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !health_price_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate health price-discipline extract row {}",
                row.record_id
            ));
        }
        if !evidence_queue_ids.contains(&row.source_evidence_queue_record_id) {
            return Err(format!(
                "{} references missing evidence queue row {}",
                row.record_id, row.source_evidence_queue_record_id
            ));
        }
        for source_id in &row.source_ids {
            if source_id.starts_with("SRC-") && !source_ledger.contains(&format!("`{source_id}`")) {
                return Err(format!(
                    "{}: source_id {source_id} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                    row.record_id
                ));
            }
        }
    }

    let health_admin_rows: Vec<HealthAdminSimplificationProbeRecord> =
        read_jsonl(root.join(HEALTH_ADMIN_SIMPLIFICATION_FIRST_PASS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
            format!(
                "{HEALTH_ADMIN_SIMPLIFICATION_FIRST_PASS_JSONL_PATH} row failed to parse: {err}"
            )
        })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if health_admin_rows.len() != 6 {
        return Err(format!(
            "health administrative-simplification first-pass extract must contain 6 rows, got {}",
            health_admin_rows.len()
        ));
    }
    let mut health_admin_ids = BTreeSet::new();
    for row in &health_admin_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !health_admin_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate health administrative-simplification extract row {}",
                row.record_id
            ));
        }
        if !evidence_queue_ids.contains(&row.source_evidence_queue_record_id) {
            return Err(format!(
                "{} references missing evidence queue row {}",
                row.record_id, row.source_evidence_queue_record_id
            ));
        }
        for source_id in &row.source_ids {
            if source_id.starts_with("SRC-") && !source_ledger.contains(&format!("`{source_id}`")) {
                return Err(format!(
                    "{}: source_id {source_id} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                    row.record_id
                ));
            }
        }
    }

    let first_pass_rollup_rows: Vec<CostDownFirstPassRollupRecord> =
        read_jsonl(root.join(COST_DOWN_FIRST_PASS_ROLLUP_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!("{COST_DOWN_FIRST_PASS_ROLLUP_JSONL_PATH} row failed to parse: {err}")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if first_pass_rollup_rows.len() != 10 {
        return Err(format!(
            "cost-down first-pass rollup must contain 10 rows, got {}",
            first_pass_rollup_rows.len()
        ));
    }
    let mut first_pass_rollup_ids = BTreeSet::new();
    let mut first_pass_rollup_queue_ids = BTreeSet::new();
    for row in &first_pass_rollup_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !first_pass_rollup_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate cost-down first-pass rollup row {}",
                row.record_id
            ));
        }
        if !first_pass_rollup_queue_ids.insert(row.source_evidence_queue_record_id.clone()) {
            return Err(format!(
                "duplicate cost-down first-pass rollup queue reference {}",
                row.source_evidence_queue_record_id
            ));
        }
        if !evidence_queue_ids.contains(&row.source_evidence_queue_record_id) {
            return Err(format!(
                "{} references missing evidence queue row {}",
                row.record_id, row.source_evidence_queue_record_id
            ));
        }
        for artifact in &row.first_pass_artifacts {
            if !root.join(artifact).exists() {
                return Err(format!(
                    "{} references missing first-pass artifact {}",
                    row.record_id, artifact
                ));
            }
        }
    }
    for evidence_queue_id in &evidence_queue_ids {
        if !first_pass_rollup_queue_ids.contains(evidence_queue_id) {
            return Err(format!(
                "cost-down first-pass rollup is missing evidence queue row {evidence_queue_id}"
            ));
        }
    }

    let scoring_readiness_rows: Vec<CostDownScoringReadinessRecord> =
        read_jsonl(root.join(COST_DOWN_SCORING_READINESS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!("{COST_DOWN_SCORING_READINESS_JSONL_PATH} row failed to parse: {err}")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    if scoring_readiness_rows.len() != 10 {
        return Err(format!(
            "cost-down scoring readiness must contain 10 rows, got {}",
            scoring_readiness_rows.len()
        ));
    }
    let rollup_ids: BTreeSet<_> = first_pass_rollup_rows
        .iter()
        .map(|row| row.record_id.clone())
        .collect();
    let mut readiness_ids = BTreeSet::new();
    let mut readiness_rollup_ids = BTreeSet::new();
    let mut readiness_ranks = BTreeSet::new();
    for row in &scoring_readiness_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !readiness_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate cost-down scoring readiness row {}",
                row.record_id
            ));
        }
        if !readiness_ranks.insert(row.prioritization_rank) {
            return Err(format!(
                "duplicate cost-down scoring readiness rank {}",
                row.prioritization_rank
            ));
        }
        if !rollup_ids.contains(&row.source_rollup_record_id) {
            return Err(format!(
                "{} references missing rollup row {}",
                row.record_id, row.source_rollup_record_id
            ));
        }
        if !readiness_rollup_ids.insert(row.source_rollup_record_id.clone()) {
            return Err(format!(
                "duplicate cost-down scoring readiness rollup reference {}",
                row.source_rollup_record_id
            ));
        }
        if !evidence_queue_ids.contains(&row.source_evidence_queue_record_id) {
            return Err(format!(
                "{} references missing evidence queue row {}",
                row.record_id, row.source_evidence_queue_record_id
            ));
        }
    }
    for rollup_id in &rollup_ids {
        if !readiness_rollup_ids.contains(rollup_id) {
            return Err(format!(
                "cost-down scoring readiness is missing rollup row {rollup_id}"
            ));
        }
    }

    for path in [
        EFFICIENCY_PRESSURE_README_PATH,
        EFFICIENCY_PRESSURE_SCHEMA_PATH,
        COST_DOWN_BACKLOG_SCHEMA_PATH,
        COST_DOWN_SOURCE_PACKETS_SCHEMA_PATH,
        COST_DOWN_EVIDENCE_QUEUE_SCHEMA_PATH,
        COST_DOWN_FIRST_PASS_ROLLUP_SCHEMA_PATH,
        COST_DOWN_SCORING_READINESS_SCHEMA_PATH,
        PAYMENT_INTEGRITY_ELIGIBILITY_FIRST_PASS_SCHEMA_PATH,
        PAYMENT_INTEGRITY_SCORECARDS_Q4_2025_SCHEMA_PATH,
        PAYMENT_INTEGRITY_PROGRAM_REVIEW_GATE_SCHEMA_PATH,
        PAYMENT_INTEGRITY_PROGRAM_REVIEW_TASKS_SCHEMA_PATH,
        PAYMENT_INTEGRITY_PROGRAM_REVIEW_STATUS_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_PLANS_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_FIELDS_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_TARGETS_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_QUERIES_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_QUERY_RUNS_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_RESULTS_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_RESULT_REVIEW_READINESS_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_FIELD_REVIEWS_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_GAP_FOLLOWUPS_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_GAP_SOURCE_CAPTURES_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_CAPTURE_ROLLUP_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_READINESS_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_DECISIONS_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_SOURCE_GAPS_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_COVERAGE_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_SCORING_GATE_SCHEMA_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_PROGRAM_ROLLUP_SCHEMA_PATH,
        PAYMENT_INTEGRITY_NEXT_PROGRAM_SELECTION_SCHEMA_PATH,
        PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_SCHEMA_PATH,
        DEBT_MATURITY_RISK_FIRST_PASS_SCHEMA_PATH,
        DEBT_PRIMARY_BALANCE_FIRST_PASS_SCHEMA_PATH,
        DISASTER_SUPPLEMENTAL_TRACKING_FIRST_PASS_SCHEMA_PATH,
        DISASTER_MITIGATION_FIRST_PASS_SCHEMA_PATH,
        DEFENSE_AUDIT_CONTROL_FIRST_PASS_SCHEMA_PATH,
        DEFENSE_PROCUREMENT_CONTROL_FIRST_PASS_SCHEMA_PATH,
        HEALTH_PRICE_DISCIPLINE_FIRST_PASS_SCHEMA_PATH,
        HEALTH_ADMIN_SIMPLIFICATION_FIRST_PASS_SCHEMA_PATH,
        EFFICIENCY_PRESSURE_RESEARCH_PATH,
        COST_DOWN_BACKLOG_READER_PATH,
        COST_DOWN_EVIDENCE_QUEUE_READER_PATH,
        COST_DOWN_FIRST_PASS_ROLLUP_READER_PATH,
        COST_DOWN_SCORING_READINESS_READER_PATH,
        PAYMENT_INTEGRITY_FIRST_PASS_READER_PATH,
        PAYMENT_INTEGRITY_SCORECARD_READER_PATH,
        PAYMENT_INTEGRITY_PROGRAM_REVIEW_GATE_READER_PATH,
        PAYMENT_INTEGRITY_PROGRAM_REVIEW_TASKS_READER_PATH,
        PAYMENT_INTEGRITY_PROGRAM_REVIEW_STATUS_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_PLANS_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_FIELDS_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_TARGETS_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_QUERIES_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_QUERY_RUNS_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_RESULTS_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_RESULT_REVIEW_READINESS_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_FIELD_REVIEWS_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_GAP_FOLLOWUPS_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_GAP_SOURCE_CAPTURES_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_CAPTURE_ROLLUP_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_READINESS_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_DECISIONS_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_SOURCE_GAPS_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_COVERAGE_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_SCORING_GATE_READER_PATH,
        PAYMENT_INTEGRITY_METHODOLOGY_PROGRAM_ROLLUP_READER_PATH,
        PAYMENT_INTEGRITY_NEXT_PROGRAM_SELECTION_READER_PATH,
        PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_READER_PATH,
        DEBT_MATURITY_RISK_EXTRACT_READER_PATH,
        DEBT_PRIMARY_BALANCE_EXTRACT_READER_PATH,
        DISASTER_SUPPLEMENTAL_TRACKING_EXTRACT_READER_PATH,
        DISASTER_MITIGATION_EXTRACT_READER_PATH,
        DEFENSE_AUDIT_CONTROL_EXTRACT_READER_PATH,
        DEFENSE_PROCUREMENT_CONTROL_EXTRACT_READER_PATH,
        HEALTH_PRICE_DISCIPLINE_EXTRACT_READER_PATH,
        HEALTH_ADMIN_SIMPLIFICATION_EXTRACT_READER_PATH,
        HEALTH_PRICE_DISCIPLINE_SOURCE_PACKET_READER_PATH,
        HEALTH_ADMIN_SIMPLIFICATION_SOURCE_PACKET_READER_PATH,
        DEBT_PRIMARY_BALANCE_SOURCE_PACKET_READER_PATH,
        DEBT_MATURITY_RISK_SOURCE_PACKET_READER_PATH,
        DEFENSE_PROCUREMENT_CONTROL_SOURCE_PACKET_READER_PATH,
        DEFENSE_AUDIT_CONTROL_SOURCE_PACKET_READER_PATH,
        DISASTER_MITIGATION_SOURCE_PACKET_READER_PATH,
        DISASTER_SUPPLEMENTAL_TRACKING_SOURCE_PACKET_READER_PATH,
        PAYMENT_INTEGRITY_ELIGIBILITY_SOURCE_PACKET_READER_PATH,
        PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_SOURCE_PACKET_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing efficiency pressure support artifact: {path}"
            ));
        }
    }

    let reader = fs::read_to_string(root.join("docs/reading/where-federal-money-goes.md"))
        .map_err(|err| format!("failed to read docs/reading/where-federal-money-goes.md: {err}"))?;
    if !reader.contains(EFFICIENCY_PRESSURE_JSONL_PATH) {
        return Err(format!(
            "where-federal-money-goes.md must cite {EFFICIENCY_PRESSURE_JSONL_PATH}"
        ));
    }
    let schema = fs::read_to_string(root.join(EFFICIENCY_PRESSURE_SCHEMA_PATH))
        .map_err(|err| format!("failed to read {EFFICIENCY_PRESSURE_SCHEMA_PATH}: {err}"))?;
    if !schema.contains("They do not prove waste") {
        return Err("efficiency pressure schema must retain public-use boundary".to_string());
    }
    let backlog_reader = fs::read_to_string(root.join(COST_DOWN_BACKLOG_READER_PATH))
        .map_err(|err| format!("failed to read {COST_DOWN_BACKLOG_READER_PATH}: {err}"))?;
    if !backlog_reader.contains(COST_DOWN_BACKLOG_JSONL_PATH) {
        return Err(format!(
            "{COST_DOWN_BACKLOG_READER_PATH} must cite {COST_DOWN_BACKLOG_JSONL_PATH}"
        ));
    }
    let evidence_queue_reader = fs::read_to_string(root.join(COST_DOWN_EVIDENCE_QUEUE_READER_PATH))
        .map_err(|err| format!("failed to read {COST_DOWN_EVIDENCE_QUEUE_READER_PATH}: {err}"))?;
    if !evidence_queue_reader.contains(COST_DOWN_EVIDENCE_QUEUE_JSONL_PATH) {
        return Err(format!(
            "{COST_DOWN_EVIDENCE_QUEUE_READER_PATH} must cite {COST_DOWN_EVIDENCE_QUEUE_JSONL_PATH}"
        ));
    }
    let first_pass_rollup_reader =
        fs::read_to_string(root.join(COST_DOWN_FIRST_PASS_ROLLUP_READER_PATH)).map_err(|err| {
            format!("failed to read {COST_DOWN_FIRST_PASS_ROLLUP_READER_PATH}: {err}")
        })?;
    if !first_pass_rollup_reader.contains(COST_DOWN_FIRST_PASS_ROLLUP_JSONL_PATH) {
        return Err(format!(
            "{COST_DOWN_FIRST_PASS_ROLLUP_READER_PATH} must cite {COST_DOWN_FIRST_PASS_ROLLUP_JSONL_PATH}"
        ));
    }
    let scoring_readiness_reader =
        fs::read_to_string(root.join(COST_DOWN_SCORING_READINESS_READER_PATH)).map_err(|err| {
            format!("failed to read {COST_DOWN_SCORING_READINESS_READER_PATH}: {err}")
        })?;
    if !scoring_readiness_reader.contains(COST_DOWN_SCORING_READINESS_JSONL_PATH) {
        return Err(format!(
            "{COST_DOWN_SCORING_READINESS_READER_PATH} must cite {COST_DOWN_SCORING_READINESS_JSONL_PATH}"
        ));
    }
    let payment_integrity_first_pass_reader =
        fs::read_to_string(root.join(PAYMENT_INTEGRITY_FIRST_PASS_READER_PATH)).map_err(|err| {
            format!("failed to read {PAYMENT_INTEGRITY_FIRST_PASS_READER_PATH}: {err}")
        })?;
    if !payment_integrity_first_pass_reader
        .contains(PAYMENT_INTEGRITY_ELIGIBILITY_FIRST_PASS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_FIRST_PASS_READER_PATH} must cite {PAYMENT_INTEGRITY_ELIGIBILITY_FIRST_PASS_JSONL_PATH}"
        ));
    }
    let payment_integrity_scorecard_reader =
        fs::read_to_string(root.join(PAYMENT_INTEGRITY_SCORECARD_READER_PATH)).map_err(|err| {
            format!("failed to read {PAYMENT_INTEGRITY_SCORECARD_READER_PATH}: {err}")
        })?;
    if !payment_integrity_scorecard_reader.contains(PAYMENT_INTEGRITY_SCORECARDS_Q4_2025_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_SCORECARD_READER_PATH} must cite {PAYMENT_INTEGRITY_SCORECARDS_Q4_2025_JSONL_PATH}"
        ));
    }
    let payment_integrity_program_gate_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_PROGRAM_REVIEW_GATE_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {PAYMENT_INTEGRITY_PROGRAM_REVIEW_GATE_READER_PATH}: {err}")
    })?;
    if !payment_integrity_program_gate_reader
        .contains(PAYMENT_INTEGRITY_PROGRAM_REVIEW_GATE_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_PROGRAM_REVIEW_GATE_READER_PATH} must cite {PAYMENT_INTEGRITY_PROGRAM_REVIEW_GATE_JSONL_PATH}"
        ));
    }
    let payment_integrity_program_task_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_PROGRAM_REVIEW_TASKS_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {PAYMENT_INTEGRITY_PROGRAM_REVIEW_TASKS_READER_PATH}: {err}")
    })?;
    if !payment_integrity_program_task_reader
        .contains(PAYMENT_INTEGRITY_PROGRAM_REVIEW_TASKS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_PROGRAM_REVIEW_TASKS_READER_PATH} must cite {PAYMENT_INTEGRITY_PROGRAM_REVIEW_TASKS_JSONL_PATH}"
        ));
    }
    let payment_integrity_program_status_reader =
        fs::read_to_string(root.join(PAYMENT_INTEGRITY_PROGRAM_REVIEW_STATUS_READER_PATH))
            .map_err(|err| {
                format!(
                    "failed to read {PAYMENT_INTEGRITY_PROGRAM_REVIEW_STATUS_READER_PATH}: {err}"
                )
            })?;
    if !payment_integrity_program_status_reader
        .contains(PAYMENT_INTEGRITY_PROGRAM_REVIEW_STATUS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_PROGRAM_REVIEW_STATUS_READER_PATH} must cite {PAYMENT_INTEGRITY_PROGRAM_REVIEW_STATUS_JSONL_PATH}"
        ));
    }
    let methodology_plan_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_PLANS_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {PAYMENT_INTEGRITY_METHODOLOGY_PLANS_READER_PATH}: {err}")
    })?;
    if !methodology_plan_reader.contains(PAYMENT_INTEGRITY_METHODOLOGY_PLANS_JSONL_PATH) {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_PLANS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_PLANS_JSONL_PATH}"
        ));
    }
    let methodology_field_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_FIELDS_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {PAYMENT_INTEGRITY_METHODOLOGY_FIELDS_READER_PATH}: {err}")
    })?;
    if !methodology_field_reader.contains(PAYMENT_INTEGRITY_METHODOLOGY_FIELDS_JSONL_PATH) {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_FIELDS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_FIELDS_JSONL_PATH}"
        ));
    }
    let methodology_source_target_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_TARGETS_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_TARGETS_READER_PATH}: {err}")
    })?;
    if !methodology_source_target_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_TARGETS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_TARGETS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_TARGETS_JSONL_PATH}"
        ));
    }
    let methodology_query_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_QUERIES_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {PAYMENT_INTEGRITY_METHODOLOGY_QUERIES_READER_PATH}: {err}")
    })?;
    if !methodology_query_reader.contains(PAYMENT_INTEGRITY_METHODOLOGY_QUERIES_JSONL_PATH) {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_QUERIES_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_QUERIES_JSONL_PATH}"
        ));
    }
    let methodology_query_run_reader =
        fs::read_to_string(root.join(PAYMENT_INTEGRITY_METHODOLOGY_QUERY_RUNS_READER_PATH))
            .map_err(|err| {
                format!(
                    "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_QUERY_RUNS_READER_PATH}: {err}"
                )
            })?;
    if !methodology_query_run_reader.contains(PAYMENT_INTEGRITY_METHODOLOGY_QUERY_RUNS_JSONL_PATH) {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_QUERY_RUNS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_QUERY_RUNS_JSONL_PATH}"
        ));
    }
    let methodology_result_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_RESULTS_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {PAYMENT_INTEGRITY_METHODOLOGY_RESULTS_READER_PATH}: {err}")
    })?;
    if !methodology_result_reader.contains(PAYMENT_INTEGRITY_METHODOLOGY_RESULTS_JSONL_PATH) {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_RESULTS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_RESULTS_JSONL_PATH}"
        ));
    }
    let methodology_result_review_readiness_reader = fs::read_to_string(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_RESULT_REVIEW_READINESS_READER_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_RESULT_REVIEW_READINESS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_result_review_readiness_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_RESULT_REVIEW_READINESS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_RESULT_REVIEW_READINESS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_RESULT_REVIEW_READINESS_JSONL_PATH}"
        ));
    }
    let methodology_field_review_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_FIELD_REVIEWS_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {PAYMENT_INTEGRITY_METHODOLOGY_FIELD_REVIEWS_READER_PATH}: {err}")
    })?;
    if !methodology_field_review_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_FIELD_REVIEWS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_FIELD_REVIEWS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_FIELD_REVIEWS_JSONL_PATH}"
        ));
    }
    let methodology_gap_followup_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_GAP_FOLLOWUPS_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {PAYMENT_INTEGRITY_METHODOLOGY_GAP_FOLLOWUPS_READER_PATH}: {err}")
    })?;
    if !methodology_gap_followup_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_GAP_FOLLOWUPS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_GAP_FOLLOWUPS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_GAP_FOLLOWUPS_JSONL_PATH}"
        ));
    }
    let methodology_gap_source_capture_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_GAP_SOURCE_CAPTURES_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_GAP_SOURCE_CAPTURES_READER_PATH}: {err}"
        )
    })?;
    if !methodology_gap_source_capture_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_GAP_SOURCE_CAPTURES_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_GAP_SOURCE_CAPTURES_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_GAP_SOURCE_CAPTURES_JSONL_PATH}"
        ));
    }
    let methodology_source_capture_rollup_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_CAPTURE_ROLLUP_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_CAPTURE_ROLLUP_READER_PATH}: {err}"
        )
    })?;
    if !methodology_source_capture_rollup_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_CAPTURE_ROLLUP_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_CAPTURE_ROLLUP_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_SOURCE_CAPTURE_ROLLUP_JSONL_PATH}"
        ));
    }
    let methodology_closure_readiness_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_READINESS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_READINESS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_closure_readiness_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_READINESS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_READINESS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_READINESS_JSONL_PATH}"
        ));
    }
    let methodology_closure_decision_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_DECISIONS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_DECISIONS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_closure_decision_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_DECISIONS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_DECISIONS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_DECISIONS_JSONL_PATH}"
        ));
    }
    let methodology_residual_source_gap_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_SOURCE_GAPS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_SOURCE_GAPS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_residual_source_gap_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_SOURCE_GAPS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_SOURCE_GAPS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_SOURCE_GAPS_JSONL_PATH}"
        ));
    }
    let methodology_closure_coverage_reader =
        fs::read_to_string(root.join(PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_COVERAGE_READER_PATH))
            .map_err(|err| {
            format!(
                "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_COVERAGE_READER_PATH}: {err}"
            )
        })?;
    if !methodology_closure_coverage_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_COVERAGE_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_COVERAGE_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_CLOSURE_COVERAGE_JSONL_PATH}"
        ));
    }
    let methodology_scoring_gate_reader =
        fs::read_to_string(root.join(PAYMENT_INTEGRITY_METHODOLOGY_SCORING_GATE_READER_PATH))
            .map_err(|err| {
                format!(
                    "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_SCORING_GATE_READER_PATH}: {err}"
                )
            })?;
    if !methodology_scoring_gate_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_SCORING_GATE_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_SCORING_GATE_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_SCORING_GATE_JSONL_PATH}"
        ));
    }
    let methodology_program_rollup_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_PROGRAM_ROLLUP_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {PAYMENT_INTEGRITY_METHODOLOGY_PROGRAM_ROLLUP_READER_PATH}: {err}")
    })?;
    if !methodology_program_rollup_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_PROGRAM_ROLLUP_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_PROGRAM_ROLLUP_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_PROGRAM_ROLLUP_JSONL_PATH}"
        ));
    }
    let methodology_open_program_status_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_STATUS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_STATUS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_open_program_status_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_STATUS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_STATUS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_STATUS_JSONL_PATH}"
        ));
    }
    let methodology_open_program_status_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_STATUS_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_STATUS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_open_program_status_schema
        .contains("payment_integrity_methodology_open_program_status")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_STATUS_SCHEMA_PATH} must describe payment_integrity_methodology_open_program_status"
        ));
    }
    let methodology_residual_gap_priority_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_GAP_PRIORITY_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_GAP_PRIORITY_READER_PATH}: {err}"
        )
    })?;
    if !methodology_residual_gap_priority_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_GAP_PRIORITY_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_GAP_PRIORITY_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_GAP_PRIORITY_JSONL_PATH}"
        ));
    }
    let methodology_residual_gap_priority_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_GAP_PRIORITY_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_GAP_PRIORITY_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_residual_gap_priority_schema
        .contains("payment_integrity_methodology_residual_gap_priority")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_RESIDUAL_GAP_PRIORITY_SCHEMA_PATH} must describe payment_integrity_methodology_residual_gap_priority"
        ));
    }
    let methodology_priority_source_work_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_SOURCE_WORK_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_SOURCE_WORK_READER_PATH}: {err}"
        )
    })?;
    if !methodology_priority_source_work_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_SOURCE_WORK_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_SOURCE_WORK_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_SOURCE_WORK_JSONL_PATH}"
        ));
    }
    let methodology_priority_source_work_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_SOURCE_WORK_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_SOURCE_WORK_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_priority_source_work_schema
        .contains("payment_integrity_methodology_priority_source_work")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_SOURCE_WORK_SCHEMA_PATH} must describe payment_integrity_methodology_priority_source_work"
        ));
    }
    let methodology_priority_reviewer_actions_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_REVIEWER_ACTIONS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_REVIEWER_ACTIONS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_priority_reviewer_actions_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_REVIEWER_ACTIONS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_REVIEWER_ACTIONS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_REVIEWER_ACTIONS_JSONL_PATH}"
        ));
    }
    let methodology_priority_reviewer_actions_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_REVIEWER_ACTIONS_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_REVIEWER_ACTIONS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_priority_reviewer_actions_schema
        .contains("payment_integrity_methodology_priority_reviewer_action")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_PRIORITY_REVIEWER_ACTIONS_SCHEMA_PATH} must describe payment_integrity_methodology_priority_reviewer_action"
        ));
    }
    let methodology_field_updates_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_FIELD_UPDATES_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {PAYMENT_INTEGRITY_METHODOLOGY_FIELD_UPDATES_READER_PATH}: {err}")
    })?;
    if !methodology_field_updates_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_FIELD_UPDATES_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_FIELD_UPDATES_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_FIELD_UPDATES_JSONL_PATH}"
        ));
    }
    let methodology_field_updates_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_FIELD_UPDATES_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!("failed to read {PAYMENT_INTEGRITY_METHODOLOGY_FIELD_UPDATES_SCHEMA_PATH}: {err}")
    })?;
    if !methodology_field_updates_schema.contains("payment_integrity_methodology_field_update") {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_FIELD_UPDATES_SCHEMA_PATH} must describe payment_integrity_methodology_field_update"
        ));
    }
    let methodology_followup_source_queries_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERIES_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERIES_READER_PATH}: {err}"
        )
    })?;
    if !methodology_followup_source_queries_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERIES_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERIES_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERIES_JSONL_PATH}"
        ));
    }
    let methodology_followup_source_queries_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERIES_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERIES_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_followup_source_queries_schema
        .contains("payment_integrity_methodology_followup_source_query")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERIES_SCHEMA_PATH} must describe payment_integrity_methodology_followup_source_query"
        ));
    }
    let methodology_followup_source_query_runs_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERY_RUNS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERY_RUNS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_followup_source_query_runs_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERY_RUNS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERY_RUNS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERY_RUNS_JSONL_PATH}"
        ));
    }
    let methodology_followup_source_query_runs_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERY_RUNS_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERY_RUNS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_followup_source_query_runs_schema
        .contains("payment_integrity_methodology_followup_source_query_run")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_QUERY_RUNS_SCHEMA_PATH} must describe payment_integrity_methodology_followup_source_query_run"
        ));
    }
    let methodology_followup_source_captures_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURES_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURES_READER_PATH}: {err}"
        )
    })?;
    if !methodology_followup_source_captures_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURES_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURES_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURES_JSONL_PATH}"
        ));
    }
    let methodology_followup_source_captures_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURES_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURES_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_followup_source_captures_schema
        .contains("payment_integrity_methodology_followup_source_capture")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURES_SCHEMA_PATH} must describe payment_integrity_methodology_followup_source_capture"
        ));
    }
    let methodology_followup_source_capture_rollup_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURE_ROLLUP_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURE_ROLLUP_READER_PATH}: {err}"
        )
    })?;
    if !methodology_followup_source_capture_rollup_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURE_ROLLUP_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURE_ROLLUP_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURE_ROLLUP_JSONL_PATH}"
        ));
    }
    let methodology_followup_source_capture_rollup_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURE_ROLLUP_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURE_ROLLUP_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_followup_source_capture_rollup_schema
        .contains("payment_integrity_methodology_followup_source_capture_rollup")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_SOURCE_CAPTURE_ROLLUP_SCHEMA_PATH} must describe payment_integrity_methodology_followup_source_capture_rollup"
        ));
    }
    let methodology_followup_boundary_decisions_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_DECISIONS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_DECISIONS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_followup_boundary_decisions_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_DECISIONS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_DECISIONS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_DECISIONS_JSONL_PATH}"
        ));
    }
    let methodology_followup_boundary_decisions_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_DECISIONS_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_DECISIONS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_followup_boundary_decisions_schema
        .contains("payment_integrity_methodology_followup_boundary_decision")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_DECISIONS_SCHEMA_PATH} must describe payment_integrity_methodology_followup_boundary_decision"
        ));
    }
    let methodology_followup_boundary_readiness_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_READINESS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_READINESS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_followup_boundary_readiness_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_READINESS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_READINESS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_READINESS_JSONL_PATH}"
        ));
    }
    let methodology_followup_boundary_readiness_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_READINESS_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_READINESS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_followup_boundary_readiness_schema
        .contains("payment_integrity_methodology_followup_boundary_readiness")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_FOLLOWUP_BOUNDARY_READINESS_SCHEMA_PATH} must describe payment_integrity_methodology_followup_boundary_readiness"
        ));
    }
    let methodology_narrow_closure_candidates_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_CANDIDATES_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_CANDIDATES_READER_PATH}: {err}"
        )
    })?;
    if !methodology_narrow_closure_candidates_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_CANDIDATES_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_CANDIDATES_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_CANDIDATES_JSONL_PATH}"
        ));
    }
    let methodology_narrow_closure_candidates_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_CANDIDATES_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_CANDIDATES_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_narrow_closure_candidates_schema
        .contains("payment_integrity_methodology_narrow_closure_candidate")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_CANDIDATES_SCHEMA_PATH} must describe payment_integrity_methodology_narrow_closure_candidate"
        ));
    }
    let methodology_narrow_closure_decisions_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_DECISIONS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_DECISIONS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_narrow_closure_decisions_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_DECISIONS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_DECISIONS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_DECISIONS_JSONL_PATH}"
        ));
    }
    let methodology_narrow_closure_decisions_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_DECISIONS_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_DECISIONS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_narrow_closure_decisions_schema
        .contains("payment_integrity_methodology_narrow_closure_decision")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_NARROW_CLOSURE_DECISIONS_SCHEMA_PATH} must describe payment_integrity_methodology_narrow_closure_decision"
        ));
    }
    let methodology_open_program_component_progress_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_COMPONENT_PROGRESS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_COMPONENT_PROGRESS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_open_program_component_progress_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_COMPONENT_PROGRESS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_COMPONENT_PROGRESS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_COMPONENT_PROGRESS_JSONL_PATH}"
        ));
    }
    let methodology_open_program_component_progress_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_COMPONENT_PROGRESS_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_COMPONENT_PROGRESS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_open_program_component_progress_schema
        .contains("payment_integrity_methodology_open_program_component_progress")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_OPEN_PROGRAM_COMPONENT_PROGRESS_SCHEMA_PATH} must describe payment_integrity_methodology_open_program_component_progress"
        ));
    }
    let methodology_component_gate_requirements_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_REQUIREMENTS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_REQUIREMENTS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_requirements_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_REQUIREMENTS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_REQUIREMENTS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_REQUIREMENTS_JSONL_PATH}"
        ));
    }
    let methodology_component_gate_requirements_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_REQUIREMENTS_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_REQUIREMENTS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_requirements_schema
        .contains("payment_integrity_methodology_component_gate_requirement")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_REQUIREMENTS_SCHEMA_PATH} must describe payment_integrity_methodology_component_gate_requirement"
        ));
    }
    let methodology_component_gate_source_targets_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_TARGETS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_TARGETS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_source_targets_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_TARGETS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_TARGETS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_TARGETS_JSONL_PATH}"
        ));
    }
    let methodology_component_gate_source_targets_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_TARGETS_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_TARGETS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_source_targets_schema
        .contains("payment_integrity_methodology_component_gate_source_target")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_TARGETS_SCHEMA_PATH} must describe payment_integrity_methodology_component_gate_source_target"
        ));
    }
    let methodology_component_gate_source_queries_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERIES_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERIES_READER_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_source_queries_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERIES_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERIES_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERIES_JSONL_PATH}"
        ));
    }
    let methodology_component_gate_source_queries_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERIES_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERIES_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_source_queries_schema
        .contains("payment_integrity_methodology_component_gate_source_query")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERIES_SCHEMA_PATH} must describe payment_integrity_methodology_component_gate_source_query"
        ));
    }
    let methodology_component_gate_source_query_runs_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERY_RUNS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERY_RUNS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_source_query_runs_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERY_RUNS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERY_RUNS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERY_RUNS_JSONL_PATH}"
        ));
    }
    let methodology_component_gate_source_query_runs_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERY_RUNS_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERY_RUNS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_source_query_runs_schema
        .contains("payment_integrity_methodology_component_gate_source_query_run")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_QUERY_RUNS_SCHEMA_PATH} must describe payment_integrity_methodology_component_gate_source_query_run"
        ));
    }
    let methodology_component_gate_source_captures_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURES_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURES_READER_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_source_captures_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURES_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURES_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURES_JSONL_PATH}"
        ));
    }
    let methodology_component_gate_source_captures_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURES_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURES_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_source_captures_schema
        .contains("payment_integrity_methodology_component_gate_source_capture")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURES_SCHEMA_PATH} must describe payment_integrity_methodology_component_gate_source_capture"
        ));
    }
    let methodology_component_gate_source_capture_rollups_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURE_ROLLUPS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURE_ROLLUPS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_source_capture_rollups_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURE_ROLLUPS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURE_ROLLUPS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURE_ROLLUPS_JSONL_PATH}"
        ));
    }
    let methodology_component_gate_source_capture_rollups_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURE_ROLLUPS_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURE_ROLLUPS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_source_capture_rollups_schema
        .contains("payment_integrity_methodology_component_gate_source_capture_rollup")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_SOURCE_CAPTURE_ROLLUPS_SCHEMA_PATH} must describe payment_integrity_methodology_component_gate_source_capture_rollup"
        ));
    }
    let methodology_component_gate_boundary_decisions_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_DECISIONS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_DECISIONS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_boundary_decisions_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_DECISIONS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_DECISIONS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_DECISIONS_JSONL_PATH}"
        ));
    }
    let methodology_component_gate_boundary_decisions_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_DECISIONS_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_DECISIONS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_boundary_decisions_schema
        .contains("payment_integrity_methodology_component_gate_boundary_decision")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_DECISIONS_SCHEMA_PATH} must describe payment_integrity_methodology_component_gate_boundary_decision"
        ));
    }
    let methodology_component_gate_boundary_readiness_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_READINESS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_READINESS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_boundary_readiness_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_READINESS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_READINESS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_READINESS_JSONL_PATH}"
        ));
    }
    let methodology_component_gate_boundary_readiness_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_READINESS_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_READINESS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_boundary_readiness_schema
        .contains("payment_integrity_methodology_component_gate_boundary_readiness")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_BOUNDARY_READINESS_SCHEMA_PATH} must describe payment_integrity_methodology_component_gate_boundary_readiness"
        ));
    }
    let methodology_component_gate_narrow_candidates_reader = fs::read_to_string(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_CANDIDATES_READER_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_CANDIDATES_READER_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_narrow_candidates_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_CANDIDATES_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_CANDIDATES_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_CANDIDATES_JSONL_PATH}"
        ));
    }
    let methodology_component_gate_narrow_candidates_schema = fs::read_to_string(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_CANDIDATES_SCHEMA_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_CANDIDATES_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_narrow_candidates_schema
        .contains("payment_integrity_methodology_component_gate_narrow_candidate")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_CANDIDATES_SCHEMA_PATH} must describe payment_integrity_methodology_component_gate_narrow_candidate"
        ));
    }
    let methodology_component_gate_narrow_decisions_reader = fs::read_to_string(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_DECISIONS_READER_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_DECISIONS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_narrow_decisions_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_DECISIONS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_DECISIONS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_DECISIONS_JSONL_PATH}"
        ));
    }
    let methodology_component_gate_narrow_decisions_schema = fs::read_to_string(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_DECISIONS_SCHEMA_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_DECISIONS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_narrow_decisions_schema
        .contains("payment_integrity_methodology_component_gate_narrow_decision")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_NARROW_DECISIONS_SCHEMA_PATH} must describe payment_integrity_methodology_component_gate_narrow_decision"
        ));
    }
    let methodology_component_gate_progress_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_progress_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_JSONL_PATH}"
        ));
    }
    let methodology_component_gate_progress_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_progress_schema
        .contains("payment_integrity_methodology_component_gate_progress")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SCHEMA_PATH} must describe payment_integrity_methodology_component_gate_progress"
        ));
    }
    let methodology_component_gate_progress_requirements_reader = fs::read_to_string(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_REQUIREMENTS_READER_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_REQUIREMENTS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_progress_requirements_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_REQUIREMENTS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_REQUIREMENTS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_REQUIREMENTS_JSONL_PATH}"
        ));
    }
    let methodology_component_gate_progress_requirements_schema = fs::read_to_string(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_REQUIREMENTS_SCHEMA_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_REQUIREMENTS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_progress_requirements_schema
        .contains("payment_integrity_methodology_component_gate_progress_requirement")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_REQUIREMENTS_SCHEMA_PATH} must describe payment_integrity_methodology_component_gate_progress_requirement"
        ));
    }
    let methodology_component_gate_progress_source_targets_reader = fs::read_to_string(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_TARGETS_READER_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_TARGETS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_progress_source_targets_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_TARGETS_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_TARGETS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_TARGETS_JSONL_PATH}"
        ));
    }
    let methodology_component_gate_progress_source_targets_schema = fs::read_to_string(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_TARGETS_SCHEMA_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_TARGETS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_progress_source_targets_schema
        .contains("payment_integrity_methodology_component_gate_progress_source_target")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_TARGETS_SCHEMA_PATH} must describe payment_integrity_methodology_component_gate_progress_source_target"
        ));
    }
    let methodology_component_gate_progress_source_queries_reader = fs::read_to_string(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERIES_READER_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERIES_READER_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_progress_source_queries_reader
        .contains(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERIES_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERIES_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERIES_JSONL_PATH}"
        ));
    }
    let methodology_component_gate_progress_source_queries_schema = fs::read_to_string(root.join(
        PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERIES_SCHEMA_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERIES_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_progress_source_queries_schema
        .contains("payment_integrity_methodology_component_gate_progress_source_query")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERIES_SCHEMA_PATH} must describe payment_integrity_methodology_component_gate_progress_source_query"
        ));
    }
    let methodology_component_gate_progress_source_query_runs_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERY_RUNS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERY_RUNS_READER_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_progress_source_query_runs_reader.contains(
        PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERY_RUNS_JSONL_PATH,
    ) {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERY_RUNS_READER_PATH} must cite {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERY_RUNS_JSONL_PATH}"
        ));
    }
    let methodology_component_gate_progress_source_query_runs_schema = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERY_RUNS_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERY_RUNS_SCHEMA_PATH}: {err}"
        )
    })?;
    if !methodology_component_gate_progress_source_query_runs_schema
        .contains("payment_integrity_methodology_component_gate_progress_source_query_run")
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_METHODOLOGY_COMPONENT_GATE_PROGRESS_SOURCE_QUERY_RUNS_SCHEMA_PATH} must describe payment_integrity_methodology_component_gate_progress_source_query_run"
        ));
    }
    let next_program_selection_reader =
        fs::read_to_string(root.join(PAYMENT_INTEGRITY_NEXT_PROGRAM_SELECTION_READER_PATH))
            .map_err(|err| {
                format!(
                    "failed to read {PAYMENT_INTEGRITY_NEXT_PROGRAM_SELECTION_READER_PATH}: {err}"
                )
            })?;
    if !next_program_selection_reader.contains(PAYMENT_INTEGRITY_NEXT_PROGRAM_SELECTION_JSONL_PATH)
    {
        return Err(format!(
            "{PAYMENT_INTEGRITY_NEXT_PROGRAM_SELECTION_READER_PATH} must cite {PAYMENT_INTEGRITY_NEXT_PROGRAM_SELECTION_JSONL_PATH}"
        ));
    }
    let payment_integrity_claims_reader = fs::read_to_string(
        root.join(PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_READER_PATH}: {err}")
    })?;
    if !payment_integrity_claims_reader.contains(PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_JSONL_PATH) {
        return Err(format!(
            "{PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_READER_PATH} must cite {PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_JSONL_PATH}"
        ));
    }
    let debt_maturity_reader =
        fs::read_to_string(root.join(DEBT_MATURITY_RISK_EXTRACT_READER_PATH)).map_err(|err| {
            format!("failed to read {DEBT_MATURITY_RISK_EXTRACT_READER_PATH}: {err}")
        })?;
    if !debt_maturity_reader.contains(DEBT_MATURITY_RISK_FIRST_PASS_JSONL_PATH) {
        return Err(format!(
            "{DEBT_MATURITY_RISK_EXTRACT_READER_PATH} must cite {DEBT_MATURITY_RISK_FIRST_PASS_JSONL_PATH}"
        ));
    }
    let debt_primary_balance_reader =
        fs::read_to_string(root.join(DEBT_PRIMARY_BALANCE_EXTRACT_READER_PATH)).map_err(|err| {
            format!("failed to read {DEBT_PRIMARY_BALANCE_EXTRACT_READER_PATH}: {err}")
        })?;
    if !debt_primary_balance_reader.contains(DEBT_PRIMARY_BALANCE_FIRST_PASS_JSONL_PATH) {
        return Err(format!(
            "{DEBT_PRIMARY_BALANCE_EXTRACT_READER_PATH} must cite {DEBT_PRIMARY_BALANCE_FIRST_PASS_JSONL_PATH}"
        ));
    }
    let disaster_supplemental_reader = fs::read_to_string(
        root.join(DISASTER_SUPPLEMENTAL_TRACKING_EXTRACT_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {DISASTER_SUPPLEMENTAL_TRACKING_EXTRACT_READER_PATH}: {err}")
    })?;
    if !disaster_supplemental_reader.contains(DISASTER_SUPPLEMENTAL_TRACKING_FIRST_PASS_JSONL_PATH)
    {
        return Err(format!(
            "{DISASTER_SUPPLEMENTAL_TRACKING_EXTRACT_READER_PATH} must cite {DISASTER_SUPPLEMENTAL_TRACKING_FIRST_PASS_JSONL_PATH}"
        ));
    }
    let disaster_mitigation_reader =
        fs::read_to_string(root.join(DISASTER_MITIGATION_EXTRACT_READER_PATH)).map_err(|err| {
            format!("failed to read {DISASTER_MITIGATION_EXTRACT_READER_PATH}: {err}")
        })?;
    if !disaster_mitigation_reader.contains(DISASTER_MITIGATION_FIRST_PASS_JSONL_PATH) {
        return Err(format!(
            "{DISASTER_MITIGATION_EXTRACT_READER_PATH} must cite {DISASTER_MITIGATION_FIRST_PASS_JSONL_PATH}"
        ));
    }
    let defense_audit_control_reader = fs::read_to_string(
        root.join(DEFENSE_AUDIT_CONTROL_EXTRACT_READER_PATH),
    )
    .map_err(|err| format!("failed to read {DEFENSE_AUDIT_CONTROL_EXTRACT_READER_PATH}: {err}"))?;
    if !defense_audit_control_reader.contains(DEFENSE_AUDIT_CONTROL_FIRST_PASS_JSONL_PATH) {
        return Err(format!(
            "{DEFENSE_AUDIT_CONTROL_EXTRACT_READER_PATH} must cite {DEFENSE_AUDIT_CONTROL_FIRST_PASS_JSONL_PATH}"
        ));
    }
    let defense_procurement_control_reader = fs::read_to_string(
        root.join(DEFENSE_PROCUREMENT_CONTROL_EXTRACT_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {DEFENSE_PROCUREMENT_CONTROL_EXTRACT_READER_PATH}: {err}")
    })?;
    if !defense_procurement_control_reader
        .contains(DEFENSE_PROCUREMENT_CONTROL_FIRST_PASS_JSONL_PATH)
    {
        return Err(format!(
            "{DEFENSE_PROCUREMENT_CONTROL_EXTRACT_READER_PATH} must cite {DEFENSE_PROCUREMENT_CONTROL_FIRST_PASS_JSONL_PATH}"
        ));
    }
    let health_price_reader =
        fs::read_to_string(root.join(HEALTH_PRICE_DISCIPLINE_EXTRACT_READER_PATH)).map_err(
            |err| format!("failed to read {HEALTH_PRICE_DISCIPLINE_EXTRACT_READER_PATH}: {err}"),
        )?;
    if !health_price_reader.contains(HEALTH_PRICE_DISCIPLINE_FIRST_PASS_JSONL_PATH) {
        return Err(format!(
            "{HEALTH_PRICE_DISCIPLINE_EXTRACT_READER_PATH} must cite {HEALTH_PRICE_DISCIPLINE_FIRST_PASS_JSONL_PATH}"
        ));
    }
    let health_admin_reader = fs::read_to_string(
        root.join(HEALTH_ADMIN_SIMPLIFICATION_EXTRACT_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {HEALTH_ADMIN_SIMPLIFICATION_EXTRACT_READER_PATH}: {err}")
    })?;
    if !health_admin_reader.contains(HEALTH_ADMIN_SIMPLIFICATION_FIRST_PASS_JSONL_PATH) {
        return Err(format!(
            "{HEALTH_ADMIN_SIMPLIFICATION_EXTRACT_READER_PATH} must cite {HEALTH_ADMIN_SIMPLIFICATION_FIRST_PASS_JSONL_PATH}"
        ));
    }
    for source_packet_reader_path in [
        HEALTH_PRICE_DISCIPLINE_SOURCE_PACKET_READER_PATH,
        HEALTH_ADMIN_SIMPLIFICATION_SOURCE_PACKET_READER_PATH,
        DEBT_PRIMARY_BALANCE_SOURCE_PACKET_READER_PATH,
        DEBT_MATURITY_RISK_SOURCE_PACKET_READER_PATH,
        DEFENSE_PROCUREMENT_CONTROL_SOURCE_PACKET_READER_PATH,
        DEFENSE_AUDIT_CONTROL_SOURCE_PACKET_READER_PATH,
        DISASTER_MITIGATION_SOURCE_PACKET_READER_PATH,
        DISASTER_SUPPLEMENTAL_TRACKING_SOURCE_PACKET_READER_PATH,
        PAYMENT_INTEGRITY_ELIGIBILITY_SOURCE_PACKET_READER_PATH,
        PAYMENT_INTEGRITY_CLAIMS_TIMELINESS_SOURCE_PACKET_READER_PATH,
    ] {
        let source_packet_reader = fs::read_to_string(root.join(source_packet_reader_path))
            .map_err(|err| format!("failed to read {source_packet_reader_path}: {err}"))?;
        if !source_packet_reader.contains(COST_DOWN_SOURCE_PACKETS_JSONL_PATH) {
            return Err(format!(
                "{source_packet_reader_path} must cite {COST_DOWN_SOURCE_PACKETS_JSONL_PATH}"
            ));
        }
    }

    println!(
        "validated {} efficiency pressure rows, {} cost-down backlog rows, {} source packet rows, {} evidence queue rows, {} first-pass rollup rows, {} scoring-readiness rows, {} payment-integrity probe rows, {} payment-integrity scorecard rows, {} payment-integrity program-gate rows, {} payment-integrity program-task rows, {} payment-integrity program-status rows, {} payment-integrity methodology-plan rows, {} payment-integrity methodology-field rows, {} payment-integrity methodology-source rows, {} payment-integrity methodology-query rows, {} payment-integrity methodology-query-run rows, {} payment-integrity methodology-result rows, {} payment-integrity methodology-result-review-readiness rows, {} payment-integrity methodology-field-review rows, {} payment-integrity methodology-gap-followup rows, {} payment-integrity methodology-gap-source-capture rows, {} payment-integrity methodology-source-capture-rollup rows, {} payment-integrity methodology-closure-readiness rows, {} payment-integrity methodology-closure-decision rows, {} payment-integrity methodology-residual-source-gap rows, {} payment-integrity methodology-closure-coverage rows, {} payment-integrity methodology-scoring-gate rows, {} payment-integrity methodology-program-rollup rows, {} payment-integrity methodology-open-program-status rows, {} payment-integrity methodology-residual-gap-priority rows, {} payment-integrity methodology-priority-source-work rows, {} payment-integrity methodology-priority-reviewer-action rows, {} payment-integrity methodology-field-update rows, {} payment-integrity methodology-followup-source-query rows, {} payment-integrity methodology-followup-source-query-run rows, {} payment-integrity methodology-followup-source-capture rows, {} payment-integrity methodology-followup-source-capture-rollup rows, {} payment-integrity methodology-followup-boundary-decision rows, {} payment-integrity methodology-followup-boundary-readiness rows, {} payment-integrity methodology-narrow-closure-candidate rows, {} payment-integrity methodology-narrow-closure-decision rows, {} payment-integrity methodology-open-program-component-progress rows, {} payment-integrity methodology-component-gate-requirement rows, {} payment-integrity methodology-component-gate-source-target rows, {} payment-integrity methodology-component-gate-source-query rows, {} payment-integrity methodology-component-gate-source-query-run rows, {} payment-integrity methodology-component-gate-source-capture rows, {} payment-integrity methodology-component-gate-source-capture-rollup rows, {} payment-integrity methodology-component-gate-boundary-decision rows, {} payment-integrity methodology-component-gate-boundary-readiness rows, {} payment-integrity methodology-component-gate-narrow-candidate rows, {} payment-integrity methodology-component-gate-narrow-decision rows, {} payment-integrity methodology-component-gate-progress rows, {} payment-integrity methodology-component-gate-progress-requirement rows, {} payment-integrity methodology-component-gate-progress-source-target rows, {} payment-integrity methodology-component-gate-progress-source-query rows, {} payment-integrity methodology-component-gate-progress-source-query-run rows, {} payment-integrity next-program-selection rows, {} claims-timeliness rows, {} debt maturity-risk rows, {} debt primary-balance rows, {} disaster declaration rows, {} disaster mitigation rows, {} defense audit-control rows, {} defense procurement-control rows, {} health price-discipline rows, and {} health administrative-simplification rows",
        rows.len(),
        backlog_rows.len(),
        source_packet_rows.len(),
        evidence_queue_rows.len(),
        first_pass_rollup_rows.len(),
        scoring_readiness_rows.len(),
        payment_integrity_probe_rows.len(),
        payment_integrity_scorecard_rows.len(),
        payment_integrity_program_gate_rows.len(),
        payment_integrity_program_task_rows.len(),
        payment_integrity_program_status_rows.len(),
        methodology_plan_rows.len(),
        methodology_field_rows.len(),
        methodology_source_target_rows.len(),
        methodology_query_rows.len(),
        methodology_query_run_rows.len(),
        methodology_result_rows.len(),
        methodology_result_review_readiness_rows.len(),
        methodology_field_review_rows.len(),
        methodology_gap_followup_rows.len(),
        methodology_gap_source_capture_rows.len(),
        methodology_source_capture_rollup_rows.len(),
        methodology_closure_readiness_rows.len(),
        methodology_closure_decision_rows.len(),
        methodology_residual_source_gap_rows.len(),
        methodology_closure_coverage_rows.len(),
        methodology_scoring_gate_rows.len(),
        methodology_program_rollup_rows.len(),
        methodology_open_program_status_rows.len(),
        methodology_residual_gap_priority_rows.len(),
        methodology_priority_source_work_rows.len(),
        methodology_priority_reviewer_action_rows.len(),
        methodology_field_update_rows.len(),
        methodology_followup_source_query_rows.len(),
        methodology_followup_source_query_run_rows.len(),
        methodology_followup_source_capture_rows.len(),
        methodology_followup_source_capture_rollup_rows.len(),
        methodology_followup_boundary_decision_rows.len(),
        methodology_followup_boundary_readiness_rows.len(),
        methodology_narrow_closure_candidate_rows.len(),
        methodology_narrow_closure_decision_rows.len(),
        methodology_open_program_component_progress_rows.len(),
        methodology_component_gate_requirement_rows.len(),
        methodology_component_gate_source_target_rows.len(),
        methodology_component_gate_source_query_rows.len(),
        methodology_component_gate_source_query_run_rows.len(),
        methodology_component_gate_source_capture_rows.len(),
        methodology_component_gate_source_capture_rollup_rows.len(),
        methodology_component_gate_boundary_decision_rows.len(),
        methodology_component_gate_boundary_readiness_rows.len(),
        methodology_component_gate_narrow_candidate_rows.len(),
        methodology_component_gate_narrow_decision_rows.len(),
        methodology_component_gate_progress_rows.len(),
        methodology_component_gate_progress_requirement_rows.len(),
        methodology_component_gate_progress_source_target_rows.len(),
        methodology_component_gate_progress_source_query_rows.len(),
        methodology_component_gate_progress_source_query_run_rows.len(),
        next_program_selection_rows.len(),
        payment_integrity_claims_rows.len(),
        debt_maturity_rows.len(),
        debt_primary_balance_rows.len(),
        disaster_declaration_rows.len(),
        disaster_mitigation_rows.len(),
        defense_audit_control_rows.len(),
        defense_procurement_control_rows.len(),
        health_price_rows.len(),
        health_admin_rows.len()
    );
    Ok(())
}

fn validate_per_unit_display_records(root: &Path) -> Result<(), String> {
    let source_ledger = fs::read_to_string(root.join(SOURCE_VERSION_LEDGER_PATH))
        .map_err(|err| format!("failed to read {SOURCE_VERSION_LEDGER_PATH}: {err}"))?;
    let readiness_rows: Vec<PerUnitDisplayReadinessRecord> =
        read_jsonl(root.join(PER_UNIT_DISPLAY_READINESS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!("{PER_UNIT_DISPLAY_READINESS_JSONL_PATH} row failed to parse: {err}")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    let card_rows: Vec<PerUnitReceiptCardRecord> =
        read_jsonl(root.join(PER_UNIT_RECEIPT_CARDS_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!("{PER_UNIT_RECEIPT_CARDS_JSONL_PATH} row failed to parse: {err}")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

    if readiness_rows.len() != 6 {
        return Err(format!(
            "per-unit display readiness must contain 6 rows, got {}",
            readiness_rows.len()
        ));
    }
    if card_rows.len() != readiness_rows.len() {
        return Err(format!(
            "per-unit receipt cards must match readiness row count; got {} cards for {} readiness rows",
            card_rows.len(),
            readiness_rows.len()
        ));
    }

    let mut readiness_ids = BTreeSet::new();
    let mut readiness_status_by_id = BTreeMap::new();
    for row in &readiness_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !readiness_ids.insert(row.record_id.clone()) {
            return Err(format!(
                "duplicate per-unit readiness row {}",
                row.record_id
            ));
        }
        for source_id in &row.source_ids {
            if !source_ledger.contains(&format!("`{source_id}`")) {
                return Err(format!(
                    "{}: source_id {source_id} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                    row.record_id
                ));
            }
        }
        readiness_status_by_id.insert(row.record_id.clone(), row.display_status.clone());
    }

    let mut card_ids = BTreeSet::new();
    let mut card_readiness_ids = BTreeSet::new();
    for row in &card_rows {
        row.validate()
            .map_err(|err| format!("{}: {err}", row.record_id))?;
        if !card_ids.insert(row.record_id.clone()) {
            return Err(format!("duplicate per-unit receipt card {}", row.record_id));
        }
        if !readiness_ids.contains(&row.source_readiness_record_id) {
            return Err(format!(
                "{} points to missing readiness row {}",
                row.record_id, row.source_readiness_record_id
            ));
        }
        if !card_readiness_ids.insert(row.source_readiness_record_id.clone()) {
            return Err(format!(
                "multiple per-unit cards point to {}",
                row.source_readiness_record_id
            ));
        }

        let readiness_status = readiness_status_by_id
            .get(&row.source_readiness_record_id)
            .ok_or_else(|| {
                format!(
                    "{} points to missing readiness row {}",
                    row.record_id, row.source_readiness_record_id
                )
            })?;
        let expected_card_status = match readiness_status.as_str() {
            "ready_same_source_year_basis" => "source_basis_context",
            "illustrative_cross_basis" => "illustrative_cross_basis",
            "blocked_missing_denominator" => "blocked_missing_denominator",
            _ => unreachable!(),
        };
        if row.card_status != expected_card_status {
            return Err(format!(
                "{} card_status {} does not match readiness status {}",
                row.record_id, row.card_status, readiness_status
            ));
        }
    }
    if card_readiness_ids != readiness_ids {
        return Err(
            "per-unit receipt cards must cover every readiness row exactly once".to_string(),
        );
    }

    let dashboard = fs::read_to_string(root.join(PER_UNIT_DISPLAY_READINESS_DASHBOARD_PATH))
        .map_err(|err| {
            format!("failed to read {PER_UNIT_DISPLAY_READINESS_DASHBOARD_PATH}: {err}")
        })?;
    if !dashboard.contains(PER_UNIT_DISPLAY_READINESS_JSONL_PATH) {
        return Err(format!(
            "{PER_UNIT_DISPLAY_READINESS_DASHBOARD_PATH} must cite {PER_UNIT_DISPLAY_READINESS_JSONL_PATH}"
        ));
    }
    let reader = fs::read_to_string(root.join(PER_UNIT_RECEIPT_CARDS_READER_PATH))
        .map_err(|err| format!("failed to read {PER_UNIT_RECEIPT_CARDS_READER_PATH}: {err}"))?;
    if !reader.contains(PER_UNIT_RECEIPT_CARDS_JSONL_PATH) {
        return Err(format!(
            "{PER_UNIT_RECEIPT_CARDS_READER_PATH} must cite {PER_UNIT_RECEIPT_CARDS_JSONL_PATH}"
        ));
    }

    println!(
        "validated {} per-unit readiness rows and {} per-unit receipt cards",
        readiness_rows.len(),
        card_rows.len()
    );
    Ok(())
}

fn build_spend_category_dashboard(rows: &[SpendCategoryMapRecord]) -> Result<String, String> {
    let total_outlays: f64 = rows
        .iter()
        .map(|row| row.subfunction_outlays_millions)
        .sum();
    let total_share: f64 = rows
        .iter()
        .map(|row| row.share_of_total_outlays_percent)
        .sum();
    let total_modeled_income_tax: f64 = rows
        .iter()
        .map(|row| row.modeled_income_tax_allocation_millions)
        .sum();
    let top_five_share: f64 = rows
        .iter()
        .take(5)
        .map(|row| row.share_of_total_outlays_percent)
        .sum();

    let mut lines = vec![
        "# Spend Category Dashboard".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated dashboard summarizes the draft FY2025 spend-category map. It is a question-routing view, not taxpayer-dollar tracing, legal dedication, recipient-level spending, or a performance finding.".to_string(),
        String::new(),
        "## Summary".to_string(),
        String::new(),
        format!("- Rows: {}", rows.len()),
        format!(
            "- Top 15 outlays: {}",
            format_millions_as_billions_or_trillions(total_outlays)
        ),
        format!("- Share represented: {:.2}%", total_share),
        format!(
            "- Modeled income-tax allocation represented: {}",
            format_millions_as_billions_or_trillions(total_modeled_income_tax)
        ),
        format!("- Top five share: {:.2}%", top_five_share),
        String::new(),
        "## Rows".to_string(),
        String::new(),
        "| Rank | Category | OMB function | Outlays | Share | Modeled income-tax allocation | Next source need |".to_string(),
        "|---:|---|---|---:|---:|---:|---|".to_string(),
    ];

    for row in rows {
        lines.push(format!(
            "| {} | {} | {} | {} | {:.2}% | {} | {} |",
            row.rank,
            escape_table_cell(&row.subfunction_label),
            escape_table_cell(&row.function_label),
            format_millions_as_billions_or_trillions(row.subfunction_outlays_millions),
            row.share_of_total_outlays_percent,
            format_millions_as_billions_or_trillions(row.modeled_income_tax_allocation_millions),
            escape_table_cell(&row.next_source_need),
        ));
    }

    lines.extend([
        String::new(),
        "## Boundary".to_string(),
        String::new(),
        "Every row remains `question_surface_only`. Use this dashboard to choose the next source to inspect; do not use it to claim fraud, waste, abuse, poor performance, legal dedication of income-tax dollars, or recipient-level outlays.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn check_accountability_readiness_report(root: &Path) -> Result<(), String> {
    let expected = build_accountability_readiness_report(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_READINESS_REPORT_PATH,
        &expected,
        "accountability readiness report",
    )?;
    println!("validated accountability readiness report");
    Ok(())
}

fn check_accountability_action_queue(root: &Path) -> Result<(), String> {
    let expected = build_accountability_action_queue(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_ACTION_QUEUE_PATH,
        &expected,
        "accountability action queue",
    )?;
    println!("validated accountability action queue");
    Ok(())
}

fn check_accountability_performance_demand_packet(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_packet(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_PACKET_PATH,
        &expected,
        "accountability performance demand packet",
    )?;
    println!("validated accountability performance demand packet");
    Ok(())
}

fn check_accountability_work_items(root: &Path) -> Result<(), String> {
    let expected = build_accountability_work_items_jsonl(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_WORK_ITEMS_JSONL_PATH,
        &expected,
        "accountability work items",
    )?;
    println!("validated accountability work items");
    Ok(())
}

fn check_accountability_claim_guard_report(root: &Path) -> Result<(), String> {
    let expected = build_accountability_claim_guard_report(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_CLAIM_GUARD_REPORT_PATH,
        &expected,
        "accountability claim guard report",
    )?;
    println!("validated accountability claim guard report");
    Ok(())
}

fn check_accountability_public_questions(root: &Path) -> Result<(), String> {
    let expected = build_accountability_public_questions(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PUBLIC_QUESTIONS_PATH,
        &expected,
        "accountability public questions",
    )?;
    println!("validated accountability public questions");
    Ok(())
}

fn check_accountability_public_brief(root: &Path) -> Result<(), String> {
    let expected = build_accountability_public_brief(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PUBLIC_BRIEF_PATH,
        &expected,
        "accountability public brief",
    )?;
    println!("validated accountability public brief");
    Ok(())
}

fn check_accountability_public_brief_discovery(root: &Path) -> Result<(), String> {
    let root_readme = fs::read_to_string(root.join(README_PATH))
        .map_err(|err| format!("failed to read {README_PATH}: {err}"))?;
    if !root_readme.contains(ACCOUNTABILITY_PUBLIC_BRIEF_PATH) {
        return Err(format!(
            "{README_PATH} must link {ACCOUNTABILITY_PUBLIC_BRIEF_PATH}"
        ));
    }

    let reading_index = fs::read_to_string(root.join(READING_INDEX_PATH))
        .map_err(|err| format!("failed to read {READING_INDEX_PATH}: {err}"))?;
    if !reading_index.contains("accountability-public-brief.md") {
        return Err(format!(
            "{READING_INDEX_PATH} must link accountability-public-brief.md"
        ));
    }

    println!("validated accountability public brief discovery");
    Ok(())
}

fn check_accountability_artifact_map(root: &Path) -> Result<(), String> {
    let expected = build_accountability_artifact_map();
    compare_text(
        root,
        ACCOUNTABILITY_ARTIFACT_MAP_PATH,
        &expected,
        "accountability artifact map",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("artifact-map.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link artifact-map.md".to_string(),
        );
    }

    let artifact_map = fs::read_to_string(root.join(ACCOUNTABILITY_ARTIFACT_MAP_PATH))
        .map_err(|err| format!("failed to read {ACCOUNTABILITY_ARTIFACT_MAP_PATH}: {err}"))?;
    for required in [
        "performance-demand-dashboard.md",
        "performance-demand-claim-gates.json",
        "performance-demand-checklist.jsonl",
        "performance-demand-checklist.schema.md",
        "performance-demand-response-log.md",
        "performance-demand-response-log.jsonl",
        "performance-demand-response-log.schema.md",
        "performance-demand-response-status.json",
        "performance-demand-response-dashboard.md",
        "performance-demand-response-handoff.md",
        "performance-demand-response-intake.md",
        "performance-demand-response-intake.schema.md",
        "performance-demand-response-intake.example.jsonl",
        "performance-demand-response-log.applied-example.jsonl",
        "performance-demand-response-status.applied-example.json",
        "performance-demand-response-dashboard.applied-example.md",
        "performance-demand-response-handoff.applied-example.md",
        "performance-demand-response-applied-example.schema.md",
        "performance-demand-response-delta.applied-example.md",
        "performance-demand-response-delta.applied-example.jsonl",
        "performance-demand-response-delta.applied-example.schema.md",
        "performance-demand-response-bundle.applied-example.md",
        "performance-demand-response-bundle.applied-example.json",
        "performance-demand-response-bundle.applied-example.schema.md",
    ] {
        if !artifact_map.contains(required) {
            return Err(format!(
                "{ACCOUNTABILITY_ARTIFACT_MAP_PATH} must route {required}"
            ));
        }
    }

    println!("validated accountability artifact map");
    Ok(())
}

fn check_accountability_performance_demand_checklist(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_checklist(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_PATH,
        &expected,
        "accountability performance demand checklist",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-checklist.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-checklist.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand checklist");
    Ok(())
}

fn check_accountability_performance_demand_checklist_jsonl(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_checklist_jsonl(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_JSONL_PATH,
        &expected,
        "accountability performance demand checklist JSONL",
    )?;

    let rows: Vec<PerformanceDemandChecklistRecord> =
        read_jsonl(root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!("accountability performance demand checklist JSONL: {err}")
                })
            })
            .collect::<Result<_, _>>()?;
    if rows.is_empty() {
        return Err("accountability performance demand checklist JSONL has no rows".to_string());
    }
    let mut expected_rows = read_accountability_evidence_records(root)?;
    expected_rows.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    let expected_rows: Vec<PerformanceDemandChecklistRecord> = expected_rows
        .iter()
        .map(AccountabilityEvidenceRecord::performance_demand_checklist_record)
        .collect();
    if rows != expected_rows {
        return Err(
            "accountability performance demand checklist JSONL does not match core records"
                .to_string(),
        );
    }
    for row in rows {
        row.validate()?;
        if row.public_claim_allowed {
            return Err(
                "accountability performance demand checklist JSONL unexpectedly allows a public claim"
                    .to_string(),
            );
        }
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-checklist.jsonl") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-checklist.jsonl"
                .to_string(),
        );
    }
    let schema_filename = ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_SCHEMA_PATH
        .rsplit('/')
        .next()
        .unwrap_or(ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_SCHEMA_PATH);
    if !index.contains(schema_filename) {
        return Err(format!(
            "data/derived/accountability_evidence/README.md must link {schema_filename}"
        ));
    }

    println!("validated accountability performance demand checklist JSONL");
    Ok(())
}

fn check_accountability_performance_demand_claim_gates(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_claim_gates(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_CLAIM_GATES_PATH,
        &expected,
        "accountability performance demand claim gates",
    )?;

    let parsed_text = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_CLAIM_GATES_PATH),
    )
    .map_err(|err| {
        format!("failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_CLAIM_GATES_PATH}: {err}")
    })?;
    let parsed: serde_json::Value = serde_json::from_str(&parsed_text).map_err(|err| {
        format!("failed to parse {ACCOUNTABILITY_PERFORMANCE_DEMAND_CLAIM_GATES_PATH}: {err}")
    })?;
    let total_rows = parsed
        .get("total_rows")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "performance demand claim gates missing total_rows".to_string())?;
    let blocked_rows = parsed
        .get("public_claim_blocked")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "performance demand claim gates missing public_claim_blocked".to_string())?;
    let allowed_rows = parsed
        .get("public_claim_allowed")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "performance demand claim gates missing public_claim_allowed".to_string())?;
    if total_rows != blocked_rows + allowed_rows {
        return Err(
            "performance demand claim gates total does not match allowed plus blocked".to_string(),
        );
    }
    if allowed_rows != 0 {
        return Err("performance demand claim gates unexpectedly allow a public claim".to_string());
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-claim-gates.json") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-claim-gates.json"
                .to_string(),
        );
    }

    println!("validated accountability performance demand claim gates");
    Ok(())
}

fn check_accountability_performance_demand_dashboard(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_dashboard(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_DASHBOARD_PATH,
        &expected,
        "accountability performance demand dashboard",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-dashboard.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-dashboard.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand dashboard");
    Ok(())
}

fn check_accountability_performance_demand_brief(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_brief(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_BRIEF_PATH,
        &expected,
        "accountability performance demand brief",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-brief.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-brief.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand brief");
    Ok(())
}

fn check_accountability_performance_demand_letter(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_letter(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_LETTER_PATH,
        &expected,
        "accountability performance demand letter",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-letter.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-letter.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand letter");
    Ok(())
}

fn check_accountability_performance_demand_response_rubric(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_rubric(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_RUBRIC_PATH,
        &expected,
        "accountability performance demand response rubric",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-rubric.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-rubric.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response rubric");
    Ok(())
}

fn check_accountability_performance_demand_followup(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_followup(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_FOLLOWUP_PATH,
        &expected,
        "accountability performance demand follow-up",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-followup.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-followup.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand follow-up");
    Ok(())
}

fn check_accountability_performance_demand_response_log(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_log(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_PATH,
        &expected,
        "accountability performance demand response log",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-log.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-log.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response log");
    Ok(())
}

fn check_accountability_performance_demand_response_log_jsonl(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_log_jsonl(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_JSONL_PATH,
        &expected,
        "accountability performance demand response log JSONL",
    )?;

    let rows: Vec<PerformanceDemandResponseLogRecord> =
        read_jsonl(root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row)
                    .map_err(|err| format!("response log JSONL: invalid row shape: {err}"))
            })
            .collect::<Result<Vec<_>, _>>()?;
    if rows.is_empty() {
        return Err("performance demand response log JSONL has no rows".to_string());
    }
    let mut expected_records: Vec<PerformanceDemandResponseLogRecord> =
        read_accountability_evidence_records(root)?
            .into_iter()
            .map(|record| record.performance_demand_response_log_record())
            .collect();
    expected_records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    if rows != expected_records {
        return Err("response log JSONL rows do not match core-derived records".to_string());
    }
    for row in rows {
        row.validate()?;
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-log.jsonl") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-log.jsonl"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response log JSONL");
    Ok(())
}

fn check_accountability_performance_demand_response_log_schema(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_log_schema();
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_SCHEMA_PATH,
        &expected,
        "accountability performance demand response log schema",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-log.schema.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-log.schema.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response log schema");
    Ok(())
}

fn check_accountability_performance_demand_response_status(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_status(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_PATH,
        &expected,
        "accountability performance demand response status",
    )?;

    let parsed_text =
        fs::read_to_string(root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_PATH))
            .map_err(|err| {
                format!(
                    "failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_PATH}: {err}"
                )
            })?;
    let parsed: PerformanceDemandResponseStatus =
        serde_json::from_str(&parsed_text).map_err(|err| {
            format!(
                "failed to parse {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_PATH}: {err}"
            )
        })?;
    parsed.validate()?;
    if parsed.total_rows != parsed.not_yet_received {
        return Err("all generated response status rows must be not-yet-received".to_string());
    }
    if parsed.public_claim_allowed != 0 {
        return Err("response status unexpectedly allows a public claim".to_string());
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-status.json") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-status.json"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response status");
    Ok(())
}

fn check_accountability_performance_demand_response_dashboard(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_dashboard(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DASHBOARD_PATH,
        &expected,
        "accountability performance demand response dashboard",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-dashboard.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-dashboard.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response dashboard");
    Ok(())
}

fn check_accountability_performance_demand_response_handoff(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_handoff(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_HANDOFF_PATH,
        &expected,
        "accountability performance demand response handoff",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-handoff.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-handoff.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response handoff");
    Ok(())
}

fn check_accountability_performance_demand_response_intake(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_intake();
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_INTAKE_PATH,
        &expected,
        "accountability performance demand response intake",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-intake.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-intake.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response intake");
    Ok(())
}

fn check_accountability_performance_demand_response_intake_schema(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_intake_schema();
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_INTAKE_SCHEMA_PATH,
        &expected,
        "accountability performance demand response intake schema",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-intake.schema.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-intake.schema.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response intake schema");
    Ok(())
}

fn check_accountability_performance_demand_response_intake_example_jsonl(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_intake_example_jsonl(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_INTAKE_EXAMPLE_JSONL_PATH,
        &expected,
        "accountability performance demand response intake example JSONL",
    )?;

    let intake_rows: Vec<PerformanceDemandResponseIntakeRecord> = read_jsonl(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_INTAKE_EXAMPLE_JSONL_PATH),
    )?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row)
            .map_err(|err| format!("response intake example JSONL: invalid row shape: {err}"))
    })
    .collect::<Result<Vec<_>, _>>()?;
    if intake_rows.is_empty() {
        return Err("performance demand response intake example JSONL has no rows".to_string());
    }

    let mut log_rows: BTreeMap<String, PerformanceDemandResponseLogRecord> =
        read_jsonl(root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                let record: PerformanceDemandResponseLogRecord = serde_json::from_value(row)
                    .map_err(|err| format!("response log JSONL: invalid row shape: {err}"))?;
                Ok((record.record_id.clone(), record))
            })
            .collect::<Result<BTreeMap<_, _>, String>>()?;

    for intake in intake_rows {
        intake.validate()?;
        let log_record = log_rows.remove(&intake.record_id).ok_or_else(|| {
            format!(
                "response intake example row has no matching response log row: {}",
                intake.record_id
            )
        })?;
        let updated = log_record.apply_intake(&intake)?;
        updated.validate()?;
        if updated.public_claim_allowed {
            return Err("response intake example unexpectedly allowed a public claim".to_string());
        }
        if updated.claim_gate != PUBLIC_CLAIM_BLOCKED_LABEL {
            return Err("response intake example changed the blocked claim gate".to_string());
        }
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-intake.example.jsonl") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-intake.example.jsonl"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response intake example JSONL");
    Ok(())
}

fn check_accountability_performance_demand_response_log_applied_example_jsonl(
    root: &Path,
) -> Result<(), String> {
    let expected =
        build_accountability_performance_demand_response_log_applied_example_jsonl(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_APPLIED_EXAMPLE_JSONL_PATH,
        &expected,
        "accountability performance demand response log applied example JSONL",
    )?;

    let rows: Vec<PerformanceDemandResponseLogRecord> = read_jsonl(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_APPLIED_EXAMPLE_JSONL_PATH),
    )?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row)
            .map_err(|err| format!("response log applied example JSONL: invalid row shape: {err}"))
    })
    .collect::<Result<Vec<_>, _>>()?;
    if rows.is_empty() {
        return Err(
            "performance demand response log applied example JSONL has no rows".to_string(),
        );
    }

    let updated_rows = rows
        .iter()
        .filter(|row| row.response_class != PerformanceDemandResponseLogClass::NotYetReceived)
        .count();
    if updated_rows == 0 {
        return Err(
            "performance demand response log applied example JSONL has no updated rows".to_string(),
        );
    }

    for row in rows {
        row.validate()?;
        if row.public_claim_allowed {
            return Err(
                "response log applied example unexpectedly allowed a public claim".to_string(),
            );
        }
        if row.claim_gate != PUBLIC_CLAIM_BLOCKED_LABEL {
            return Err("response log applied example changed the blocked claim gate".to_string());
        }
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-log.applied-example.jsonl") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-log.applied-example.jsonl"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response log applied example JSONL");
    Ok(())
}

fn check_accountability_performance_demand_response_status_applied_example(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_status_applied_example(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_APPLIED_EXAMPLE_PATH,
        &expected,
        "accountability performance demand response status applied example",
    )?;

    let parsed_text = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_APPLIED_EXAMPLE_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_APPLIED_EXAMPLE_PATH}: {err}"
        )
    })?;
    let status: PerformanceDemandResponseStatus =
        serde_json::from_str(&parsed_text).map_err(|err| {
            format!(
                "failed to parse {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_APPLIED_EXAMPLE_PATH}: {err}"
            )
        })?;
    status.validate()?;
    if status.total_rows == status.not_yet_received {
        return Err(
            "response status applied example must include at least one updated row".to_string(),
        );
    }
    if status.public_claim_allowed != 0 {
        return Err(
            "response status applied example unexpectedly allows a public claim".to_string(),
        );
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-status.applied-example.json") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-status.applied-example.json"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response status applied example");
    Ok(())
}

fn check_accountability_performance_demand_response_dashboard_applied_example(
    root: &Path,
) -> Result<(), String> {
    let expected =
        build_accountability_performance_demand_response_dashboard_applied_example(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DASHBOARD_APPLIED_EXAMPLE_PATH,
        &expected,
        "accountability performance demand response dashboard applied example",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-dashboard.applied-example.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-dashboard.applied-example.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response dashboard applied example");
    Ok(())
}

fn check_accountability_performance_demand_response_handoff_applied_example(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_handoff_applied_example(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_HANDOFF_APPLIED_EXAMPLE_PATH,
        &expected,
        "accountability performance demand response handoff applied example",
    )?;

    let handoff_text = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_HANDOFF_APPLIED_EXAMPLE_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_HANDOFF_APPLIED_EXAMPLE_PATH}: {err}"
        )
    })?;
    for required in [
        "performance-demand-response-delta.applied-example.md",
        "performance-demand-response-delta.applied-example.jsonl",
        "performance-demand-response-delta.applied-example.schema.md",
        "performance-demand-response-bundle.applied-example.md",
        "performance-demand-response-bundle.applied-example.json",
        "performance-demand-response-bundle.applied-example.schema.md",
    ] {
        if !handoff_text.contains(required) {
            return Err(format!(
                "{ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_HANDOFF_APPLIED_EXAMPLE_PATH} must route {required}"
            ));
        }
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-handoff.applied-example.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-handoff.applied-example.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response handoff applied example");
    Ok(())
}

fn check_accountability_performance_demand_response_applied_example_schema(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_applied_example_schema();
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_APPLIED_EXAMPLE_SCHEMA_PATH,
        &expected,
        "accountability performance demand response applied example schema",
    )?;

    let schema_text = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_APPLIED_EXAMPLE_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_APPLIED_EXAMPLE_SCHEMA_PATH}: {err}"
        )
    })?;
    for required in [
        "performance-demand-response-delta.applied-example.md",
        "performance-demand-response-delta.applied-example.jsonl",
        "performance-demand-response-delta.applied-example.schema.md",
    ] {
        if !schema_text.contains(required) {
            return Err(format!(
                "{ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_APPLIED_EXAMPLE_SCHEMA_PATH} must document {required}"
            ));
        }
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-applied-example.schema.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-applied-example.schema.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response applied example schema");
    Ok(())
}

fn check_accountability_performance_demand_response_delta_applied_example(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_delta_applied_example(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_PATH,
        &expected,
        "accountability performance demand response delta applied example",
    )?;

    let delta_text = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_PATH}: {err}"
        )
    })?;
    if !delta_text.contains("- Updated rows: 1") {
        return Err("response delta applied example must report one updated row".to_string());
    }
    if !delta_text.contains(PUBLIC_CLAIM_BLOCKED_LABEL) {
        return Err(
            "response delta applied example must preserve blocked public-claim gates".to_string(),
        );
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-delta.applied-example.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-delta.applied-example.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response delta applied example");
    Ok(())
}

fn check_accountability_performance_demand_response_delta_applied_example_jsonl(
    root: &Path,
) -> Result<(), String> {
    let expected =
        build_accountability_performance_demand_response_delta_applied_example_jsonl(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_JSONL_PATH,
        &expected,
        "accountability performance demand response delta applied example JSONL",
    )?;

    let rows: Vec<PerformanceDemandResponseDeltaRow> = read_jsonl(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_JSONL_PATH),
    )?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!("response delta applied example JSONL: invalid row shape: {err}")
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if rows.is_empty() {
        return Err(
            "performance demand response delta applied example JSONL has no rows".to_string(),
        );
    }
    for row in rows {
        row.validate()?;
        if row.after_claim_gate != PUBLIC_CLAIM_BLOCKED_LABEL {
            return Err(
                "response delta applied example JSONL changed the blocked claim gate".to_string(),
            );
        }
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-delta.applied-example.jsonl") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-delta.applied-example.jsonl"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response delta applied example JSONL");
    Ok(())
}

fn check_accountability_performance_demand_response_delta_applied_example_schema(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_delta_applied_example_schema();
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_SCHEMA_PATH,
        &expected,
        "accountability performance demand response delta applied example schema",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-delta.applied-example.schema.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-delta.applied-example.schema.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response delta applied example schema");
    Ok(())
}

fn check_accountability_performance_demand_response_bundle_applied_example(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_bundle_applied_example(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_PATH,
        &expected,
        "accountability performance demand response bundle applied example",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-bundle.applied-example.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-bundle.applied-example.md"
                .to_string(),
        );
    }

    let bundle = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_PATH}: {err}"
        )
    })?;
    for required in [
        "performance-demand-response-intake.example.jsonl",
        "performance-demand-response-log.applied-example.jsonl",
        "performance-demand-response-status.applied-example.json",
        "performance-demand-response-dashboard.applied-example.md",
        "performance-demand-response-handoff.applied-example.md",
        "performance-demand-response-applied-example.schema.md",
        "performance-demand-response-delta.applied-example.md",
        "performance-demand-response-delta.applied-example.jsonl",
        "performance-demand-response-delta.applied-example.schema.md",
        "performance-demand-response-bundle.applied-example.json",
    ] {
        if !bundle.contains(required) {
            return Err(format!(
                "{ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_PATH} must include {required}"
            ));
        }
    }

    println!("validated accountability performance demand response bundle applied example");
    Ok(())
}

fn check_accountability_performance_demand_response_bundle_applied_example_json(
    root: &Path,
) -> Result<(), String> {
    let expected =
        build_accountability_performance_demand_response_bundle_applied_example_json(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_JSON_PATH,
        &expected,
        "accountability performance demand response bundle applied example JSON",
    )?;

    let manifest_text = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_JSON_PATH}: {err}"
        )
    })?;
    let manifest: PerformanceDemandResponseBundleManifest = serde_json::from_str(&manifest_text)
        .map_err(|err| format!("failed to parse applied response bundle JSON: {err}"))?;
    manifest.validate()?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-bundle.applied-example.json") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-bundle.applied-example.json"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response bundle applied example JSON");
    Ok(())
}

fn check_accountability_performance_demand_response_bundle_applied_example_schema(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_bundle_applied_example_schema();
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_SCHEMA_PATH,
        &expected,
        "accountability performance demand response bundle applied example schema",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-bundle.applied-example.schema.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-bundle.applied-example.schema.md"
                .to_string(),
        );
    }

    let schema = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_SCHEMA_PATH}: {err}"
        )
    })?;
    for required in [
        "PerformanceDemandResponseBundleManifest",
        "PerformanceDemandResponseBundleArtifact",
        "`artifact`",
        "`bundle_kind`",
        "`total_rows`",
        "`updated_rows`",
        "`public_claim_allowed`",
        "`public_claim_blocked`",
        "`artifacts`",
        "`boundary`",
        "`use_rule`",
        "`row_count`",
        "`sha256`",
    ] {
        if !schema.contains(required) {
            return Err(format!(
                "{ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_SCHEMA_PATH} must document {required}"
            ));
        }
    }

    println!("validated accountability performance demand response bundle applied example schema");
    Ok(())
}

fn build_accountability_readiness_report(root: &Path) -> Result<String, String> {
    let records = read_accountability_evidence_records(root)?;
    let mut lines = vec![
        "# Accountability Evidence Readiness Report".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This report classifies draft accountability evidence records by public-claim readiness.".to_string(),
        "It is not a list of fraud, waste, abuse, or performance findings.".to_string(),
        String::new(),
        "## Readiness States".to_string(),
        String::new(),
        "| State | Meaning |".to_string(),
        "|---|---|".to_string(),
        "| `EvidenceOnly` | Internal evidence review only; not ready for public claims. |".to_string(),
        "| `NeedsRoleReview` | Source/accountability reviewed and waiting for public wording review. |".to_string(),
        "| `PublicClaimEligible` | Role reviewed with official finding or adjudicated status. |".to_string(),
        String::new(),
        "## Records".to_string(),
        String::new(),
        "| Record ID | Lane | Evidence Kind | Anomaly Class | Allegation Status | Review Status | Readiness | Next Action | Public Summary |".to_string(),
        "|---|---|---|---|---|---|---|---|---|".to_string(),
    ];

    for record in records {
        let readiness = record.public_claim_readiness();
        lines.push(format!(
            "| `{}` | {} | {:?} | {:?} | {:?} | {:?} | `{}` | {} | {} |",
            record.record_id,
            record.lane_id.as_deref().unwrap_or("n/a"),
            record.evidence_kind,
            record.anomaly_class,
            record.allegation_status,
            record.review_status,
            readiness.as_str(),
            record.accountability_next_action().replace('|', "\\|"),
            record.public_summary.replace('|', "\\|")
        ));
    }

    lines.push(String::new());
    lines.push("## Guardrail".to_string());
    lines.push(String::new());
    lines.push(
        "Records marked `EvidenceOnly` or `NeedsRoleReview` must not be presented as public fraud, waste, abuse, or performance findings.".to_string(),
    );

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_action_queue(root: &Path) -> Result<String, String> {
    let records = read_accountability_evidence_records(root)?;
    let mut queue: BTreeMap<&'static str, Vec<AccountabilityEvidenceRecord>> = BTreeMap::new();
    for record in records {
        queue
            .entry(record.accountability_next_action())
            .or_default()
            .push(record);
    }

    let mut lines = vec![
        "# Accountability Evidence Action Queue".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated queue turns draft accountability evidence records into reviewer work."
            .to_string(),
        "It is not a public fraud, waste, abuse, or performance scorecard.".to_string(),
        String::new(),
        "## Queue".to_string(),
    ];

    for (action, mut records) in queue {
        records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
        lines.push(String::new());
        lines.push(format!("### {action}"));
        lines.push(String::new());
        lines.push("| Record ID | Lane | Readiness | Public-Use Blocker |".to_string());
        lines.push("|---|---|---|---|".to_string());
        for record in records {
            lines.push(format!(
                "| `{}` | {} | `{}` | {} |",
                record.record_id,
                record.lane_id.as_deref().unwrap_or("n/a"),
                record.public_claim_readiness().as_str(),
                record
                    .accountability_public_use_blocker()
                    .replace('|', "\\|")
            ));
        }
    }

    lines.push(String::new());
    lines.push("## Guardrail".to_string());
    lines.push(String::new());
    lines.push(
        "Queue entries are tasks for evidence review. They are not publishable claims by themselves."
            .to_string(),
    );

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_packet(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Accountability Performance Demand Packet".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated packet turns accountability evidence blockers into questions people can ask before demanding performance on public money.".to_string(),
        "It explains what TAXLANE can say now, what evidence is still missing, and what claim boundary remains in force.".to_string(),
        String::new(),
        "## Demand Questions".to_string(),
        String::new(),
        "| Record ID | Lane | What TAXLANE Can Say Now | Demand Question | Claim Boundary |".to_string(),
        "|---|---|---|---|---|".to_string(),
    ];

    for record in records {
        lines.push(format!(
            "| `{}` | {} | {} | {} | {} |",
            record.record_id,
            record.lane_id.as_deref().unwrap_or("n/a"),
            record.public_summary.replace('|', "\\|"),
            record.accountability_demand_question().replace('|', "\\|"),
            record
                .accountability_public_use_blocker()
                .replace('|', "\\|")
        ));
    }

    lines.push(String::new());
    lines.push("## Public-Use Rule".to_string());
    lines.push(String::new());
    lines.push(
        "Use these rows to request evidence, reviewed wording, or official findings. Do not present them as fraud, waste, abuse, or performance findings.".to_string(),
    );

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_work_items_jsonl(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    let mut lines = Vec::new();
    for record in records {
        lines.push(
            serde_json::to_string(&record.accountability_work_item())
                .map_err(|err| format!("failed to serialize accountability work item: {err}"))?,
        );
    }
    Ok(lines.join("\n") + "\n")
}

fn build_accountability_claim_guard_report(root: &Path) -> Result<String, String> {
    let records = read_accountability_evidence_records(root)?;
    let mut readiness_counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut blocker_counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut public_claim_allowed = 0usize;

    for record in &records {
        let work_item = record.accountability_work_item();
        *readiness_counts.entry(work_item.readiness).or_default() += 1;
        *blocker_counts
            .entry(work_item.public_use_blocker)
            .or_default() += 1;
        if work_item.public_claim_allowed {
            public_claim_allowed += 1;
        }
    }

    let mut lines = vec![
        "# Accountability Claim Guard Report".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated report summarizes whether accountability evidence records can support public claims.".to_string(),
        "It is a guardrail report, not a fraud, waste, abuse, or performance scorecard.".to_string(),
        String::new(),
        "## Claim Guard Summary".to_string(),
        String::new(),
        format!("- Total records: {}", records.len()),
        format!("- Public claims currently allowed: {public_claim_allowed}"),
        format!(
            "- Public claims currently blocked: {}",
            records.len().saturating_sub(public_claim_allowed)
        ),
        String::new(),
        "## Readiness Counts".to_string(),
        String::new(),
        "| Readiness | Records |".to_string(),
        "|---|---:|".to_string(),
    ];

    for (readiness, count) in readiness_counts {
        lines.push(format!("| `{readiness}` | {count} |"));
    }

    lines.extend([
        String::new(),
        "## Public-Use Blockers".to_string(),
        String::new(),
        "| Blocker | Records |".to_string(),
        "|---|---:|".to_string(),
    ]);

    for (blocker, count) in blocker_counts {
        let escaped_blocker = blocker.replace('|', "\\|");
        lines.push(format!("| {escaped_blocker} | {count} |"));
    }

    lines.extend([
        String::new(),
        "## Allowed Public Use".to_string(),
        String::new(),
        "Current safe use: ask the demand questions and request the missing reviewed evidence or role-approved wording.".to_string(),
        "Current unsafe use: present these draft records as fraud, waste, abuse, or performance findings.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_public_questions(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Public Accountability Questions".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "These generated questions are safe to ask publicly because they request reviewed evidence or role-approved wording.".to_string(),
        "They are not findings of fraud, waste, abuse, or poor performance.".to_string(),
        String::new(),
        "## Questions".to_string(),
        String::new(),
        "| Lane | Public-Safe Question | Why This Is Still Blocked |".to_string(),
        "|---|---|---|".to_string(),
    ];

    for record in records {
        let work_item = record.accountability_work_item();
        lines.push(format!(
            "| {} | {} | {} |",
            work_item.lane_id.unwrap_or("n/a"),
            work_item.demand_question.replace('|', "\\|"),
            work_item.public_use_blocker.replace('|', "\\|")
        ));
    }

    lines.extend([
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Use these questions to ask for evidence. Do not present the underlying draft records as public claims.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_public_brief(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    let total_records = records.len();
    let public_claim_allowed = records
        .iter()
        .filter(|record| record.accountability_work_item().public_claim_allowed)
        .count();

    let mut lines = vec![
        "# Accountability Public Brief".to_string(),
        String::new(),
        "## What TAXLANE Can Say Now".to_string(),
        String::new(),
        "TAXLANE can model how ordinary individual income-tax receipts compare with broad federal outlay categories.".to_string(),
        "That model is a visibility tool, not a legal claim that a taxpayer's dollars are dedicated to a specific program.".to_string(),
        String::new(),
        "TAXLANE can also ask accountability questions about whether spending has reviewed performance evidence.".to_string(),
        "Current accountability records are not public findings of fraud, waste, abuse, or poor performance.".to_string(),
        String::new(),
        "## Current Claim Guard".to_string(),
        String::new(),
        format!("- Accountability records reviewed for public use: {total_records}"),
        format!("- Records currently public-claim eligible: {public_claim_allowed}"),
        format!(
            "- Records still blocked from public claims: {}",
            total_records.saturating_sub(public_claim_allowed)
        ),
        String::new(),
        "## Safe Public Questions".to_string(),
        String::new(),
        "| Lane | Question To Ask | Why It Matters |".to_string(),
        "|---|---|---|".to_string(),
    ];

    for record in records {
        let work_item = record.accountability_work_item();
        lines.push(format!(
            "| {} | {} | {} |",
            work_item.lane_id.unwrap_or("n/a"),
            work_item.demand_question.replace('|', "\\|"),
            work_item.public_use_blocker.replace('|', "\\|")
        ));
    }

    lines.extend([
        String::new(),
        "## Use / Avoid".to_string(),
        String::new(),
        "| Use | Avoid |".to_string(),
        "|---|---|".to_string(),
        "| Ask for reviewed performance targets, outcome measures, audit sources, or role-approved wording. | Do not say TAXLANE found fraud, waste, abuse, or poor performance from these draft records. |".to_string(),
        "| Use modeled allocation language when explaining income-tax visibility. | Do not say ordinary income-tax dollars are legally dedicated to the displayed lanes. |".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_artifact_map() -> String {
    let rows = [
        (
            "accountability_evidence.omb-fy2027-v1.2026-06-23.draft.jsonl",
            "Internal evidence reviewers",
            "Validate source-custodied evidence shape.",
            "Do not publish as findings.",
        ),
        (
            "readiness-report.md",
            "Accountability researchers",
            "See readiness and next action per record.",
            "Do not treat readiness as a performance score.",
        ),
        (
            "action-queue.md",
            "Review leads",
            "Work records by next task.",
            "Do not publish queue rows as claims.",
        ),
        (
            "performance-demand-packet.md",
            "Accountability researchers",
            "Ask what evidence, reviewed wording, or official finding is missing.",
            "Do not allege misconduct.",
        ),
        (
            "accountability-work-items.jsonl",
            "Product implementers",
            "Feed future UI/API workflow from structured fields.",
            "Do not infer public eligibility except from `public_claim_allowed`.",
        ),
        (
            "claim-guard-report.md",
            "Review leads",
            "Check allowed versus blocked public claims.",
            "Do not publish findings from blocked records.",
        ),
        (
            "public-questions.md",
            "Citizen readers",
            "Ask safe public questions about performance evidence.",
            "Do not expose raw draft evidence as claims.",
        ),
        (
            "performance-demand-checklist.md",
            "Citizen readers",
            "Demand source, performance, official-finding, wording, and claim-gate evidence.",
            "Do not treat demand rows as findings.",
        ),
        (
            "performance-demand-dashboard.md",
            "Citizen readers",
            "Scan demand-row claim gates before public use.",
            "Do not publish blocked rows as claims.",
        ),
        (
            "performance-demand-brief.md",
            "Citizen readers",
            "Use a compact ask packet for current blocked demand rows.",
            "Do not present the brief as a finding or scorecard.",
        ),
        (
            "performance-demand-letter.md",
            "Citizen readers",
            "Adapt a public-safe evidence request template.",
            "Do not send it as an accusation or legal conclusion.",
        ),
        (
            "performance-demand-response-rubric.md",
            "Citizen readers",
            "Classify replies to evidence requests.",
            "Do not turn incomplete replies into findings.",
        ),
        (
            "performance-demand-followup.md",
            "Citizen readers",
            "Send a narrower follow-up for missing evidence.",
            "Do not escalate missing evidence into accusations.",
        ),
        (
            "performance-demand-response-log.md",
            "Citizen readers",
            "Track replies and remaining missing evidence.",
            "Do not treat log status as a finding.",
        ),
        (
            "performance-demand-response-log.jsonl",
            "Product implementers",
            "Feed neutral response log rows into future UI/API surfaces.",
            "Do not infer public eligibility except from `public_claim_allowed`.",
        ),
        (
            "performance-demand-response-log.schema.md",
            "Product implementers",
            "Inspect the response log row contract.",
            "Do not add UI/API fields that weaken the use rule.",
        ),
        (
            "performance-demand-response-status.json",
            "Product implementers",
            "Display response-log counts without recomputing rows.",
            "Do not treat status counts as findings.",
        ),
        (
            "performance-demand-response-dashboard.md",
            "Citizen readers",
            "Scan response-log counts without opening JSON.",
            "Do not treat dashboard counts as findings.",
        ),
        (
            "performance-demand-response-handoff.md",
            "Citizen readers / product implementers",
            "Choose the response tracking artifact for each task.",
            "Do not treat navigation guidance as findings.",
        ),
        (
            "performance-demand-response-intake.md",
            "Citizen readers / product implementers",
            "Capture reply evidence before updating response status.",
            "Do not treat unreviewed replies as findings or claim eligibility.",
        ),
        (
            "performance-demand-response-intake.schema.md",
            "Product implementers",
            "Inspect the reply intake field contract.",
            "Do not add importer fields that bypass role review or claim gates.",
        ),
        (
            "performance-demand-response-intake.example.jsonl",
            "Product implementers",
            "Exercise the typed intake-to-log importer handoff.",
            "Do not treat example replies as findings or claim eligibility.",
        ),
        (
            "performance-demand-response-log.applied-example.jsonl",
            "Product implementers",
            "Inspect response-log rows after applying example intake.",
            "Do not treat applied example rows as findings or claim eligibility.",
        ),
        (
            "performance-demand-response-status.applied-example.json",
            "Product implementers",
            "Display applied response-log counts without recomputing rows.",
            "Do not treat applied status counts as findings.",
        ),
        (
            "performance-demand-response-dashboard.applied-example.md",
            "Product implementers",
            "Scan applied response-log counts without opening JSON.",
            "Do not treat applied dashboard counts as findings.",
        ),
        (
            "performance-demand-response-handoff.applied-example.md",
            "Product implementers",
            "Route the response importer fixture artifacts by task.",
            "Do not treat applied handoff guidance as findings.",
        ),
        (
            "performance-demand-response-applied-example.schema.md",
            "Product implementers",
            "Inspect the applied importer fixture artifact contract.",
            "Do not weaken intake, log, status, or claim-gate guardrails.",
        ),
        (
            "performance-demand-response-delta.applied-example.md",
            "Product implementers",
            "Inspect exact row-level changes after applying example intake.",
            "Do not treat applied deltas as findings or canonical status.",
        ),
        (
            "performance-demand-response-delta.applied-example.jsonl",
            "Product implementers",
            "Feed exact applied response delta rows into future UI/API surfaces.",
            "Do not treat applied delta rows as findings or canonical status.",
        ),
        (
            "performance-demand-response-delta.applied-example.schema.md",
            "Product implementers",
            "Inspect the applied response delta row contract.",
            "Do not add UI/API fields that weaken fixture or claim-gate guardrails.",
        ),
        (
            "performance-demand-response-bundle.applied-example.md",
            "Product implementers",
            "Open one index for every applied response importer fixture artifact.",
            "Do not treat bundle membership as canonical response status or findings.",
        ),
        (
            "performance-demand-response-bundle.applied-example.json",
            "Product implementers",
            "Load the applied response importer fixture bundle without scraping Markdown.",
            "Do not treat manifest rows as canonical response status or findings.",
        ),
        (
            "performance-demand-response-bundle.applied-example.schema.md",
            "Product implementers",
            "Inspect the applied response bundle JSON contract.",
            "Do not add fields that weaken fixture-only or blocked-claim boundaries.",
        ),
        (
            "performance-demand-checklist.jsonl",
            "Product implementers",
            "Feed demand rows into future UI/API surfaces.",
            "Do not infer public eligibility except from `public_claim_allowed`.",
        ),
        (
            "performance-demand-claim-gates.json",
            "Product implementers",
            "Display allowed versus blocked demand-row counts.",
            "Do not recompute or override claim gates downstream.",
        ),
        (
            "performance-demand-checklist.schema.md",
            "Product implementers",
            "Inspect the demand checklist row contract.",
            "Do not add UI/API fields that weaken the use rule.",
        ),
        (
            "docs/reading/accountability-public-brief.md",
            "Citizen readers",
            "Read the current public handoff.",
            "Do not describe modeled allocation as legal dedication.",
        ),
    ];

    let mut lines = vec![
        "# Accountability Artifact Map".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This map shows which accountability artifact to use for evidence review, performance-demand questions, and public-safe reader handoff.".to_string(),
        "It is not a list of fraud, waste, abuse, or performance findings.".to_string(),
        String::new(),
        "## Use Order".to_string(),
        String::new(),
        "1. Start with the draft JSONL records for source custody.".to_string(),
        "2. Use readiness, queue, demand, work-item, and claim-guard artifacts for internal review workflow.".to_string(),
        "3. Use public questions and the public brief only for outward-facing questions and handoff wording.".to_string(),
        String::new(),
        "## Artifacts".to_string(),
        String::new(),
        "| Artifact | Audience | Use | Avoid |".to_string(),
        "|---|---|---|---|".to_string(),
    ];

    for (artifact, audience, use_case, avoid) in rows {
        lines.push(format!(
            "| `{artifact}` | {audience} | {use_case} | {avoid} |"
        ));
    }

    lines.extend([
        String::new(),
        "## Public-Use Rule".to_string(),
        String::new(),
        "Public artifacts may ask for performance evidence and official findings. They must not claim fraud, waste, abuse, legal dedication of income taxes, or program performance without reviewed evidence and claim eligibility.".to_string(),
    ]);

    lines.join("\n") + "\n"
}

fn build_accountability_performance_demand_checklist(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Performance Demand Checklist".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated checklist turns TAXLANE accountability blockers into evidence requests a citizen can make before accepting performance or misconduct claims.".to_string(),
        "It is not a finding of fraud, waste, abuse, or poor performance.".to_string(),
        String::new(),
        "## Before Accepting A Claim".to_string(),
        String::new(),
        "- Ask for the source record and source version.".to_string(),
        "- Ask for the reviewed performance target, outcome measure, audit source, or official finding.".to_string(),
        "- Ask whether role review approved the exact public wording.".to_string(),
        "- Ask whether the record is public-claim eligible.".to_string(),
        String::new(),
        "## Record Checklist".to_string(),
        String::new(),
        "| Lane | Demand This Evidence | Do Not Accept Yet | Claim Gate |".to_string(),
        "|---|---|---|---|".to_string(),
    ];

    for record in records {
        let work_item = record.accountability_work_item();
        let claim_gate = if work_item.public_claim_allowed {
            PUBLIC_CLAIM_ALLOWED_LABEL
        } else {
            PUBLIC_CLAIM_BLOCKED_LABEL
        };
        lines.push(format!(
            "| {} | {} | {} | {} |",
            work_item.lane_id.unwrap_or("n/a"),
            work_item.demand_question.replace('|', "\\|"),
            work_item.public_use_blocker.replace('|', "\\|"),
            claim_gate
        ));
    }

    lines.extend([
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Use this checklist to demand performance evidence and reviewed wording. Do not use it to claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, or poor performance.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_checklist_jsonl(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = Vec::new();
    for record in records {
        let row = record.performance_demand_checklist_row();
        lines.push(
            serde_json::to_string(&row)
                .map_err(|err| format!("failed to serialize demand checklist row: {err}"))?,
        );
    }

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_claim_gates(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    let rows: Vec<PerformanceDemandChecklistRecord> = records
        .iter()
        .map(AccountabilityEvidenceRecord::performance_demand_checklist_record)
        .collect();
    let total_rows = rows.len();
    let public_claim_allowed = rows.iter().filter(|row| row.public_claim_allowed).count();
    let public_claim_blocked = total_rows.saturating_sub(public_claim_allowed);
    let mut gate_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for row in &rows {
        *gate_counts.entry(&row.claim_gate).or_default() += 1;
    }
    let claim_gates: Vec<String> = gate_counts
        .into_iter()
        .map(|(claim_gate, rows)| {
            format!(
                "    {{\"claim_gate\":{},\"rows\":{rows}}}",
                json_string(claim_gate)
            )
        })
        .collect();

    Ok(format!(
        concat!(
            "{{\n",
            "  \"artifact\": {},\n",
            "  \"total_rows\": {},\n",
            "  \"public_claim_allowed\": {},\n",
            "  \"public_claim_blocked\": {},\n",
            "  \"claim_gates\": [\n",
            "{}\n",
            "  ],\n",
            "  \"use_rule\": {}\n",
            "}}\n"
        ),
        json_string(ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_JSONL_PATH),
        total_rows,
        public_claim_allowed,
        public_claim_blocked,
        claim_gates.join(",\n"),
        json_string(
            "Demand evidence and reviewed wording; do not claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, or poor performance."
        )
    ))
}

fn build_accountability_performance_demand_dashboard(root: &Path) -> Result<String, String> {
    let claim_gates_text = build_accountability_performance_demand_claim_gates(root)?;
    let claim_gates: serde_json::Value =
        serde_json::from_str(&claim_gates_text).map_err(|err| {
            format!("failed to parse generated performance demand claim gates: {err}")
        })?;
    let total_rows = claim_gates
        .get("total_rows")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "generated claim gates missing total_rows".to_string())?;
    let allowed_rows = claim_gates
        .get("public_claim_allowed")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "generated claim gates missing public_claim_allowed".to_string())?;
    let blocked_rows = claim_gates
        .get("public_claim_blocked")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "generated claim gates missing public_claim_blocked".to_string())?;
    let use_rule = claim_gates
        .get("use_rule")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "generated claim gates missing use_rule".to_string())?;

    let mut lines = vec![
        "# Performance Demand Dashboard".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated dashboard summarizes whether performance demand checklist rows can support public claims.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, or poor performance.".to_string(),
        String::new(),
        "## Claim Gate Summary".to_string(),
        String::new(),
        format!("- Demand rows: {total_rows}"),
        format!("- Public claims currently allowed: {allowed_rows}"),
        format!("- Public claims currently blocked: {blocked_rows}"),
        String::new(),
        "## Claim Gates".to_string(),
        String::new(),
        "| Claim Gate | Rows |".to_string(),
        "|---|---:|".to_string(),
    ];

    let gate_rows = claim_gates
        .get("claim_gates")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "generated claim gates missing claim_gates".to_string())?;
    for gate in gate_rows {
        let claim_gate = gate
            .get("claim_gate")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "generated claim gate row missing claim_gate".to_string())?;
        let rows = gate
            .get("rows")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| "generated claim gate row missing rows".to_string())?;
        lines.push(format!("| {} | {rows} |", claim_gate.replace('|', "\\|")));
    }

    lines.extend([
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        use_rule.to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_brief(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Performance Demand Brief".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated brief turns blocked performance demand rows into a compact ask packet for citizen readers.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Current Claim Status".to_string(),
        String::new(),
        "TAXLANE currently has no performance demand rows that are public-claim eligible.".to_string(),
        "Use the rows below to ask for evidence, not to assert wrongdoing or performance failure.".to_string(),
        String::new(),
        "## Ask Packet".to_string(),
    ];

    for record in records {
        let row = record.performance_demand_checklist_record();
        let label = row.lane_id.as_deref().unwrap_or("n/a");
        lines.extend([
            String::new(),
            format!("### {label}"),
            String::new(),
            format!("- Ask: {}", row.demand_question),
            format!("- Do not accept yet: {}", row.do_not_accept_yet),
            format!("- Claim gate: {}", row.claim_gate),
            format!("- Public claim allowed: {}", row.public_claim_allowed),
            "- Required evidence:".to_string(),
        ]);
        for evidence in row.demand_evidence {
            lines.push(format!("  - {evidence}"));
        }
    }

    lines.extend([
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Use this brief to demand source records, reviewed performance evidence, official findings, role-approved wording, and public-claim eligibility. Do not use it to claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_letter(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Performance Demand Letter Template".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated template helps a citizen ask for performance evidence behind public-money claims.".to_string(),
        "It is not an accusation, legal demand, fraud finding, waste finding, abuse finding, or performance scorecard.".to_string(),
        String::new(),
        "## Template".to_string(),
        String::new(),
        "Subject: Request for reviewed performance evidence and public-claim basis".to_string(),
        String::new(),
        "To [office or program contact],".to_string(),
        String::new(),
        "I am reviewing public-money claims with TAXLANE's modeled income-tax visibility materials. I understand those materials do not show legal dedication of income-tax dollars and do not, by themselves, prove fraud, waste, abuse, or poor performance.".to_string(),
        String::new(),
        "Before I accept or repeat a public claim, please provide the evidence listed below or identify where it is already published.".to_string(),
        String::new(),
        "Requested evidence:".to_string(),
        String::new(),
        "- Source record and source version.".to_string(),
        "- Reviewed performance target, outcome measure, audit source, or official finding.".to_string(),
        "- Exact public wording approved by role review, if any.".to_string(),
        "- Public-claim eligibility basis for any performance or misconduct statement.".to_string(),
        String::new(),
        "Current TAXLANE demand rows to resolve:".to_string(),
    ];

    for record in records {
        let row = record.performance_demand_checklist_record();
        let label = row.lane_id.as_deref().unwrap_or("n/a");
        lines.extend([
            String::new(),
            format!("- {label}: {}", row.demand_question),
            format!("  Blocker: {}", row.do_not_accept_yet),
            format!("  Claim gate: {}", row.claim_gate),
        ]);
    }

    lines.extend([
        String::new(),
        "Please treat this as a request for evidence and reviewed wording, not as an allegation that misconduct or poor performance occurred.".to_string(),
        String::new(),
        "Sincerely,".to_string(),
        String::new(),
        "[name]".to_string(),
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Customize names, dates, and addressee details before use. Keep the modeled-not-legal tax boundary and no-finding language intact unless reviewed evidence and public-claim eligibility support a stronger statement.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_response_rubric(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Performance Demand Response Rubric".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated rubric helps classify replies to performance evidence requests.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Score Meanings".to_string(),
        String::new(),
        "| Response Class | Meaning | Next Action |".to_string(),
        "|---|---|---|".to_string(),
    ];

    for response_class in PerformanceDemandResponseLogClass::rubric_classes() {
        lines.push(format!(
            "| {} | {} | {} |",
            response_class.label(),
            response_class.rubric_meaning(),
            response_class.rubric_next_action()
        ));
    }

    lines.extend([
        String::new(),
        "## Row-Specific Checks".to_string(),
        String::new(),
        "| Lane | Original Ask | Current Blocker | Response Must Provide |".to_string(),
        "|---|---|---|---|".to_string(),
    ]);

    for record in records {
        let row = record.performance_demand_checklist_record();
        let label = row.lane_id.as_deref().unwrap_or("n/a");
        lines.push(format!(
            "| {label} | {} | {} | source version; reviewed performance evidence or official finding; role-approved wording; public-claim basis |",
            row.demand_question.replace('|', "\\|"),
            row.do_not_accept_yet.replace('|', "\\|")
        ));
    }

    lines.extend([
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Use this rubric to decide what evidence is still missing after a reply. Do not use an incomplete, process-only, or no-evidence response to claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_followup(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Performance Demand Follow-Up Template".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated template helps a citizen follow up when a reply is partial, process-only, or provides no evidence.".to_string(),
        "It is not an accusation, legal demand, fraud finding, waste finding, abuse finding, or performance scorecard.".to_string(),
        String::new(),
        "## Template".to_string(),
        String::new(),
        "Subject: Follow-up request for missing performance evidence".to_string(),
        String::new(),
        "To [office or program contact],".to_string(),
        String::new(),
        "Thank you for the response. I am treating it as an evidence response, not as proof of misconduct or poor performance.".to_string(),
        String::new(),
        "The reply appears to leave at least one requested item missing or unclear. Please provide the missing item, identify where it is published, or state that the office does not have it.".to_string(),
        String::new(),
        "Missing evidence to clarify:".to_string(),
        String::new(),
        "- Source record and source version, if not already provided.".to_string(),
        "- Reviewed performance target, outcome measure, audit source, or official finding, if not already provided.".to_string(),
        "- Exact role-approved public wording, if any.".to_string(),
        "- Public-claim eligibility basis for any performance or misconduct statement.".to_string(),
        String::new(),
        "Current unresolved TAXLANE demand rows:".to_string(),
    ];

    for record in records {
        let row = record.performance_demand_checklist_record();
        let label = row.lane_id.as_deref().unwrap_or("n/a");
        lines.extend([
            String::new(),
            format!("- {label}: {}", row.do_not_accept_yet),
            format!("  Follow-up ask: {}", row.demand_question),
            format!("  Claim gate remains: {}", row.claim_gate),
        ]);
    }

    lines.extend([
        String::new(),
        "Please keep this as an evidence clarification request. If the evidence does not exist or is not yet reviewed, a clear statement of that status is useful.".to_string(),
        String::new(),
        "Sincerely,".to_string(),
        String::new(),
        "[name]".to_string(),
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Use this follow-up only after a reply leaves requested evidence missing or unclear. Do not use a missing or incomplete reply to claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_response_log(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Performance Demand Response Log".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated log gives each current performance demand row a neutral place to track replies and remaining evidence gaps.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Response Classes".to_string(),
        String::new(),
    ];

    for response_class in PerformanceDemandResponseLogClass::all_classes() {
        lines.push(format!(
            "- `{}`: {}",
            response_class.wire_value(),
            response_class.rubric_meaning()
        ));
    }

    lines.extend([
        String::new(),
        "## Current Log".to_string(),
        String::new(),
        "| Lane | Response Class | Evidence Received | Missing Evidence | Claim Gate | Next Action |"
            .to_string(),
        "|---|---|---|---|---|---|".to_string(),
    ]);

    for record in records {
        let row = record.performance_demand_response_log_record();
        row.validate()?;
        let label = row.lane_id.as_deref().unwrap_or("n/a");
        let evidence_received = if row.evidence_received.is_empty() {
            "none logged".to_string()
        } else {
            row.evidence_received.join("; ")
        };
        lines.push(format!(
            "| {label} | `{}` | {} | {} | {} | {} |",
            row.response_class.wire_value(),
            evidence_received.replace('|', "\\|"),
            row.missing_evidence.replace('|', "\\|"),
            row.claim_gate.replace('|', "\\|"),
            row.next_action.replace('|', "\\|")
        ));
    }

    lines.extend([
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Use this log to track response status and remaining evidence gaps. Do not use an empty, partial, process-only, or no-evidence log row to claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_response_log_jsonl(
    root: &Path,
) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = Vec::new();
    for record in records {
        let row = record.performance_demand_response_log_record();
        row.validate()?;
        lines.push(
            serde_json::to_string(&row)
                .map_err(|err| format!("failed to serialize response log row: {err}"))?,
        );
    }

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_response_log_schema() -> String {
    let mut lines = vec![
        "# Performance Demand Response Log JSONL Schema".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This schema documents the generated `performance-demand-response-log.jsonl` rows."
            .to_string(),
        "Rows track replies to evidence requests without changing claim gates or creating findings."
            .to_string(),
        String::new(),
        "## Row Fields".to_string(),
        String::new(),
        "| Field | Type | Required | Meaning |".to_string(),
        "|---|---|---|---|".to_string(),
        "| `record_id` | string | yes | Accountability evidence record ID. |".to_string(),
        "| `lane_id` | string or null | conditional | Public-purpose lane when available. |"
            .to_string(),
        "| `program_or_account_id` | string or null | conditional | Program, account, or OMB function identifier when available. |".to_string(),
        "| `response_class` | string | yes | Current response status. Initial generated value is `not-yet-received`. |".to_string(),
        "| `evidence_received` | array of strings | yes | Evidence items logged from a reply. Initial generated value is empty. |".to_string(),
        "| `missing_evidence` | string | yes | Current blocker or missing evidence item. |"
            .to_string(),
        "| `claim_gate` | string | yes | Human-readable claim-gate label. Initial generated value is `Public claim blocked.` |".to_string(),
        "| `public_claim_allowed` | boolean | yes | Explicit claim gate for public use. Initial generated value is `false`. |".to_string(),
        "| `next_action` | string | yes | Safe next workflow action. |".to_string(),
        "| `use_rule` | string | yes | Boundary rule for using the row. |".to_string(),
        String::new(),
        "At least one of `lane_id` or `program_or_account_id` must be present.".to_string(),
        String::new(),
        "## Response Classes".to_string(),
        String::new(),
    ];

    for response_class in PerformanceDemandResponseLogClass::all_classes() {
        lines.push(format!(
            "- `{}`: {}",
            response_class.wire_value(),
            response_class.rubric_meaning()
        ));
    }

    lines.extend([
        String::new(),
        "## Gate Rules".to_string(),
        String::new(),
        "- `evidence_received` must be non-empty when `response_class` is `complete-evidence-response` or `partial-evidence-response`.".to_string(),
        "- `evidence_received` must be empty when `response_class` is `not-yet-received`, `process-only-response`, or `no-evidence-response`.".to_string(),
        "- `public_claim_allowed` must remain `false` unless a separate reviewed evidence record and public-claim gate allow a public statement.".to_string(),
        String::new(),
        "## Public-Use Rule".to_string(),
        String::new(),
        "Rows may support response tracking. They must not be used as findings of fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ]);

    lines.join("\n") + "\n"
}

fn build_accountability_performance_demand_response_status(root: &Path) -> Result<String, String> {
    let response_log = build_accountability_performance_demand_response_log_jsonl(root)?;
    let rows: Vec<PerformanceDemandResponseLogRecord> = response_log
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str(line)
                .map_err(|err| format!("failed to parse generated response log row: {err}"))
        })
        .collect::<Result<_, _>>()?;
    let status = PerformanceDemandResponseStatus::from_response_log_records(
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_JSONL_PATH,
        &rows,
    )?;
    status.validate()?;
    serde_json::to_string_pretty(&status)
        .map(|text| text + "\n")
        .map_err(|err| format!("failed to serialize response status: {err}"))
}

fn generated_accountability_performance_demand_response_status(
    root: &Path,
) -> Result<PerformanceDemandResponseStatus, String> {
    let status_text = build_accountability_performance_demand_response_status(root)?;
    let status: PerformanceDemandResponseStatus = serde_json::from_str(&status_text)
        .map_err(|err| format!("failed to parse generated response status: {err}"))?;
    status.validate()?;
    Ok(status)
}

fn build_accountability_performance_demand_response_dashboard(
    root: &Path,
) -> Result<String, String> {
    let status = generated_accountability_performance_demand_response_status(root)?;

    let lines = vec![
        "# Performance Demand Response Dashboard".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated dashboard summarizes response-log status for performance demand rows."
            .to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Response Status Summary".to_string(),
        String::new(),
        format!("- Response rows: {}", status.total_rows),
        format!("- Not-yet-received rows: {}", status.not_yet_received),
        format!(
            "- Public claims currently allowed: {}",
            status.public_claim_allowed
        ),
        format!(
            "- Public claims currently blocked: {}",
            status.public_claim_blocked
        ),
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        status.use_rule,
    ];

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_response_handoff(root: &Path) -> Result<String, String> {
    let status = generated_accountability_performance_demand_response_status(root)?;

    let lines = vec![
        "# Performance Demand Response Handoff".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated handoff routes readers and implementers through response tracking artifacts."
            .to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Use Order".to_string(),
        String::new(),
        "1. Start with `performance-demand-response-dashboard.md` to scan response counts."
            .to_string(),
        "2. Use `performance-demand-response-log.md` to track current reply status and missing evidence.".to_string(),
        "3. Use `performance-demand-response-rubric.md` to classify replies as complete, partial, process-only, or no-evidence.".to_string(),
        "4. Use `performance-demand-followup.md` when a reply leaves requested evidence missing or unclear.".to_string(),
        "5. Use `performance-demand-response-intake.md` when a real reply arrives and source custody must be captured before updating the log.".to_string(),
        "6. Use `performance-demand-response-log.jsonl`, `performance-demand-response-log.schema.md`, and `performance-demand-response-status.json` for UI/API consumers.".to_string(),
        String::new(),
        "## Current Status".to_string(),
        String::new(),
        format!("- Response rows: {}", status.total_rows),
        format!("- Not-yet-received rows: {}", status.not_yet_received),
        format!(
            "- Public claims currently allowed: {}",
            status.public_claim_allowed
        ),
        format!(
            "- Public claims currently blocked: {}",
            status.public_claim_blocked
        ),
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Track response status and missing evidence; do not claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ];

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_response_intake() -> String {
    let mut lines = vec![
        "# Performance Demand Response Intake".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "Use this generated intake template when a reply arrives for a performance demand."
            .to_string(),
        "It records source custody and classification inputs before any response-log row is updated."
            .to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Required Capture".to_string(),
        String::new(),
        "| Field | Capture Rule |".to_string(),
        "|---|---|".to_string(),
        "| `record_id` | Copy from `performance-demand-response-log.jsonl`. |".to_string(),
        "| `reply_source_id` | Assign or cite a source-ledger ID for the reply artifact. |".to_string(),
        "| `reply_received_date` | Record the received date as `YYYY-MM-DD`. |".to_string(),
        "| `sender_or_office` | Name the responding office or official exactly as written. |"
            .to_string(),
        "| `response_class` | Choose one class from the response log schema. |".to_string(),
        "| `evidence_received` | List concrete documents, datasets, citations, or official findings supplied by the reply. |".to_string(),
        "| `missing_evidence` | State the remaining missing source, performance, wording, or claim-basis evidence. |".to_string(),
        "| `role_review_needed` | Keep `true` until role review approves exact public wording. |".to_string(),
        "| `public_claim_allowed` | Keep `false` unless the claim gate is explicitly revalidated. |".to_string(),
        String::new(),
        "## Response Classes".to_string(),
        String::new(),
    ];

    for response_class in PerformanceDemandResponseClass::all_classes() {
        lines.push(format!(
            "- `{}`: {}",
            response_class.wire_value(),
            response_class.intake_meaning()
        ));
    }

    lines.extend([
        String::new(),
        "## Update Rule".to_string(),
        String::new(),
        "After intake, update `performance-demand-response-log.jsonl` only with source-custodied reply evidence and rerun validation.".to_string(),
        "Do not convert a reply into a fraud, waste, abuse, legal dedication, poor performance, or reform-benefit claim without reviewed evidence and an explicit public-claim gate.".to_string(),
    ]);

    lines.join("\n") + "\n"
}

fn build_accountability_performance_demand_response_intake_schema() -> String {
    let mut lines = vec![
        "# Performance Demand Response Intake Schema".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This schema documents the fields a future UI/API importer should capture from `performance-demand-response-intake.md`.".to_string(),
        "It defines an intake contract only; it does not authorize public claims or response-log updates without validation.".to_string(),
        String::new(),
        "## Row Fields".to_string(),
        String::new(),
        "| Field | Type | Required | Meaning |".to_string(),
        "|---|---|---|---|".to_string(),
        "| `record_id` | string | yes | Accountability evidence record ID copied from the response log row. |".to_string(),
        "| `reply_source_id` | string | yes | Source-ledger identifier or custody pointer for the received reply artifact. |".to_string(),
        "| `reply_received_date` | string | yes | ISO date (`YYYY-MM-DD`) when the reply was received. |".to_string(),
        "| `sender_or_office` | string | yes | Responding office or official exactly as written in the reply. |".to_string(),
        "| `response_class` | string | yes | One allowed response class from this schema. |".to_string(),
        "| `evidence_received` | array of strings | yes | Concrete documents, datasets, citations, or official findings supplied by the reply. |".to_string(),
        "| `missing_evidence` | string | yes | Remaining source, performance, wording, or claim-basis evidence gap. |".to_string(),
        "| `role_review_needed` | boolean | yes | Must remain `true` until exact public wording receives role review. |".to_string(),
        "| `public_claim_allowed` | boolean | yes | Must remain `false` unless claim gates are explicitly revalidated. |".to_string(),
        "| `use_rule` | string | yes | Boundary rule for using the intake row. |".to_string(),
        String::new(),
        "## Allowed Response Classes".to_string(),
        String::new(),
    ];

    for response_class in PerformanceDemandResponseClass::all_classes() {
        lines.push(format!("- `{}`", response_class.wire_value()));
    }

    lines.extend([
        String::new(),
        "## Gate Rules".to_string(),
        String::new(),
        "- `role_review_needed` must be `true` for unreviewed replies.".to_string(),
        "- `public_claim_allowed` must be `false` until the response log, role review, and claim gates are revalidated.".to_string(),
        "- `evidence_received` must be non-empty when `response_class` is `complete-evidence-response` or `partial-evidence-response`.".to_string(),
        "- `evidence_received` must be empty when `response_class` is `process-only-response` or `no-evidence-response`.".to_string(),
        String::new(),
        "## Public-Use Rule".to_string(),
        String::new(),
        PERFORMANCE_DEMAND_RESPONSE_INTAKE_USE_RULE.to_string(),
    ]);

    lines.join("\n") + "\n"
}

fn build_accountability_performance_demand_response_intake_example_jsonl(
    root: &Path,
) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    let record = records.first().ok_or_else(|| {
        "cannot build performance demand response intake example without accountability records"
            .to_string()
    })?;

    let intake = PerformanceDemandResponseIntakeRecord {
        record_id: record.record_id.clone(),
        reply_source_id: "SRC-REPLY-EXAMPLE".to_string(),
        reply_received_date: "2026-06-23".to_string(),
        sender_or_office: "Example program office".to_string(),
        response_class: PerformanceDemandResponseClass::PartialEvidenceResponse,
        evidence_received: vec![
            "Example reply cites a source record and a performance target.".to_string(),
        ],
        missing_evidence: "Role-approved public wording and public-claim basis remain missing."
            .to_string(),
        role_review_needed: true,
        public_claim_allowed: false,
        use_rule: PERFORMANCE_DEMAND_RESPONSE_INTAKE_USE_RULE.to_string(),
    };
    intake.validate()?;

    serde_json::to_string(&intake)
        .map(|line| format!("{line}\n"))
        .map_err(|err| format!("failed to serialize response intake example row: {err}"))
}

fn build_accountability_performance_demand_response_log_applied_example_jsonl(
    root: &Path,
) -> Result<String, String> {
    let response_log = build_accountability_performance_demand_response_log_jsonl(root)?;
    let intake_jsonl = build_accountability_performance_demand_response_intake_example_jsonl(root)?;
    let mut log_rows: BTreeMap<String, PerformanceDemandResponseLogRecord> = response_log
        .lines()
        .map(|line| {
            let record: PerformanceDemandResponseLogRecord = serde_json::from_str(line)
                .map_err(|err| format!("failed to parse generated response log row: {err}"))?;
            Ok((record.record_id.clone(), record))
        })
        .collect::<Result<BTreeMap<_, _>, String>>()?;

    for line in intake_jsonl.lines() {
        let intake: PerformanceDemandResponseIntakeRecord = serde_json::from_str(line)
            .map_err(|err| format!("failed to parse generated intake example row: {err}"))?;
        let log_record = log_rows.remove(&intake.record_id).ok_or_else(|| {
            format!(
                "response intake example row has no matching response log row: {}",
                intake.record_id
            )
        })?;
        let updated = log_record.apply_intake(&intake)?;
        log_rows.insert(updated.record_id.clone(), updated);
    }

    let mut lines = Vec::new();
    for row in log_rows.values() {
        row.validate()?;
        lines.push(
            serde_json::to_string(row)
                .map_err(|err| format!("failed to serialize applied response log row: {err}"))?,
        );
    }

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_response_status_applied_example(
    root: &Path,
) -> Result<String, String> {
    let applied_log =
        build_accountability_performance_demand_response_log_applied_example_jsonl(root)?;
    let rows: Vec<PerformanceDemandResponseLogRecord> = applied_log
        .lines()
        .map(|line| {
            serde_json::from_str(line)
                .map_err(|err| format!("failed to parse applied response log row: {err}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let status = PerformanceDemandResponseStatus::from_response_log_records(
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_APPLIED_EXAMPLE_JSONL_PATH,
        &rows,
    )?;
    status.validate()?;

    serde_json::to_string_pretty(&status)
        .map(|text| format!("{text}\n"))
        .map_err(|err| format!("failed to serialize applied response status: {err}"))
}

fn build_accountability_performance_demand_response_dashboard_applied_example(
    root: &Path,
) -> Result<String, String> {
    let status_text =
        build_accountability_performance_demand_response_status_applied_example(root)?;
    let status: PerformanceDemandResponseStatus = serde_json::from_str(&status_text)
        .map_err(|err| format!("failed to parse applied response status: {err}"))?;
    status.validate()?;
    let updated_rows = status.total_rows.saturating_sub(status.not_yet_received);

    let lines = vec![
        "# Performance Demand Response Applied Example Dashboard".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated dashboard summarizes the importer fixture after applying example intake rows.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Applied Response Status Summary".to_string(),
        String::new(),
        format!("- Response rows: {}", status.total_rows),
        format!("- Updated rows: {updated_rows}"),
        format!("- Not-yet-received rows: {}", status.not_yet_received),
        format!(
            "- Public claims currently allowed: {}",
            status.public_claim_allowed
        ),
        format!(
            "- Public claims currently blocked: {}",
            status.public_claim_blocked
        ),
        String::new(),
        "## Fixture Boundary".to_string(),
        String::new(),
        "Use this dashboard to inspect importer behavior only. Do not treat applied example rows as canonical response status or public-claim eligibility.".to_string(),
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        status.use_rule,
    ];

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_response_handoff_applied_example(
    root: &Path,
) -> Result<String, String> {
    let status_text =
        build_accountability_performance_demand_response_status_applied_example(root)?;
    let status: PerformanceDemandResponseStatus = serde_json::from_str(&status_text)
        .map_err(|err| format!("failed to parse applied response status: {err}"))?;
    status.validate()?;
    let updated_rows = status.total_rows.saturating_sub(status.not_yet_received);

    let lines = vec![
        "# Performance Demand Response Applied Example Handoff".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated handoff routes implementers through the response importer fixture artifacts.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Use Order".to_string(),
        String::new(),
        "1. Start with `performance-demand-response-intake.example.jsonl` to inspect a source-custodied intake row.".to_string(),
        "2. Use `performance-demand-response-log.applied-example.jsonl` to inspect typed response-log rows after intake application.".to_string(),
        "3. Use `performance-demand-response-status.applied-example.json` when a UI/API needs compact applied counts.".to_string(),
        "4. Use `performance-demand-response-dashboard.applied-example.md` for quick human inspection of importer behavior.".to_string(),
        "5. Use `performance-demand-response-delta.applied-example.md` to inspect row-level changes without opening JSONL.".to_string(),
        "6. Use `performance-demand-response-delta.applied-example.jsonl` and `performance-demand-response-delta.applied-example.schema.md` for UI/API delta consumers.".to_string(),
        "7. Use `performance-demand-response-bundle.applied-example.md` to inspect the complete applied fixture bundle.".to_string(),
        "8. Use `performance-demand-response-bundle.applied-example.json` and `performance-demand-response-bundle.applied-example.schema.md` for UI/API bundle consumers.".to_string(),
        String::new(),
        "## Applied Fixture Status".to_string(),
        String::new(),
        format!("- Response rows: {}", status.total_rows),
        format!("- Updated rows: {updated_rows}"),
        format!("- Not-yet-received rows: {}", status.not_yet_received),
        format!(
            "- Public claims currently allowed: {}",
            status.public_claim_allowed
        ),
        format!(
            "- Public claims currently blocked: {}",
            status.public_claim_blocked
        ),
        String::new(),
        "## Boundary".to_string(),
        String::new(),
        "Applied example artifacts are importer fixtures, not canonical response status. Do not use them as public-claim eligibility, misconduct findings, or performance findings.".to_string(),
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        status.use_rule,
    ];

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_response_bundle_applied_example(
    root: &Path,
) -> Result<String, String> {
    let status_text =
        build_accountability_performance_demand_response_status_applied_example(root)?;
    let status: PerformanceDemandResponseStatus = serde_json::from_str(&status_text)
        .map_err(|err| format!("failed to parse applied response status: {err}"))?;
    status.validate()?;
    let updated_rows = status.total_rows.saturating_sub(status.not_yet_received);

    let lines = vec![
        "# Performance Demand Response Applied Example Bundle".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated bundle index gives importer and UI/API consumers one place to find every applied response fixture artifact.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Applied Fixture Summary".to_string(),
        String::new(),
        format!("- Response rows: {}", status.total_rows),
        format!("- Updated rows: {updated_rows}"),
        format!("- Not-yet-received rows: {}", status.not_yet_received),
        format!(
            "- Public claims currently allowed: {}",
            status.public_claim_allowed
        ),
        format!(
            "- Public claims currently blocked: {}",
            status.public_claim_blocked
        ),
        String::new(),
        "## Bundle Artifacts".to_string(),
        String::new(),
        "| Artifact | Role | Consumer Use |".to_string(),
        "|---|---|---|".to_string(),
        "| `performance-demand-response-intake.example.jsonl` | Source-custodied intake fixture row. | Exercise importer parsing and record-id matching. |".to_string(),
        "| `performance-demand-response-log.applied-example.jsonl` | Response-log rows after applying example intake. | Inspect typed applied rows without changing canonical response status. |".to_string(),
        "| `performance-demand-response-status.applied-example.json` | Compact applied response counts. | Feed fixture counts into UI/API tests without recomputing rows. |".to_string(),
        "| `performance-demand-response-dashboard.applied-example.md` | Human-readable applied response counts. | Scan importer behavior without opening JSON. |".to_string(),
        "| `performance-demand-response-handoff.applied-example.md` | Task routing for the applied fixture set. | Choose the right applied artifact by implementation task. |".to_string(),
        "| `performance-demand-response-applied-example.schema.md` | Fixture artifact contract. | Confirm roles and guardrails for applied importer artifacts. |".to_string(),
        "| `performance-demand-response-delta.applied-example.md` | Human-readable changed fields. | Inspect row-level changes after applying example intake. |".to_string(),
        "| `performance-demand-response-delta.applied-example.jsonl` | Machine-readable changed fields. | Feed delta rows into UI/API diff consumers. |".to_string(),
        "| `performance-demand-response-delta.applied-example.schema.md` | Delta row field contract. | Confirm field meanings and blocked-claim guardrails. |".to_string(),
        "| `performance-demand-response-bundle.applied-example.json` | Machine-readable bundle manifest. | Load fixture artifact roles, row counts, hashes, and boundaries without scraping Markdown. |".to_string(),
        "| `performance-demand-response-bundle.applied-example.schema.md` | Bundle manifest field contract. | Inspect manifest and artifact field meanings. |".to_string(),
        String::new(),
        "## Boundary".to_string(),
        String::new(),
        "Bundle artifacts are importer fixtures, not canonical response status. Do not use them as public-claim eligibility, misconduct findings, performance findings, or proof of reform benefits.".to_string(),
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        status.use_rule,
    ];

    Ok(lines.join("\n") + "\n")
}

fn performance_demand_response_bundle_artifacts(
    root: &Path,
) -> Result<Vec<PerformanceDemandResponseBundleArtifact>, String> {
    let rows = [
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_INTAKE_EXAMPLE_JSONL_PATH,
            "Source-custodied intake fixture row.",
            "jsonl",
            "Exercise importer parsing and record-id matching.",
        ),
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_APPLIED_EXAMPLE_JSONL_PATH,
            "Response-log rows after applying example intake.",
            "jsonl",
            "Inspect typed applied rows without changing canonical response status.",
        ),
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_APPLIED_EXAMPLE_PATH,
            "Compact applied response counts.",
            "json",
            "Feed fixture counts into UI/API tests without recomputing rows.",
        ),
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DASHBOARD_APPLIED_EXAMPLE_PATH,
            "Human-readable applied response counts.",
            "markdown",
            "Scan importer behavior without opening JSON.",
        ),
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_HANDOFF_APPLIED_EXAMPLE_PATH,
            "Task routing for the applied fixture set.",
            "markdown",
            "Choose the right applied artifact by implementation task.",
        ),
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_APPLIED_EXAMPLE_SCHEMA_PATH,
            "Fixture artifact contract.",
            "markdown",
            "Confirm roles and guardrails for applied importer artifacts.",
        ),
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_PATH,
            "Human-readable changed fields.",
            "markdown",
            "Inspect row-level changes after applying example intake.",
        ),
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_JSONL_PATH,
            "Machine-readable changed fields.",
            "jsonl",
            "Feed delta rows into UI/API diff consumers.",
        ),
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_SCHEMA_PATH,
            "Delta row field contract.",
            "markdown",
            "Confirm field meanings and blocked-claim guardrails.",
        ),
    ];

    rows.into_iter()
        .map(|(artifact, role, kind, consumer_use)| {
            let path = root.join(artifact);
            Ok(PerformanceDemandResponseBundleArtifact {
                artifact: artifact.to_string(),
                role: role.to_string(),
                kind: kind.to_string(),
                row_count: count_rows(&path, kind)?,
                sha256: sha256_file(&path)?,
                consumer_use: consumer_use.to_string(),
            })
        })
        .collect()
}

fn build_accountability_performance_demand_response_bundle_applied_example_json(
    root: &Path,
) -> Result<String, String> {
    let status_text =
        build_accountability_performance_demand_response_status_applied_example(root)?;
    let status: PerformanceDemandResponseStatus = serde_json::from_str(&status_text)
        .map_err(|err| format!("failed to parse applied response status: {err}"))?;
    let manifest = PerformanceDemandResponseBundleManifest::from_status(
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_JSON_PATH,
        &status,
        performance_demand_response_bundle_artifacts(root)?,
    )?;
    serde_json::to_string_pretty(&manifest)
        .map(|text| text + "\n")
        .map_err(|err| format!("failed to serialize applied response bundle manifest: {err}"))
}

fn build_accountability_performance_demand_response_bundle_applied_example_schema() -> String {
    let lines = vec![
        "# Performance Demand Response Bundle Applied Example JSON Schema".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This schema documents `performance-demand-response-bundle.applied-example.json` fields.".to_string(),
        "The JSON is serialized from `PerformanceDemandResponseBundleManifest` and lists `PerformanceDemandResponseBundleArtifact` rows for importer and UI/API consumers.".to_string(),
        String::new(),
        "## Manifest Fields".to_string(),
        String::new(),
        "| Field | Type | Required | Meaning |".to_string(),
        "|---|---|---|---|".to_string(),
        "| `artifact` | string | yes | Repo-relative path for this bundle manifest JSON. |".to_string(),
        "| `bundle_kind` | string | yes | Fixed value `applied-response-importer-fixture`. |".to_string(),
        "| `total_rows` | integer | yes | Applied response-log row count from the fixture status. |".to_string(),
        "| `updated_rows` | integer | yes | Rows changed by applying example intake. Must not exceed `total_rows`. |".to_string(),
        "| `public_claim_allowed` | integer | yes | Must remain `0` for the applied fixture bundle. |".to_string(),
        "| `public_claim_blocked` | integer | yes | Blocked public-claim row count. With allowed count, must sum to `total_rows`. |".to_string(),
        "| `artifacts` | array | yes | Ordered applied fixture artifact entries. Must include intake, applied log, applied status, dashboard, handoff, applied schema, delta Markdown, delta JSONL, and delta schema artifacts. |".to_string(),
        "| `boundary` | string | yes | Fixture-only boundary statement. |".to_string(),
        "| `use_rule` | string | yes | Response tracking use rule; must match the core response-log use rule. |".to_string(),
        String::new(),
        "## Artifact Fields".to_string(),
        String::new(),
        "| Field | Type | Required | Meaning |".to_string(),
        "|---|---|---|---|".to_string(),
        "| `artifact` | string | yes | Repo-relative artifact path using forward slashes. |".to_string(),
        "| `role` | string | yes | Artifact role in the applied fixture bundle. |".to_string(),
        "| `kind` | string | yes | One of `jsonl`, `json`, or `markdown`. |".to_string(),
        "| `row_count` | string | yes | JSONL row count as a string, or `n/a` for non-JSONL artifacts. |".to_string(),
        "| `sha256` | string | yes | SHA-256 digest of the generated artifact bytes. |".to_string(),
        "| `consumer_use` | string | yes | Intended importer or UI/API use. |".to_string(),
        String::new(),
        "## Validation Rules".to_string(),
        String::new(),
        "- JSON must deserialize as `PerformanceDemandResponseBundleManifest`.".to_string(),
        "- Every artifact entry must validate as `PerformanceDemandResponseBundleArtifact`.".to_string(),
        "- The manifest must include all required applied fixture artifacts.".to_string(),
        "- Every artifact entry must include row-count and SHA-256 integrity metadata.".to_string(),
        "- Public claims must remain blocked for this fixture manifest.".to_string(),
        String::new(),
        "## Public-Use Rule".to_string(),
        String::new(),
        "The manifest is fixture metadata only. It must not be used as canonical response status, public-claim eligibility, a finding of fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ];

    lines.join("\n") + "\n"
}

fn build_accountability_performance_demand_response_applied_example_schema() -> String {
    let lines = vec![
        "# Performance Demand Response Applied Example Schema".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This schema note documents the generated response importer fixture artifacts.".to_string(),
        "It does not authorize public claims, findings, or canonical response-log updates.".to_string(),
        String::new(),
        "## Artifact Roles".to_string(),
        String::new(),
        "| Artifact | Role | Guardrail |".to_string(),
        "|---|---|---|".to_string(),
        "| `performance-demand-response-intake.example.jsonl` | Source-custodied intake fixture row parsed as `PerformanceDemandResponseIntakeRecord`. | Must keep `role_review_needed: true`, `public_claim_allowed: false`, and the intake use rule. |".to_string(),
        "| `performance-demand-response-log.applied-example.jsonl` | Response-log rows after applying intake fixture rows through `PerformanceDemandResponseLogRecord::apply_intake`. | Must validate as response-log records and keep `Public claim blocked.`. |".to_string(),
        "| `performance-demand-response-status.applied-example.json` | Compact counts aggregated from applied response-log rows through `PerformanceDemandResponseStatus`. | Must report zero allowed public claims and at least one updated row. |".to_string(),
        "| `performance-demand-response-dashboard.applied-example.md` | Human-readable applied status summary. | Must state fixture-only and no-finding boundaries. |".to_string(),
        "| `performance-demand-response-handoff.applied-example.md` | Task routing for importer fixture consumers. | Must not describe applied examples as canonical status or public-claim eligibility. |".to_string(),
        "| `performance-demand-response-delta.applied-example.md` | Row-level comparison between canonical response-log rows and applied example rows. | Must show changed fields while preserving blocked public-claim gates. |".to_string(),
        "| `performance-demand-response-delta.applied-example.jsonl` | Machine-readable delta rows serialized from `PerformanceDemandResponseDeltaRow`. | Must validate as core delta rows and preserve blocked public-claim gates. |".to_string(),
        "| `performance-demand-response-delta.applied-example.schema.md` | Field contract for machine-readable applied delta rows. | Must preserve fixture-only and blocked-claim guardrails. |".to_string(),
        "| `performance-demand-response-bundle.applied-example.md` | Human-readable index for the complete applied fixture bundle. | Must preserve fixture-only and blocked-claim guardrails. |".to_string(),
        "| `performance-demand-response-bundle.applied-example.json` | Machine-readable bundle manifest serialized from `PerformanceDemandResponseBundleManifest`. | Must validate through core and keep public claims blocked. |".to_string(),
        "| `performance-demand-response-bundle.applied-example.schema.md` | Field contract for the machine-readable bundle manifest. | Must document manifest fields and blocked-claim guardrails. |".to_string(),
        String::new(),
        "## Importer Rule".to_string(),
        String::new(),
        "Importers may use these artifacts to test response intake handling. They must not treat example rows as real agency replies, public fraud/waste/abuse findings, legal dedication of income taxes, poor-performance findings, or reform benefits.".to_string(),
    ];

    lines.join("\n") + "\n"
}

fn build_accountability_performance_demand_response_delta_applied_example_schema() -> String {
    let lines = vec![
        "# Performance Demand Response Delta Applied Example JSONL Schema".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This schema documents `performance-demand-response-delta.applied-example.jsonl` rows.".to_string(),
        "Rows are generated from `PerformanceDemandResponseDeltaRow` to show importer fixture changes without creating findings.".to_string(),
        String::new(),
        "## Row Fields".to_string(),
        String::new(),
        "| Field | Type | Required | Meaning |".to_string(),
        "|---|---|---|---|".to_string(),
        "| `record_id` | string | yes | Accountability evidence record ID for the changed response row. |".to_string(),
        "| `before_response_class` | string | yes | Response-log class before applying the intake fixture. |".to_string(),
        "| `after_response_class` | string | yes | Response-log class after applying the intake fixture. |".to_string(),
        "| `before_evidence_received_count` | integer | yes | Count of evidence items before applying intake. |".to_string(),
        "| `after_evidence_received_count` | integer | yes | Count of evidence items after applying intake. |".to_string(),
        "| `missing_evidence_changed` | boolean | yes | Whether the missing-evidence text changed. |".to_string(),
        "| `next_action_changed` | boolean | yes | Whether the next-action text changed. |".to_string(),
        "| `before_claim_gate` | string | yes | Claim-gate label before applying intake. Must remain `Public claim blocked.`. |".to_string(),
        "| `after_claim_gate` | string | yes | Claim-gate label after applying intake. Must remain `Public claim blocked.`. |".to_string(),
        String::new(),
        "## Gate Rules".to_string(),
        String::new(),
        "- Rows must validate through `PerformanceDemandResponseDeltaRow`.".to_string(),
        "- Both claim-gate fields must remain `Public claim blocked.`.".to_string(),
        "- Rows describe fixture deltas only; they are not canonical response status.".to_string(),
        String::new(),
        "## Public-Use Rule".to_string(),
        String::new(),
        "Rows may support importer and UI/API testing. They must not be used as findings of fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ];

    lines.join("\n") + "\n"
}

fn build_accountability_performance_demand_response_delta_applied_example_jsonl(
    root: &Path,
) -> Result<String, String> {
    let rows = build_accountability_performance_demand_response_delta_rows(root)?;
    let mut lines = Vec::new();
    for row in rows {
        row.validate()?;
        lines.push(
            serde_json::to_string(&row)
                .map_err(|err| format!("failed to serialize applied response delta row: {err}"))?,
        );
    }

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_response_delta_applied_example(
    root: &Path,
) -> Result<String, String> {
    let changed_rows = build_accountability_performance_demand_response_delta_rows(root)?;
    let canonical_rows = parse_response_log_jsonl(
        &build_accountability_performance_demand_response_log_jsonl(root)?,
        "canonical response log",
    )?;
    let applied_rows = parse_response_log_jsonl(
        &build_accountability_performance_demand_response_log_applied_example_jsonl(root)?,
        "applied response log",
    )?;

    let mut lines = vec![
        "# Performance Demand Response Applied Example Delta".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated delta compares canonical response-log rows with the importer fixture after applying example intake rows.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Summary".to_string(),
        String::new(),
        format!("- Canonical rows: {}", canonical_rows.len()),
        format!("- Applied rows: {}", applied_rows.len()),
        format!("- Updated rows: {}", changed_rows.len()),
        String::new(),
        "## Row Changes".to_string(),
        String::new(),
        "| Record ID | Before response class | After response class | Evidence received change | Missing evidence change | Next action change | Claim gate |".to_string(),
        "|---|---|---|---|---|---|---|".to_string(),
    ];

    for row in changed_rows {
        row.validate()?;
        lines.push(format!(
            "| `{}` | `{}` | `{}` | {} -> {} item(s) | {} | {} | {} -> {} |",
            escape_table_cell(&row.record_id),
            row.before_response_class.wire_value(),
            row.after_response_class.wire_value(),
            row.before_evidence_received_count,
            row.after_evidence_received_count,
            bool_marker(row.missing_evidence_changed),
            bool_marker(row.next_action_changed),
            escape_table_cell(&row.before_claim_gate),
            escape_table_cell(&row.after_claim_gate),
        ));
    }

    lines.extend([
        String::new(),
        "## Fixture Boundary".to_string(),
        String::new(),
        "Use this delta to inspect importer behavior only. Do not treat changed fixture rows as canonical response status, public-claim eligibility, misconduct findings, performance findings, or reform benefits.".to_string(),
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Applied response deltas are implementation fixtures. Public wording must keep source custody, role review, public-claim gates, and no-finding boundaries intact.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_response_delta_rows(
    root: &Path,
) -> Result<Vec<PerformanceDemandResponseDeltaRow>, String> {
    let canonical_log = build_accountability_performance_demand_response_log_jsonl(root)?;
    let applied_log =
        build_accountability_performance_demand_response_log_applied_example_jsonl(root)?;
    let canonical_rows = parse_response_log_jsonl(&canonical_log, "canonical response log")?;
    let applied_rows = parse_response_log_jsonl(&applied_log, "applied response log")?;
    PerformanceDemandResponseDeltaRow::from_response_log_records(&canonical_rows, &applied_rows)
}

fn parse_response_log_jsonl(
    text: &str,
    label: &str,
) -> Result<Vec<PerformanceDemandResponseLogRecord>, String> {
    text.lines()
        .map(|line| {
            let record: PerformanceDemandResponseLogRecord = serde_json::from_str(line)
                .map_err(|err| format!("failed to parse {label} row: {err}"))?;
            record.validate()?;
            Ok(record)
        })
        .collect::<Result<Vec<_>, String>>()
}

fn bool_marker(changed: bool) -> &'static str {
    if changed { "changed" } else { "unchanged" }
}

fn escape_table_cell(value: &str) -> String {
    value.replace('|', "\\|")
}

fn read_accountability_evidence_records(
    root: &Path,
) -> Result<Vec<AccountabilityEvidenceRecord>, String> {
    read_jsonl(root.join(ACCOUNTABILITY_EVIDENCE_JSONL_PATH))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row)
                .map_err(|err| format!("accountability evidence: invalid record shape: {err}"))
        })
        .collect()
}

fn int_field(row: &serde_json::Value, field: &str) -> Result<i64, String> {
    row.get(field)
        .and_then(serde_json::Value::as_i64)
        .ok_or_else(|| format!("missing integer field {field}"))
}

fn number_field(row: &serde_json::Value, field: &str) -> Result<f64, String> {
    row.get(field)
        .and_then(serde_json::Value::as_f64)
        .ok_or_else(|| format!("missing number field {field}"))
}

fn string_field(row: &serde_json::Value, field: &str) -> Result<String, String> {
    row.get(field)
        .and_then(serde_json::Value::as_str)
        .map(ToString::to_string)
        .ok_or_else(|| format!("missing string field {field}"))
}

fn insert_number(row: &mut BTreeMap<String, String>, field: &str, value: f64) {
    row.insert(field.to_string(), compact_decimal(value));
}

fn insert_rounded_number(
    row: &mut BTreeMap<String, String>,
    field: &str,
    value: f64,
    decimals: usize,
) {
    row.insert(field.to_string(), rounded_decimal(value, decimals));
}

fn insert_json_number(
    row: &mut BTreeMap<String, String>,
    field: &str,
    source: &serde_json::Value,
    source_field: &str,
) {
    row.insert(field.to_string(), json_number_string(source, source_field));
}

fn json_number_string(row: &serde_json::Value, field: &str) -> String {
    let value = row
        .get(field)
        .unwrap_or_else(|| panic!("missing number field {field}"));
    if let Some(number) = value.as_i64() {
        number.to_string()
    } else if let Some(number) = value.as_f64() {
        compact_decimal(number)
    } else {
        panic!("missing number field {field}")
    }
}

fn round6(value: f64) -> f64 {
    (value * 1_000_000.0).round() / 1_000_000.0
}

fn round9(value: f64) -> f64 {
    (value * 1_000_000_000.0).round() / 1_000_000_000.0
}

fn compact_decimal(value: f64) -> String {
    if value.is_finite() && value.fract() == 0.0 {
        format!("{value:.1}")
    } else {
        let text = format!("{value:.12}");
        text.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

fn rounded_decimal(value: f64, decimals: usize) -> String {
    if !value.is_finite() {
        return value.to_string();
    }

    let factor = 10_i128.pow(decimals as u32);
    let scaled = (value * factor as f64).round() as i128;
    let sign = if scaled < 0 { "-" } else { "" };
    let absolute = scaled.abs();
    let integer = absolute / factor;
    let fraction = absolute % factor;

    if decimals == 0 || fraction == 0 {
        return format!("{sign}{integer}");
    }

    let mut fraction_text = format!("{fraction:0decimals$}");
    while fraction_text.ends_with('0') {
        fraction_text.pop();
    }
    format!("{sign}{integer}.{fraction_text}")
}

fn format_millions_as_billions_or_trillions(value_millions: f64) -> String {
    if value_millions.abs() >= 1_000_000.0 {
        format!("${:.3}T", value_millions / 1_000_000.0)
    } else {
        format!("${:.3}B", value_millions / 1_000.0)
    }
}

fn decimal_string(value: f64, decimals: usize) -> String {
    let text = format!("{value:.decimals$}");
    let trimmed = text.trim_end_matches('0').trim_end_matches('.');
    if trimmed == "-0" {
        "0.0".to_string()
    } else if trimmed.contains('.') {
        trimmed.to_string()
    } else {
        format!("{trimmed}.0")
    }
}

fn json_amount(value: f64) -> String {
    if value == 0.0 {
        "0".to_string()
    } else if value.is_finite() && value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        decimal_string(value, 6)
    }
}

fn comma_number(value: f64, decimals: usize) -> String {
    let text = format!("{value:.decimals$}");
    let (sign, unsigned) = text
        .strip_prefix('-')
        .map_or(("", text.as_str()), |rest| ("-", rest));
    let (integer, fraction) = unsigned
        .split_once('.')
        .map_or((unsigned, None), |(integer, fraction)| {
            (integer, Some(fraction))
        });
    let mut grouped = String::new();
    for (index, char) in integer.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(char);
    }
    let integer = grouped.chars().rev().collect::<String>();
    match fraction {
        Some(fraction) => format!("{sign}{integer}.{fraction}"),
        None => format!("{sign}{integer}"),
    }
}

fn annual_deficit_gap_string(value: f64) -> String {
    if value == 0.0 {
        "0".to_string()
    } else {
        decimal_string(value, 6)
    }
}

fn decade_label(year: i64) -> String {
    let start = year - year % 10;
    format!("{start}s")
}

fn sum_field(rows: &[&serde_json::Value], field: &str) -> Result<f64, String> {
    rows.iter().map(|row| number_field(row, field)).sum()
}

fn json_string(value: &str) -> String {
    serde_json::to_string(value).expect("serializing string should not fail")
}

fn json_option_string(value: Option<&str>) -> String {
    value.map_or_else(|| "null".to_string(), json_string)
}

fn json_owned_option_string(value: Option<&String>) -> String {
    value.map_or_else(|| "null".to_string(), |value| json_string(value))
}

fn check_manifest(root: &Path) -> Result<(), String> {
    let expected = normalize_newlines(&build_manifest(root)?);
    let current = fs::read_to_string(root.join(MANIFEST_PATH))
        .map_err(|err| format!("failed to read {MANIFEST_PATH}: {err}"))?;
    if normalize_newlines(&current) != expected {
        return Err(format!(
            "stale manifest: run `cargo run -p taxlane-tools -- income-tax-outlay manifest`"
        ));
    }
    println!("validated income-tax outlay artifact manifest");
    Ok(())
}

fn build_manifest(root: &Path) -> Result<String, String> {
    let metadata: Vec<ArtifactMetadata<'_>> = ARTIFACTS.iter().map(Artifact::metadata).collect();
    taxlane_core::validate_artifact_metadata(&metadata)?;

    let mut rows = Vec::new();
    for artifact in ARTIFACTS {
        let path = root.join(artifact.path);
        if !path.exists() {
            return Err(format!("missing artifact: {}", artifact.path));
        }
        rows.push((
            artifact,
            count_rows(&path, artifact.kind)?,
            sha256_file(&path)?,
        ));
    }

    let mut lines = vec![
        "# Income-Tax Outlay Model Artifact Manifest".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This manifest records the artifact chain for modeled allocations of".to_string(),
        "ordinary individual income-tax receipts by OMB outlay share.".to_string(),
        String::new(),
        "The annual, decade, and subfunction JSONL files are canonical model".to_string(),
        "outputs. CSV files, Markdown notes, and chart specs are derived or".to_string(),
        "supporting views.".to_string(),
        String::new(),
        "## Model".to_string(),
        String::new(),
        "- Broad model ID: `individual-income-tax-proportional-outlays-v1`".to_string(),
        "- Subfunction model ID: `individual-income-tax-proportional-subfunction-outlays-v1`"
            .to_string(),
        "- Broad coverage: fiscal years 1940-2025 for annual actual-year rows".to_string(),
        "- Subfunction coverage: fiscal years 1962-2025 for Table 3.2 actual-year rows".to_string(),
        "- Projection treatment: FY2026-FY2031 excluded".to_string(),
        "- Legal status: modeled allocation, not legal dedication".to_string(),
        String::new(),
        "## Artifacts".to_string(),
        String::new(),
        "| Path | Role | Grain | Rows | Canonical | SHA-256 |".to_string(),
        "|---|---|---|---:|---|---|".to_string(),
    ];

    for (artifact, rows, sha) in rows {
        lines.push(format!(
            "| `{}` | {} | {} | {} | {} | `{}` |",
            artifact.path, artifact.role, artifact.grain, rows, artifact.canonical, sha
        ));
    }

    lines.extend([
        String::new(),
        "## Regeneration Order".to_string(),
        String::new(),
        "1. `cargo run -p taxlane-tools -- income-tax-outlay model`".to_string(),
        "2. `cargo run -p taxlane-tools -- income-tax-outlay summary`".to_string(),
        "3. `cargo run -p taxlane-tools -- income-tax-outlay export`".to_string(),
        "4. `cargo run -p taxlane-tools -- income-tax-outlay subfunction-model`".to_string(),
        "5. `cargo run -p taxlane-tools -- income-tax-outlay subfunction-export`".to_string(),
        "6. `cargo run -p taxlane-tools -- income-tax-outlay manifest`".to_string(),
        String::new(),
        "Run validation after regeneration:".to_string(),
        String::new(),
        "```powershell".to_string(),
        "cargo run -p taxlane-tools -- income-tax-outlay validate".to_string(),
        "```".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn count_rows(path: &Path, kind: &str) -> Result<String, String> {
    match kind {
        "jsonl" => {
            let content = fs::read_to_string(path)
                .map_err(|err| format!("failed to read {:?}: {err}", path))?;
            let mut count = 0usize;
            for line in content.lines() {
                serde_json::from_str::<serde_json::Value>(line)
                    .map_err(|err| format!("failed to parse JSONL {:?}: {err}", path))?;
                count += 1;
            }
            Ok(count.to_string())
        }
        "csv" => {
            let mut reader = csv::Reader::from_path(path)
                .map_err(|err| format!("failed to read CSV {:?}: {err}", path))?;
            let count = reader.records().count();
            Ok(count.to_string())
        }
        _ => Ok("n/a".to_string()),
    }
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|err| format!("failed to open {:?}: {err}", path))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024 * 64];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|err| format!("failed to read {:?}: {err}", path))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n")
}
