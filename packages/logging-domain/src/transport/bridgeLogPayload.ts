import {
  type Infer,
  NonEmptyStringSchema,
  Schema,
  withParser,
} from '@ocentra-parent/schema-domain/effect';
import { LogLevelSchema } from '../contracts';
import {
  NullableIntegerSchema,
  NullableStringSchema,
  RunType,
  RunTypeSchema,
  TestLogScope,
  TestLogScopeSchema,
  TestLogSchemaVersion,
  TestLogOriginSchema,
  TestSuiteTypeSchema,
  type StoredTestLogLine,
} from '../test-log/types';

export const BridgeLogPayloadSchema = withParser(
  Schema.Struct({
    log_timestamp: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    level: LogLevelSchema,
    source: NullableStringSchema,
    context: NullableStringSchema,
    message: NonEmptyStringSchema,
    data: Schema.optionalWith(NullableStringSchema, { default: () => null }),
    file: Schema.optionalWith(NullableStringSchema, { default: () => null }),
    file_path: Schema.optionalWith(NullableStringSchema, { default: () => null }),
    line: Schema.optionalWith(NullableIntegerSchema, { default: () => null }),
    column: Schema.optionalWith(NullableIntegerSchema, { default: () => null }),
    correlation_id: Schema.optionalWith(NullableStringSchema, { default: () => null }),
    tags: Schema.optionalWith(Schema.Array(Schema.String), { default: () => [] }),
    stack: Schema.optionalWith(NullableStringSchema, { default: () => null }),
    suite_type: Schema.optionalWith(Schema.NullOr(TestSuiteTypeSchema), { default: () => null }),
    origin: Schema.optionalWith(Schema.NullOr(TestLogOriginSchema), { default: () => null }),
    environment: Schema.optionalWith(NullableStringSchema, { default: () => null }),
  })
);

export const BridgeEntrySchema = withParser(
  Schema.Struct({
    testName: NonEmptyStringSchema,
    runId: NonEmptyStringSchema,
    log: BridgeLogPayloadSchema,
    consumer: Schema.optionalWith(Schema.NullOr(TestLogScopeSchema), { default: () => null }),
    runType: Schema.optionalWith(RunTypeSchema, { default: () => RunType.Single }),
  })
);

export const BridgeEntryArraySchema = withParser(Schema.Array(BridgeEntrySchema));

export type BridgeLogPayload = Infer<typeof BridgeLogPayloadSchema>;
export type BridgeEntry = Infer<typeof BridgeEntrySchema>;

export function bridgeEntryToStoredLog(entry: BridgeEntry): StoredTestLogLine {
  return {
    schemaVersion: TestLogSchemaVersion,
    type: 'log',
    scope: entry.consumer ?? TestLogScope.ParentTest,
    runId: entry.runId,
    runType: entry.runType,
    suiteType: entry.log.suite_type,
    testName: entry.testName,
    timestamp: entry.log.log_timestamp,
    level: entry.log.level,
    source: entry.log.source,
    context: entry.log.context,
    message: entry.log.message,
    data: entry.log.data,
    file: entry.log.file,
    filePath: entry.log.file_path,
    line: entry.log.line,
    column: entry.log.column,
    correlationId: entry.log.correlation_id,
    tags: entry.log.tags,
    stack: entry.log.stack,
    origin: entry.log.origin,
    environment: entry.log.environment,
  };
}
