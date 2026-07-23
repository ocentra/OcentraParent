use super::trust_bootstrap_store_schema::{
    assert_store_rejected_without_byte_changes, create_existing_store,
    create_existing_store_with_outbox, execute_existing_store_sql, TestResult, TestStore,
    VALID_CHALLENGE_STORE_SCHEMA, VALID_DECISION_OUTBOX_SCHEMA, VALID_RECEIPT_STORE_SCHEMA,
};

fn mutate_once(schema: &str, original: &str, replacement: &str) -> String {
    assert_eq!(
        schema.matches(original).count(),
        1,
        "fixture mutation target must occur exactly once"
    );
    schema.replacen(original, replacement, 1)
}

fn assert_challenge_schema_mutation_rejected(
    prefix: &str,
    original: &str,
    replacement: &str,
) -> TestResult {
    let store = TestStore::new(prefix);
    let challenge_schema = mutate_once(VALID_CHALLENGE_STORE_SCHEMA, original, replacement);
    create_existing_store(&store, &challenge_schema, VALID_RECEIPT_STORE_SCHEMA)?;
    assert_store_rejected_without_byte_changes(&store)
}

fn assert_receipt_schema_mutation_rejected(
    prefix: &str,
    original: &str,
    replacement: &str,
) -> TestResult {
    let store = TestStore::new(prefix);
    let receipt_schema = mutate_once(VALID_RECEIPT_STORE_SCHEMA, original, replacement);
    create_existing_store(&store, VALID_CHALLENGE_STORE_SCHEMA, &receipt_schema)?;
    assert_store_rejected_without_byte_changes(&store)
}

#[test]
fn parent_presence_store_rejects_missing_required_column_without_writes() -> TestResult {
    assert_challenge_schema_mutation_rejected(
        "missing-required-column",
        "    privileged_action_json TEXT NOT NULL,\n",
        "",
    )
}

#[test]
fn parent_presence_store_rejects_extra_unexpected_column_without_writes() -> TestResult {
    assert_challenge_schema_mutation_rejected(
        "extra-unexpected-column",
        "    lifecycle_state TEXT NOT NULL CHECK (",
        "    unexpected_metadata TEXT NOT NULL,\n    lifecycle_state TEXT NOT NULL CHECK (",
    )
}

#[test]
fn parent_presence_store_rejects_wrong_column_nullability_only_without_writes() -> TestResult {
    assert_challenge_schema_mutation_rejected(
        "wrong-column-nullability-only",
        "    expires_at TEXT NOT NULL,\n",
        "    expires_at TEXT,\n",
    )
}

#[test]
fn parent_presence_store_rejects_missing_challenge_primary_key_position_without_writes(
) -> TestResult {
    assert_challenge_schema_mutation_rejected(
        "missing-challenge-primary-key-position",
        "    challenge_ref TEXT PRIMARY KEY NOT NULL,\n",
        "    challenge_ref TEXT NOT NULL UNIQUE,\n",
    )
}

#[test]
fn parent_presence_store_rejects_missing_named_nonce_index_without_writes() -> TestResult {
    assert_challenge_schema_mutation_rejected(
        "missing-named-nonce-index",
        "CREATE UNIQUE INDEX parent_presence_nonce_identity\n",
        "CREATE UNIQUE INDEX parent_presence_nonce_identity_renamed\n",
    )
}

#[test]
fn parent_presence_store_rejects_nonunique_named_nonce_index_without_writes() -> TestResult {
    assert_challenge_schema_mutation_rejected(
        "nonunique-named-nonce-index",
        "CREATE UNIQUE INDEX parent_presence_nonce_identity\n",
        "CREATE INDEX parent_presence_nonce_identity\n",
    )
}

#[test]
fn parent_presence_store_rejects_partial_nonce_index_signature_without_writes() -> TestResult {
    assert_challenge_schema_mutation_rejected(
        "partial-nonce-index-signature",
        "ON parent_presence_challenges(nonce_ref);\n",
        "ON parent_presence_challenges(nonce_ref)\nWHERE nonce_ref <> '';\n",
    )
}

#[test]
fn parent_presence_store_rejects_wrong_nonce_index_column_order_without_writes() -> TestResult {
    assert_challenge_schema_mutation_rejected(
        "wrong-nonce-index-column-order",
        "ON parent_presence_challenges(nonce_ref);\n",
        "ON parent_presence_challenges(challenge_ref, nonce_ref);\n",
    )
}

#[test]
fn parent_presence_store_rejects_wrong_receipt_integrity_index_without_writes() -> TestResult {
    assert_receipt_schema_mutation_rejected(
        "wrong-receipt-integrity-index",
        "    receipt_ref TEXT NOT NULL UNIQUE,\n",
        "    receipt_ref TEXT NOT NULL,\n    UNIQUE (receipt_ref, challenge_ref),\n",
    )
}

