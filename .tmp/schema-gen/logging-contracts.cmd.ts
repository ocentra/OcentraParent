/* generated from crates/schema/src/logging_contracts.rs */

export const LoggingContractRuntime = {
  SchemaVersion: 1,
  SnapshotSchemaVersion: 1,
} as const;

export type GeneratedLogFieldValue = string | number | boolean | null;
export type GeneratedLogFields = Record<string, GeneratedLogFieldValue>;
export type GeneratedAgentDeviceId = string;
export type GeneratedAgentHostname = string;
export type GeneratedAgentPlatform = string;
export type GeneratedAgentServiceVersion = string;
export type GeneratedLogEntryId = string;
export type GeneratedLogTimestamp = string;
export type GeneratedLogMessage = string;
export type GeneratedStackTrace = string;
export type GeneratedLogSnapshotState = string;

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
  deviceId: GeneratedAgentDeviceId;
  hostname: GeneratedAgentHostname;
  platform: GeneratedAgentPlatform;
  serviceVersion: GeneratedAgentServiceVersion;
}

export interface GeneratedAgentLogEntry {
  schemaVersion: typeof LoggingContractRuntime.SchemaVersion;
  id: GeneratedLogEntryId;
  timestamp: GeneratedLogTimestamp;
  level: GeneratedLogLevel;
  source: GeneratedLogSource;
  message: GeneratedLogMessage;
  fields: GeneratedLogFields;
}

export interface GeneratedAgentLogSnapshot {
  schemaVersion: typeof LoggingContractRuntime.SchemaVersion;
  agent: GeneratedAgentIdentity;
  entries: GeneratedAgentLogEntry[];
}

export type GeneratedDevLogEntry = GeneratedAgentLogEntry;

export interface GeneratedParentLogEvent {
  schemaVersion: typeof LoggingContractRuntime.SchemaVersion;
  id: GeneratedLogEntryId;
  timestamp: GeneratedLogTimestamp;
  level: GeneratedLogLevel;
  source: GeneratedLogSource;
  message: GeneratedLogMessage;
  fields: GeneratedLogFields;
  runId?: string;
  laneId?: string;
  commandId?: string;
  correlationId?: string;
  file?: string;
  line?: number;
  column?: number;
}

export interface GeneratedLogSnapshot {
  schemaVersion: typeof LoggingContractRuntime.SnapshotSchemaVersion;
  status: GeneratedLogSnapshotState;
  entries: GeneratedParentLogEvent[];
}
