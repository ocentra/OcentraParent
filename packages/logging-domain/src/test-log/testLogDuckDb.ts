import fs from 'node:fs';
import path from 'node:path';
import { DuckDBInstance, type DuckDBConnection, type DuckDBValue } from '@duckdb/node-api';
import { getChangedFiles, removeManifest, updateManifest } from './ingestManifest';
import { getDbDir, getTestLogScopeDir, listNdjsonFiles } from './ndjsonPaths';
import { readTestLogEntriesFromFile } from './ndjsonWriter';
import {
  buildGeneratedSearchLikeQuery,
  encodeGeneratedDuckDbTags,
  GeneratedCreateTestLogsTableSql,
  GeneratedDeleteByFileSql,
  GeneratedIndexScopeLevelSql,
  GeneratedIndexScopeRunSql,
  GeneratedLatestFailuresQuerySql,
  GeneratedSearchQuerySql,
  GeneratedStatsQuerySql,
  getGeneratedDefaultDuckDbFileName,
  rowToGeneratedStats,
  rowToGeneratedStoredLog,
} from '../duckdb-log-query';
import { type StoredTestLogLine, type TestLogScope as TestLogScopeType, type TestLogStats } from './types';

export interface IngestResult {
  readonly mode: 'rebuild' | 'incremental';
  readonly filesProcessed: number;
  readonly logsInserted: number;
}

interface StoredLogRow {
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
}

interface StatsRow {
  readonly total_logs: number;
  readonly error_logs: number;
  readonly warn_logs: number;
  readonly distinct_runs: number;
  readonly distinct_tests: number;
  readonly newest_timestamp: number | null;
}

function getDefaultDbPath(scope: TestLogScopeType, rootDir?: string): string {
  return path.join(getDbDir(rootDir), getGeneratedDefaultDuckDbFileName(scope));
}

function openDatabase(filePath: string): Promise<DuckDBInstance> {
  return DuckDBInstance.create(filePath);
}

function runAsync(connection: DuckDBConnection, sql: string, ...params: DuckDBValue[]): Promise<void> {
  return connection.run(sql, params).then(() => undefined);
}

async function allAsync<T extends object>(
  connection: DuckDBConnection,
  sql: string,
  ...params: DuckDBValue[]
): Promise<T[]> {
  const reader = await connection.runAndReadAll(sql, params);
  return reader.getRowObjects() as T[];
}

function closeConnection(connection: DuckDBConnection): void {
  connection.disconnectSync();
}

function closeDatabase(database: DuckDBInstance): void {
  database.closeSync();
}

function rowToStoredLog(row: StoredLogRow): StoredTestLogLine {
  return rowToGeneratedStoredLog(row);
}

function rowToStats(row: StatsRow | undefined): TestLogStats {
  return rowToGeneratedStats(row);
}

export class TestLogDuckDb {
  private readonly dbPath: string;
  private readonly database: DuckDBInstance;
  private readonly connection: DuckDBConnection;

  private constructor(dbPath: string, database: DuckDBInstance, connection: DuckDBConnection) {
    this.dbPath = dbPath;
    this.database = database;
    this.connection = connection;
  }

  static async create(scope: TestLogScopeType, rootDir?: string): Promise<TestLogDuckDb> {
    const dbPath = getDefaultDbPath(scope, rootDir);
    fs.mkdirSync(path.dirname(dbPath), { recursive: true });
    const database = await openDatabase(dbPath);
    const connection = await database.connect();
    const instance = new TestLogDuckDb(dbPath, database, connection);
    await instance.ensureSchema();
    return instance;
  }

  async close(): Promise<void> {
    closeConnection(this.connection);
    closeDatabase(this.database);
  }

  async ensureSchema(): Promise<void> {
    await runAsync(this.connection, GeneratedCreateTestLogsTableSql);

    await runAsync(this.connection, GeneratedIndexScopeLevelSql);
    await runAsync(this.connection, GeneratedIndexScopeRunSql);
  }

  async reset(): Promise<void> {
    await runAsync(this.connection, 'DELETE FROM test_logs');
  }

  async insertLogs(filePath: string, logs: readonly StoredTestLogLine[]): Promise<number> {
    if (logs.length === 0) {
      return 0;
    }

    await runAsync(this.connection, GeneratedDeleteByFileSql, filePath);

    for (const log of logs) {
      await runAsync(
        this.connection,
        `INSERT INTO test_logs (
          ndjson_file,
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
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
        filePath,
        log.scope,
        log.runId,
        log.runType,
        log.suiteType,
        log.testName,
        log.timestamp,
        log.level,
        log.source,
        log.context,
        log.message,
        log.data,
        log.file,
        log.filePath,
        log.line,
        log.column,
        log.correlationId,
        encodeGeneratedDuckDbTags(log.tags),
        log.stack,
        log.origin,
        log.environment
      );
    }

    return logs.length;
  }

  async ingestFromScope(scope: TestLogScopeType, rootDir?: string, rebuild = false): Promise<IngestResult> {
    const logsDir = getTestLogScopeDir(scope, rootDir);

    if (rebuild) {
      await this.reset();
      removeManifest(scope, rootDir);
    }

    const filesToProcess = rebuild
      ? listNdjsonFiles(logsDir)
      : (() => {
          const { newFiles, changedFiles } = getChangedFiles(scope, logsDir, rootDir);
          return [...newFiles, ...changedFiles].sort((left, right) => left.localeCompare(right));
        })();

    let inserted = 0;
    for (const filePath of filesToProcess) {
      inserted += await this.insertLogs(filePath, readTestLogEntriesFromFile(filePath));
    }

    updateManifest(scope, logsDir, rootDir);

    return {
      mode: rebuild ? 'rebuild' : 'incremental',
      filesProcessed: filesToProcess.length,
      logsInserted: inserted,
    };
  }

  async getStats(scope: TestLogScopeType): Promise<TestLogStats> {
    const rows = await allAsync<StatsRow>(this.connection, GeneratedStatsQuerySql, scope);

    return rowToStats(rows[0]);
  }

  async latestFailures(scope: TestLogScopeType, limit = 20): Promise<StoredTestLogLine[]> {
    const rows = await allAsync<StoredLogRow>(this.connection, GeneratedLatestFailuresQuerySql, scope, limit);

    return rows.map(rowToStoredLog);
  }

  async search(scope: TestLogScopeType, query: string, limit = 20): Promise<StoredTestLogLine[]> {
    const likeQuery = buildGeneratedSearchLikeQuery(query);
    const rows = await allAsync<StoredLogRow>(
      this.connection,
      GeneratedSearchQuerySql,
      scope,
      likeQuery,
      likeQuery,
      likeQuery,
      limit
    );

    return rows.map(rowToStoredLog);
  }

  dbFilePath(): string {
    return this.dbPath;
  }
}
