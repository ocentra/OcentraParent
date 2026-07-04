pub const CREATE_TABLE_SQL: &str = r#"CREATE TABLE IF NOT EXISTS test_logs (
        ndjson_file VARCHAR NOT NULL,
        scope VARCHAR NOT NULL,
        run_id VARCHAR NOT NULL,
        run_type VARCHAR NOT NULL,
        suite_type VARCHAR,
        test_name VARCHAR NOT NULL,
        log_timestamp BIGINT NOT NULL,
        level VARCHAR NOT NULL,
        source VARCHAR,
        context VARCHAR,
        message VARCHAR NOT NULL,
        data VARCHAR,
        file VARCHAR,
        file_path VARCHAR,
        line BIGINT,
        column_value BIGINT,
        correlation_id VARCHAR,
        tags VARCHAR,
        stack VARCHAR,
        origin VARCHAR,
        environment VARCHAR
      )"#;
pub const INDEX_SCOPE_LEVEL_SQL: &str =
    "CREATE INDEX IF NOT EXISTS idx_test_logs_scope_level ON test_logs(scope, level)";
pub const INDEX_SCOPE_RUN_SQL: &str =
    "CREATE INDEX IF NOT EXISTS idx_test_logs_scope_run ON test_logs(scope, run_id)";
pub const DELETE_BY_FILE_SQL: &str = "DELETE FROM test_logs WHERE ndjson_file = ?";
pub const STATS_QUERY_SQL: &str = r#"SELECT
        COUNT(*)::BIGINT AS total_logs,
        SUM(CASE WHEN level = 'error' THEN 1 ELSE 0 END)::BIGINT AS error_logs,
        SUM(CASE WHEN level = 'warn' THEN 1 ELSE 0 END)::BIGINT AS warn_logs,
        COUNT(DISTINCT run_id)::BIGINT AS distinct_runs,
        COUNT(DISTINCT test_name)::BIGINT AS distinct_tests,
        MAX(log_timestamp)::BIGINT AS newest_timestamp
      FROM test_logs
      WHERE scope = ?"#;
pub const LATEST_FAILURES_QUERY_SQL: &str = r#"SELECT
        scope,
        run_id,
        run_type,
        suite_type,
        test_name,
        log_timestamp,
        level,
        source,
        context,
        message,
        data,
        file,
        file_path,
        line,
        column_value,
        correlation_id,
        tags,
        stack,
        origin,
        environment
      FROM test_logs
      WHERE scope = ? AND level = 'error'
      ORDER BY log_timestamp DESC
      LIMIT ?"#;
pub const SEARCH_QUERY_SQL: &str = r#"SELECT
        scope,
        run_id,
        run_type,
        suite_type,
        test_name,
        log_timestamp,
        level,
        source,
        context,
        message,
        data,
        file,
        file_path,
        line,
        column_value,
        correlation_id,
        tags,
        stack,
        origin,
        environment
      FROM test_logs
      WHERE scope = ?
        AND (
          message LIKE ?
          OR COALESCE(context, '') LIKE ?
          OR COALESCE(data, '') LIKE ?
        )
      ORDER BY log_timestamp DESC
      LIMIT ?"#;

pub fn default_db_file_name(scope: &str) -> String {
    format!("{scope}-test-log.duckdb")
}

pub fn encode_tags(tags: &[String]) -> Option<String> {
    if tags.is_empty() {
        None
    } else {
        Some(tags.join(","))
    }
}

pub fn decode_tags(tags: Option<&str>) -> Vec<String> {
    tags.map(str::trim)
        .filter(|tags| !tags.is_empty())
        .map(|tags| tags.split(',').map(ToOwned::to_owned).collect())
        .unwrap_or_default()
}

pub fn search_like_query(query: &str) -> String {
    format!("%{query}%")
}

pub fn duckdb_log_query_typescript() -> &'static str {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../logging-core-generated/duckdb_log_query.ts"
    ))
}
