/* generated from crates/schema/src/logging_contracts.rs */

import { Schema, brandedNonEmptyStringSchema, withParser } from './effect';

export const LoggingContractRuntime = {
  SchemaVersion: 1,
  SnapshotSchemaVersion: 1,
} as const;

export type GeneratedLogFieldValue = string | number | boolean | null;
export type GeneratedLogFields = Record<string, GeneratedLogFieldValue>;
export const GeneratedAgentDeviceIdSchema = brandedNonEmptyStringSchema('GeneratedAgentDeviceId');
export const GeneratedAgentHostnameSchema = brandedNonEmptyStringSchema('GeneratedAgentHostname');
export const GeneratedAgentPlatformSchema = brandedNonEmptyStringSchema('GeneratedAgentPlatform');
export const GeneratedAgentServiceVersionSchema = brandedNonEmptyStringSchema('GeneratedAgentServiceVersion');
export const GeneratedLogEntryIdSchema = brandedNonEmptyStringSchema('GeneratedLogEntryId');
export const GeneratedLogTimestampSchema = brandedNonEmptyStringSchema('GeneratedLogTimestamp');
export const GeneratedLogMessageSchema = brandedNonEmptyStringSchema('GeneratedLogMessage');
export const GeneratedStackTraceSchema = withParser(Schema.String.pipe(Schema.brand('GeneratedStackTrace')));
export const GeneratedLogSnapshotStateSchema = brandedNonEmptyStringSchema('GeneratedLogSnapshotState');

export type GeneratedAgentDeviceId = typeof GeneratedAgentDeviceIdSchema.Type;
export type GeneratedAgentHostname = typeof GeneratedAgentHostnameSchema.Type;
export type GeneratedAgentPlatform = typeof GeneratedAgentPlatformSchema.Type;
export type GeneratedAgentServiceVersion = typeof GeneratedAgentServiceVersionSchema.Type;
export type GeneratedLogEntryId = typeof GeneratedLogEntryIdSchema.Type;
export type GeneratedLogTimestamp = typeof GeneratedLogTimestampSchema.Type;
export type GeneratedLogMessage = typeof GeneratedLogMessageSchema.Type;
export type GeneratedStackTrace = typeof GeneratedStackTraceSchema.Type;
export type GeneratedLogSnapshotState = typeof GeneratedLogSnapshotStateSchema.Type;

export type GeneratedLogLevel = 'trace' | 'debug' | 'info' | 'warn' | 'error';

export const GeneratedLogLevel = {
  Trace: 'trace',
  Debug: 'debug',
  Info: 'info',
  Warn: 'warn',
  Error: 'error',
} as const;

export type GeneratedLogSource =
  | 'agent-service'
  | 'dev-server'
  | 'local-api'
  | 'portal'
  | 'codex'
  | 'validation'
  | 'rust-test';

export const GeneratedLogSource = {
  AgentService: 'agent-service',
  DevServer: 'dev-server',
  LocalApi: 'local-api',
  Portal: 'portal',
  Codex: 'codex',
  Validation: 'validation',
  RustTest: 'rust-test',
} as const;

export const GeneratedLoggerRuntimeEnvironment = {
  RunId: 'OCENTRA_PARENT_LOG_RUN_ID',
  TestName: 'OCENTRA_PARENT_LOG_TEST_NAME',
  Scope: 'OCENTRA_PARENT_LOG_SCOPE',
  RunType: 'OCENTRA_PARENT_LOG_RUN_TYPE',
  SuiteType: 'OCENTRA_PARENT_LOG_SUITE_TYPE',
  Origin: 'OCENTRA_PARENT_LOG_ORIGIN',
  Environment: 'OCENTRA_PARENT_LOG_ENVIRONMENT',
} as const;

export const GeneratedLoggerRuntimeDefaults = {
  GeneratedRunIdPrefix: 'parent-log-run-',
  TestName: 'parent-runtime-logger',
  UnknownModule: 'UnknownModule',
  ModuleContextSuffix: 'module',
} as const;

