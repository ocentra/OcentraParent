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
    delivery_state TEXT NOT NULL CHECK (
        delivery_state IN ('pending', 'delivered')
    )
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
