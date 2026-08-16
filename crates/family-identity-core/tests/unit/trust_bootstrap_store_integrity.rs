use ocentra_family_identity_core::parent_presence::ParentPresenceStorageFailureReason;

use super::trust_bootstrap_store_schema::{
    assert_store_rejected_without_byte_changes, create_existing_store, execute_existing_store_sql,
    TestResult, TestStore, VALID_CHALLENGE_STORE_SCHEMA, VALID_RECEIPT_STORE_SCHEMA,
};

fn inject_schema_object(
    store: &TestStore,
    object_type: &str,
    name: &str,
    table: &str,
    root_page_sql: &str,
    create_sql: &str,
) -> TestResult {
    let escaped_create_sql = create_sql.replace('\'', "''");
    execute_existing_store_sql(
        store,
        &format!(
            "PRAGMA writable_schema = ON;\n\
             INSERT INTO sqlite_schema(type, name, tbl_name, rootpage, sql)\n\
             VALUES ('{object_type}', '{name}', '{table}', {root_page_sql},\n\
                     '{escaped_create_sql}');\n\
             PRAGMA writable_schema = OFF;"
        ),
    )
}

fn assert_challenge_unconsumed_without_receipt(store: &TestStore) -> TestResult {
    let connection = rusqlite::Connection::open(store.path())
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    connection
        .execute_batch("PRAGMA writable_schema = ON;")
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let lifecycle_state = connection
        .query_row(
            "SELECT lifecycle_state FROM parent_presence_challenges WHERE challenge_ref = 'challenge-injection'",
            [],
            |row| row.get::<_, String>(0),
        )
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let receipt_count = connection
        .query_row("SELECT COUNT(*) FROM parent_presence_receipts", [], |row| {
            row.get::<_, i64>(0)
        })
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    assert_eq!(lifecycle_state, "issued");
    assert_eq!(receipt_count, 0);
    Ok(())
}

#[test]
fn parent_presence_store_rejects_writable_schema_sqlite_trigger_before_receipt_execution(
) -> TestResult {
    let store = TestStore::new("writable-schema-sqlite-trigger");
    create_existing_store(
        &store,
        VALID_CHALLENGE_STORE_SCHEMA,
        VALID_RECEIPT_STORE_SCHEMA,
    )?;
    execute_existing_store_sql(
        &store,
        r#"
        INSERT INTO parent_presence_challenges (
            challenge_ref,
            challenge_json,
            privileged_action_json,
            expires_at,
            nonce_ref,
            lifecycle_state
        ) VALUES (
            'challenge-injection',
            '{}',
            '{}',
            '2999-01-01T00:00:00.000Z',
            'nonce-injection',
            'issued'
        );
        "#,
    )?;
    inject_schema_object(
        &store,
        "trigger",
        "sqlite_autoindex_parent_presence_receipts_99",
        "parent_presence_receipts",
        "0",
        "CREATE TRIGGER sqlite_autoindex_parent_presence_receipts_99 AFTER INSERT ON parent_presence_receipts BEGIN UPDATE parent_presence_challenges SET lifecycle_state = 'consumed' WHERE challenge_ref = NEW.challenge_ref; END",
    )?;

    assert_store_rejected_without_byte_changes(&store)?;
    assert_challenge_unconsumed_without_receipt(&store)
}

#[test]
fn parent_presence_store_rejects_writable_schema_sqlite_view_on_reopen() -> TestResult {
    let store = TestStore::new("writable-schema-sqlite-view");
    create_existing_store(
        &store,
        VALID_CHALLENGE_STORE_SCHEMA,
        VALID_RECEIPT_STORE_SCHEMA,
    )?;
    inject_schema_object(
        &store,
        "view",
        "sqlite_parent_presence_receipt_projection",
        "sqlite_parent_presence_receipt_projection",
        "0",
        "CREATE VIEW sqlite_parent_presence_receipt_projection AS SELECT receipt_ref FROM parent_presence_receipts",
    )?;

    assert_store_rejected_without_byte_changes(&store)
}

#[test]
fn parent_presence_store_rejects_sqlite_like_wildcard_name_on_reopen() -> TestResult {
    let store = TestStore::new("writable-schema-sqlite-like-wildcard");
    create_existing_store(
        &store,
        VALID_CHALLENGE_STORE_SCHEMA,
        VALID_RECEIPT_STORE_SCHEMA,
    )?;
    inject_schema_object(
        &store,
        "view",
        "sqliteXparent_presence_receipt_projection",
        "sqliteXparent_presence_receipt_projection",
        "0",
        "CREATE VIEW sqliteXparent_presence_receipt_projection AS SELECT receipt_ref FROM parent_presence_receipts",
    )?;

    assert_store_rejected_without_byte_changes(&store)
}

#[test]
fn parent_presence_store_rejects_writable_schema_sqlite_index_on_reopen() -> TestResult {
    let store = TestStore::new("writable-schema-sqlite-index");
    create_existing_store(
        &store,
        VALID_CHALLENGE_STORE_SCHEMA,
        VALID_RECEIPT_STORE_SCHEMA,
    )?;
    inject_schema_object(
        &store,
        "index",
        "sqlite_parent_presence_sequence_lookup",
        "sqlite_sequence",
        "(SELECT rootpage FROM sqlite_schema WHERE name = 'parent_presence_nonce_identity')",
        "CREATE INDEX sqlite_parent_presence_sequence_lookup ON sqlite_sequence(name)",
    )?;

    assert_store_rejected_without_byte_changes(&store)
}

#[test]
fn parent_presence_store_rejects_orphan_receipt_on_reopen_without_byte_changes() -> TestResult {
    let store = TestStore::new("orphan-receipt");
    create_existing_store(
        &store,
        VALID_CHALLENGE_STORE_SCHEMA,
        VALID_RECEIPT_STORE_SCHEMA,
    )?;
    execute_existing_store_sql(
        &store,
        "PRAGMA foreign_keys = OFF;\n\
         INSERT INTO parent_presence_receipts(challenge_ref, receipt_ref)\n\
         VALUES ('missing-challenge', 'orphan-receipt');",
    )?;

    assert_store_rejected_without_byte_changes(&store)
}
