use serde::{Deserialize, Serialize};

const DEFAULT_SCOPE: &str = "parent-test";
const DEFAULT_RUN_TYPE: &str = "single";
const TEST_LOG_SCHEMA_VERSION: u32 = 1;
const TEST_LOG_ENTRY_TYPE: &str = "log";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TestLogOrigin {
    Test,
    Worker,
    Portal,
    AgentService,
    Codex,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeLogPayload {
    pub log_timestamp: u64,
    pub level: String,
    pub source: Option<String>,
    pub context: Option<String>,
    pub message: String,
    pub data: Option<String>,
    pub file: Option<String>,
    pub file_path: Option<String>,
    pub line: Option<u64>,
    pub column: Option<u64>,
    pub correlation_id: Option<String>,
    pub tags: Vec<String>,
    pub stack: Option<String>,
    pub suite_type: Option<String>,
    pub origin: Option<TestLogOrigin>,
    pub environment: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeEntry {
    pub test_name: String,
    pub run_id: String,
    pub run_type: String,
    pub consumer: Option<String>,
    pub log: BridgeLogPayload,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgePayloadToStoredLogOptions {
    pub test_name: String,
    pub run_id: String,
    pub consumer: Option<String>,
    pub run_type: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeEntryOverrides {
    pub test_name: Option<String>,
    pub run_id: Option<String>,
    pub run_type: Option<String>,
    pub consumer: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoredTestLogLine {
    pub schema_version: u32,
    pub entry_type: String,
    pub scope: String,
    pub run_id: String,
    pub run_type: String,
    pub suite_type: Option<String>,
    pub test_name: String,
    pub timestamp: u64,
    pub level: String,
    pub source: Option<String>,
    pub context: Option<String>,
    pub message: String,
    pub data: Option<String>,
    pub file: Option<String>,
    pub file_path: Option<String>,
    pub line: Option<u64>,
    pub column: Option<u64>,
    pub correlation_id: Option<String>,
    pub tags: Vec<String>,
    pub stack: Option<String>,
    pub origin: Option<TestLogOrigin>,
    pub environment: Option<String>,
}

pub fn bridge_payload_to_stored_log(
    payload: &BridgeLogPayload,
    options: &BridgePayloadToStoredLogOptions,
) -> StoredTestLogLine {
    StoredTestLogLine {
        schema_version: TEST_LOG_SCHEMA_VERSION,
        entry_type: TEST_LOG_ENTRY_TYPE.to_owned(),
        scope: options
            .consumer
            .clone()
            .unwrap_or_else(|| DEFAULT_SCOPE.to_owned()),
        run_id: options.run_id.clone(),
        run_type: options
            .run_type
            .clone()
            .unwrap_or_else(|| DEFAULT_RUN_TYPE.to_owned()),
        suite_type: payload.suite_type.clone(),
        test_name: options.test_name.clone(),
        timestamp: payload.log_timestamp,
        level: payload.level.clone(),
        source: payload.source.clone(),
        context: payload.context.clone(),
        message: payload.message.clone(),
        data: payload.data.clone(),
        file: payload.file.clone(),
        file_path: payload.file_path.clone(),
        line: payload.line,
        column: payload.column,
        correlation_id: payload.correlation_id.clone(),
        tags: payload.tags.clone(),
        stack: payload.stack.clone(),
        origin: payload.origin.clone(),
        environment: payload.environment.clone(),
    }
}

pub fn bridge_entry_to_stored_log(entry: &BridgeEntry) -> StoredTestLogLine {
    bridge_payload_to_stored_log(
        &entry.log,
        &BridgePayloadToStoredLogOptions {
            test_name: entry.test_name.clone(),
            run_id: entry.run_id.clone(),
            consumer: entry.consumer.clone(),
            run_type: Some(entry.run_type.clone()),
        },
    )
}

pub fn stored_log_to_bridge_payload(log: &StoredTestLogLine) -> BridgeLogPayload {
    BridgeLogPayload {
        log_timestamp: log.timestamp,
        level: log.level.clone(),
        source: log.source.clone(),
        context: log.context.clone(),
        message: log.message.clone(),
        data: log.data.clone(),
        file: log.file.clone(),
        file_path: log.file_path.clone(),
        line: log.line,
        column: log.column,
        correlation_id: log.correlation_id.clone(),
        tags: log.tags.clone(),
        stack: log.stack.clone(),
        suite_type: log.suite_type.clone(),
        origin: log.origin.clone(),
        environment: log.environment.clone(),
    }
}

pub fn stored_log_to_bridge_entry(log: &StoredTestLogLine) -> BridgeEntry {
    BridgeEntry {
        test_name: log.test_name.clone(),
        run_id: log.run_id.clone(),
        run_type: log.run_type.clone(),
        consumer: Some(log.scope.clone()),
        log: stored_log_to_bridge_payload(log),
    }
}

pub fn create_bridge_entry_from_stored_log(
    log: &StoredTestLogLine,
    overrides: &BridgeEntryOverrides,
) -> BridgeEntry {
    BridgeEntry {
        test_name: overrides
            .test_name
            .clone()
            .unwrap_or_else(|| log.test_name.clone()),
        run_id: overrides
            .run_id
            .clone()
            .unwrap_or_else(|| log.run_id.clone()),
        run_type: overrides
            .run_type
            .clone()
            .unwrap_or_else(|| log.run_type.clone()),
        consumer: overrides
            .consumer
            .clone()
            .or_else(|| Some(log.scope.clone())),
        log: stored_log_to_bridge_payload(log),
    }
}

pub fn bridge_log_runtime_typescript() -> &'static str {
    BRIDGE_LOG_RUNTIME_TYPESCRIPT
}

const BRIDGE_LOG_RUNTIME_TYPESCRIPT: &str = r#"/* generated from crates/logging-core/src/bridge_log_runtime.rs */

import { LogLevelSchema, type LogLevel } from './logging-contracts';
import type { RunType, StoredTestLogLine, TestLogOrigin, TestLogScope, TestSuiteType } from './test-log/types';

export interface BridgeLogPayload {
  readonly log_timestamp: number;
  readonly level: LogLevel;
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
  readonly suite_type: TestSuiteType | null;
  readonly origin: TestLogOrigin | null;
  readonly environment: string | null;
}

export interface BridgeEntry {
  readonly testName: string;
  readonly runId: string;
  readonly log: BridgeLogPayload;
  readonly consumer: TestLogScope | null;
  readonly runType: RunType;
}

export type GeneratedBridgePayloadToStoredLogOptions = {
  readonly testName: string;
  readonly runId: string;
  readonly consumer?: TestLogScope | null;
  readonly runType?: RunType;
};

export type GeneratedBridgeEntryOverrides = Partial<Pick<BridgeEntry, 'consumer' | 'runId' | 'runType' | 'testName'>>;

export function bridgePayloadToGeneratedStoredLog(
  payload: BridgeLogPayload,
  options: GeneratedBridgePayloadToStoredLogOptions
): StoredTestLogLine {
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

export function bridgeEntryToGeneratedStoredLog(entry: BridgeEntry): StoredTestLogLine {
  return bridgePayloadToGeneratedStoredLog(entry.log, {
    testName: entry.testName,
    runId: entry.runId,
    consumer: entry.consumer,
    runType: entry.runType,
  });
}

export function storedGeneratedLogToBridgePayload(log: StoredTestLogLine): BridgeLogPayload {
  return {
    log_timestamp: log.timestamp,
    level: LogLevelSchema.parse(log.level),
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

export function storedGeneratedLogToBridgeEntry(log: StoredTestLogLine): BridgeEntry {
  return {
    testName: log.testName,
    runId: log.runId,
    runType: log.runType,
    consumer: log.scope,
    log: storedGeneratedLogToBridgePayload(log),
  };
}

export function createGeneratedBridgeEntryFromStoredLog(
  log: StoredTestLogLine,
  overrides: GeneratedBridgeEntryOverrides = {}
): BridgeEntry {
  return {
    testName: overrides.testName ?? log.testName,
    runId: overrides.runId ?? log.runId,
    runType: overrides.runType ?? log.runType,
    consumer: overrides.consumer ?? log.scope,
    log: storedGeneratedLogToBridgePayload(log),
  };
}
"#;
