use ocentra_parent_agent_protocol::network_flow::{
    NetworkLiveCaptureStatus, NetworkLiveCaptureStatusRow,
};

pub(super) fn apply_boolean_counts(
    status: &mut NetworkLiveCaptureStatus,
    row: &NetworkLiveCaptureStatusRow,
) {
    status.metadata_snapshot_executed_count += count_bool(row.metadata_snapshot_executed);
    status.raw_artifact_created_count += count_bool(row.raw_artifact_created);
    status.capture_ready_count += count_bool(row.capture_ready);
    status.raw_artifact_storage_authorized_count += count_bool(row.raw_artifact_storage_authorized);
    status.driver_invoked_count += count_bool(row.driver_invoked);
    status.live_capture_executed_count += count_bool(row.live_capture_executed);
    status.remote_upload_enabled_count += count_bool(row.remote_upload_enabled);
    status.raw_pcap_without_custody_available_count +=
        count_bool(row.raw_pcap_without_custody_available);
    status.exact_url_available_count += count_bool(row.exact_url_available);
    status.decrypted_payload_available_count += count_bool(row.decrypted_payload_available);
    status.page_content_available_count += count_bool(row.page_content_available);
    status.private_message_available_count += count_bool(row.private_message_available);
    status.search_query_available_count += count_bool(row.search_query_available);
    status.policy_authority_count += count_bool(row.policy_authority);
    status.adapter_authority_count += count_bool(row.adapter_authority);
    status.netstat_metadata_substitution_count +=
        count_bool(row.netstat_metadata_substituted_for_live_capture);
    status.host_filtering_claim_count += count_bool(row.host_filtering_claimed);
}

fn count_bool(value: bool) -> u64 {
    if value {
        1
    } else {
        0
    }
}
