/* thin adapter over Rust-generated logging contracts */

import {
  AgentDeviceIdSchema as EventAgentDeviceIdSchema,
  AgentPlatformSchema as EventAgentPlatformSchema,
} from './event-primitives';
import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import {
  GeneratedLogLevel,
  GeneratedDevLogEndpoint,
  GeneratedDevLogEnvironment,
  GeneratedDevLogFile,
  GeneratedDevLogField,
  GeneratedDevLogIdPrefix,
  GeneratedLogSource,
  GeneratedDevLogMessage,
  type GeneratedLogLevel as GeneratedLogLevelValue,
  type GeneratedLogMessage,
  type GeneratedLogSource as GeneratedLogSourceValue,
  type GeneratedStackTrace,
  LoggingContractRuntime,
  type GeneratedAgentIdentity,
  type GeneratedAgentLogEntry,
  type GeneratedAgentLogSnapshot,
  type GeneratedAgentDeviceId,
  type GeneratedAgentHostname,
  type GeneratedAgentPlatform,
  type GeneratedAgentServiceVersion,
  type GeneratedDevLogEntry,
  type GeneratedLogEntryId,
  type GeneratedLogFieldValue,
  type GeneratedLogFields,
  type GeneratedLogSnapshotState,
  type GeneratedLogTimestamp,
} from './generated-logging-contracts';

export {
  GeneratedDevLogEndpoint,
  GeneratedDevLogEnvironment,
  GeneratedDevLogFile,
  GeneratedDevLogField,
  GeneratedDevLogIdPrefix,
  GeneratedDevLogMessage,
  GeneratedLogLevel,
  GeneratedLogSource,
};

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
export const LogLevelSchema = withParser(
  Schema.Literal(
    GeneratedLogLevel.Trace,
    GeneratedLogLevel.Debug,
    GeneratedLogLevel.Info,
    GeneratedLogLevel.Warn,
    GeneratedLogLevel.Error
  )
);
export const LogSourceSchema = withParser(
  Schema.Literal(
    GeneratedLogSource.AgentService,
    GeneratedLogSource.DevServer,
    GeneratedLogSource.LocalApi,
    GeneratedLogSource.Portal,
    GeneratedLogSource.Codex,
    GeneratedLogSource.Validation,
    GeneratedLogSource.RustTest
  )
);

const createLogEntrySchema = () =>
  withParser(
    Schema.Struct({
      schemaVersion: Schema.Literal(LoggingContractRuntime.SchemaVersion),
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
    schemaVersion: Schema.Literal(LoggingContractRuntime.SchemaVersion),
    agent: AgentIdentitySchema,
    entries: Schema.Array(AgentLogEntrySchema),
  })
);
export const DevLogEntrySchema = createLogEntrySchema();

export type LogFieldValue = Infer<typeof LogFieldValueSchema> & GeneratedLogFieldValue;
export type LogFields = Infer<typeof LogFieldsSchema> & GeneratedLogFields;
export type LogLevel = Infer<typeof LogLevelSchema> & GeneratedLogLevelValue;
export type LogSource = Infer<typeof LogSourceSchema> & GeneratedLogSourceValue;
export type AgentDeviceId = typeof AgentDeviceIdSchema.Type & GeneratedAgentDeviceId;
export type AgentHostname = typeof AgentHostnameSchema.Type & GeneratedAgentHostname;
export type AgentPlatform = typeof AgentPlatformSchema.Type & GeneratedAgentPlatform;
export type AgentServiceVersion = typeof AgentServiceVersionSchema.Type & GeneratedAgentServiceVersion;
export type LogEntryId = typeof LogEntryIdSchema.Type & GeneratedLogEntryId;
export type LogTimestamp = typeof LogTimestampSchema.Type & GeneratedLogTimestamp;
export type LogMessage = typeof LogMessageSchema.Type & GeneratedLogMessage;
export type StackTrace = typeof StackTraceSchema.Type & GeneratedStackTrace;
export type AgentIdentity = Infer<typeof AgentIdentitySchema> & GeneratedAgentIdentity;
export type AgentLogEntry = Infer<typeof AgentLogEntrySchema> & GeneratedAgentLogEntry;
export type AgentLogSnapshot = Infer<typeof AgentLogSnapshotSchema> & GeneratedAgentLogSnapshot;
export type DevLogEntry = Infer<typeof DevLogEntrySchema> & GeneratedDevLogEntry;
export type LogSnapshotState = string & GeneratedLogSnapshotState;

export const decodeLogEntryId = Schema.decodeUnknownSync(LogEntryIdSchema);
export const decodeLogMessage = Schema.decodeUnknownSync(LogMessageSchema);
export const decodeLogTimestamp = Schema.decodeUnknownSync(LogTimestampSchema);
export const decodeStackTrace = Schema.decodeUnknownSync(StackTraceSchema);

export const LogLevel = {
  Trace: LogLevelSchema.parse(GeneratedLogLevel.Trace),
  Debug: LogLevelSchema.parse(GeneratedLogLevel.Debug),
  Info: LogLevelSchema.parse(GeneratedLogLevel.Info),
  Warn: LogLevelSchema.parse(GeneratedLogLevel.Warn),
  Error: LogLevelSchema.parse(GeneratedLogLevel.Error),
} as const;

export const LogSource = {
  AgentService: LogSourceSchema.parse(GeneratedLogSource.AgentService),
  DevServer: LogSourceSchema.parse(GeneratedLogSource.DevServer),
  LocalApi: LogSourceSchema.parse(GeneratedLogSource.LocalApi),
  Portal: LogSourceSchema.parse(GeneratedLogSource.Portal),
  Codex: LogSourceSchema.parse(GeneratedLogSource.Codex),
  Validation: LogSourceSchema.parse(GeneratedLogSource.Validation),
  RustTest: LogSourceSchema.parse(GeneratedLogSource.RustTest),
} as const;
