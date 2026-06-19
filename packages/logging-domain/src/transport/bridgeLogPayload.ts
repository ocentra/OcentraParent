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
  TestLogScopeSchema,
  TestLogOriginSchema,
  TestSuiteTypeSchema,
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
