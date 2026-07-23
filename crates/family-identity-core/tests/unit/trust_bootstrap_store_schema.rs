use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use ocentra_family_identity_core::parent_presence::ParentPresenceStorageFailureReason;

use super::open_parent_presence_test_port;

pub(super) const VALID_CHALLENGE_STORE_SCHEMA: &str = r#"
CREATE TABLE parent_presence_challenges (
    challenge_ref TEXT PRIMARY KEY NOT NULL,
    challenge_json TEXT NOT NULL,
    privileged_action_json TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    nonce_ref TEXT NOT NULL UNIQUE,
    lifecycle_state TEXT NOT NULL CHECK (
        lifecycle_state IN ('issued', 'consumed')
    )
) STRICT;
CREATE UNIQUE INDEX parent_presence_nonce_identity
ON parent_presence_challenges(nonce_ref);
"#;

pub(super) const VALID_RECEIPT_STORE_SCHEMA: &str = r#"
CREATE TABLE parent_presence_receipts (
    receipt_sequence INTEGER PRIMARY KEY AUTOINCREMENT,
    challenge_ref TEXT NOT NULL UNIQUE,
    receipt_ref TEXT NOT NULL UNIQUE,
    FOREIGN KEY (challenge_ref)
        REFERENCES parent_presence_challenges(challenge_ref)
        ON DELETE RESTRICT
) STRICT;
"#;

pub(super) const VALID_DECISION_OUTBOX_SCHEMA: &str = r#"
CREATE TABLE parent_presence_decision_outbox (
    decision_id TEXT PRIMARY KEY NOT NULL,
    envelope_json TEXT NOT NULL,
    delivery_state TEXT NOT NULL CHECK (
        delivery_state IN ('pending', 'delivered')
    )
) STRICT;
"#;

static NEXT_CASE_ID: AtomicU64 = AtomicU64::new(1);

pub(super) type TestResult = Result<(), ParentPresenceStorageFailureReason>;

pub(super) struct TestStore {
    root: PathBuf,
    path: PathBuf,
}

impl TestStore {
    pub(super) fn new(prefix: &str) -> Self {
        let id = NEXT_CASE_ID.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "ocentra-parent-presence-schema-{prefix}-{}-{id}",
            std::process::id()
        ));
        assert!(matches!(fs::create_dir_all(&root), Ok(())));
        let path = root.join("parent-presence.sqlite");
        Self { root, path }
    }

    pub(super) fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestStore {
    fn drop(&mut self) {
        let _cleanup_result = fs::remove_dir_all(&self.root);
    }
}

pub(super) fn create_existing_store(
    store: &TestStore,
    challenge_schema: &str,
    receipt_schema: &str,
) -> TestResult {
    create_existing_store_with_outbox(
        store,
        challenge_schema,
        receipt_schema,
        VALID_DECISION_OUTBOX_SCHEMA,
    )
}

pub(super) fn create_existing_store_with_outbox(
    store: &TestStore,
    challenge_schema: &str,
    receipt_schema: &str,
    outbox_schema: &str,
) -> TestResult {
    let connection = rusqlite::Connection::open(store.path())
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    connection
        .execute_batch(&format!(
            "{challenge_schema}\n{receipt_schema}\n{outbox_schema}"
        ))
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    drop(connection);
    set_private_fixture_permissions(store.path())?;
    Ok(())
}

pub(super) fn execute_existing_store_sql(store: &TestStore, sql: &str) -> TestResult {
    let connection = rusqlite::Connection::open(store.path())
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    connection
        .execute_batch(sql)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    drop(connection);
    set_private_fixture_permissions(store.path())
}

#[cfg(unix)]
fn set_private_fixture_permissions(path: &Path) -> TestResult {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?
        .permissions();
    permissions.set_mode(0o600);
    fs::set_permissions(path, permissions)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)
}

#[cfg(windows)]
fn set_private_fixture_permissions(_path: &Path) -> TestResult {
    Ok(())
}

pub(super) fn assert_store_rejected_without_byte_changes(store: &TestStore) -> TestResult {
    let store_before = fs::read(store.path())
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    assert!(matches!(
        open_parent_presence_test_port(store.path()),
        Err(ParentPresenceStorageFailureReason::CustodyUnavailable)
    ));
    assert_eq!(
        fs::read(store.path())
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?,
        store_before
    );
    Ok(())
}

