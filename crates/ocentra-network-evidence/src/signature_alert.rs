use serde::{Deserialize, Serialize};

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

fn normalize_records(
    ingestion_run_ref: &str,
    fixture_ref: &str,
    rows: &[NetworkSignatureAlertFixtureRow],
) -> Result<Vec<NetworkAnalyzerAlertRecord>, NetworkSignatureAlertIngestionError> {
    let mut records = Vec::new();
    let mut alert_refs = Vec::new();
    for row in rows {
        reject_row_claims(row)?;
        let alert_ref = normalize_ref(&row.alert_ref)
            .ok_or(NetworkSignatureAlertIngestionError::EmptyAlertRef)?;
        if alert_refs.contains(&alert_ref) {
            return Err(NetworkSignatureAlertIngestionError::DuplicateAlertRef);
        }
        alert_refs.push(alert_ref.clone());

        let signature_id = normalize_ref(&row.signature_id)
            .ok_or(NetworkSignatureAlertIngestionError::EmptySignatureId)?;
        let signature_name = normalize_ref(&row.signature_name)
            .ok_or(NetworkSignatureAlertIngestionError::EmptySignatureName)?;
        let rule_source_ref = normalize_ref(&row.rule_source_ref)
            .ok_or(NetworkSignatureAlertIngestionError::EmptyRuleSourceRef)?;
        let flow_ref = normalize_ref(&row.flow_ref)
            .ok_or(NetworkSignatureAlertIngestionError::EmptyFlowRef)?;
        let evidence_ref = normalize_ref(&row.evidence_ref)
            .ok_or(NetworkSignatureAlertIngestionError::EmptyEvidenceRef)?;
        let custody_ref = normalize_ref(&row.custody_ref)
            .ok_or(NetworkSignatureAlertIngestionError::EmptyCustodyRef)?;
        let alert_state = alert_state(row.severity, row.known_false_positive);

        records.push(NetworkAnalyzerAlertRecord {
            alert_ref,
            ingestion_run_ref: ingestion_run_ref.to_owned(),
            fixture_ref: fixture_ref.to_owned(),
            source: row.source,
            signature_id,
            signature_name,
            rule_source_ref,
            severity: row.severity,
            observed_at_micros: row.observed_at_micros,
            flow_ref,
            evidence_ref,
            custody_ref,
            alert_state,
            analyzer_alert_event_published: true,
            detection_candidate: alert_state == NetworkSignatureAlertState::ReviewCandidate,
            parent_review_candidate: alert_state == NetworkSignatureAlertState::ReviewCandidate,
            false_positive: row.known_false_positive,
            exact_url_available: false,
            decrypted_payload_available: false,
            page_content_available: false,
            policy_authority: false,
            adapter_authority: false,
            enforcement_command_published: false,
        });
    }
    Ok(records)
}

fn alert_state(
    severity: NetworkSignatureAlertSeverity,
    known_false_positive: bool,
) -> NetworkSignatureAlertState {
    if known_false_positive {
        return NetworkSignatureAlertState::FalsePositiveNonEnforcing;
    }
    match severity {
        NetworkSignatureAlertSeverity::High | NetworkSignatureAlertSeverity::Critical => {
            NetworkSignatureAlertState::ReviewCandidate
        }
        NetworkSignatureAlertSeverity::Informational
        | NetworkSignatureAlertSeverity::Low
        | NetworkSignatureAlertSeverity::Medium => NetworkSignatureAlertState::AnalyzerEvidenceOnly,
    }
}

fn reject_global_claims(
    input: &NetworkSignatureAlertIngestionInput,
) -> Result<(), NetworkSignatureAlertIngestionError> {
    if input.live_suricata_invocation_claimed {
        return Err(NetworkSignatureAlertIngestionError::LiveSuricataInvocationClaimRejected);
    }
    if input.live_snort_invocation_claimed {
        return Err(NetworkSignatureAlertIngestionError::LiveSnortInvocationClaimRejected);
    }
    if input.ips_prevention_claimed {
        return Err(NetworkSignatureAlertIngestionError::IpsPreventionClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkSignatureAlertIngestionError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkSignatureAlertIngestionError::AdapterAuthorityClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkSignatureAlertIngestionError::EnforcementCommandClaimRejected);
    }
    Ok(())
}

fn reject_row_claims(
    row: &NetworkSignatureAlertFixtureRow,
) -> Result<(), NetworkSignatureAlertIngestionError> {
    if row.exact_url_claimed {
        return Err(NetworkSignatureAlertIngestionError::ExactUrlClaimRejected);
    }
    if row.decrypted_payload_claimed {
        return Err(NetworkSignatureAlertIngestionError::DecryptedPayloadClaimRejected);
    }
    if row.page_content_claimed {
        return Err(NetworkSignatureAlertIngestionError::PageContentClaimRejected);
    }
    Ok(())
}

fn count_source(
    records: &[NetworkAnalyzerAlertRecord],
    source: NetworkSignatureAlertSource,
) -> usize {
    records
        .iter()
        .filter(|record| record.source == source)
        .count()
}

fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