#[test]
fn parent_presence_store_rejects_extra_receipt_integrity_index_without_writes() -> TestResult {
    assert_receipt_schema_mutation_rejected(
        "extra-receipt-integrity-index",
        ") STRICT;\n",
        ") STRICT;\nCREATE UNIQUE INDEX parent_presence_receipt_sequence_lookup\nON parent_presence_receipts(receipt_sequence);\n",
    )
}

#[test]
fn parent_presence_store_rejects_extra_table_check_constraint_without_writes() -> TestResult {
    assert_challenge_schema_mutation_rejected(
        "extra-table-check-constraint",
        ") STRICT;\nCREATE UNIQUE INDEX",
        ", CHECK (lifecycle_state = 'issued')) STRICT;\nCREATE UNIQUE INDEX",
    )
}

#[test]
fn parent_presence_store_rejects_extra_foreign_key_constraint_without_writes() -> TestResult {
    assert_challenge_schema_mutation_rejected(
        "extra-foreign-key-constraint",
        ") STRICT;\nCREATE UNIQUE INDEX",
        ", FOREIGN KEY (expires_at) REFERENCES parent_presence_challenges(expires_at)) STRICT;\nCREATE UNIQUE INDEX",
    )
}

#[test]
fn parent_presence_store_rejects_column_collation_without_writes() -> TestResult {
    assert_challenge_schema_mutation_rejected(
        "column-collation",
        "    expires_at TEXT NOT NULL,\n",
        "    expires_at TEXT COLLATE NOCASE NOT NULL,\n",
    )
}

#[test]
fn parent_presence_store_rejects_unique_conflict_clause_without_writes() -> TestResult {
    assert_challenge_schema_mutation_rejected(
        "unique-conflict-clause",
        "    nonce_ref TEXT NOT NULL UNIQUE,\n",
        "    nonce_ref TEXT NOT NULL UNIQUE ON CONFLICT IGNORE,\n",
    )
}

#[test]
fn parent_presence_store_rejects_extra_outbox_constraint_without_writes() -> TestResult {
    let store = TestStore::new("extra-outbox-constraint");
    let outbox_schema = mutate_once(
        VALID_DECISION_OUTBOX_SCHEMA,
        ") STRICT;\n",
        ", CHECK (length(envelope_json) > 0)) STRICT;\n",
    );
    create_existing_store_with_outbox(
        &store,
        VALID_CHALLENGE_STORE_SCHEMA,
        VALID_RECEIPT_STORE_SCHEMA,
        &outbox_schema,
    )?;
    assert_store_rejected_without_byte_changes(&store)
}

#[test]
fn parent_presence_store_rejects_executable_trigger_without_byte_changes() -> TestResult {
    let store = TestStore::new("executable-trigger");
    create_existing_store(
        &store,
        VALID_CHALLENGE_STORE_SCHEMA,
        VALID_RECEIPT_STORE_SCHEMA,
    )?;
    execute_existing_store_sql(
        &store,
        r#"
        CREATE TRIGGER reset_consumed_challenge_after_receipt
        AFTER INSERT ON parent_presence_receipts
        BEGIN
            DELETE FROM parent_presence_receipts
            WHERE challenge_ref = NEW.challenge_ref;
            UPDATE parent_presence_challenges
            SET lifecycle_state = 'issued'
            WHERE challenge_ref = NEW.challenge_ref;
        END;
        "#,
    )?;
    assert_store_rejected_without_byte_changes(&store)
}

#[test]
fn parent_presence_store_rejects_extra_view_without_byte_changes() -> TestResult {
    let store = TestStore::new("extra-view");
    create_existing_store(
        &store,
        VALID_CHALLENGE_STORE_SCHEMA,
        VALID_RECEIPT_STORE_SCHEMA,
    )?;
    execute_existing_store_sql(
        &store,
        "CREATE VIEW parent_presence_receipt_projection AS SELECT receipt_ref FROM parent_presence_receipts;",
    )?;
    assert_store_rejected_without_byte_changes(&store)
}

#[test]
fn parent_presence_store_rejects_virtual_table_structures_without_byte_changes() -> TestResult {
    let store = TestStore::new("virtual-table");
    create_existing_store(
        &store,
        VALID_CHALLENGE_STORE_SCHEMA,
        VALID_RECEIPT_STORE_SCHEMA,
    )?;
    execute_existing_store_sql(
        &store,
        "CREATE VIRTUAL TABLE parent_presence_search USING fts5(content);",
    )?;
    assert_store_rejected_without_byte_changes(&store)
}
