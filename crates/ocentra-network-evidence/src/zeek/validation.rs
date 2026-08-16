mod claims;
mod comparison;

use super::{
    NetworkZeekAnalyzerComparisonArtifact, NetworkZeekAnalyzerError, NetworkZeekAnalyzerInput,
    NetworkZeekConnectionRow, NetworkZeekDnsRow, NetworkZeekHttpRow, NetworkZeekLogKind,
    NetworkZeekTlsRow,
};

pub(super) fn validate_comparison_artifacts(
    artifacts: &[NetworkZeekAnalyzerComparisonArtifact],
) -> Result<(), NetworkZeekAnalyzerError> {
    comparison::validate_comparison_artifacts(artifacts)
}

pub(super) fn required_log_kinds(
    connection_rows: &[NetworkZeekConnectionRow],
    dns_rows: &[NetworkZeekDnsRow],
    http_rows: &[NetworkZeekHttpRow],
    tls_rows: &[NetworkZeekTlsRow],
    ssl_rows: &[NetworkZeekTlsRow],
) -> Vec<NetworkZeekLogKind> {
    comparison::required_log_kinds(connection_rows, dns_rows, http_rows, tls_rows, ssl_rows)
}

pub(super) fn missing_comparison_log_kinds(
    required_log_kinds: &[NetworkZeekLogKind],
    artifacts: &[NetworkZeekAnalyzerComparisonArtifact],
) -> Vec<NetworkZeekLogKind> {
    comparison::missing_comparison_log_kinds(required_log_kinds, artifacts)
}

pub(super) fn validate_comparison_counts(
    artifacts: &[NetworkZeekAnalyzerComparisonArtifact],
    connection_rows: &[NetworkZeekConnectionRow],
    dns_rows: &[NetworkZeekDnsRow],
    http_rows: &[NetworkZeekHttpRow],
    tls_rows: &[NetworkZeekTlsRow],
    ssl_rows: &[NetworkZeekTlsRow],
) -> Result<(), NetworkZeekAnalyzerError> {
    comparison::validate_comparison_counts(
        artifacts,
        connection_rows,
        dns_rows,
        http_rows,
        tls_rows,
        ssl_rows,
    )
}

pub(super) fn reject_unsupported_claims(
    input: &NetworkZeekAnalyzerInput,
) -> Result<(), NetworkZeekAnalyzerError> {
    claims::reject_unsupported_claims(input)
}
