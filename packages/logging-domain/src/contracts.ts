import {
  AgentDeviceIdSchema as EventAgentDeviceIdSchema,
  AgentPlatformSchema as EventAgentPlatformSchema,
} from '@ocentra-parent/event-domain/primitives';
import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const AgentDeviceIdSchema = EventAgentDeviceIdSchema;
export const AgentHostnameSchema = brandedNonEmptyStringSchema('AgentHostname');
export const AgentPlatformSchema = EventAgentPlatformSchema;
export const AgentServiceVersionSchema = brandedNonEmptyStringSchema('AgentServiceVersion');
export const LogEntryIdSchema = brandedNonEmptyStringSchema('LogEntryId');
export const LogTimestampSchema = brandedNonEmptyStringSchema('LogTimestamp');
export const LogMessageSchema = brandedNonEmptyStringSchema('LogMessage');

export const LogFieldValueSchema = withParser(Schema.Union(Schema.String, Schema.Number, Schema.Boolean, Schema.Null));

export const LogFieldsSchema = withParser(
  Schema.Record({
    key: Schema.String,
    value: LogFieldValueSchema,
  })
);

export const LogLevelSchema = withParser(Schema.Literal('trace', 'debug', 'info', 'warn', 'error'));

export const LogSourceSchema = withParser(Schema.Literal('agent-service', 'dev-server', 'local-api', 'portal'));

export const AgentIdentitySchema = withParser(
  Schema.Struct({
    deviceId: AgentDeviceIdSchema,
    hostname: AgentHostnameSchema,
    platform: AgentPlatformSchema,
    serviceVersion: AgentServiceVersionSchema,
  })
);

export const AgentLogEntrySchema = withParser(
  Schema.Struct({
    id: LogEntryIdSchema,
    timestamp: LogTimestampSchema,
    level: LogLevelSchema,
    source: LogSourceSchema,
    message: LogMessageSchema,
    fields: LogFieldsSchema,
  })
);

export const AgentLogSnapshotSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    agent: AgentIdentitySchema,
    entries: Schema.Array(AgentLogEntrySchema),
  })
);

export const DevLogEntrySchema = withParser(
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
export type AgentIdentity = Infer<typeof AgentIdentitySchema>;
export type AgentLogEntry = Infer<typeof AgentLogEntrySchema>;
export type AgentLogSnapshot = Infer<typeof AgentLogSnapshotSchema>;
export type DevLogEntry = Infer<typeof DevLogEntrySchema>;

export const decodeLogEntryId = Schema.decodeUnknownSync(LogEntryIdSchema);
export const decodeLogMessage = Schema.decodeUnknownSync(LogMessageSchema);
export const decodeLogTimestamp = Schema.decodeUnknownSync(LogTimestampSchema);

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
} as const;

export const DevLogEndpoint = {
  Write: '/__ocentra-parent-dev-log',
} as const;

export const DevLogHttp = {
  MethodPost: 'POST',
  HeaderContentType: 'Content-Type',
  ContentTypeJson: 'application/json',
  CredentialsSameOrigin: 'same-origin',
} as const;

export const DevLogEnvironment = {
  Directory: 'OCENTRA_PARENT_DEV_LOG_DIR',
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

