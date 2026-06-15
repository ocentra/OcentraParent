use serde::{Deserialize, Serialize};

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
    input: NetworkAiDetectionEvaluationInput,
) -> Result<NetworkAiDetectionEvaluationProof, NetworkAiDetectionEvaluationError> {
    reject_global_claims(&input)?;
    if input.minimum_precision_basis_points > 10_000
        || input.minimum_recall_basis_points > 10_000
        || input.maximum_average_drift_basis_points > 10_000
    {
        return Err(NetworkAiDetectionEvaluationError::BasisPointsOutOfRange);
    }
    if input.cases.is_empty() {
        return Err(NetworkAiDetectionEvaluationError::EmptyFixtureCases);
    }

    let evaluation_run_ref = normalize_ref(&input.evaluation_run_ref)
        .ok_or(NetworkAiDetectionEvaluationError::EmptyEvaluationRunRef)?;
    let fixture_set_ref = normalize_ref(&input.fixture_set_ref)
        .ok_or(NetworkAiDetectionEvaluationError::EmptyFixtureSetRef)?;
    let model_card_ref = normalize_ref(&input.model_card_ref)
        .ok_or(NetworkAiDetectionEvaluationError::EmptyModelCardRef)?;
    let model_version_ref = normalize_ref(&input.model_version_ref)
        .ok_or(NetworkAiDetectionEvaluationError::EmptyModelVersionRef)?;
    let baseline_ref = normalize_ref(&input.baseline_ref)
        .ok_or(NetworkAiDetectionEvaluationError::EmptyBaselineRef)?;
    let minimum_precision_basis_points = input.minimum_precision_basis_points;
    let minimum_recall_basis_points = input.minimum_recall_basis_points;
    let maximum_average_drift_basis_points = input.maximum_average_drift_basis_points;

    let results = normalize_results(&input)?;
    let counts = count_detection_results(&results);
    let precision_basis_points =
        ratio_basis_points(counts.true_positive, counts.predicted_positive);
    let recall_basis_points = ratio_basis_points(counts.true_positive, counts.expected_positive);
    let accuracy_basis_points =
        ratio_basis_points(counts.true_positive + counts.true_negative, results.len())
            .unwrap_or_default();
    let average_confidence_drift_basis_points = average_drift_basis_points(&results);
    let precision_state = precision_state(precision_basis_points, minimum_precision_basis_points);
    let recall_state = recall_state(recall_basis_points, minimum_recall_basis_points);
    let drift_state = drift_state(
        average_confidence_drift_basis_points,
        maximum_average_drift_basis_points,
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
        evaluation_state: evaluation_state(precision_state, recall_state, drift_state),
        results,
        model_executed: false,
        remote_ai_used: false,
        policy_authority: false,
        adapter_authority: false,
        enforcement_commands_published: 0,
    })
}

fn normalize_results(
    input: &NetworkAiDetectionEvaluationInput,
) -> Result<Vec<NetworkAiDetectionResult>, NetworkAiDetectionEvaluationError> {
    let mut detection_refs = Vec::new();
    let mut results = Vec::new();
    for case in &input.cases {
        reject_case_claims(case)?;
        let detection_ref = normalize_ref(&case.detection_ref)
            .ok_or(NetworkAiDetectionEvaluationError::EmptyDetectionRef)?;
        if detection_refs.contains(&detection_ref) {
            return Err(NetworkAiDetectionEvaluationError::DuplicateDetectionRef);
        }
        detection_refs.push(detection_ref.clone());
        results.push(normalize_case_result(
            case,
            detection_ref,
            input.maximum_average_drift_basis_points,
        )?);
    }
    Ok(results)
}

fn normalize_case_result(
    case: &NetworkAiDetectionFixtureCase,
    detection_ref: String,
    maximum_case_drift_basis_points: u16,
) -> Result<NetworkAiDetectionResult, NetworkAiDetectionEvaluationError> {
    let fixture_ref = normalize_ref(&case.fixture_ref)
        .ok_or(NetworkAiDetectionEvaluationError::EmptyFixtureRef)?;
    let summary_ref = normalize_ref(&case.summary_ref)
        .ok_or(NetworkAiDetectionEvaluationError::EmptySummaryRef)?;
    let evidence_refs = normalized_evidence_refs(&case.evidence_refs)?;
    let analyzer_alert_refs = normalized_analyzer_alert_refs(&case.analyzer_alert_refs)?;
    let input_kinds = normalized_input_kinds(&case.input_kinds)?;
    let label_match = case.expected_label == case.predicted_label;
    let expected_positive = is_positive_label(case.expected_label);
    let predicted_positive = is_positive_label(case.predicted_label);
    let confidence_drift_basis_points = case
        .confidence_basis_points
        .abs_diff(case.baseline_confidence_basis_points);

    Ok(NetworkAiDetectionResult {
        detection_ref,
        fixture_ref,
        summary_ref,
        evidence_refs,
        analyzer_alert_refs,
        expected_label: case.expected_label,
        predicted_label: case.predicted_label,
        confidence_basis_points: case.confidence_basis_points,
        baseline_confidence_basis_points: case.baseline_confidence_basis_points,
        confidence_drift_basis_points,
        risk_level: case.risk_level,
        input_kinds,
        label_match,
        expected_positive,
        predicted_positive,
        true_positive: expected_positive && label_match,
        false_positive: predicted_positive && !label_match,
        false_negative: expected_positive && !label_match,
        true_negative: !expected_positive && !predicted_positive && label_match,
        uncertainty_codes: uncertainty_codes(
            case,
            confidence_drift_basis_points,
            maximum_case_drift_basis_points,
        ),
        raw_pcap_available: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
        policy_authority: false,
        adapter_authority: false,
        enforcement_command_published: false,
    })
}

