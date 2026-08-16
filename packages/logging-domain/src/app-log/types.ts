import { LogLevelSchema, type LogLevel } from '../logging-contracts';
import {
  createRuntimeSchema,
  parseNonEmptyString,
  parseNonNegativeInteger,
  parseNullableInteger,
  parseNullableString,
  parseRecord,
} from '../contracts/runtime-schema';
import { TestLogScopeSchema, type TestLogScope } from '../test-log/types';

export const AppLogSchemaVersion = 1;

export interface AppLogEntry {
  readonly schemaVersion: typeof AppLogSchemaVersion;
  readonly sessionId: string;
  readonly scope: TestLogScope;
  readonly timestamp: number;
  readonly level: LogLevel;
  readonly source: string | null;
  readonly context: string | null;
  readonly message: string;
  readonly data: string | null;
  readonly file: string | null;
  readonly filePath: string | null;
  readonly line: number | null;
  readonly column: number | null;
  readonly correlationId: string | null;
  readonly environment: string | null;
}

export interface AppLogQuery {
  readonly level?: LogLevel;
  readonly search?: string;
  readonly limit?: number;
}

export interface AppLogStats {
  readonly totalLogs: number;
  readonly sessions: number;
  readonly newestTimestamp: number | null;
}

export const AppLogEntrySchema = createRuntimeSchema((input): AppLogEntry => {
  const record = parseRecord(input, 'app log entry');
  return {
    schemaVersion: record.schemaVersion === AppLogSchemaVersion ? AppLogSchemaVersion : failSchemaVersion(),
    sessionId: parseNonEmptyString(record.sessionId, 'sessionId'),
    scope: TestLogScopeSchema.parse(record.scope),
    timestamp: parseNonNegativeInteger(record.timestamp, 'timestamp'),
    level: LogLevelSchema.parse(record.level),
    source: NullableStringSchema.parse(record.source),
    context: NullableStringSchema.parse(record.context),
    message: parseNonEmptyString(record.message, 'message'),
    data: NullableStringSchema.parse(record.data ?? null),
    file: NullableStringSchema.parse(record.file ?? null),
    filePath: NullableStringSchema.parse(record.filePath ?? null),
    line: NullableIntegerSchema.parse(record.line ?? null),
    column: NullableIntegerSchema.parse(record.column ?? null),
    correlationId: NullableStringSchema.parse(record.correlationId ?? null),
    environment: NullableStringSchema.parse(record.environment ?? null),
  };
});

const NullableStringSchema = createRuntimeSchema((input) => parseNullableString(input, 'nullable string'));
const NullableIntegerSchema = createRuntimeSchema((input) => parseNullableInteger(input, 'nullable integer'));

function failSchemaVersion(): never {
  throw new Error('schemaVersion must be 1');
}
