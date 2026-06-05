use serde::{Deserialize, Serialize};

use crate::{DnsObservation, NetworkEvidenceGrade, NetworkFlowProtocol, NetworkFlowSession};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkZeekLogKind {
    Conn,
    Dns,
    Http,
    Tls,
    Ssl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkZeekVisibilityState {
    Visible,
    Unknown,
    Missing,
    Ambiguous,
    Encrypted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkZeekAnalyzerComparisonState {
    Matched,
    MissingApprovedComparison,
    Mismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkZeekHttpEvidence {
    pub evidence_ref: String,
    pub flow_ref: String,
    pub observed_at_micros: u64,
    pub host: Option<String>,
    pub visibility_state: NetworkZeekVisibilityState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkZeekTlsEvidence {
    pub evidence_ref: String,
    pub flow_ref: String,
    pub observed_at_micros: u64,
    pub server_name: Option<String>,
    pub visibility_state: NetworkZeekVisibilityState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkZeekAnalyzerComparisonArtifact {
    pub log_kind: NetworkZeekLogKind,
    pub artifact_ref: String,
    pub expected_row_count: usize,
    pub observed_row_count: usize,
    pub matched_row_count: usize,
    pub preserved_unknown_or_ambiguous_rows: usize,
    pub approved_fixture_output: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkZeekAnalyzerInput {
    pub analyzer_run_ref: String,
    pub source_fixture_ref: String,
    pub flow_sessions: Vec<NetworkFlowSession>,
    pub dns_observations: Vec<DnsObservation>,
    pub http_evidence: Vec<NetworkZeekHttpEvidence>,
    pub tls_evidence: Vec<NetworkZeekTlsEvidence>,
    pub ssl_evidence: Vec<NetworkZeekTlsEvidence>,
    pub comparison_artifacts: Vec<NetworkZeekAnalyzerComparisonArtifact>,
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub signature_alert_claimed: bool,
    pub live_analyzer_invocation_claimed: bool,
    pub policy_authority_claimed: bool,
    pub adapter_authority_claimed: bool,
    pub enforcement_command_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkZeekConnectionRow {
    pub row_ref: String,
    pub source_fixture_ref: String,
    pub source_ip: String,
    pub source_port: u16,
    pub destination_ip: String,
    pub destination_port: u16,
    pub protocol: NetworkFlowProtocol,
    pub first_seen_micros: u64,
    pub duration_micros: u64,
    pub origin_bytes: usize,
    pub response_bytes: usize,
    pub origin_packets: usize,
    pub response_packets: usize,
    pub evidence_grade: NetworkEvidenceGrade,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkZeekDnsRow {
    pub row_ref: String,
    pub source_fixture_ref: String,
    pub query_name: String,
    pub query_type: String,
    pub source_ip: String,
    pub destination_ip: String,
    pub observed_at_micros: u64,
    pub evidence_grade: NetworkEvidenceGrade,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkZeekHttpRow {
    pub row_ref: String,
    pub evidence_ref: String,
    pub flow_ref: String,
    pub source_fixture_ref: String,
    pub observed_at_micros: u64,
    pub host: Option<String>,
    pub visibility_state: NetworkZeekVisibilityState,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkZeekTlsRow {
    pub row_ref: String,
    pub evidence_ref: String,
    pub flow_ref: String,
    pub source_fixture_ref: String,
    pub observed_at_micros: u64,
    pub server_name: Option<String>,
    pub visibility_state: NetworkZeekVisibilityState,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkZeekAnalyzerProof {
    pub analyzer_run_ref: String,
    pub source_fixture_ref: String,
    pub connection_rows: Vec<NetworkZeekConnectionRow>,
    pub dns_rows: Vec<NetworkZeekDnsRow>,
    pub http_rows: Vec<NetworkZeekHttpRow>,
    pub tls_rows: Vec<NetworkZeekTlsRow>,
    pub ssl_rows: Vec<NetworkZeekTlsRow>,
    pub comparison_artifacts: Vec<NetworkZeekAnalyzerComparisonArtifact>,
    pub comparison_state: NetworkZeekAnalyzerComparisonState,
    pub missing_comparison_log_kinds: Vec<NetworkZeekLogKind>,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
    pub signature_alert_ingested: bool,
    pub live_analyzer_invoked: bool,
    pub policy_authority: bool,
    pub adapter_authority: bool,
    pub enforcement_command_published: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkZeekAnalyzerError {
    EmptyAnalyzerRunRef,
    EmptySourceFixtureRef,
    EmptyHttpEvidenceRef,
    EmptyHttpFlowRef,
    EmptyTlsEvidenceRef,
    EmptyTlsFlowRef,
    EmptyComparisonArtifactRef(NetworkZeekLogKind),
    VisibleHttpHostMissing,
    VisibleTlsServerNameMissing,
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    SignatureAlertClaimRejected,
    LiveAnalyzerInvocationClaimRejected,
    PolicyAuthorityClaimRejected,
    AdapterAuthorityClaimRejected,
    EnforcementCommandClaimRejected,
    MissingApprovedComparison(NetworkZeekLogKind),
    ComparisonMismatch(NetworkZeekLogKind),
}

pub fn generate_network_zeek_analyzer_proof(
    input: NetworkZeekAnalyzerInput,
) -> Result<NetworkZeekAnalyzerProof, NetworkZeekAnalyzerError> {
    reject_unsupported_claims(&input)?;
    let analyzer_run_ref = normalize_ref(&input.analyzer_run_ref)
        .ok_or(NetworkZeekAnalyzerError::EmptyAnalyzerRunRef)?;
    let source_fixture_ref = normalize_ref(&input.source_fixture_ref)
        .ok_or(NetworkZeekAnalyzerError::EmptySourceFixtureRef)?;

    let connection_rows =
        connection_rows(&analyzer_run_ref, &source_fixture_ref, &input.flow_sessions);
    let dns_rows = dns_rows(
        &analyzer_run_ref,
        &source_fixture_ref,
        &input.dns_observations,
    );
    let http_rows = http_rows(&analyzer_run_ref, &source_fixture_ref, &input.http_evidence)?;
    let tls_rows = build_tls_rows(&analyzer_run_ref, &source_fixture_ref, &input.tls_evidence)?;
    let ssl_rows = build_tls_rows(&analyzer_run_ref, &source_fixture_ref, &input.ssl_evidence)?;
    validate_comparison_artifacts(&input.comparison_artifacts)?;

    let required_log_kinds = required_log_kinds(
        &connection_rows,
        &dns_rows,
        &http_rows,
        &tls_rows,
        &ssl_rows,
    );
    let missing_comparison_log_kinds =
        missing_comparison_log_kinds(&required_log_kinds, &input.comparison_artifacts);
    if let Some(missing) = missing_comparison_log_kinds.first() {
        return Err(NetworkZeekAnalyzerError::MissingApprovedComparison(
            *missing,
        ));
    }

    validate_comparison_counts(
        &input.comparison_artifacts,
        &connection_rows,
        &dns_rows,
        &http_rows,
        &tls_rows,
        &ssl_rows,
    )?;

    Ok(NetworkZeekAnalyzerProof {
        analyzer_run_ref,
        source_fixture_ref,
        connection_rows,
        dns_rows,
        http_rows,
        tls_rows,
        ssl_rows,
        comparison_artifacts: input.comparison_artifacts,
        comparison_state: NetworkZeekAnalyzerComparisonState::Matched,
        missing_comparison_log_kinds,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
        signature_alert_ingested: false,
        live_analyzer_invoked: false,
        policy_authority: false,
        adapter_authority: false,
        enforcement_command_published: false,
    })
}

fn connection_rows(
    analyzer_run_ref: &str,
    source_fixture_ref: &str,
    sessions: &[NetworkFlowSession],
) -> Vec<NetworkZeekConnectionRow> {
    sessions
        .iter()
        .enumerate()
        .map(|(index, session)| NetworkZeekConnectionRow {
            row_ref: row_ref(analyzer_run_ref, NetworkZeekLogKind::Conn, index),
            source_fixture_ref: source_fixture_ref.to_owned(),
            source_ip: session.key.initiator_ip.clone(),
            source_port: session.key.initiator_port,
            destination_ip: session.key.responder_ip.clone(),
            destination_port: session.key.responder_port,
            protocol: session.key.protocol,
            first_seen_micros: session.first_seen_micros,
            duration_micros: session.duration_micros,
            origin_bytes: session.initiator_to_responder_bytes,
            response_bytes: session.responder_to_initiator_bytes,
            origin_packets: session.initiator_to_responder_packets,
            response_packets: session.responder_to_initiator_packets,
            evidence_grade: session.evidence_grade,
            exact_url_available: false,
            decrypted_payload_available: false,
        })
        .collect()
}

fn dns_rows(
    analyzer_run_ref: &str,
    source_fixture_ref: &str,
    observations: &[DnsObservation],
) -> Vec<NetworkZeekDnsRow> {
    observations
        .iter()
        .enumerate()
        .map(|(index, observation)| NetworkZeekDnsRow {
            row_ref: row_ref(analyzer_run_ref, NetworkZeekLogKind::Dns, index),
            source_fixture_ref: source_fixture_ref.to_owned(),
            query_name: observation.query_name.clone(),
            query_type: format!("{:?}", observation.query_type),
            source_ip: observation.source_ip.clone(),
            destination_ip: observation.destination_ip.clone(),
            observed_at_micros: observation.observed_at_micros,
            evidence_grade: observation.evidence_grade,
            exact_url_available: false,
            decrypted_payload_available: false,
        })
        .collect()
}

fn http_rows(
    analyzer_run_ref: &str,
    source_fixture_ref: &str,
    evidence: &[NetworkZeekHttpEvidence],
) -> Result<Vec<NetworkZeekHttpRow>, NetworkZeekAnalyzerError> {
    evidence
        .iter()
        .enumerate()
        .map(|(index, input)| {
            let evidence_ref = normalize_ref(&input.evidence_ref)
                .ok_or(NetworkZeekAnalyzerError::EmptyHttpEvidenceRef)?;
            let flow_ref =
                normalize_ref(&input.flow_ref).ok_or(NetworkZeekAnalyzerError::EmptyHttpFlowRef)?;
            if input.visibility_state == NetworkZeekVisibilityState::Visible
                && normalized_optional_text(input.host.as_deref()).is_none()
            {
                return Err(NetworkZeekAnalyzerError::VisibleHttpHostMissing);
            }

            Ok(NetworkZeekHttpRow {
                row_ref: row_ref(analyzer_run_ref, NetworkZeekLogKind::Http, index),
                evidence_ref,
                flow_ref,
                source_fixture_ref: source_fixture_ref.to_owned(),
                observed_at_micros: input.observed_at_micros,
                host: normalized_optional_text(input.host.as_deref()),
                visibility_state: input.visibility_state,
                exact_url_available: false,
                decrypted_payload_available: false,
                page_content_available: false,
            })
        })
        .collect()
}

fn build_tls_rows(
    analyzer_run_ref: &str,
    source_fixture_ref: &str,
    evidence: &[NetworkZeekTlsEvidence],
) -> Result<Vec<NetworkZeekTlsRow>, NetworkZeekAnalyzerError> {
    evidence
        .iter()
        .enumerate()
        .map(|(index, input)| {
            let evidence_ref = normalize_ref(&input.evidence_ref)
                .ok_or(NetworkZeekAnalyzerError::EmptyTlsEvidenceRef)?;
            let flow_ref =
                normalize_ref(&input.flow_ref).ok_or(NetworkZeekAnalyzerError::EmptyTlsFlowRef)?;
            if input.visibility_state == NetworkZeekVisibilityState::Visible
                && normalized_optional_text(input.server_name.as_deref()).is_none()
            {
                return Err(NetworkZeekAnalyzerError::VisibleTlsServerNameMissing);
            }

            Ok(NetworkZeekTlsRow {
                row_ref: row_ref(analyzer_run_ref, NetworkZeekLogKind::Tls, index),
                evidence_ref,
                flow_ref,
                source_fixture_ref: source_fixture_ref.to_owned(),
                observed_at_micros: input.observed_at_micros,
                server_name: normalized_optional_text(input.server_name.as_deref()),
                visibility_state: input.visibility_state,
                exact_url_available: false,
                decrypted_payload_available: false,
                page_content_available: false,
            })
        })
        .collect()
}

fn validate_comparison_artifacts(
    artifacts: &[NetworkZeekAnalyzerComparisonArtifact],
) -> Result<(), NetworkZeekAnalyzerError> {
    for artifact in artifacts {
        if normalize_ref(&artifact.artifact_ref).is_none() {
            return Err(NetworkZeekAnalyzerError::EmptyComparisonArtifactRef(
                artifact.log_kind,
            ));
        }
        if !artifact.approved_fixture_output {
            return Err(NetworkZeekAnalyzerError::MissingApprovedComparison(
                artifact.log_kind,
            ));
        }
    }
    Ok(())
}

fn required_log_kinds(
    connection_rows: &[NetworkZeekConnectionRow],
    dns_rows: &[NetworkZeekDnsRow],
    http_rows: &[NetworkZeekHttpRow],
    tls_rows: &[NetworkZeekTlsRow],
    ssl_rows: &[NetworkZeekTlsRow],
) -> Vec<NetworkZeekLogKind> {
    let mut kinds = Vec::new();
    push_required(
        &mut kinds,
        NetworkZeekLogKind::Conn,
        !connection_rows.is_empty(),
    );
    push_required(&mut kinds, NetworkZeekLogKind::Dns, !dns_rows.is_empty());
    push_required(&mut kinds, NetworkZeekLogKind::Http, !http_rows.is_empty());
    push_required(&mut kinds, NetworkZeekLogKind::Tls, !tls_rows.is_empty());
    push_required(&mut kinds, NetworkZeekLogKind::Ssl, !ssl_rows.is_empty());
    kinds
}

fn missing_comparison_log_kinds(
    required_log_kinds: &[NetworkZeekLogKind],
    artifacts: &[NetworkZeekAnalyzerComparisonArtifact],
) -> Vec<NetworkZeekLogKind> {
    required_log_kinds
        .iter()
        .copied()
        .filter(|kind| {
            !artifacts
                .iter()
                .any(|artifact| artifact.log_kind == *kind && artifact.approved_fixture_output)
        })
        .collect()
}

fn validate_comparison_counts(
    artifacts: &[NetworkZeekAnalyzerComparisonArtifact],
    connection_rows: &[NetworkZeekConnectionRow],
    dns_rows: &[NetworkZeekDnsRow],
    http_rows: &[NetworkZeekHttpRow],
    tls_rows: &[NetworkZeekTlsRow],
    ssl_rows: &[NetworkZeekTlsRow],
) -> Result<(), NetworkZeekAnalyzerError> {
    for artifact in artifacts {
        let row_count = row_count(
            artifact.log_kind,
            connection_rows,
            dns_rows,
            http_rows,
            tls_rows,
            ssl_rows,
        );
        if artifact.expected_row_count != row_count
            || artifact.observed_row_count != row_count
            || artifact.matched_row_count != row_count
        {
            return Err(NetworkZeekAnalyzerError::ComparisonMismatch(
                artifact.log_kind,
            ));
        }
    }
    Ok(())
}

fn row_count(
    log_kind: NetworkZeekLogKind,
    connection_rows: &[NetworkZeekConnectionRow],
    dns_rows: &[NetworkZeekDnsRow],
    http_rows: &[NetworkZeekHttpRow],
    tls_rows: &[NetworkZeekTlsRow],
    ssl_rows: &[NetworkZeekTlsRow],
) -> usize {
    match log_kind {
        NetworkZeekLogKind::Conn => connection_rows.len(),
        NetworkZeekLogKind::Dns => dns_rows.len(),
        NetworkZeekLogKind::Http => http_rows.len(),
        NetworkZeekLogKind::Tls => tls_rows.len(),
        NetworkZeekLogKind::Ssl => ssl_rows.len(),
    }
}

fn reject_unsupported_claims(
    input: &NetworkZeekAnalyzerInput,
) -> Result<(), NetworkZeekAnalyzerError> {
    if input.exact_url_claimed {
        return Err(NetworkZeekAnalyzerError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkZeekAnalyzerError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkZeekAnalyzerError::PageContentClaimRejected);
    }
    if input.signature_alert_claimed {
        return Err(NetworkZeekAnalyzerError::SignatureAlertClaimRejected);
    }
    if input.live_analyzer_invocation_claimed {
        return Err(NetworkZeekAnalyzerError::LiveAnalyzerInvocationClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkZeekAnalyzerError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkZeekAnalyzerError::AdapterAuthorityClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkZeekAnalyzerError::EnforcementCommandClaimRejected);
    }
    Ok(())
}

fn row_ref(analyzer_run_ref: &str, log_kind: NetworkZeekLogKind, index: usize) -> String {
    format!("{analyzer_run_ref}::{:?}::{index}", log_kind).to_ascii_lowercase()
}

fn push_required(kinds: &mut Vec<NetworkZeekLogKind>, kind: NetworkZeekLogKind, required: bool) {
    if required {
        kinds.push(kind);
    }
}

fn normalized_optional_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|trimmed| !trimmed.is_empty())
        .map(|trimmed| trimmed.to_ascii_lowercase())
}

fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