#[test]
fn parent_presence_store_accepts_canonical_tokens_across_comments_case_and_whitespace() -> TestResult
{
    let store = TestStore::new("equivalent-token-shape");
    let challenge_schema = r#"
        create table parent_presence_challenges (
            challenge_ref TEXT primary key not null,
            challenge_json TEXT not null,
            privileged_action_json TEXT not null,
            expires_at TEXT not null,
            nonce_ref TEXT not null unique,
            "lifecycle_state" TEXT not null check (
                "lifecycle_state" in ( 'issued' , 'consumed' )
            )
        ) strict;
        create unique index parent_presence_nonce_identity
        on parent_presence_challenges(nonce_ref);
    "#;
    let receipt_schema = r#"
        create table parent_presence_receipts (
            "receipt_sequence" INTEGER primary /* harmless comment */ key
                autoincrement,
            challenge_ref TEXT not null unique,
            receipt_ref TEXT not null unique,
            foreign key (challenge_ref)
                references parent_presence_challenges(challenge_ref)
                on delete restrict
        ) strict;
    "#;
    create_existing_store(&store, challenge_schema, receipt_schema)?;
    let port = open_parent_presence_test_port(store.path())?;
    drop(port);
    Ok(())
}

#[test]
fn parent_presence_store_rejects_corruption_without_recreation() {
    let store = TestStore::new("corrupt-store");
    let corrupt = b"not-a-sqlite-database";
    assert!(matches!(fs::write(store.path(), corrupt), Ok(())));
    assert!(matches!(
        open_parent_presence_test_port(store.path()),
        Err(ParentPresenceStorageFailureReason::CustodyUnavailable)
    ));
    assert!(matches!(
        fs::read(store.path()),
        Ok(content) if content == corrupt
    ));
}

#[test]
fn parent_presence_store_rejects_autoincrement_comment_decoy_without_writes() -> TestResult {
    let store = TestStore::new("autoincrement-comment-decoy");
    let receipt_schema = r#"
        CREATE TABLE parent_presence_receipts (
            receipt_sequence INTEGER PRIMARY KEY
                /* RECEIPT_SEQUENCE INTEGER PRIMARY KEY AUTOINCREMENT */,
            challenge_ref TEXT NOT NULL UNIQUE,
            receipt_ref TEXT NOT NULL UNIQUE,
            FOREIGN KEY (challenge_ref)
                REFERENCES parent_presence_challenges(challenge_ref)
                ON DELETE RESTRICT
        ) STRICT;
    "#;
    create_existing_store(&store, VALID_CHALLENGE_STORE_SCHEMA, receipt_schema)?;
    assert_store_rejected_without_byte_changes(&store)
}

#[test]
fn parent_presence_store_rejects_lifecycle_check_comment_and_literal_decoys_without_writes(
) -> TestResult {
    let store = TestStore::new("lifecycle-check-decoys");
    let challenge_schema = r#"
        CREATE TABLE parent_presence_challenges (
            challenge_ref TEXT PRIMARY KEY NOT NULL,
            challenge_json TEXT NOT NULL,
            privileged_action_json TEXT NOT NULL,
            expires_at TEXT NOT NULL,
            nonce_ref TEXT NOT NULL UNIQUE,
            lifecycle_state TEXT NOT NULL
                DEFAULT "CHECK(LIFECYCLE_STATE IN ('ISSUED','CONSUMED'))"
                /* CHECK(LIFECYCLE_STATE IN ('ISSUED','CONSUMED')) */
        ) STRICT;
        CREATE UNIQUE INDEX parent_presence_nonce_identity
        ON parent_presence_challenges(nonce_ref);
    "#;
    create_existing_store(&store, challenge_schema, VALID_RECEIPT_STORE_SCHEMA)?;
    assert_store_rejected_without_byte_changes(&store)
}

#[test]
fn parent_presence_store_rejects_missing_receipt_ref_integrity_index_without_writes() -> TestResult
{
    let store = TestStore::new("missing-receipt-ref-integrity-index");
    let receipt_schema = r#"
        CREATE TABLE parent_presence_receipts (
            receipt_sequence INTEGER PRIMARY KEY AUTOINCREMENT,
            challenge_ref TEXT NOT NULL UNIQUE,
            receipt_ref TEXT NOT NULL,
            FOREIGN KEY (challenge_ref)
                REFERENCES parent_presence_challenges(challenge_ref)
                ON DELETE RESTRICT
        ) STRICT;
    "#;
    create_existing_store(&store, VALID_CHALLENGE_STORE_SCHEMA, receipt_schema)?;
    assert_store_rejected_without_byte_changes(&store)
}

