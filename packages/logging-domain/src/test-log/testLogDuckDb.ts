import fs from 'node:fs';
import path from 'node:path';
import { createRequire } from 'node:module';
import {
  getChangedFiles,
  removeManifest,
  updateManifest,
} from './ingestManifest';
import {
  getDbDir,
  getTestLogScopeDir,
  listNdjsonFiles,
} from './ndjsonPaths';
import { readTestLogEntriesFromFile } from './ndjsonWriter';
import {
  type StoredTestLogLine,
  type TestLogScope as TestLogScopeType,
  type TestLogStats,
} from '@ocentra-parent/schema-domain/test-log/types';

const require = createRequire(import.meta.url);

type DuckDbConnection = {
  readonly all: (sql: string, ...args: unknown[]) => void;
  readonly run: (sql: string, ...args: unknown[]) => void;
  readonly close: (callback: (error: Error | null) => void) => void;
};

type DuckDbDatabase = {
  readonly connect: () => DuckDbConnection;
  readonly close: (callback: (error: Error | null) => void) => void;
};

type DuckDbModule = {
  readonly Database: new (
    filename: string,
    callback: (error: Error | null) => void
  ) => DuckDbDatabase;
};

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

function loadDuckDb(): DuckDbModule {
  return require('duckdb') as DuckDbModule;
}

function getDefaultDbPath(scope: TestLogScopeType, rootDir?: string): string {
  return path.join(getDbDir(rootDir), `${scope}-test-log.duckdb`);
}

function openDatabase(filePath: string): Promise<DuckDbDatabase> {
  const DuckDb = loadDuckDb();
  return new Promise((resolve, reject) => {
    const database = new DuckDb.Database(filePath, (error) => {
      if (error != null) {
        reject(error);
        return;
      }
      resolve(database);
    });
  });
}

function runAsync(connection: DuckDbConnection, sql: string, ...params: unknown[]): Promise<void> {
  return new Promise((resolve, reject) => {
    connection.run(sql, ...params, (error: Error | null) => {
      if (error != null) {
        reject(error);
        return;
      }
      resolve();
    });
  });
}

function allAsync<T extends object>(
  connection: DuckDbConnection,
  sql: string,
  ...params: unknown[]
): Promise<T[]> {
  return new Promise((resolve, reject) => {
    connection.all(sql, ...params, (error: Error | null, rows: T[]) => {
      if (error != null) {
        reject(error);
        return;
      }
      resolve(rows);
    });
  });
}

function closeConnection(connection: DuckDbConnection): Promise<void> {
  return new Promise((resolve, reject) => {
    connection.close((error) => {
      if (error != null) {
        reject(error);
        return;
      }
      resolve();
    });
  });
}

function closeDatabase(database: DuckDbDatabase): Promise<void> {
  return new Promise((resolve, reject) => {
    database.close((error) => {
      if (error != null) {
        reject(error);
        return;
      }
      resolve();
    });
  });
}

function encodeTags(tags: readonly string[]): string | null {
  return tags.length > 0 ? tags.join(',') : null;
}

function decodeTags(tags: string | null): string[] {
  if (tags == null || tags.trim().length === 0) {
    return [];
  }
  return tags.split(',');
}

function rowToStoredLog(row: StoredLogRow): StoredTestLogLine {
  return {
    schemaVersion: 1,
    type: 'log',
    scope: row.scope as TestLogScopeType,
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
    tags: decodeTags(row.tags),
    stack: row.stack,
    origin: row.origin as StoredTestLogLine['origin'],
    environment: row.environment,
  };
}

function numberOrZero(value: number | undefined): number {
  return Number(value ?? 0);
}

function nullableNumber(value: number | null | undefined): number | null {
  return value == null ? null : Number(value);
}

