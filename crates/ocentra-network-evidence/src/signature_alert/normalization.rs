use super::{
    NetworkAnalyzerAlertRecord, NetworkSignatureAlertFixtureRow,
    NetworkSignatureAlertIngestionError, NetworkSignatureAlertSource, NetworkSignatureAlertState,
};

pub(super) fn normalize_records(
    ingestion_run_ref: &str,
    fixture_ref: &str,
    rows: &[NetworkSignatureAlertFixtureRow],
) -> Result<Vec<NetworkAnalyzerAlertRecord>, NetworkSignatureAlertIngestionError> {
    let mut records = Vec::new();
    let mut alert_refs = Vec::new();
    for row in rows {
        super::validation::reject_row_claims(row)?;
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

pub(super) fn count_source(
    records: &[NetworkAnalyzerAlertRecord],
    source: NetworkSignatureAlertSource,
) -> usize {
    records
        .iter()
        .filter(|record| record.source == source)
        .count()
}

pub(super) fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}

fn alert_state(
    severity: super::NetworkSignatureAlertSeverity,
    known_false_positive: bool,
) -> NetworkSignatureAlertState {
    if known_false_positive {
        return NetworkSignatureAlertState::FalsePositiveNonEnforcing;
    }
    match severity {
        super::NetworkSignatureAlertSeverity::High
        | super::NetworkSignatureAlertSeverity::Critical => {
            NetworkSignatureAlertState::ReviewCandidate
        }
        super::NetworkSignatureAlertSeverity::Informational
        | super::NetworkSignatureAlertSeverity::Low
        | super::NetworkSignatureAlertSeverity::Medium => {
            NetworkSignatureAlertState::AnalyzerEvidenceOnly
        }
    }
}
