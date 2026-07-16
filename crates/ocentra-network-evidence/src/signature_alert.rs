mod normalization;
mod validation;

use serde::{Deserialize, Serialize};

use self::{
    normalization::{count_source, normalize_records, normalize_ref},
    validation::reject_global_claims,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkSignatureAlertSource {
    Suricata,
    SnortCompatible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkSignatureAlertSeverity {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkSignatureAlertState {
    AnalyzerEvidenceOnly,
    ReviewCandidate,
    FalsePositiveNonEnforcing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkSignatureAlertFixtureRow {
    pub alert_ref: String,
    pub source: NetworkSignatureAlertSource,
    pub signature_id: String,
    pub signature_name: String,
    pub rule_source_ref: String,
    pub severity: NetworkSignatureAlertSeverity,
    pub observed_at_micros: u64,
    pub flow_ref: String,
    pub evidence_ref: String,
    pub custody_ref: String,
    pub known_false_positive: bool,
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkSignatureAlertIngestionInput {
    pub ingestion_run_ref: String,
    pub fixture_ref: String,
    pub rows: Vec<NetworkSignatureAlertFixtureRow>,
    pub live_suricata_invocation_claimed: bool,
    pub live_snort_invocation_claimed: bool,
    pub ips_prevention_claimed: bool,
    pub policy_authority_claimed: bool,
    pub adapter_authority_claimed: bool,
    pub enforcement_command_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAnalyzerAlertRecord {
    pub alert_ref: String,
    pub ingestion_run_ref: String,
    pub fixture_ref: String,
    pub source: NetworkSignatureAlertSource,
    pub signature_id: String,
    pub signature_name: String,
    pub rule_source_ref: String,
    pub severity: NetworkSignatureAlertSeverity,
    pub observed_at_micros: u64,
    pub flow_ref: String,
    pub evidence_ref: String,
    pub custody_ref: String,
    pub alert_state: NetworkSignatureAlertState,
    pub analyzer_alert_event_published: bool,
    pub detection_candidate: bool,
    pub parent_review_candidate: bool,
    pub false_positive: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
    pub policy_authority: bool,
    pub adapter_authority: bool,
    pub enforcement_command_published: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkSignatureAlertIngestionProof {
    pub ingestion_run_ref: String,
    pub fixture_ref: String,
    pub records: Vec<NetworkAnalyzerAlertRecord>,
    pub suricata_record_count: usize,
    pub snort_compatible_record_count: usize,
    pub false_positive_record_count: usize,
    pub detection_candidate_count: usize,
    pub analyzer_alert_events_published: usize,
    pub adapter_calls_authorized: usize,
    pub enforcement_commands_published: usize,
    pub live_suricata_invoked: bool,
    pub live_snort_invoked: bool,
    pub ips_prevention_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkSignatureAlertIngestionError {
    EmptyIngestionRunRef,
    EmptyFixtureRef,
    EmptyAlertRows,
    EmptyAlertRef,
    DuplicateAlertRef,
    EmptySignatureId,
    EmptySignatureName,
    EmptyRuleSourceRef,
    EmptyFlowRef,
    EmptyEvidenceRef,
    EmptyCustodyRef,
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    LiveSuricataInvocationClaimRejected,
    LiveSnortInvocationClaimRejected,
    IpsPreventionClaimRejected,
    PolicyAuthorityClaimRejected,
    AdapterAuthorityClaimRejected,
    EnforcementCommandClaimRejected,
}

pub fn ingest_network_signature_alerts(
    input: &NetworkSignatureAlertIngestionInput,
) -> Result<NetworkSignatureAlertIngestionProof, NetworkSignatureAlertIngestionError> {
    reject_global_claims(input)?;
    if input.rows.is_empty() {
        return Err(NetworkSignatureAlertIngestionError::EmptyAlertRows);
    }

    let ingestion_run_ref = normalize_ref(&input.ingestion_run_ref)
        .ok_or(NetworkSignatureAlertIngestionError::EmptyIngestionRunRef)?;
    let fixture_ref = normalize_ref(&input.fixture_ref)
        .ok_or(NetworkSignatureAlertIngestionError::EmptyFixtureRef)?;
    let records = normalize_records(&ingestion_run_ref, &fixture_ref, &input.rows)?;

    Ok(NetworkSignatureAlertIngestionProof {
        ingestion_run_ref,
        fixture_ref,
        suricata_record_count: count_source(&records, NetworkSignatureAlertSource::Suricata),
        snort_compatible_record_count: count_source(
            &records,
            NetworkSignatureAlertSource::SnortCompatible,
        ),
        false_positive_record_count: records
            .iter()
            .filter(|record| record.false_positive)
            .count(),
        detection_candidate_count: records
            .iter()
            .filter(|record| record.detection_candidate)
            .count(),
        analyzer_alert_events_published: records
            .iter()
            .filter(|record| record.analyzer_alert_event_published)
            .count(),
        adapter_calls_authorized: records
            .iter()
            .filter(|record| record.adapter_authority)
            .count(),
        enforcement_commands_published: records
            .iter()
            .filter(|record| record.enforcement_command_published)
            .count(),
        records,
        live_suricata_invoked: false,
        live_snort_invoked: false,
        ips_prevention_claimed: false,
    })
}