fn reject_global_claims(
    input: &NetworkAiDetectionEvaluationInput,
) -> Result<(), NetworkAiDetectionEvaluationError> {
    if input.model_execution_claimed {
        return Err(NetworkAiDetectionEvaluationError::ModelExecutionClaimRejected);
    }
    if input.remote_ai_claimed {
        return Err(NetworkAiDetectionEvaluationError::RemoteAiClaimRejected);
    }
    if input.raw_pcap_input_claimed {
        return Err(NetworkAiDetectionEvaluationError::RawPcapInputRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkAiDetectionEvaluationError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkAiDetectionEvaluationError::PageContentClaimRejected);
    }
    if input.exact_url_claimed {
        return Err(NetworkAiDetectionEvaluationError::ExactUrlClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkAiDetectionEvaluationError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkAiDetectionEvaluationError::AdapterAuthorityClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkAiDetectionEvaluationError::EnforcementCommandClaimRejected);
    }
    Ok(())
}

fn reject_case_claims(
    case: &NetworkAiDetectionFixtureCase,
) -> Result<(), NetworkAiDetectionEvaluationError> {
    if case.confidence_basis_points > 10_000 || case.baseline_confidence_basis_points > 10_000 {
        return Err(NetworkAiDetectionEvaluationError::BasisPointsOutOfRange);
    }
    if case.raw_pcap_input_claimed {
        return Err(NetworkAiDetectionEvaluationError::RawPcapInputRejected);
    }
    if case.decrypted_payload_claimed {
        return Err(NetworkAiDetectionEvaluationError::DecryptedPayloadClaimRejected);
    }
    if case.page_content_claimed {
        return Err(NetworkAiDetectionEvaluationError::PageContentClaimRejected);
    }
    if case.exact_url_claimed {
        return Err(NetworkAiDetectionEvaluationError::ExactUrlClaimRejected);
    }
    Ok(())
}

fn normalized_evidence_refs(
    values: &[String],
) -> Result<Vec<String>, NetworkAiDetectionEvaluationError> {
    if values.is_empty() {
        return Err(NetworkAiDetectionEvaluationError::EmptyEvidenceRefs);
    }
    let mut refs = Vec::new();
    for value in values {
        let Some(normalized) = normalize_ref(value) else {
            return Err(NetworkAiDetectionEvaluationError::EmptyEvidenceRef);
        };
        if !refs.contains(&normalized) {
            refs.push(normalized);
        }
    }
    Ok(refs)
}

fn normalized_analyzer_alert_refs(
    values: &[String],
) -> Result<Vec<String>, NetworkAiDetectionEvaluationError> {
    let mut refs = Vec::new();
    for value in values {
        let Some(normalized) = normalize_ref(value) else {
            return Err(NetworkAiDetectionEvaluationError::EmptyAnalyzerAlertRef);
        };
        if !refs.contains(&normalized) {
            refs.push(normalized);
        }
    }
    Ok(refs)
}

fn normalized_input_kinds(
    values: &[NetworkAiDetectionInputKind],
) -> Result<Vec<NetworkAiDetectionInputKind>, NetworkAiDetectionEvaluationError> {
    if values.is_empty() {
        return Err(NetworkAiDetectionEvaluationError::EmptyInputKinds);
    }
    let mut kinds = Vec::new();
    for value in values {
        if !kinds.contains(value) {
            kinds.push(*value);
        }
    }
    Ok(kinds)
}

