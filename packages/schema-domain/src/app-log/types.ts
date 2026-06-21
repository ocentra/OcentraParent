import {
  type Infer,
  NonEmptyStringSchema,
  Schema,
  withParser,
} from '../effect';
import { LogLevelSchema } from '../logging-contracts';
import { NullableIntegerSchema, NullableStringSchema, TestLogScopeSchema } from '../test-log/types';

export const AppLogSchemaVersion = 1;

export const AppLogEntrySchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppLogSchemaVersion),
    sessionId: NonEmptyStringSchema,
    scope: TestLogScopeSchema,
    timestamp: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    level: LogLevelSchema,
    source: NullableStringSchema,
    context: NullableStringSchema,
    message: NonEmptyStringSchema,
    data: Schema.optionalWith(NullableStringSchema, { default: () => null }),
    file: Schema.optionalWith(NullableStringSchema, { default: () => null }),
    filePath: Schema.optionalWith(NullableStringSchema, { default: () => null }),
    line: Schema.optionalWith(NullableIntegerSchema, { default: () => null }),
    column: Schema.optionalWith(NullableIntegerSchema, { default: () => null }),
    correlationId: Schema.optionalWith(NullableStringSchema, { default: () => null }),
    environment: Schema.optionalWith(NullableStringSchema, { default: () => null }),
  })
);

export interface AppLogQuery {
  readonly level?: Infer<typeof LogLevelSchema>;
  readonly limit?: number;
  readonly search?: string;
}

export interface AppLogStats {
  readonly totalLogs: number;
  readonly sessions: number;
  readonly newestTimestamp: number | null;
}

export type AppLogEntry = Infer<typeof AppLogEntrySchema>;
