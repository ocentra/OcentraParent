use serde::{Deserialize, Serialize};

mod rows;
mod validation;

use crate::dns::types::{DnsObservation, NetworkEvidenceGrade};
use crate::flow::{NetworkFlowProtocol, NetworkFlowSession};

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
    let analyzer_run_ref = rows::normalize_ref(&input.analyzer_run_ref)
        .ok_or(NetworkZeekAnalyzerError::EmptyAnalyzerRunRef)?;
    let source_fixture_ref = rows::normalize_ref(&input.source_fixture_ref)
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
    rows::connection_rows(analyzer_run_ref, source_fixture_ref, sessions)
}

fn dns_rows(
    analyzer_run_ref: &str,
    source_fixture_ref: &str,
    observations: &[DnsObservation],
) -> Vec<NetworkZeekDnsRow> {
    rows::dns_rows(analyzer_run_ref, source_fixture_ref, observations)
}

fn http_rows(
    analyzer_run_ref: &str,
    source_fixture_ref: &str,
    evidence: &[NetworkZeekHttpEvidence],
) -> Result<Vec<NetworkZeekHttpRow>, NetworkZeekAnalyzerError> {
    rows::http_rows(analyzer_run_ref, source_fixture_ref, evidence)
}

fn build_tls_rows(
    analyzer_run_ref: &str,
    source_fixture_ref: &str,
    evidence: &[NetworkZeekTlsEvidence],
) -> Result<Vec<NetworkZeekTlsRow>, NetworkZeekAnalyzerError> {
    rows::build_tls_rows(analyzer_run_ref, source_fixture_ref, evidence)
}

fn validate_comparison_artifacts(
    artifacts: &[NetworkZeekAnalyzerComparisonArtifact],
) -> Result<(), NetworkZeekAnalyzerError> {
    validation::validate_comparison_artifacts(artifacts)
}

fn required_log_kinds(
    connection_rows: &[NetworkZeekConnectionRow],
    dns_rows: &[NetworkZeekDnsRow],
    http_rows: &[NetworkZeekHttpRow],
    tls_rows: &[NetworkZeekTlsRow],
    ssl_rows: &[NetworkZeekTlsRow],
) -> Vec<NetworkZeekLogKind> {
    validation::required_log_kinds(connection_rows, dns_rows, http_rows, tls_rows, ssl_rows)
}

fn missing_comparison_log_kinds(
    required_log_kinds: &[NetworkZeekLogKind],
    artifacts: &[NetworkZeekAnalyzerComparisonArtifact],
) -> Vec<NetworkZeekLogKind> {
    validation::missing_comparison_log_kinds(required_log_kinds, artifacts)
}

fn validate_comparison_counts(
    artifacts: &[NetworkZeekAnalyzerComparisonArtifact],
    connection_rows: &[NetworkZeekConnectionRow],
    dns_rows: &[NetworkZeekDnsRow],
    http_rows: &[NetworkZeekHttpRow],
    tls_rows: &[NetworkZeekTlsRow],
    ssl_rows: &[NetworkZeekTlsRow],
) -> Result<(), NetworkZeekAnalyzerError> {
    validation::validate_comparison_counts(
        artifacts,
        connection_rows,
        dns_rows,
        http_rows,
        tls_rows,
        ssl_rows,
    )
}

fn reject_unsupported_claims(
    input: &NetworkZeekAnalyzerInput,
) -> Result<(), NetworkZeekAnalyzerError> {
    validation::reject_unsupported_claims(input)
}
