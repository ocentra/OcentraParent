import path from 'node:path';
import { type DuckDBConnection, type DuckDBInstance } from '@duckdb/node-api';
import { getChangedFiles, removeManifest, updateManifest } from './ingestManifest';
import { getDbDir, getDefaultLogRoot, getTestLogScopeDir, listNdjsonFiles } from './ndjsonPaths';
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
} from '../duckdb-log-query';
import { type StoredTestLogLine, type TestLogScope as TestLogScopeType, type TestLogStats } from './types';
import { recoverLocalArtifactAppends } from '../local-artifact-append';
import { statLocalArtifact, type LocalArtifactStat } from '../local-artifact-file';
import { withLocalArtifactLockAsync } from '../local-artifact-lock';
import { closeDuckDbResources, openTestLogDuckDbResources, readDuckDbRows, runDuckDb } from './testLogDuckDbResources';
import { type StatsRow, type StoredLogRow, testLogRow, testLogStats } from './testLogDuckDbRows';

export interface IngestResult {
  readonly mode: 'rebuild' | 'incremental';
  readonly filesProcessed: number;
  readonly logsInserted: number;
}

function getDefaultDbPath(scope: TestLogScopeType, rootDir?: string): string {
  return path.join(getDbDir(rootDir), getGeneratedDefaultDuckDbFileName(scope));
}

function sameFileSystemPath(left: string, right: string): boolean {
  const resolvedLeft = path.resolve(left);
  const resolvedRight = path.resolve(right);
  return process.platform === 'win32'
    ? resolvedLeft.toLowerCase() === resolvedRight.toLowerCase()
    : resolvedLeft === resolvedRight;
}

export class TestLogDuckDb {
  private readonly rootDir: string;
  private readonly dbPath: string;
  private readonly databaseIdentity: LocalArtifactStat['identity'];
  private readonly database: DuckDBInstance;
  private readonly connection: DuckDBConnection;
  private closed = false;

  private constructor(
    rootDir: string,
    dbPath: string,
    databaseIdentity: LocalArtifactStat['identity'],
    database: DuckDBInstance,
    connection: DuckDBConnection
  ) {
    this.rootDir = rootDir;
    this.dbPath = dbPath;
    this.databaseIdentity = databaseIdentity;
    this.database = database;
    this.connection = connection;
  }

  static async create(scope: TestLogScopeType, rootDir?: string): Promise<TestLogDuckDb> {
    const resolvedRoot = rootDir ?? getDefaultLogRoot();
    return withLocalArtifactLockAsync(resolvedRoot, async () => {
      recoverLocalArtifactAppends(resolvedRoot);
      const dbPath = getDefaultDbPath(scope, resolvedRoot);
      const resources = await openTestLogDuckDbResources(dbPath, resolvedRoot, async (connection) => {
        await runDuckDb(connection, GeneratedCreateTestLogsTableSql);
        await runDuckDb(connection, GeneratedIndexScopeLevelSql);
        await runDuckDb(connection, GeneratedIndexScopeRunSql);
      });
      return new TestLogDuckDb(resolvedRoot, dbPath, resources.identity, resources.database, resources.connection);
    });
  }

  async close(): Promise<void> {
    if (this.closed) {
      return;
    }
    await withLocalArtifactLockAsync(this.rootDir, async () => {
      if (this.closed) {
        return;
      }
      closeDuckDbResources(this.database, this.connection);
      this.closed = true;
      this.assertDatabaseIdentity();
    });
  }

  async ensureSchema(): Promise<void> {
    await this.withOwnedDatabase(async () => {
      await runDuckDb(this.connection, GeneratedCreateTestLogsTableSql);
      await runDuckDb(this.connection, GeneratedIndexScopeLevelSql);
      await runDuckDb(this.connection, GeneratedIndexScopeRunSql);
    });
  }

  async reset(): Promise<void> {
    await this.withOwnedDatabase(() => runDuckDb(this.connection, 'DELETE FROM test_logs'));
  }

  async insertLogs(filePath: string, logs: readonly StoredTestLogLine[]): Promise<number> {
    return this.withOwnedDatabase(async () => {
      if (logs.length === 0) {
        return 0;
      }
      await runDuckDb(this.connection, GeneratedDeleteByFileSql, filePath);
      for (const log of logs) {
        await runDuckDb(
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
    });
  }

  async ingestFromScope(scope: TestLogScopeType, rootDir?: string, rebuild = false): Promise<IngestResult> {
    const resolvedRoot = rootDir ?? this.rootDir;
    if (!sameFileSystemPath(resolvedRoot, this.rootDir)) {
      throw new Error('DuckDB ingest root must match its database root');
    }
    return this.withOwnedDatabase(async () => {
      const logsDir = getTestLogScopeDir(scope, resolvedRoot);
      if (rebuild) {
        await this.reset();
        removeManifest(scope, resolvedRoot);
      }
      const filesToProcess = rebuild
        ? listNdjsonFiles(logsDir)
        : (() => {
            const { newFiles, changedFiles } = getChangedFiles(scope, logsDir, resolvedRoot);
            return [...newFiles, ...changedFiles].sort((left, right) => left.localeCompare(right));
          })();
      let inserted = 0;
      for (const filePath of filesToProcess) {
        inserted += await this.insertLogs(filePath, readTestLogEntriesFromFile(filePath, resolvedRoot));
      }
      updateManifest(scope, logsDir, resolvedRoot);
      return {
        mode: rebuild ? 'rebuild' : 'incremental',
        filesProcessed: filesToProcess.length,
        logsInserted: inserted,
      };
    });
  }

  async getStats(scope: TestLogScopeType): Promise<TestLogStats> {
    return this.withOwnedDatabase(async () => {
      const rows = await readDuckDbRows<StatsRow>(this.connection, GeneratedStatsQuerySql, scope);
      return testLogStats(rows[0]);
    });
  }

  async latestFailures(scope: TestLogScopeType, limit = 20): Promise<StoredTestLogLine[]> {
    return this.withOwnedDatabase(async () => {
      const rows = await readDuckDbRows<StoredLogRow>(this.connection, GeneratedLatestFailuresQuerySql, scope, limit);
      return rows.map(testLogRow);
    });
  }

  async search(scope: TestLogScopeType, query: string, limit = 20): Promise<StoredTestLogLine[]> {
    return this.withOwnedDatabase(async () => {
      const likeQuery = buildGeneratedSearchLikeQuery(query);
      const rows = await readDuckDbRows<StoredLogRow>(
        this.connection,
        GeneratedSearchQuerySql,
        scope,
        likeQuery,
        likeQuery,
        likeQuery,
        limit
      );
      return rows.map(testLogRow);
    });
  }

  dbFilePath(): string {
    return this.dbPath;
  }

  private async withOwnedDatabase<T>(operation: () => Promise<T>): Promise<T> {
    return withLocalArtifactLockAsync(this.rootDir, async () => {
      if (this.closed) {
        throw new Error('DuckDB logging index is closed');
      }
      return operation();
    });
  }

  private assertDatabaseIdentity(): void {
    const current = statLocalArtifact(this.dbPath, this.rootDir)?.identity;
    if (
      current == null ||
      current.device !== this.databaseIdentity.device ||
      current.inode !== this.databaseIdentity.inode
    ) {
      throw new Error('DuckDB logging index file identity changed');
    }
  }
}
