import { DuckDBInstance, type DuckDBConnection, type DuckDBValue } from '@duckdb/node-api';
import { statLocalArtifact, type LocalArtifactStat } from '../local-artifact-file';

export interface TestLogDuckDbResources {
  readonly database: DuckDBInstance;
  readonly connection: DuckDBConnection;
  readonly identity: LocalArtifactStat['identity'];
}

interface OpenDuckDbResources {
  readonly database: DuckDBInstance;
  readonly connection: DuckDBConnection;
}

export function runDuckDb(connection: DuckDBConnection, sql: string, ...params: DuckDBValue[]): Promise<void> {
  return connection.run(sql, params).then(() => undefined);
}

export async function readDuckDbRows<T extends object>(
  connection: DuckDBConnection,
  sql: string,
  ...params: DuckDBValue[]
): Promise<T[]> {
  const reader = await connection.runAndReadAll(sql, params);
  return reader.getRowObjects() as T[];
}

export function closeDuckDbResources(database: DuckDBInstance, connection: DuckDBConnection): void {
  connection.disconnectSync();
  database.closeSync();
}

async function openDuckDbResources(filePath: string): Promise<OpenDuckDbResources> {
  const database = await DuckDBInstance.create(filePath);
  try {
    return { database, connection: await database.connect() };
  } catch (error) {
    database.closeSync();
    throw error;
  }
}

function closePartialResources(
  database: DuckDBInstance | null,
  connection: DuckDBConnection | null,
  originalError: unknown
): never {
  const cleanupErrors: unknown[] = [];
  if (connection != null) {
    try {
      connection.disconnectSync();
    } catch (cleanupError) {
      cleanupErrors.push(cleanupError);
    }
  }
  if (database != null) {
    try {
      database.closeSync();
    } catch (cleanupError) {
      cleanupErrors.push(cleanupError);
    }
  }
  if (cleanupErrors.length > 0) {
    throw new AggregateError(
      [originalError, ...cleanupErrors],
      'DuckDB initialization failed and native resource cleanup was incomplete'
    );
  }
  throw originalError;
}

export async function openTestLogDuckDbResources(
  filePath: string,
  rootDir: string,
  initialize: (connection: DuckDBConnection) => Promise<void>
): Promise<TestLogDuckDbResources> {
  let database: DuckDBInstance | null = null;
  let connection: DuckDBConnection | null = null;
  try {
    ({ database, connection } = await openDuckDbResources(filePath));
    await initialize(connection);
    closeDuckDbResources(database, connection);
    database = null;
    connection = null;

    const stat = statLocalArtifact(filePath, rootDir);
    if (stat == null) throw new Error('DuckDB did not create its owned database file');

    ({ database, connection } = await openDuckDbResources(filePath));
    return { database, connection, identity: stat.identity };
  } catch (error) {
    return closePartialResources(database, connection, error);
  }
}