#[test]
fn parent_presence_store_rejects_missing_receipt_foreign_key_only_without_writes() -> TestResult {
    let store = TestStore::new("missing-receipt-foreign-key-only");
    let receipt_schema = r#"
        CREATE TABLE parent_presence_receipts (
            receipt_sequence INTEGER PRIMARY KEY AUTOINCREMENT,
            challenge_ref TEXT NOT NULL UNIQUE,
            receipt_ref TEXT NOT NULL UNIQUE
        ) STRICT;
    "#;
    create_existing_store(&store, VALID_CHALLENGE_STORE_SCHEMA, receipt_schema)?;
    assert_store_rejected_without_byte_changes(&store)
}

#[test]
fn parent_presence_store_rejects_wrong_receipt_foreign_key_target_only_without_writes() -> TestResult
{
    let store = TestStore::new("wrong-receipt-foreign-key-target-only");
    let receipt_schema = r#"
        CREATE TABLE parent_presence_receipts (
            receipt_sequence INTEGER PRIMARY KEY AUTOINCREMENT,
            challenge_ref TEXT NOT NULL UNIQUE,
            receipt_ref TEXT NOT NULL UNIQUE,
            FOREIGN KEY (challenge_ref)
                REFERENCES parent_presence_challenges(nonce_ref)
                ON DELETE RESTRICT
        ) STRICT;
    "#;
    create_existing_store(&store, VALID_CHALLENGE_STORE_SCHEMA, receipt_schema)?;
    assert_store_rejected_without_byte_changes(&store)
}

#[test]
fn parent_presence_store_rejects_receipt_sequence_wrong_nullability_only_without_writes(
) -> TestResult {
    let store = TestStore::new("receipt-sequence-wrong-nullability-only");
    let receipt_schema = r#"
        CREATE TABLE parent_presence_receipts (
            receipt_sequence INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            challenge_ref TEXT NOT NULL UNIQUE,
            receipt_ref TEXT NOT NULL UNIQUE,
            FOREIGN KEY (challenge_ref)
                REFERENCES parent_presence_challenges(challenge_ref)
                ON DELETE RESTRICT
        ) STRICT;
    "#;
    create_existing_store(&store, VALID_CHALLENGE_STORE_SCHEMA, receipt_schema)?;
    assert_store_rejected_without_byte_changes(&store)
}

#[test]
fn parent_presence_store_rejects_receipt_sequence_missing_primary_key_and_autoincrement_only_without_writes(
) -> TestResult {
    let store = TestStore::new("receipt-sequence-missing-pk-autoincrement-only");
    let receipt_schema = r#"
        CREATE TABLE parent_presence_receipts (
            receipt_sequence INTEGER,
            challenge_ref TEXT NOT NULL UNIQUE,
            receipt_ref TEXT NOT NULL UNIQUE,
            FOREIGN KEY (challenge_ref)
                REFERENCES parent_presence_challenges(challenge_ref)
                ON DELETE RESTRICT
        ) STRICT;
    "#;
    create_existing_store(&store, VALID_CHALLENGE_STORE_SCHEMA, receipt_schema)?;
    assert_store_rejected_without_byte_changes(&store)
}

#[test]
fn parent_presence_store_rejects_wrong_receipt_on_delete_only_without_writes() -> TestResult {
    let store = TestStore::new("wrong-receipt-on-delete-only");
    let receipt_schema = r#"
        CREATE TABLE parent_presence_receipts (
            receipt_sequence INTEGER PRIMARY KEY AUTOINCREMENT,
            challenge_ref TEXT NOT NULL UNIQUE,
            receipt_ref TEXT NOT NULL UNIQUE,
            FOREIGN KEY (challenge_ref)
                REFERENCES parent_presence_challenges(challenge_ref)
                ON DELETE CASCADE
        ) STRICT;
    "#;
    create_existing_store(&store, VALID_CHALLENGE_STORE_SCHEMA, receipt_schema)?;
    assert_store_rejected_without_byte_changes(&store)
}

#[test]
fn parent_presence_store_rejects_combined_malformed_receipt_shape_without_writes() -> TestResult {
    let store = TestStore::new("combined-malformed-receipt");
    let receipt_schema = r#"
        CREATE TABLE parent_presence_receipts (
            receipt_sequence TEXT NOT NULL,
            challenge_ref TEXT NOT NULL UNIQUE,
            receipt_ref TEXT NOT NULL
        ) STRICT;
    "#;
    create_existing_store(&store, VALID_CHALLENGE_STORE_SCHEMA, receipt_schema)?;
    assert_store_rejected_without_byte_changes(&store)
}
