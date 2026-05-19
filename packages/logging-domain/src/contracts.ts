import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyLogText = Schema.String.pipe(Schema.minLength(1));

export const AgentDeviceIdSchema = NonEmptyLogText.pipe(Schema.brand('AgentDeviceId'));
export const AgentHostnameSchema = NonEmptyLogText.pipe(Schema.brand('AgentHostname'));
export const AgentPlatformSchema = NonEmptyLogText.pipe(Schema.brand('AgentPlatform'));
export const AgentServiceVersionSchema = NonEmptyLogText.pipe(Schema.brand('AgentServiceVersion'));
export const LogEntryIdSchema = NonEmptyLogText.pipe(Schema.brand('LogEntryId'));
export const LogTimestampSchema = NonEmptyLogText.pipe(Schema.brand('LogTimestamp'));
export const LogMessageSchema = NonEmptyLogText.pipe(Schema.brand('LogMessage'));

export const LogFieldValueSchema = withParser(Schema.Union(Schema.String, Schema.Number, Schema.Boolean, Schema.Null));

export const LogFieldsSchema = withParser(
  Schema.Record({
    key: Schema.String,
    value: LogFieldValueSchema,
  })
);

export const LogLevelSchema = withParser(Schema.Literal('trace', 'debug', 'info', 'warn', 'error'));

export const LogSourceSchema = withParser(Schema.Literal('agent-service', 'local-api', 'portal'));

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
