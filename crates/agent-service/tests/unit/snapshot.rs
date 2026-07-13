#[path = "../../src/snapshot.rs"]
mod snapshot;

use ocentra_parent_agent_protocol::constants;

#[test]
fn build_dev_log_snapshot_uses_protocol_owned_constants() {
    let snapshot = snapshot::build_dev_log_snapshot();

    assert_eq!(snapshot.agent.device_id, constants::peer::LOCAL_DEV_AGENT);
    assert!(snapshot.entries[0]
        .fields
        .get(constants::field::CAPTURE_ENABLED)
        .is_some());
}

#[test]
fn timestamp_helpers_keep_epoch_values_in_order() {
    let epoch = crate::time::timestamp_from_epoch_seconds(0);
    let later = crate::time::timestamp_after_epoch_seconds(0, 1);

    assert_ne!(epoch, later);
    assert!(epoch.starts_with("1970-01-01T"));
    assert!(later.starts_with("1970-01-01T"));
}
