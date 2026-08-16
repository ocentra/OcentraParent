use serde::{Deserialize, Serialize};

mod evaluation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAiDetectionLabel {
    BenignExpected,
    GameTraffic,
    SocialVideo,
    VpnProxyTunnel,
    RemoteDesktop,
    TorrentTransfer,
    SignatureThreat,
    UnknownHighVolume,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAiDetectionRiskLevel {
    Low,
    Medium,
    High,
    Critical,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAiDetectionInputKind {
    SummaryRefs,
    EvidenceRefs,
    AnalyzerAlertRefs,
    FixtureLabel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAiDetectionPrecisionState {
    MeetsThreshold,
    BelowThreshold,
    NoPositivePredictions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAiDetectionRecallState {
    MeetsThreshold,
    BelowThreshold,
    NoExpectedPositives,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAiDetectionDriftState {
    WithinTolerance,
    ExceededTolerance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAiDetectionEvaluationState {
    MeetsFixtureGate,
    BelowQualityThreshold,
    DriftExceeded,
    BelowQualityAndDriftExceeded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAiDetectionUncertaintyCode {
    LabelMismatch,
    FalsePositiveFixture,
    FalseNegativeFixture,
    UnknownPrediction,
    ConfidenceDriftExceeded,
    LowConfidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAiDetectionFixtureCase {
    pub detection_ref: String,
    pub fixture_ref: String,
    pub summary_ref: String,
    pub evidence_refs: Vec<String>,
    pub analyzer_alert_refs: Vec<String>,
    pub expected_label: NetworkAiDetectionLabel,
    pub predicted_label: NetworkAiDetectionLabel,
    pub confidence_basis_points: u16,
    pub baseline_confidence_basis_points: u16,
    pub risk_level: NetworkAiDetectionRiskLevel,
    pub input_kinds: Vec<NetworkAiDetectionInputKind>,
    pub raw_pcap_input_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub exact_url_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAiDetectionEvaluationInput {
    pub evaluation_run_ref: String,
    pub fixture_set_ref: String,
    pub model_card_ref: String,
    pub model_version_ref: String,
    pub baseline_ref: String,
    pub cases: Vec<NetworkAiDetectionFixtureCase>,
    pub minimum_precision_basis_points: u16,
    pub minimum_recall_basis_points: u16,
    pub maximum_average_drift_basis_points: u16,
    pub model_execution_claimed: bool,
    pub remote_ai_claimed: bool,
    pub raw_pcap_input_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub exact_url_claimed: bool,
    pub policy_authority_claimed: bool,
    pub adapter_authority_claimed: bool,
    pub enforcement_command_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAiDetectionResult {
    pub detection_ref: String,
    pub fixture_ref: String,
    pub summary_ref: String,
    pub evidence_refs: Vec<String>,
    pub analyzer_alert_refs: Vec<String>,
    pub expected_label: NetworkAiDetectionLabel,
    pub predicted_label: NetworkAiDetectionLabel,
    pub confidence_basis_points: u16,
    pub baseline_confidence_basis_points: u16,
    pub confidence_drift_basis_points: u16,
    pub risk_level: NetworkAiDetectionRiskLevel,
    pub input_kinds: Vec<NetworkAiDetectionInputKind>,
    pub label_match: bool,
    pub expected_positive: bool,
    pub predicted_positive: bool,
    pub true_positive: bool,
    pub false_positive: bool,
    pub false_negative: bool,
    pub true_negative: bool,
    pub uncertainty_codes: Vec<NetworkAiDetectionUncertaintyCode>,
    pub raw_pcap_available: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
    pub policy_authority: bool,
    pub adapter_authority: bool,
    pub enforcement_command_published: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAiDetectionEvaluationProof {
    pub evaluation_run_ref: String,
    pub fixture_set_ref: String,
    pub model_card_ref: String,
    pub model_version_ref: String,
    pub baseline_ref: String,
    pub results: Vec<NetworkAiDetectionResult>,
    pub fixture_count: usize,
    pub true_positive_count: usize,
    pub false_positive_count: usize,
    pub false_negative_count: usize,
    pub true_negative_count: usize,
    pub precision_basis_points: Option<u16>,
    pub recall_basis_points: Option<u16>,
    pub accuracy_basis_points: u16,
    pub average_confidence_drift_basis_points: u16,
    pub precision_state: NetworkAiDetectionPrecisionState,
    pub recall_state: NetworkAiDetectionRecallState,
    pub drift_state: NetworkAiDetectionDriftState,
    pub evaluation_state: NetworkAiDetectionEvaluationState,
    pub model_executed: bool,
    pub remote_ai_used: bool,
    pub policy_authority: bool,
    pub adapter_authority: bool,
    pub enforcement_commands_published: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkAiDetectionEvaluationError {
    EmptyEvaluationRunRef,
    EmptyFixtureSetRef,
    EmptyModelCardRef,
    EmptyModelVersionRef,
    EmptyBaselineRef,
    EmptyFixtureCases,
    EmptyDetectionRef,
    DuplicateDetectionRef,
    EmptyFixtureRef,
    EmptySummaryRef,
    EmptyEvidenceRefs,
    EmptyEvidenceRef,
    EmptyAnalyzerAlertRef,
    EmptyInputKinds,
    BasisPointsOutOfRange,
    ModelExecutionClaimRejected,
    RemoteAiClaimRejected,
    RawPcapInputRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    ExactUrlClaimRejected,
    PolicyAuthorityClaimRejected,
    AdapterAuthorityClaimRejected,
    EnforcementCommandClaimRejected,
}

pub fn evaluate_network_ai_detection_fixtures(
    input: &NetworkAiDetectionEvaluationInput,
) -> Result<NetworkAiDetectionEvaluationProof, NetworkAiDetectionEvaluationError> {
    evaluation::reject_global_claims(input)?;
    if input.minimum_precision_basis_points > 10_000
        || input.minimum_recall_basis_points > 10_000
        || input.maximum_average_drift_basis_points > 10_000
    {
        return Err(NetworkAiDetectionEvaluationError::BasisPointsOutOfRange);
    }
    if input.cases.is_empty() {
        return Err(NetworkAiDetectionEvaluationError::EmptyFixtureCases);
    }

    let evaluation_run_ref = evaluation::normalize_ref(&input.evaluation_run_ref)
        .ok_or(NetworkAiDetectionEvaluationError::EmptyEvaluationRunRef)?;
    let fixture_set_ref = evaluation::normalize_ref(&input.fixture_set_ref)
        .ok_or(NetworkAiDetectionEvaluationError::EmptyFixtureSetRef)?;
    let model_card_ref = evaluation::normalize_ref(&input.model_card_ref)
        .ok_or(NetworkAiDetectionEvaluationError::EmptyModelCardRef)?;
    let model_version_ref = evaluation::normalize_ref(&input.model_version_ref)
        .ok_or(NetworkAiDetectionEvaluationError::EmptyModelVersionRef)?;
    let baseline_ref = evaluation::normalize_ref(&input.baseline_ref)
        .ok_or(NetworkAiDetectionEvaluationError::EmptyBaselineRef)?;

    let results = evaluation::normalize_results(input)?;
    let counts = evaluation::count_detection_results(&results);
    let precision_basis_points =
        evaluation::ratio_basis_points(counts.true_positive, counts.predicted_positive);
    let recall_basis_points =
        evaluation::ratio_basis_points(counts.true_positive, counts.expected_positive);
    let accuracy_basis_points =
        evaluation::ratio_basis_points(counts.true_positive + counts.true_negative, results.len())
            .unwrap_or_default();
    let average_confidence_drift_basis_points = evaluation::average_drift_basis_points(&results);
    let precision_state =
        evaluation::precision_state(precision_basis_points, input.minimum_precision_basis_points);
    let recall_state =
        evaluation::recall_state(recall_basis_points, input.minimum_recall_basis_points);
    let drift_state = evaluation::drift_state(
        average_confidence_drift_basis_points,
        input.maximum_average_drift_basis_points,
    );

    Ok(NetworkAiDetectionEvaluationProof {
        evaluation_run_ref,
        fixture_set_ref,
        model_card_ref,
        model_version_ref,
        baseline_ref,
        fixture_count: results.len(),
        true_positive_count: counts.true_positive,
        false_positive_count: counts.false_positive,
        false_negative_count: counts.false_negative,
        true_negative_count: counts.true_negative,
        precision_basis_points,
        recall_basis_points,
        accuracy_basis_points,
        average_confidence_drift_basis_points,
        precision_state,
        recall_state,
        drift_state,
        evaluation_state: evaluation::evaluation_state(precision_state, recall_state, drift_state),
        results,
        model_executed: false,
        remote_ai_used: false,
        policy_authority: false,
        adapter_authority: false,
        enforcement_commands_published: 0,
    })
}
