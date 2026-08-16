/* generated from crates/logging-core/src/duckdb_log_query.rs */

import type { StoredTestLogLine, TestLogStats } from './test-log/types';

export const GeneratedCreateTestLogsTableSql = `CREATE TABLE IF NOT EXISTS test_logs (
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
      )`;
export const GeneratedIndexScopeLevelSql =
  'CREATE INDEX IF NOT EXISTS idx_test_logs_scope_level ON test_logs(scope, level)';
export const GeneratedIndexScopeRunSql =
  'CREATE INDEX IF NOT EXISTS idx_test_logs_scope_run ON test_logs(scope, run_id)';
export const GeneratedDeleteByFileSql = 'DELETE FROM test_logs WHERE ndjson_file = ?';
export const GeneratedStatsQuerySql = `SELECT
        COUNT(*)::BIGINT AS total_logs,
        SUM(CASE WHEN level = 'error' THEN 1 ELSE 0 END)::BIGINT AS error_logs,
        SUM(CASE WHEN level = 'warn' THEN 1 ELSE 0 END)::BIGINT AS warn_logs,
        COUNT(DISTINCT run_id)::BIGINT AS distinct_runs,
        COUNT(DISTINCT test_name)::BIGINT AS distinct_tests,
        MAX(log_timestamp)::BIGINT AS newest_timestamp
      FROM test_logs
      WHERE scope = ?`;
export const GeneratedLatestFailuresQuerySql = `SELECT
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
      LIMIT ?`;
export const GeneratedSearchQuerySql = `SELECT
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
      LIMIT ?`;

export function getGeneratedDefaultDuckDbFileName(scope: string): string {
  return `${scope}-test-log.duckdb`;
}

export function encodeGeneratedDuckDbTags(tags: readonly string[]): string | null {
  return tags.length > 0 ? tags.join(',') : null;
}

export function decodeGeneratedDuckDbTags(tags: string | null): string[] {
  if (tags == null || tags.trim().length === 0) {
    return [];
  }
  return tags.split(',');
}

export function buildGeneratedSearchLikeQuery(query: string): string {
  return `%${query}%`;
}

export function rowToGeneratedStoredLog(row: {
  readonly scope: string;
  readonly run_id: string;
  readonly run_type: string;
  readonly suite_type: string | null;
  readonly test_name: string;
  readonly log_timestamp: number;
  readonly level: string;
  readonly source: string | null;
  readonly context: string | null;
  readonly message: string;
  readonly data: string | null;
  readonly file: string | null;
  readonly file_path: string | null;
  readonly line: number | null;
  readonly column_value: number | null;
  readonly correlation_id: string | null;
  readonly tags: string | null;
  readonly stack: string | null;
  readonly origin: string | null;
  readonly environment: string | null;
}): StoredTestLogLine {
  return {
    schemaVersion: 1,
    type: 'log',
    scope: row.scope as StoredTestLogLine['scope'],
    runId: row.run_id,
    runType: row.run_type as StoredTestLogLine['runType'],
    suiteType: row.suite_type as StoredTestLogLine['suiteType'],
    testName: row.test_name,
    timestamp: Number(row.log_timestamp),
    level: row.level as StoredTestLogLine['level'],
    source: row.source,
    context: row.context,
    message: row.message,
    data: row.data,
    file: row.file,
    filePath: row.file_path,
    line: row.line == null ? null : Number(row.line),
    column: row.column_value == null ? null : Number(row.column_value),
    correlationId: row.correlation_id,
    tags: decodeGeneratedDuckDbTags(row.tags),
    stack: row.stack,
    origin: row.origin as StoredTestLogLine['origin'],
    environment: row.environment,
  };
}

type GeneratedStatsRow = {
  readonly total_logs?: number;
  readonly error_logs?: number;
  readonly warn_logs?: number;
  readonly distinct_runs?: number;
  readonly distinct_tests?: number;
  readonly newest_timestamp?: number | null;
};

function generatedStatsCount(row: GeneratedStatsRow | undefined, key: keyof GeneratedStatsRow): number {
  return Number(row?.[key] ?? 0);
}

function generatedStatsNewestTimestamp(row: GeneratedStatsRow | undefined): number | null {
  const newestTimestamp = row?.newest_timestamp;
  return newestTimestamp == null ? null : Number(newestTimestamp);
}

export function rowToGeneratedStats(row: GeneratedStatsRow | undefined): TestLogStats {
  return {
    totalLogs: generatedStatsCount(row, 'total_logs'),
    errorLogs: generatedStatsCount(row, 'error_logs'),
    warnLogs: generatedStatsCount(row, 'warn_logs'),
    distinctRuns: generatedStatsCount(row, 'distinct_runs'),
    distinctTests: generatedStatsCount(row, 'distinct_tests'),
    newestTimestamp: generatedStatsNewestTimestamp(row),
  };
}
