use serde::{Deserialize, Serialize};

use crate::{
    NetworkAiDetectionResult, NetworkAiDetectionRiskLevel, NetworkAiDetectionUncertaintyCode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAiAuditNarrativeState {
    Ready,
    UncertainReviewRequired,
    MonitorOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAiAuditRecommendationKind {
    ReviewWithParent,
    ConfirmWithManagedBrowser,
    ConfirmWithScreenSummary,
    ReviewPolicyRule,
    MonitorOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAiAuditUncertaintyCode {
    DetectionMismatch,
    FalsePositiveFixture,
    FalseNegativeFixture,
    UnknownPrediction,
    ConfidenceDriftExceeded,
    LowConfidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAiAuditReportInput {
    pub audit_report_ref: String,
    pub narrative_template_ref: String,
    pub model_version_ref: String,
    pub policy_context_ref: String,
    pub detection_results: Vec<NetworkAiDetectionResult>,
    pub parent_rule_refs: Vec<String>,
    pub remote_ai_claimed: bool,
    pub raw_pcap_input_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub exact_url_claimed: bool,
    pub private_message_claimed: bool,
    pub search_query_claimed: bool,
    pub policy_authority_claimed: bool,
    pub adapter_authority_claimed: bool,
    pub enforcement_command_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAiAuditRecommendation {
    pub recommendation_ref: String,
    pub kind: NetworkAiAuditRecommendationKind,
    pub cited_detection_refs: Vec<String>,
    pub cited_evidence_refs: Vec<String>,
    pub cited_parent_rule_refs: Vec<String>,
    pub advisory_only: bool,
    pub policy_authority: bool,
    pub adapter_authority: bool,
    pub enforcement_command_published: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAiAuditReport {
    pub audit_report_ref: String,
    pub narrative_template_ref: String,
    pub model_version_ref: String,
    pub policy_context_ref: String,
    pub narrative_state: NetworkAiAuditNarrativeState,
    pub narrative_headline: String,
    pub cited_detection_refs: Vec<String>,
    pub cited_evidence_refs: Vec<String>,
    pub cited_analyzer_alert_refs: Vec<String>,
    pub cited_parent_rule_refs: Vec<String>,
    pub uncertainty_codes: Vec<NetworkAiAuditUncertaintyCode>,
    pub recommendations: Vec<NetworkAiAuditRecommendation>,
    pub parent_readable: bool,
    pub advisory_only: bool,
    pub raw_pcap_available: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
    pub private_message_available: bool,
    pub search_query_available: bool,
    pub remote_ai_used: bool,
    pub policy_authority: bool,
    pub adapter_authority: bool,
    pub enforcement_commands_published: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkAiAuditReportError {
    EmptyAuditReportRef,
    EmptyNarrativeTemplateRef,
    EmptyModelVersionRef,
    EmptyPolicyContextRef,
    EmptyDetectionResults,
    EmptyDetectionRef,
    DuplicateDetectionRef,
    EmptyEvidenceRefs,
    EmptyEvidenceRef,
    EmptyAnalyzerAlertRef,
    EmptyParentRuleRefs,
    EmptyParentRuleRef,
    RemoteAiClaimRejected,
    RawPcapInputRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    ExactUrlClaimRejected,
    PrivateMessageClaimRejected,
    SearchQueryClaimRejected,
    PolicyAuthorityClaimRejected,
    AdapterAuthorityClaimRejected,
    EnforcementCommandClaimRejected,
}

pub fn build_network_ai_audit_report(
    input: NetworkAiAuditReportInput,
) -> Result<NetworkAiAuditReport, NetworkAiAuditReportError> {
    reject_global_claims(&input)?;
    if input.detection_results.is_empty() {
        return Err(NetworkAiAuditReportError::EmptyDetectionResults);
    }

    let audit_report_ref = normalize_ref(&input.audit_report_ref)
        .ok_or(NetworkAiAuditReportError::EmptyAuditReportRef)?;
    let narrative_template_ref = normalize_ref(&input.narrative_template_ref)
        .ok_or(NetworkAiAuditReportError::EmptyNarrativeTemplateRef)?;
    let model_version_ref = normalize_ref(&input.model_version_ref)
        .ok_or(NetworkAiAuditReportError::EmptyModelVersionRef)?;
    let policy_context_ref = normalize_ref(&input.policy_context_ref)
        .ok_or(NetworkAiAuditReportError::EmptyPolicyContextRef)?;
    let parent_rule_refs = normalized_refs(
        &input.parent_rule_refs,
        NetworkAiAuditReportError::EmptyParentRuleRefs,
        NetworkAiAuditReportError::EmptyParentRuleRef,
    )?;
    let detection_refs = normalized_detection_refs(&input.detection_results)?;
    let evidence_refs = cited_evidence_refs(&input.detection_results)?;
    let analyzer_alert_refs = cited_analyzer_alert_refs(&input.detection_results)?;
    let uncertainty_codes = audit_uncertainty_codes(&input.detection_results);
    let narrative_state = narrative_state(&input.detection_results, &uncertainty_codes);
    let detection_results = input.detection_results;
    let recommendations = recommendations(
        &audit_report_ref,
        &detection_refs,
        &evidence_refs,
        &parent_rule_refs,
        recommendation_kinds(&detection_results, &uncertainty_codes),
    );

    Ok(NetworkAiAuditReport {
        audit_report_ref,
        narrative_template_ref,
        model_version_ref,
        policy_context_ref,
        narrative_state,
        narrative_headline: narrative_headline(narrative_state),
        cited_detection_refs: detection_refs,
        cited_evidence_refs: evidence_refs,
        cited_analyzer_alert_refs: analyzer_alert_refs,
        cited_parent_rule_refs: parent_rule_refs,
        recommendations,
        uncertainty_codes,
        parent_readable: true,
        advisory_only: true,
        raw_pcap_available: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
        private_message_available: false,
        search_query_available: false,
        remote_ai_used: false,
        policy_authority: false,
        adapter_authority: false,
        enforcement_commands_published: 0,
    })
}

fn reject_global_claims(
    input: &NetworkAiAuditReportInput,
) -> Result<(), NetworkAiAuditReportError> {
    if input.remote_ai_claimed {
        return Err(NetworkAiAuditReportError::RemoteAiClaimRejected);
    }
    if input.raw_pcap_input_claimed {
        return Err(NetworkAiAuditReportError::RawPcapInputRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkAiAuditReportError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkAiAuditReportError::PageContentClaimRejected);
    }
    if input.exact_url_claimed {
        return Err(NetworkAiAuditReportError::ExactUrlClaimRejected);
    }
    if input.private_message_claimed {
        return Err(NetworkAiAuditReportError::PrivateMessageClaimRejected);
    }
    if input.search_query_claimed {
        return Err(NetworkAiAuditReportError::SearchQueryClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkAiAuditReportError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkAiAuditReportError::AdapterAuthorityClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkAiAuditReportError::EnforcementCommandClaimRejected);
    }
    Ok(())
}

fn normalized_detection_refs(
    detections: &[NetworkAiDetectionResult],
) -> Result<Vec<String>, NetworkAiAuditReportError> {
    let mut refs = Vec::new();
    for detection in detections {
        reject_detection_claims(detection)?;
        let detection_ref = normalize_ref(&detection.detection_ref)
            .ok_or(NetworkAiAuditReportError::EmptyDetectionRef)?;
        if refs.contains(&detection_ref) {
            return Err(NetworkAiAuditReportError::DuplicateDetectionRef);
        }
        refs.push(detection_ref);
    }
    Ok(refs)
}

fn reject_detection_claims(
    detection: &NetworkAiDetectionResult,
) -> Result<(), NetworkAiAuditReportError> {
    if detection.raw_pcap_available {
        return Err(NetworkAiAuditReportError::RawPcapInputRejected);
    }
    if detection.decrypted_payload_available {
        return Err(NetworkAiAuditReportError::DecryptedPayloadClaimRejected);
    }
    if detection.page_content_available {
        return Err(NetworkAiAuditReportError::PageContentClaimRejected);
    }
    if detection.exact_url_available {
        return Err(NetworkAiAuditReportError::ExactUrlClaimRejected);
    }
    if detection.policy_authority {
        return Err(NetworkAiAuditReportError::PolicyAuthorityClaimRejected);
    }
    if detection.adapter_authority {
        return Err(NetworkAiAuditReportError::AdapterAuthorityClaimRejected);
    }
    if detection.enforcement_command_published {
        return Err(NetworkAiAuditReportError::EnforcementCommandClaimRejected);
    }
    Ok(())
}

fn cited_evidence_refs(
    detections: &[NetworkAiDetectionResult],
) -> Result<Vec<String>, NetworkAiAuditReportError> {
    let mut refs = Vec::new();
    for detection in detections {
        let detection_refs = normalized_refs(
            &detection.evidence_refs,
            NetworkAiAuditReportError::EmptyEvidenceRefs,
            NetworkAiAuditReportError::EmptyEvidenceRef,
        )?;
        extend_unique(&mut refs, detection_refs);
    }
    Ok(refs)
}

fn cited_analyzer_alert_refs(
    detections: &[NetworkAiDetectionResult],
) -> Result<Vec<String>, NetworkAiAuditReportError> {
    let mut refs = Vec::new();
    for detection in detections {
        let detection_refs = normalized_refs(
            &detection.analyzer_alert_refs,
            NetworkAiAuditReportError::EmptyAnalyzerAlertRef,
            NetworkAiAuditReportError::EmptyAnalyzerAlertRef,
        )?;
        extend_unique(&mut refs, detection_refs);
    }
    Ok(refs)
}

fn normalized_refs(
    values: &[String],
    empty_values_error: NetworkAiAuditReportError,
    empty_ref_error: NetworkAiAuditReportError,
) -> Result<Vec<String>, NetworkAiAuditReportError> {
    if values.is_empty() && empty_values_error != NetworkAiAuditReportError::EmptyAnalyzerAlertRef {
        return Err(empty_values_error);
    }
    let mut refs = Vec::new();
    for value in values {
        let Some(normalized) = normalize_ref(value) else {
            return Err(empty_ref_error);
        };
        if !refs.contains(&normalized) {
            refs.push(normalized);
        }
    }
    Ok(refs)
}

fn audit_uncertainty_codes(
    detections: &[NetworkAiDetectionResult],
) -> Vec<NetworkAiAuditUncertaintyCode> {
    let mut codes = Vec::new();
    for detection in detections {
        for code in &detection.uncertainty_codes {
            let audit_code = match code {
                NetworkAiDetectionUncertaintyCode::LabelMismatch => {
                    NetworkAiAuditUncertaintyCode::DetectionMismatch
                }
                NetworkAiDetectionUncertaintyCode::FalsePositiveFixture => {
                    NetworkAiAuditUncertaintyCode::FalsePositiveFixture
                }
                NetworkAiDetectionUncertaintyCode::FalseNegativeFixture => {
                    NetworkAiAuditUncertaintyCode::FalseNegativeFixture
                }
                NetworkAiDetectionUncertaintyCode::UnknownPrediction => {
                    NetworkAiAuditUncertaintyCode::UnknownPrediction
                }
                NetworkAiDetectionUncertaintyCode::ConfidenceDriftExceeded => {
                    NetworkAiAuditUncertaintyCode::ConfidenceDriftExceeded
                }
                NetworkAiDetectionUncertaintyCode::LowConfidence => {
                    NetworkAiAuditUncertaintyCode::LowConfidence
                }
            };
            if !codes.contains(&audit_code) {
                codes.push(audit_code);
            }
        }
    }
    codes
}

fn recommendation_kinds(
    detections: &[NetworkAiDetectionResult],
    uncertainty_codes: &[NetworkAiAuditUncertaintyCode],
) -> Vec<NetworkAiAuditRecommendationKind> {
    let mut kinds = Vec::new();
    if has_high_risk_true_positive(detections) {
        kinds.push(NetworkAiAuditRecommendationKind::ReviewWithParent);
        kinds.push(NetworkAiAuditRecommendationKind::ReviewPolicyRule);
    }
    if !uncertainty_codes.is_empty() {
        kinds.push(NetworkAiAuditRecommendationKind::ConfirmWithManagedBrowser);
        kinds.push(NetworkAiAuditRecommendationKind::ConfirmWithScreenSummary);
    }
    if kinds.is_empty() {
        kinds.push(NetworkAiAuditRecommendationKind::MonitorOnly);
    }
    kinds
}

fn recommendations(
    audit_report_ref: &str,
    detection_refs: &[String],
    evidence_refs: &[String],
    parent_rule_refs: &[String],
    kinds: Vec<NetworkAiAuditRecommendationKind>,
) -> Vec<NetworkAiAuditRecommendation> {
    kinds
        .into_iter()
        .map(|kind| NetworkAiAuditRecommendation {
            recommendation_ref: format!("{audit_report_ref}:{kind:?}"),
            kind,
            cited_detection_refs: detection_refs.to_vec(),
            cited_evidence_refs: evidence_refs.to_vec(),
            cited_parent_rule_refs: parent_rule_refs.to_vec(),
            advisory_only: true,
            policy_authority: false,
            adapter_authority: false,
            enforcement_command_published: false,
        })
        .collect()
}

fn narrative_state(
    detections: &[NetworkAiDetectionResult],
    uncertainty_codes: &[NetworkAiAuditUncertaintyCode],
) -> NetworkAiAuditNarrativeState {
    if !uncertainty_codes.is_empty() {
        return NetworkAiAuditNarrativeState::UncertainReviewRequired;
    }
    if has_high_risk_true_positive(detections) {
        return NetworkAiAuditNarrativeState::Ready;
    }
    NetworkAiAuditNarrativeState::MonitorOnly
}

fn narrative_headline(state: NetworkAiAuditNarrativeState) -> String {
    match state {
        NetworkAiAuditNarrativeState::Ready => {
            "Network AI audit recommends parent review for cited high-risk network detections."
        }
        NetworkAiAuditNarrativeState::UncertainReviewRequired => {
            "Network AI audit found uncertainty and recommends evidence confirmation before policy action."
        }
        NetworkAiAuditNarrativeState::MonitorOnly => {
            "Network AI audit recommends monitor-only handling for cited network detections."
        }
    }
    .to_owned()
}

fn has_high_risk_true_positive(detections: &[NetworkAiDetectionResult]) -> bool {
    detections.iter().any(|detection| {
        detection.true_positive
            && matches!(
                detection.risk_level,
                NetworkAiDetectionRiskLevel::High | NetworkAiDetectionRiskLevel::Critical
            )
    })
}

fn extend_unique(target: &mut Vec<String>, values: Vec<String>) {
    for value in values {
        if !target.contains(&value) {
            target.push(value);
        }
    }
}

fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
