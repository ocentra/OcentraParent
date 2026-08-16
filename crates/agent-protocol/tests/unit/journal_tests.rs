use super::{
    journal::{
        ActivityJournalCipher, ActivityJournalLine, ActivityJournalRotationPolicy,
        ActivityJournalStatus,
    },
    ACTIVITY_JOURNAL_SCHEMA_VERSION,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn activity_journal_line_serializes_to_typescript_contract_shape() {
    let line = ActivityJournalLine {
        schema_version: ACTIVITY_JOURNAL_SCHEMA_VERSION,
        entry_id: "journal-entry-1".to_string(),
        segment_id: "journal-segment-1".to_string(),
        written_at: "2026-05-20T00:00:00Z".to_string(),
        event_id: "activity-event-1".to_string(),
        cipher: ActivityJournalCipher::XChaCha20Poly1305,
        nonce: "nonce-bytes-base64".to_string(),
        ciphertext: "ciphertext-bytes-base64".to_string(),
        activity_digest: "sha256-activity-digest-base64".to_string(),
    };

    let serialized = serde_json::to_value(line).expect_value("journal line serializes");

    assert_eq!(serialized["schemaVersion"], 1);
    assert_eq!(serialized["entryId"], "journal-entry-1");
    assert_eq!(serialized["segmentId"], "journal-segment-1");
    assert_eq!(serialized["cipher"], "xchacha20poly1305");
    assert_eq!(serialized["nonce"], "nonce-bytes-base64");
    assert_eq!(serialized["ciphertext"], "ciphertext-bytes-base64");
    assert!(serialized.get("subject").is_none());
    assert!(serialized.get("fields").is_none());
}

#[test]
fn activity_journal_status_serializes_to_typescript_contract_shape() {
    let status = ActivityJournalStatus {
        schema_version: ACTIVITY_JOURNAL_SCHEMA_VERSION,
        encrypted: true,
        entries_written: 2,
        bytes_written: 512,
        active_segment_id: "journal-segment-1".to_string(),
        segment_count: 1,
        rotation_max_bytes: 1048576,
        last_entry_id: Some("journal-entry-2".to_string()),
    };

    let serialized = serde_json::to_value(status).expect_value("journal status serializes");

    assert_eq!(serialized["schemaVersion"], 1);
    assert_eq!(serialized["encrypted"], true);
    assert_eq!(serialized["entriesWritten"], 2);
    assert_eq!(serialized["bytesWritten"], 512);
    assert_eq!(serialized["activeSegmentId"], "journal-segment-1");
    assert_eq!(serialized["segmentCount"], 1);
    assert_eq!(serialized["rotationMaxBytes"], 1048576);
    assert_eq!(serialized["lastEntryId"], "journal-entry-2");
}

#[test]
fn activity_journal_rotation_policy_serializes_to_typescript_contract_shape() {
    let policy = ActivityJournalRotationPolicy {
        max_segment_bytes: 4096,
    };

    let serialized =
        serde_json::to_value(policy).expect_value("journal rotation policy serializes");

    assert_eq!(serialized["maxSegmentBytes"], 4096);
}
