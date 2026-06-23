import {
  AgentDeviceIdSchema as EventAgentDeviceIdSchema,
  AgentPlatformSchema as EventAgentPlatformSchema,
} from './event-primitives';
import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';

export const AgentDeviceIdSchema = EventAgentDeviceIdSchema;
export const AgentHostnameSchema = brandedNonEmptyStringSchema('AgentHostname');
export const AgentPlatformSchema = EventAgentPlatformSchema;
export const AgentServiceVersionSchema = brandedNonEmptyStringSchema('AgentServiceVersion');
export const LogEntryIdSchema = brandedNonEmptyStringSchema('LogEntryId');
export const LogTimestampSchema = brandedNonEmptyStringSchema('LogTimestamp');
export const LogMessageSchema = brandedNonEmptyStringSchema('LogMessage');
export const StackTraceSchema = Schema.String.pipe(Schema.brand('StackTrace'));

export const LogFieldValueSchema = withParser(Schema.Union(Schema.String, Schema.Number, Schema.Boolean, Schema.Null));
export const LogFieldsSchema = withParser(Schema.Record({ key: Schema.String, value: LogFieldValueSchema }));
export const LogLevelSchema = withParser(Schema.Literal('trace', 'debug', 'info', 'warn', 'error'));
export const LogSourceSchema = withParser(
  Schema.Literal('agent-service', 'dev-server', 'local-api', 'portal', 'codex', 'validation', 'rust-test')
);

const createLogEntrySchema = () =>
  withParser(
    Schema.Struct({
      schemaVersion: Schema.Literal(1),
      id: LogEntryIdSchema,
      timestamp: LogTimestampSchema,
      level: LogLevelSchema,
      source: LogSourceSchema,
      message: LogMessageSchema,
      fields: LogFieldsSchema,
    })
  );

export const AgentIdentitySchema = withParser(
  Schema.Struct({
    deviceId: AgentDeviceIdSchema,
    hostname: AgentHostnameSchema,
    platform: AgentPlatformSchema,
    serviceVersion: AgentServiceVersionSchema,
  })
);

export const AgentLogEntrySchema = createLogEntrySchema();
export const AgentLogSnapshotSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    agent: AgentIdentitySchema,
    entries: Schema.Array(AgentLogEntrySchema),
  })
);
export const DevLogEntrySchema = createLogEntrySchema();

export type LogFieldValue = Infer<typeof LogFieldValueSchema>;
export type LogFields = Infer<typeof LogFieldsSchema>;
export type LogLevel = Infer<typeof LogLevelSchema>;
export type LogSource = Infer<typeof LogSourceSchema>;
export type AgentDeviceId = typeof AgentDeviceIdSchema.Type;
export type AgentHostname = typeof AgentHostnameSchema.Type;
export type AgentPlatform = typeof AgentPlatformSchema.Type;
export type AgentServiceVersion = typeof AgentServiceVersionSchema.Type;
export type LogEntryId = typeof LogEntryIdSchema.Type;
export type LogTimestamp = typeof LogTimestampSchema.Type;
export type LogMessage = typeof LogMessageSchema.Type;
export type StackTrace = typeof StackTraceSchema.Type;
export type AgentIdentity = Infer<typeof AgentIdentitySchema>;
export type AgentLogEntry = Infer<typeof AgentLogEntrySchema>;
export type AgentLogSnapshot = Infer<typeof AgentLogSnapshotSchema>;
export type DevLogEntry = Infer<typeof DevLogEntrySchema>;

export const decodeLogEntryId = Schema.decodeUnknownSync(LogEntryIdSchema);
export const decodeLogMessage = Schema.decodeUnknownSync(LogMessageSchema);
export const decodeLogTimestamp = Schema.decodeUnknownSync(LogTimestampSchema);
export const decodeStackTrace = Schema.decodeUnknownSync(StackTraceSchema);

export const LogLevel = {
  Trace: LogLevelSchema.parse('trace'),
  Debug: LogLevelSchema.parse('debug'),
  Info: LogLevelSchema.parse('info'),
  Warn: LogLevelSchema.parse('warn'),
  Error: LogLevelSchema.parse('error'),
} as const;

export const LogSource = {
  AgentService: LogSourceSchema.parse('agent-service'),
  DevServer: LogSourceSchema.parse('dev-server'),
  LocalApi: LogSourceSchema.parse('local-api'),
  Portal: LogSourceSchema.parse('portal'),
  Codex: LogSourceSchema.parse('codex'),
  Validation: LogSourceSchema.parse('validation'),
  RustTest: LogSourceSchema.parse('rust-test'),
} as const;

export const LoggerRuntimeEnvironment = {
  RunId: 'OCENTRA_PARENT_LOG_RUN_ID',
  TestName: 'OCENTRA_PARENT_LOG_TEST_NAME',
  Scope: 'OCENTRA_PARENT_LOG_SCOPE',
  RunType: 'OCENTRA_PARENT_LOG_RUN_TYPE',
  SuiteType: 'OCENTRA_PARENT_LOG_SUITE_TYPE',
  Origin: 'OCENTRA_PARENT_LOG_ORIGIN',
  Environment: 'OCENTRA_PARENT_LOG_ENVIRONMENT',
} as const;

export const LoggerRuntimeDefaults = {
  GeneratedRunIdPrefix: 'parent-log-run-',
  TestName: 'parent-runtime-logger',
  UnknownModule: 'UnknownModule',
  ModuleContextSuffix: 'module',
} as const;

export const DevLogEndpoint = { Write: '/__ocentra-parent-dev-log' } as const;
export const DevLogHttp = {
  MethodPost: 'POST',
  HeaderContentType: 'Content-Type',
  ContentTypeJson: 'application/json',
  CredentialsSameOrigin: 'same-origin',
} as const;
export const DevLogEnvironment = { Directory: 'OCENTRA_PARENT_DEV_LOG_DIR' } as const;
export const DevLogBridge = {
  DefaultUrl: 'http://127.0.0.1:4479',
  EnvironmentUrl: 'VITE_OCENTRA_PARENT_LOG_BRIDGE_URL',
  GlobalUrlKey: '__OCENTRA_PARENT_LOG_BRIDGE_URL',
  PortalContext: 'portal-dev-observability',
  PortalEnvironment: 'dev',
  PortalTestName: 'portal-dev-runtime',
} as const;
export const DevLogFile = {
  DirectoryName: 'dev',
  Extension: 'ndjson',
  AgentServicePrefix: 'agent-service',
  PortalPrefix: 'portal',
  DevServerPrefix: 'dev-server',
} as const;
export const DevLogField = {
  AgentWebSocketUrl: 'agentWebSocketUrl',
  Command: 'command',
  ConnectionState: 'connectionState',
  Event: 'event',
  EventsBuffered: 'eventsBuffered',
  Port: 'port',
} as const;
export const DevLogIdPrefix = {
  Portal: 'portal-log-',
  DevServer: 'dev-server-log-',
} as const;
export const DevLogMessage = {
  PortalStarted: decodeLogMessage('Portal dev runtime started.'),
  PortalCommandSent: decodeLogMessage('Portal command sent.'),
  PortalEventReceived: decodeLogMessage('Portal WebSocket event received.'),
  PortalResultCopied: decodeLogMessage('Portal command result copied.'),
  DevServerStarted: decodeLogMessage('Vite dev server started.'),
} as const;
