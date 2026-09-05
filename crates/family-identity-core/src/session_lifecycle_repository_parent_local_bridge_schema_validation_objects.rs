#![forbid(unsafe_code)]

//! Exact object and DDL validation for current and migratable bridge schemas.

use rusqlite::Connection;

use super::{validation, BRIDGE_SCHEMA_SQL};

pub(super) fn validate(connection: &Connection, legacy_v2: bool) -> Result<(), ()> {
    for (kind, names) in [
        ("table", validation::TABLES),
        ("index", validation::INDEXES),
    ] {
        for name in names {
            let actual = connection
                .query_row(
                    "SELECT sql FROM sqlite_master WHERE type = ?1 AND name = ?2 LIMIT 1",
                    [kind, name],
                    |row| row.get::<_, String>(0),
                )
                .map_err(|_error| ())?;
            let expected = expected_statement(kind, name, legacy_v2).ok_or(())?;
            if normalize_sql(&actual) != normalize_sql(&expected) {
                return Err(());
            }
        }
    }
    let unknown = connection
        .query_row(
            "SELECT count(*) FROM sqlite_master
              WHERE type IN ('table','index','trigger','view')
                AND name LIKE 'account_identity_parent_local_bridge_%'
                AND name NOT IN (
                    'account_identity_parent_local_bridge_schema',
                    'account_identity_parent_local_bridge_revoke_epoch',
                    'account_identity_parent_local_bridge_session',
                    'account_identity_parent_local_bridge_audit_outbox',
                    'account_identity_parent_local_bridge_account',
                    'account_identity_parent_local_bridge_audit_delivery',
                    'account_identity_parent_local_bridge_audit_retention'
                )",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_error| ())?;
    (unknown == 0).then_some(()).ok_or(())
}

fn expected_statement(kind: &str, name: &str, legacy_v2: bool) -> Option<String> {
    if legacy_v2 && kind == "table" {
        if name == validation::TABLES[0] {
            return Some(super::v2::SCHEMA_TABLE_SQL.to_owned());
        }
        if name == validation::TABLES[3] {
            return Some(super::v2::AUDIT_TABLE_SQL.to_owned());
        }
    }
    canonical_statement(kind, name)
}

fn canonical_statement(kind: &str, name: &str) -> Option<String> {
    let marker = match kind {
        // SQLite omits `IF NOT EXISTS` when it stores canonical SQL.
        "table" => format!("CREATETABLE{}", normalize_sql(name)),
        "index" => format!("CREATEINDEX{}", normalize_sql(name)),
        _ => return None,
    };
    BRIDGE_SCHEMA_SQL
        .split(';')
        .map(str::trim)
        .find(|statement| normalize_sql(statement).starts_with(&marker))
        .map(ToOwned::to_owned)
}

fn normalize_sql(sql: &str) -> String {
    sql.chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_uppercase)
        .collect::<String>()
        .replace("IFNOTEXISTS", "")
}