export const GeneratedDevLogEndpoint = { Write: '/__ocentra-parent-dev-log' } as const;

export const GeneratedDevLogHttp = {
  MethodPost: 'POST',
  HeaderContentType: 'Content-Type',
  ContentTypeJson: 'application/json',
  CredentialsSameOrigin: 'same-origin',
} as const;

export const GeneratedDevLogEnvironment = { Directory: 'OCENTRA_PARENT_DEV_LOG_DIR' } as const;

export const GeneratedDevLogBridge = {
  DefaultUrl: 'http://127.0.0.1:4479',
  EnvironmentUrl: 'VITE_OCENTRA_PARENT_LOG_BRIDGE_URL',
  GlobalUrlKey: '__OCENTRA_PARENT_LOG_BRIDGE_URL',
  PortalContext: 'portal-dev-observability',
  PortalEnvironment: 'dev',
  PortalTestName: 'portal-dev-runtime',
} as const;

export const GeneratedDevLogFile = {
  DirectoryName: 'dev',
  Extension: 'ndjson',
  AgentServicePrefix: 'agent-service',
  PortalPrefix: 'portal',
  DevServerPrefix: 'dev-server',
} as const;

export const GeneratedDevLogField = {
  AgentWebSocketUrl: 'agentWebSocketUrl',
  Command: 'command',
  ConnectionState: 'connectionState',
  Event: 'event',
  EventsBuffered: 'eventsBuffered',
  Port: 'port',
} as const;

export const GeneratedDevLogIdPrefix = {
  Portal: 'portal-log-',
  DevServer: 'dev-server-log-',
} as const;

export const GeneratedDevLogMessage = {
  PortalStarted: 'Portal dev runtime started.',
  PortalCommandSent: 'Portal command sent.',
  PortalEventReceived: 'Portal host bridge event received.',
  PortalResultCopied: 'Portal command result copied.',
  DevServerStarted: 'Vite dev server started.',
} as const;

export interface GeneratedAgentIdentity {
  readonly deviceId: GeneratedAgentDeviceId;
  readonly hostname: GeneratedAgentHostname;
  readonly platform: GeneratedAgentPlatform;
  readonly serviceVersion: GeneratedAgentServiceVersion;
}

export interface GeneratedAgentLogEntry {
  readonly schemaVersion: typeof LoggingContractRuntime.SchemaVersion;
  readonly id: GeneratedLogEntryId;
  readonly timestamp: GeneratedLogTimestamp;
  readonly level: GeneratedLogLevel;
  readonly source: GeneratedLogSource;
  readonly message: GeneratedLogMessage;
  readonly fields: GeneratedLogFields;
}

export interface GeneratedAgentLogSnapshot {
  readonly schemaVersion: typeof LoggingContractRuntime.SchemaVersion;
  readonly agent: GeneratedAgentIdentity;
  readonly entries: readonly GeneratedAgentLogEntry[];
}

export type GeneratedDevLogEntry = GeneratedAgentLogEntry;

export interface GeneratedParentLogEvent {
  readonly schemaVersion: typeof LoggingContractRuntime.SchemaVersion;
  readonly id: GeneratedLogEntryId;
  readonly timestamp: GeneratedLogTimestamp;
  readonly level: GeneratedLogLevel;
  readonly source: GeneratedLogSource;
  readonly message: GeneratedLogMessage;
  readonly fields: GeneratedLogFields;
  readonly runId?: string;
  readonly laneId?: string;
  readonly commandId?: string;
  readonly correlationId?: string;
  readonly file?: string;
  readonly line?: number;
  readonly column?: number;
}

export interface GeneratedLogSnapshot {
  readonly schemaVersion: typeof LoggingContractRuntime.SnapshotSchemaVersion;
  readonly status: GeneratedLogSnapshotState;
  readonly entries: readonly GeneratedParentLogEvent[];
}