function rowToStats(row: StatsRow | undefined): TestLogStats {
  return {
    totalLogs: numberOrZero(row?.total_logs),
    errorLogs: numberOrZero(row?.error_logs),
    warnLogs: numberOrZero(row?.warn_logs),
    distinctRuns: numberOrZero(row?.distinct_runs),
    distinctTests: numberOrZero(row?.distinct_tests),
    newestTimestamp: nullableNumber(row?.newest_timestamp),
  };
}

export class TestLogDuckDb {
  private readonly dbPath: string;
  private readonly database: DuckDbDatabase;
  private readonly connection: DuckDbConnection;

  private constructor(dbPath: string, database: DuckDbDatabase, connection: DuckDbConnection) {
    this.dbPath = dbPath;
    this.database = database;
    this.connection = connection;
  }

  static async create(scope: TestLogScopeType, rootDir?: string): Promise<TestLogDuckDb> {
    const dbPath = getDefaultDbPath(scope, rootDir);
    fs.mkdirSync(path.dirname(dbPath), { recursive: true });
    const database = await openDatabase(dbPath);
    const connection = database.connect();
    const instance = new TestLogDuckDb(dbPath, database, connection);
    await instance.ensureSchema();
    return instance;
  }

  async close(): Promise<void> {
    await closeConnection(this.connection);
    await closeDatabase(this.database);
  }

  async ensureSchema(): Promise<void> {
    await runAsync(
      this.connection,
      `CREATE TABLE IF NOT EXISTS test_logs (
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
      )`
    );

    await runAsync(
      this.connection,
      'CREATE INDEX IF NOT EXISTS idx_test_logs_scope_level ON test_logs(scope, level)'
    );
    await runAsync(
      this.connection,
      'CREATE INDEX IF NOT EXISTS idx_test_logs_scope_run ON test_logs(scope, run_id)'
    );
  }

  async reset(): Promise<void> {
    await runAsync(this.connection, 'DELETE FROM test_logs');
  }

  async insertLogs(filePath: string, logs: readonly StoredTestLogLine[]): Promise<number> {
    if (logs.length === 0) {
      return 0;
    }

    await runAsync(this.connection, 'DELETE FROM test_logs WHERE ndjson_file = ?', filePath);

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
        encodeTags(log.tags),
        log.stack,
        log.origin,
        log.environment
      );
    }

    return logs.length;
  }

  async ingestFromScope(
    scope: TestLogScopeType,
    rootDir?: string,
    rebuild = false
  ): Promise<IngestResult> {
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
    const rows = await allAsync<StatsRow>(
      this.connection,
      `SELECT
        COUNT(*)::BIGINT AS total_logs,
        SUM(CASE WHEN level = 'error' THEN 1 ELSE 0 END)::BIGINT AS error_logs,
        SUM(CASE WHEN level = 'warn' THEN 1 ELSE 0 END)::BIGINT AS warn_logs,
        COUNT(DISTINCT run_id)::BIGINT AS distinct_runs,
        COUNT(DISTINCT test_name)::BIGINT AS distinct_tests,
        MAX(log_timestamp)::BIGINT AS newest_timestamp
      FROM test_logs
      WHERE scope = ?`,
      scope
    );

    return rowToStats(rows[0]);
  }

  async latestFailures(scope: TestLogScopeType, limit = 20): Promise<StoredTestLogLine[]> {
    const rows = await allAsync<StoredLogRow>(
      this.connection,
      `SELECT
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
      LIMIT ?`,
      scope,
      limit
    );

    return rows.map(rowToStoredLog);
  }

  async search(scope: TestLogScopeType, query: string, limit = 20): Promise<StoredTestLogLine[]> {
    const likeQuery = `%${query}%`;
    const rows = await allAsync<StoredLogRow>(
      this.connection,
      `SELECT
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
      LIMIT ?`,
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

export async function withTestLogDuckDb<T>(
  scope: TestLogScopeType,
  work: (db: TestLogDuckDb) => Promise<T>,
  rootDir?: string
): Promise<T> {
  const db = await TestLogDuckDb.create(scope, rootDir);
  try {
    return await work(db);
  } finally {
    await db.close();
  }
}
