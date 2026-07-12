use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::{
    NetworkLiveCaptureStatus, NetworkLiveCaptureStatusRow,
};

use super::boolean_counts::apply_boolean_counts;
use super::mapping::count;
use super::state_counts::apply_state_counts;

const LIVE_CAPTURE_REQUIRED_ARTIFACTS_PER_ROW: u64 = 9;

pub(super) fn status_from_rows(rows: Vec<NetworkLiveCaptureStatusRow>) -> NetworkLiveCaptureStatus {
    let mut status = NetworkLiveCaptureStatus {
        status_ref: constants::network_flow::TEST_LIVE_CAPTURE_STATUS_REF.to_string(),
        row13_status_ref: constants::network_flow::TEST_LIVE_CAPTURE_ROW13_STATUS_REF.to_string(),
        execution_status_ref: constants::network_flow::TEST_LIVE_CAPTURE_EXECUTION_STATUS_REF
            .to_string(),
        raw_storage_status_ref: constants::network_flow::TEST_LIVE_CAPTURE_STORAGE_STATUS_REF
            .to_string(),
        platform_row_count: count(rows.len()),
        required_artifact_count: count(rows.len()) * LIVE_CAPTURE_REQUIRED_ARTIFACTS_PER_ROW,
        ..NetworkLiveCaptureStatus::default()
    };

    for row in &rows {
        apply_state_counts(&mut status, row);
        apply_boolean_counts(&mut status, row);
    }
    status.rows = rows;

    status
}
