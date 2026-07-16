use super::{
    NetworkZeekAnalyzerComparisonArtifact, NetworkZeekAnalyzerError, NetworkZeekConnectionRow,
    NetworkZeekDnsRow, NetworkZeekHttpRow, NetworkZeekLogKind, NetworkZeekTlsRow,
};

use super::super::rows::normalize_ref;

pub(super) fn validate_comparison_artifacts(
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

pub(super) fn required_log_kinds(
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

pub(super) fn missing_comparison_log_kinds(
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

pub(super) fn validate_comparison_counts(
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

fn push_required(kinds: &mut Vec<NetworkZeekLogKind>, kind: NetworkZeekLogKind, required: bool) {
    if required {
        kinds.push(kind);
    }
}
