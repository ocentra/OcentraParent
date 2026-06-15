import type { BridgeEntry, BridgeLogPayload } from '../transport/bridgeLogPayload';
import {
  RunType,
  TestLogScope,
  type RunType as RunTypeValue,
  type StoredTestLogLine,
  type TestLogScope as TestLogScopeValue,
} from './types';

export interface BridgePayloadToStoredLogOptions {
  readonly testName: string;
  readonly runId: string;
  readonly consumer?: TestLogScopeValue | null;
  readonly runType?: RunTypeValue;
}

export function bridgePayloadToStoredLog(
  payload: BridgeLogPayload,
  options: BridgePayloadToStoredLogOptions
): StoredTestLogLine {
  return {
    schemaVersion: 1,
    type: 'log',
    scope: options.consumer ?? TestLogScope.ParentTest,
    runId: options.runId,
    runType: options.runType ?? RunType.Single,
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
    tags: payload.tags,
    stack: payload.stack,
    origin: payload.origin,
    environment: payload.environment,
  };
}

export function storedLogToBridgePayload(log: StoredTestLogLine): BridgeLogPayload {
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
    tags: log.tags,
    stack: log.stack,
    suite_type: log.suiteType,
    origin: log.origin,
    environment: log.environment,
  };
}

export function storedLogToBridgeEntry(log: StoredTestLogLine): BridgeEntry {
  return {
    testName: log.testName,
    runId: log.runId,
    runType: log.runType,
    consumer: log.scope,
    log: storedLogToBridgePayload(log),
  };
}

export function createBridgeEntryFromStoredLog(
  log: StoredTestLogLine,
  overrides: Partial<Pick<BridgeEntry, 'consumer' | 'runId' | 'runType' | 'testName'>> = {}
): BridgeEntry {
  return {
    testName: overrides.testName ?? log.testName,
    runId: overrides.runId ?? log.runId,
    runType: overrides.runType ?? log.runType,
    consumer: overrides.consumer ?? log.scope,
    log: storedLogToBridgePayload(log),
  };
}
