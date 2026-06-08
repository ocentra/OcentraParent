use std::fs::{read, remove_dir_all};

use ocentra_parent_agent_protocol::constants;

use super::{
    NetworkRawArtifactStore, NetworkRawArtifactStoreConfig, NetworkRawArtifactStoreError,
    NetworkRawCaptureArtifactInput,
};

#[test]
fn raw_artifact_store_writes_reads_exports_and_deletes_real_files() {
    let root = temp_root(constants::network_raw_artifact::TEST_WRITE_READ_SUFFIX);
    cleanup_root(&root);
    let store = NetworkRawArtifactStore::open(store_config(root.clone(), usize::MAX))
        .expect(constants::error::NETWORK_RAW_ARTIFACT_OPENS);
    let input = test_input(test_payload());

    let record = store
        .write_artifact(input.clone())
        .expect(constants::error::NETWORK_RAW_ARTIFACT_WRITES);
    let exists_after_write = record.artifact_path.exists();
    let read_back = store
        .read_artifact(&record)
        .expect(constants::error::NETWORK_RAW_ARTIFACT_READS);
    let export_root = export_root(&root);
    let exported = store
        .export_artifact(&record, &export_root)
        .expect(constants::error::NETWORK_RAW_ARTIFACT_EXPORTS);
    let exported_bytes = read(&exported.artifact_path).unwrap_or_default();
    let deleted = store
        .delete_artifact(&record)
        .expect(constants::error::NETWORK_RAW_ARTIFACT_DELETES);
    let exists_after_delete = record.artifact_path.exists();
    let read_deleted = store.read_artifact(&deleted);
    cleanup_root(&root);

    assert!(exists_after_write);
    assert_eq!(record.artifact_id, input.artifact_id);
    assert_eq!(record.byte_len, input.bytes.len());
    assert_eq!(
        record.hash_algorithm,
        constants::network_raw_artifact::HASH_ALGORITHM_SHA256
    );
    assert_eq!(record.state, constants::network_raw_artifact::STATE_ACTIVE);
    assert_eq!(read_back.bytes, input.bytes);
    assert_eq!(exported_bytes, input.bytes);
    assert_eq!(
        deleted.state,
        constants::network_raw_artifact::STATE_DELETED
    );
    assert!(!exists_after_delete);
    assert!(matches!(
        read_deleted,
        Err(NetworkRawArtifactStoreError::ArtifactDeleted)
    ));
}

#[test]
fn raw_artifact_store_rejects_unsafe_artifact_ids() {
    let root = temp_root(constants::network_raw_artifact::TEST_REJECT_IDS_SUFFIX);
    cleanup_root(&root);
    let store = NetworkRawArtifactStore::open(store_config(root.clone(), usize::MAX))
        .expect(constants::error::NETWORK_RAW_ARTIFACT_OPENS);

    let slash = invalid_id(constants::delimiter::SLASH);
    let backslash = invalid_id(constants::delimiter::BACKSLASH);
    let dot = invalid_id(constants::delimiter::DOT);
    let empty_result = store.write_artifact(test_input_with_id(String::new()));
    let slash_result = store.write_artifact(test_input_with_id(slash));
    let backslash_result = store.write_artifact(test_input_with_id(backslash));
    let dot_result = store.write_artifact(test_input_with_id(dot));
    cleanup_root(&root);

    assert!(matches!(
        empty_result,
        Err(NetworkRawArtifactStoreError::EmptyArtifactId)
    ));
    assert!(matches!(
        slash_result,
        Err(NetworkRawArtifactStoreError::UnsafeArtifactId)
    ));
    assert!(matches!(
        backslash_result,
        Err(NetworkRawArtifactStoreError::UnsafeArtifactId)
    ));
    assert!(matches!(
        dot_result,
        Err(NetworkRawArtifactStoreError::UnsafeArtifactId)
    ));
}

#[test]
fn raw_artifact_store_rejects_empty_payload() {
    let root = temp_root(constants::network_raw_artifact::TEST_EMPTY_PAYLOAD_SUFFIX);
    cleanup_root(&root);
    let store = NetworkRawArtifactStore::open(store_config(root.clone(), usize::MAX))
        .expect(constants::error::NETWORK_RAW_ARTIFACT_OPENS);

    let result = store.write_artifact(test_input(Vec::new()));
    cleanup_root(&root);

    assert!(matches!(
        result,
        Err(NetworkRawArtifactStoreError::EmptyBytes)
    ));
}

#[test]
fn raw_artifact_store_enforces_max_artifact_size() {
    let root = temp_root(constants::network_raw_artifact::TEST_QUOTA_SUFFIX);
    cleanup_root(&root);
    let store = NetworkRawArtifactStore::open(store_config(root.clone(), 1))
        .expect(constants::error::NETWORK_RAW_ARTIFACT_OPENS);

    let result = store.write_artifact(NetworkRawCaptureArtifactInput {
        artifact_id: constants::network_raw_artifact::TEST_OVERSIZED_ARTIFACT_ID.to_string(),
        ..test_input(test_payload())
    });
    cleanup_root(&root);

    assert!(matches!(
        result,
        Err(NetworkRawArtifactStoreError::ArtifactTooLarge)
    ));
}

fn test_input(bytes: Vec<u8>) -> NetworkRawCaptureArtifactInput {
    NetworkRawCaptureArtifactInput {
        artifact_id: constants::network_raw_artifact::TEST_ARTIFACT_ID.to_string(),
        captured_at: constants::network_raw_artifact::TEST_CAPTURED_AT.to_string(),
        source_event_id: constants::network_raw_artifact::TEST_SOURCE_EVENT_ID.to_string(),
        custody_label: constants::network_raw_artifact::TEST_CUSTODY_LABEL.to_string(),
        bytes,
    }
}

fn test_input_with_id(artifact_id: String) -> NetworkRawCaptureArtifactInput {
    NetworkRawCaptureArtifactInput {
        artifact_id,
        ..test_input(test_payload())
    }
}

fn test_payload() -> Vec<u8> {
    Vec::from([0xd4, 0xc3, 0xb2, 0xa1, 0x02, 0x00, 0x04, 0x00])
}

fn store_config(
    root: std::path::PathBuf,
    max_artifact_bytes: usize,
) -> NetworkRawArtifactStoreConfig {
    NetworkRawArtifactStoreConfig {
        root,
        max_artifact_bytes,
    }
}

fn temp_root(suffix: &str) -> std::path::PathBuf {
    let mut name = String::from(constants::network_raw_artifact::TEST_DIR_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);

    let mut path = std::env::temp_dir();
    path.push(name);
    path
}

fn export_root(root: &std::path::Path) -> std::path::PathBuf {
    let mut path = root.to_path_buf();
    path.push(constants::network_raw_artifact::TEST_EXPORT_DIR_SUFFIX);
    path
}

fn invalid_id(character: char) -> String {
    let mut value = String::from(constants::network_raw_artifact::TEST_ARTIFACT_ID);
    value.push(character);
    value
}

fn cleanup_root(root: &std::path::Path) {
    let _ = remove_dir_all(root);
}
