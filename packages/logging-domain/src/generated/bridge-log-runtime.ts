/* generated from crates/logging-core/src/bridge_log_runtime.rs */

export interface GeneratedBridgeLogPayload {
  readonly log_timestamp: number;
  readonly level: string;
  readonly source: string | null;
  readonly context: string | null;
  readonly message: string;
  readonly data: string | null;
  readonly file: string | null;
  readonly file_path: string | null;
  readonly line: number | null;
  readonly column: number | null;
  readonly correlation_id: string | null;
  readonly tags: readonly string[];
  readonly stack: string | null;
  readonly suite_type: string | null;
  readonly origin: string | null;
  readonly environment: string | null;
}

export interface GeneratedBridgeEntry {
  readonly testName: string;
  readonly runId: string;
  readonly runType: string;
  readonly consumer: string | null;
  readonly log: GeneratedBridgeLogPayload;
}

export interface GeneratedBridgePayloadToStoredLogOptions {
  readonly testName: string;
  readonly runId: string;
  readonly consumer?: string | null;
  readonly runType?: string | null;
}

export interface GeneratedStoredTestLogLine {
  readonly schemaVersion: number;
  readonly type: 'log';
  readonly scope: string;
  readonly runId: string;
  readonly runType: string;
  readonly suiteType: string | null;
  readonly testName: string;
  readonly timestamp: number;
  readonly level: string;
  readonly source: string | null;
  readonly context: string | null;
  readonly message: string;
  readonly data: string | null;
  readonly file: string | null;
  readonly filePath: string | null;
  readonly line: number | null;
  readonly column: number | null;
  readonly correlationId: string | null;
  readonly tags: readonly string[];
  readonly stack: string | null;
  readonly origin: string | null;
  readonly environment: string | null;
}

export interface GeneratedBridgeEntryOverrides {
  readonly testName?: string;
  readonly runId?: string;
  readonly runType?: string;
  readonly consumer?: string | null;
}

export function bridgePayloadToGeneratedStoredLog(
  payload: GeneratedBridgeLogPayload,
  options: GeneratedBridgePayloadToStoredLogOptions
): GeneratedStoredTestLogLine {
  return {
    schemaVersion: 1,
    type: 'log',
    scope: options.consumer ?? 'parent-test',
    runId: options.runId,
    runType: options.runType ?? 'single',
    suiteType: payload.suite_type,
    testName: options.testName,
    timestamp: payload.log_timestamp,
    level: payload.level,
    source: payload.source,
    context: payload.context,
    message: payload.message,
    data: payload.data,
    file: payload.file,
    filePath: payload.file_path,
    line: payload.line,
    column: payload.column,
    correlationId: payload.correlation_id,
    tags: [...payload.tags],
    stack: payload.stack,
    origin: payload.origin,
    environment: payload.environment,
  };
}

export function bridgeEntryToGeneratedStoredLog(entry: GeneratedBridgeEntry): GeneratedStoredTestLogLine {
  return bridgePayloadToGeneratedStoredLog(entry.log, {
    testName: entry.testName,
    runId: entry.runId,
    consumer: entry.consumer,
    runType: entry.runType,
  });
}

export function storedGeneratedLogToBridgePayload(log: GeneratedStoredTestLogLine): GeneratedBridgeLogPayload {
  return {
    log_timestamp: log.timestamp,
    level: log.level,
    source: log.source,
    context: log.context,
    message: log.message,
    data: log.data,
    file: log.file,
    file_path: log.filePath,
    line: log.line,
    column: log.column,
    correlation_id: log.correlationId,
    tags: [...log.tags],
    stack: log.stack,
    suite_type: log.suiteType,
    origin: log.origin,
    environment: log.environment,
  };
}

export function storedGeneratedLogToBridgeEntry(log: GeneratedStoredTestLogLine): GeneratedBridgeEntry {
  return {
    testName: log.testName,
    runId: log.runId,
    runType: log.runType,
    consumer: log.scope,
    log: storedGeneratedLogToBridgePayload(log),
  };
}

export function createGeneratedBridgeEntryFromStoredLog(
  log: GeneratedStoredTestLogLine,
  overrides: GeneratedBridgeEntryOverrides = {}
): GeneratedBridgeEntry {
  return {
    testName: overrides.testName ?? log.testName,
    runId: overrides.runId ?? log.runId,
    runType: overrides.runType ?? log.runType,
    consumer: overrides.consumer ?? log.scope,
    log: storedGeneratedLogToBridgePayload(log),
  };
}
