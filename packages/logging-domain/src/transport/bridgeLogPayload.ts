import { LogLevelSchema, type LogLevel } from '../logging-contracts';
import {
  createRuntimeSchema,
  parseNonEmptyString,
  parseNonNegativeInteger,
  parseNullableInteger,
  parseNullableString,
  parseRecord,
  parseStringArray,
} from '../contracts/runtime-schema';
import {
  RunType,
  RunTypeSchema,
  TestLogOriginSchema,
  TestLogScopeSchema,
  TestSuiteTypeSchema,
  type RunType as RunTypeValue,
  type TestLogOrigin,
  type TestLogScope,
  type TestSuiteType,
} from '../test-log/types';

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
  readonly runType: RunTypeValue;
}

export const BridgeLogPayloadSchema = createRuntimeSchema((input): BridgeLogPayload => {
  const record = parseRecord(input, 'bridge log payload');
  return {
    log_timestamp: parseNonNegativeInteger(record.log_timestamp, 'log_timestamp'),
    level: LogLevelSchema.parse(record.level),
    source: NullableStringSchema.parse(record.source),
    context: NullableStringSchema.parse(record.context),
    message: parseNonEmptyString(record.message, 'message'),
    data: NullableStringSchema.parse(record.data ?? null),
    file: NullableStringSchema.parse(record.file ?? null),
    file_path: NullableStringSchema.parse(record.file_path ?? null),
    line: NullableIntegerSchema.parse(record.line ?? null),
    column: NullableIntegerSchema.parse(record.column ?? null),
    correlation_id: NullableStringSchema.parse(record.correlation_id ?? null),
    tags: StringArraySchema.parse(record.tags ?? []),
    stack: NullableStringSchema.parse(record.stack ?? null),
    suite_type: record.suite_type == null ? null : TestSuiteTypeSchema.parse(record.suite_type),
    origin: record.origin == null ? null : TestLogOriginSchema.parse(record.origin),
    environment: NullableStringSchema.parse(record.environment ?? null),
  };
});

export const BridgeEntrySchema = createRuntimeSchema((input): BridgeEntry => {
  const record = parseRecord(input, 'bridge entry');
  return {
    testName: parseNonEmptyString(record.testName, 'testName'),
    runId: parseNonEmptyString(record.runId, 'runId'),
    log: BridgeLogPayloadSchema.parse(record.log),
    consumer: record.consumer == null ? null : TestLogScopeSchema.parse(record.consumer),
    runType: record.runType == null ? RunType.Single : RunTypeSchema.parse(record.runType),
  };
});

export const BridgeEntryArraySchema = createRuntimeSchema((input): readonly BridgeEntry[] => {
  if (!Array.isArray(input)) {
    throw new Error('bridge entries must be an array');
  }
  return input.map((entry) => BridgeEntrySchema.parse(entry));
});

const NullableStringSchema = createRuntimeSchema((input) => parseNullableString(input, 'nullable string'));
const NullableIntegerSchema = createRuntimeSchema((input) => parseNullableInteger(input, 'nullable integer'));
const StringArraySchema = createRuntimeSchema((input) => parseStringArray(input, 'string array'));
