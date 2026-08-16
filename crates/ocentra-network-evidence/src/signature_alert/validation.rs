use super::{
    NetworkSignatureAlertFixtureRow, NetworkSignatureAlertIngestionError,
    NetworkSignatureAlertIngestionInput,
};

pub(super) fn reject_global_claims(
    input: &NetworkSignatureAlertIngestionInput,
) -> Result<(), NetworkSignatureAlertIngestionError> {
    [
        (
            input.live_suricata_invocation_claimed,
            NetworkSignatureAlertIngestionError::LiveSuricataInvocationClaimRejected,
        ),
        (
            input.live_snort_invocation_claimed,
            NetworkSignatureAlertIngestionError::LiveSnortInvocationClaimRejected,
        ),
        (
            input.ips_prevention_claimed,
            NetworkSignatureAlertIngestionError::IpsPreventionClaimRejected,
        ),
        (
            input.policy_authority_claimed,
            NetworkSignatureAlertIngestionError::PolicyAuthorityClaimRejected,
        ),
        (
            input.adapter_authority_claimed,
            NetworkSignatureAlertIngestionError::AdapterAuthorityClaimRejected,
        ),
        (
            input.enforcement_command_claimed,
            NetworkSignatureAlertIngestionError::EnforcementCommandClaimRejected,
        ),
    ]
    .into_iter()
    .find_map(|(claimed, error)| claimed.then_some(error))
    .map_or(Ok(()), Err)
}

pub(super) fn reject_row_claims(
    row: &NetworkSignatureAlertFixtureRow,
) -> Result<(), NetworkSignatureAlertIngestionError> {
    [
        (
            row.exact_url_claimed,
            NetworkSignatureAlertIngestionError::ExactUrlClaimRejected,
        ),
        (
            row.decrypted_payload_claimed,
            NetworkSignatureAlertIngestionError::DecryptedPayloadClaimRejected,
        ),
        (
            row.page_content_claimed,
            NetworkSignatureAlertIngestionError::PageContentClaimRejected,
        ),
    ]
    .into_iter()
    .find_map(|(claimed, error)| claimed.then_some(error))
    .map_or(Ok(()), Err)
}
