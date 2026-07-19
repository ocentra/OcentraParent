use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use ocentra_family_identity_core::parent_presence::{
    ParentPresenceStorageFailureReason, ParentPresenceVerificationPort,
};

const VALID_CHALLENGE_STORE_SCHEMA: &str = r#"
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

const VALID_RECEIPT_STORE_SCHEMA: &str = r#"
CREATE TABLE parent_presence_receipts (
    receipt_sequence INTEGER PRIMARY KEY AUTOINCREMENT,
    challenge_ref TEXT NOT NULL UNIQUE,
    receipt_ref TEXT NOT NULL UNIQUE,
    FOREIGN KEY (challenge_ref)
        REFERENCES parent_presence_challenges(challenge_ref)
        ON DELETE RESTRICT
) STRICT;
"#;

const SENTINEL_CHALLENGE: &str = r#"
INSERT INTO parent_presence_challenges VALUES (
    'sentinel-challenge', '{}', '{}', '2099-01-01T00:00:00.000Z',
    'sentinel-nonce', 'issued'
);
"#;

static NEXT_CASE_ID: AtomicU64 = AtomicU64::new(1);

type TestResult = Result<(), ParentPresenceStorageFailureReason>;

struct TestStore {
    root: PathBuf,
    path: PathBuf,
}

impl TestStore {
    fn new(prefix: &str) -> Self {
        let id = NEXT_CASE_ID.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "ocentra-parent-presence-schema-{prefix}-{}-{id}",
            std::process::id()
        ));
        assert!(matches!(fs::create_dir_all(&root), Ok(())));
        let path = root.join("parent-presence.sqlite");
        Self { root, path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestStore {
    fn drop(&mut self) {
        let _cleanup_result = fs::remove_dir_all(&self.root);
    }
}

fn create_existing_store(
    store: &TestStore,
    challenge_schema: &str,
    receipt_schema: &str,
) -> TestResult {
    let connection = rusqlite::Connection::open(store.path())
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    connection
        .execute_batch(&format!(
            "{challenge_schema}\n{receipt_schema}\n{SENTINEL_CHALLENGE}"
        ))
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    drop(connection);
    Ok(())
}

fn assert_store_rejected_without_byte_changes(store: &TestStore) -> TestResult {
    let store_before = fs::read(store.path())
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    assert!(matches!(
        ParentPresenceVerificationPort::open(store.path()),
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
    let port = ParentPresenceVerificationPort::open(store.path())?;
    drop(port);
    Ok(())
}

#[test]
fn parent_presence_store_rejects_corruption_without_recreation() {
    let store = TestStore::new("corrupt-store");
    let corrupt = b"not-a-sqlite-database";
    assert!(matches!(fs::write(store.path(), corrupt), Ok(())));
    assert!(matches!(
        ParentPresenceVerificationPort::open(store.path()),
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
fn parent_presence_store_rejects_nonunique_receipt_ref_only_without_writes() -> TestResult {
    let store = TestStore::new("nonunique-receipt-only");
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
fn parent_presence_store_rejects_wrong_receipt_sequence_shape_only_without_writes() -> TestResult {
    let store = TestStore::new("wrong-receipt-sequence-only");
    let receipt_schema = r#"
        CREATE TABLE parent_presence_receipts (
            receipt_sequence INTEGER NOT NULL,
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
