use crate::parent_presence_store_sql_tokenizer::{tokenize, word, SqlToken};

const CHALLENGE_TABLE_SHAPE: &str = r#"
CREATE TABLE parent_presence_challenges (
    challenge_ref TEXT PRIMARY KEY NOT NULL,
    challenge_json TEXT NOT NULL,
    privileged_action_json TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    nonce_ref TEXT NOT NULL UNIQUE,
    lifecycle_state TEXT NOT NULL CHECK (
        lifecycle_state IN ('issued', 'consumed')
    )
) STRICT
"#;

const RECEIPT_TABLE_SHAPE: &str = r#"
CREATE TABLE parent_presence_receipts (
    receipt_sequence INTEGER PRIMARY KEY AUTOINCREMENT,
    challenge_ref TEXT NOT NULL UNIQUE,
    receipt_ref TEXT NOT NULL UNIQUE,
    FOREIGN KEY (challenge_ref)
        REFERENCES parent_presence_challenges(challenge_ref)
        ON DELETE RESTRICT
) STRICT
"#;

const DECISION_OUTBOX_TABLE_SHAPE: &str = r#"
CREATE TABLE parent_presence_decision_outbox (
    decision_id TEXT PRIMARY KEY NOT NULL,
    envelope_json TEXT NOT NULL,
    delivery_claim TEXT,
    delivery_claimed_at INTEGER,
    delivery_state TEXT NOT NULL CHECK (
        delivery_state IN ('pending', 'claimed', 'delivered')
    )
) STRICT
"#;

const PARENT_STEP_UP_INTENT_TABLE_SHAPE: &str = r#"
CREATE TABLE parent_step_up_intents (
    challenge_ref TEXT PRIMARY KEY NOT NULL,
    nonce_ref TEXT NOT NULL UNIQUE,
    intent_digest TEXT NOT NULL UNIQUE,
    family_id TEXT NOT NULL,
    trust_subject TEXT NOT NULL,
    parent_account_id TEXT NOT NULL,
    parent_device_id TEXT NOT NULL,
    child_device_id TEXT NOT NULL,
    installation_id TEXT NOT NULL,
    pairing_id TEXT NOT NULL,
    route_id TEXT NOT NULL,
    signer_public_key BLOB NOT NULL CHECK (length(signer_public_key) = 32),
    lifecycle_generation INTEGER NOT NULL CHECK (lifecycle_generation > 0),
    installation_binding_generation INTEGER NOT NULL CHECK (installation_binding_generation > 0),
    authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
    correlation_id TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    lifecycle_state TEXT NOT NULL CHECK (
        lifecycle_state IN ('issued', 'consumed')
    ),
    registration_state TEXT NOT NULL CHECK (
        registration_state IN ('pending', 'completed')
    ),
    parent_presence_receipt TEXT CHECK (
        parent_presence_receipt IS NULL OR length(parent_presence_receipt) = 64
    ),
    credential_id TEXT CHECK (credential_id IS NULL OR length(credential_id) BETWEEN 1 AND 512),
    credential_algorithm INTEGER CHECK (credential_algorithm IS NULL OR credential_algorithm = -8),
    credential_sign_count INTEGER CHECK (credential_sign_count IS NULL OR credential_sign_count >= 0),
    FOREIGN KEY (challenge_ref)
        REFERENCES parent_presence_challenges(challenge_ref)
        ON DELETE RESTRICT
) STRICT
"#;

const LEGACY_PARENT_STEP_UP_INTENT_TABLE_SHAPE: &str = r#"
CREATE TABLE parent_step_up_intents (
    challenge_ref TEXT PRIMARY KEY NOT NULL,
    nonce_ref TEXT NOT NULL UNIQUE,
    intent_digest TEXT NOT NULL UNIQUE,
    family_id TEXT NOT NULL,
    trust_subject TEXT NOT NULL,
    parent_account_id TEXT NOT NULL,
    parent_device_id TEXT NOT NULL,
    child_device_id TEXT NOT NULL,
    installation_id TEXT NOT NULL,
    pairing_id TEXT NOT NULL,
    route_id TEXT NOT NULL,
    signer_public_key BLOB NOT NULL CHECK (length(signer_public_key) = 32),
    lifecycle_generation INTEGER NOT NULL CHECK (lifecycle_generation > 0),
    installation_binding_generation INTEGER NOT NULL CHECK (installation_binding_generation > 0),
    authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
    correlation_id TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    lifecycle_state TEXT NOT NULL CHECK (
        lifecycle_state IN ('issued', 'consumed')
    ),
    registration_state TEXT NOT NULL CHECK (
        registration_state IN ('pending', 'completed')
    ),
    FOREIGN KEY (challenge_ref)
        REFERENCES parent_presence_challenges(challenge_ref)
        ON DELETE RESTRICT
) STRICT
"#;

pub(crate) fn challenge_table_is_canonical(sql: &str) -> bool {
    complete_shape_matches(sql, CHALLENGE_TABLE_SHAPE)
}

pub(crate) fn receipt_table_is_canonical(sql: &str) -> bool {
    complete_shape_matches(sql, RECEIPT_TABLE_SHAPE)
}

pub(crate) fn decision_outbox_table_is_canonical(sql: &str) -> bool {
    complete_shape_matches(sql, DECISION_OUTBOX_TABLE_SHAPE)
}

pub(crate) fn parent_step_up_intent_table_is_canonical(sql: &str) -> bool {
    complete_shape_matches(sql, PARENT_STEP_UP_INTENT_TABLE_SHAPE)
}

pub(crate) fn legacy_parent_step_up_intent_table_is_canonical(sql: &str) -> bool {
    complete_shape_matches(sql, LEGACY_PARENT_STEP_UP_INTENT_TABLE_SHAPE)
}

fn complete_shape_matches(actual: &str, expected: &str) -> bool {
    normalized_table_tokens(actual)
        .zip(normalized_table_tokens(expected))
        .is_some_and(|(actual, expected)| actual == expected)
}

fn normalized_table_tokens(sql: &str) -> Option<Vec<SqlToken>> {
    let mut tokens = tokenize(sql)?;
    if tokens.get(2..5) == Some(&[word("IF"), word("NOT"), word("EXISTS")]) {
        tokens.drain(2..5);
    }
    if tokens.last() == Some(&SqlToken::Symbol(';')) {
        tokens.pop();
    }
    Some(tokens)
}