fn uncertainty_codes(
    case: &NetworkAiDetectionFixtureCase,
    drift_basis_points: u16,
    maximum_drift_basis_points: u16,
) -> Vec<NetworkAiDetectionUncertaintyCode> {
    let mut codes = Vec::new();
    if case.expected_label != case.predicted_label {
        codes.push(NetworkAiDetectionUncertaintyCode::LabelMismatch);
    }
    if is_positive_label(case.predicted_label) && case.expected_label != case.predicted_label {
        codes.push(NetworkAiDetectionUncertaintyCode::FalsePositiveFixture);
    }
    if is_positive_label(case.expected_label) && case.expected_label != case.predicted_label {
        codes.push(NetworkAiDetectionUncertaintyCode::FalseNegativeFixture);
    }
    if case.predicted_label == NetworkAiDetectionLabel::Unknown {
        codes.push(NetworkAiDetectionUncertaintyCode::UnknownPrediction);
    }
    if drift_basis_points > maximum_drift_basis_points {
        codes.push(NetworkAiDetectionUncertaintyCode::ConfidenceDriftExceeded);
    }
    if case.confidence_basis_points < 5_000 {
        codes.push(NetworkAiDetectionUncertaintyCode::LowConfidence);
    }
    codes
}

fn count_detection_results(results: &[NetworkAiDetectionResult]) -> DetectionCounts {
    DetectionCounts {
        true_positive: results.iter().filter(|result| result.true_positive).count(),
        false_positive: results
            .iter()
            .filter(|result| result.false_positive)
            .count(),
        false_negative: results
            .iter()
            .filter(|result| result.false_negative)
            .count(),
        true_negative: results.iter().filter(|result| result.true_negative).count(),
        predicted_positive: results
            .iter()
            .filter(|result| result.predicted_positive)
            .count(),
        expected_positive: results
            .iter()
            .filter(|result| result.expected_positive)
            .count(),
    }
}

fn ratio_basis_points(numerator: usize, denominator: usize) -> Option<u16> {
    if denominator == 0 {
        return None;
    }
    let scaled = (numerator as u32 * 10_000 + denominator as u32 / 2) / denominator as u32;
    Some(scaled as u16)
}

fn average_drift_basis_points(results: &[NetworkAiDetectionResult]) -> u16 {
    let total: u32 = results
        .iter()
        .map(|result| result.confidence_drift_basis_points as u32)
        .sum();
    ((total + results.len() as u32 / 2) / results.len() as u32) as u16
}

fn precision_state(
    precision_basis_points: Option<u16>,
    minimum_precision_basis_points: u16,
) -> NetworkAiDetectionPrecisionState {
    match precision_basis_points {
        None => NetworkAiDetectionPrecisionState::NoPositivePredictions,
        Some(precision) if precision >= minimum_precision_basis_points => {
            NetworkAiDetectionPrecisionState::MeetsThreshold
        }
        Some(_) => NetworkAiDetectionPrecisionState::BelowThreshold,
    }
}

fn recall_state(
    recall_basis_points: Option<u16>,
    minimum_recall_basis_points: u16,
) -> NetworkAiDetectionRecallState {
    match recall_basis_points {
        None => NetworkAiDetectionRecallState::NoExpectedPositives,
        Some(recall) if recall >= minimum_recall_basis_points => {
            NetworkAiDetectionRecallState::MeetsThreshold
        }
        Some(_) => NetworkAiDetectionRecallState::BelowThreshold,
    }
}

fn drift_state(
    average_confidence_drift_basis_points: u16,
    maximum_average_drift_basis_points: u16,
) -> NetworkAiDetectionDriftState {
    if average_confidence_drift_basis_points > maximum_average_drift_basis_points {
        NetworkAiDetectionDriftState::ExceededTolerance
    } else {
        NetworkAiDetectionDriftState::WithinTolerance
    }
}

fn evaluation_state(
    precision_state: NetworkAiDetectionPrecisionState,
    recall_state: NetworkAiDetectionRecallState,
    drift_state: NetworkAiDetectionDriftState,
) -> NetworkAiDetectionEvaluationState {
    let quality_passed = precision_state == NetworkAiDetectionPrecisionState::MeetsThreshold
        && recall_state == NetworkAiDetectionRecallState::MeetsThreshold;
    match (quality_passed, drift_state) {
        (true, NetworkAiDetectionDriftState::WithinTolerance) => {
            NetworkAiDetectionEvaluationState::MeetsFixtureGate
        }
        (true, NetworkAiDetectionDriftState::ExceededTolerance) => {
            NetworkAiDetectionEvaluationState::DriftExceeded
        }
        (false, NetworkAiDetectionDriftState::WithinTolerance) => {
            NetworkAiDetectionEvaluationState::BelowQualityThreshold
        }
        (false, NetworkAiDetectionDriftState::ExceededTolerance) => {
            NetworkAiDetectionEvaluationState::BelowQualityAndDriftExceeded
        }
    }
}

fn is_positive_label(label: NetworkAiDetectionLabel) -> bool {
    !matches!(
        label,
        NetworkAiDetectionLabel::BenignExpected | NetworkAiDetectionLabel::Unknown
    )
}

fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

struct DetectionCounts {
    true_positive: usize,
    false_positive: usize,
    false_negative: usize,
    true_negative: usize,
    predicted_positive: usize,
    expected_positive: usize,
}
