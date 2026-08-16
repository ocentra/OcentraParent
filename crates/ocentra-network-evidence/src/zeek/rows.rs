use crate::dns::types::DnsObservation;
use crate::flow::NetworkFlowSession;

use super::{
    NetworkZeekAnalyzerError, NetworkZeekConnectionRow, NetworkZeekDnsRow, NetworkZeekHttpEvidence,
    NetworkZeekHttpRow, NetworkZeekLogKind, NetworkZeekTlsEvidence, NetworkZeekTlsRow,
    NetworkZeekVisibilityState,
};

pub(super) fn connection_rows(
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

pub(super) fn dns_rows(
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

pub(super) fn http_rows(
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

pub(super) fn build_tls_rows(
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

pub(super) fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

fn row_ref(analyzer_run_ref: &str, log_kind: NetworkZeekLogKind, index: usize) -> String {
    format!("{analyzer_run_ref}::{:?}::{index}", log_kind).to_ascii_lowercase()
}

fn normalized_optional_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|trimmed| !trimmed.is_empty())
        .map(|trimmed| trimmed.to_ascii_lowercase())
}
