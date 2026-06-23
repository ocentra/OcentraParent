import { type Infer, NonEmptyStringSchema, Schema, withParser } from '../effect';
import { LogLevelSchema } from '../logging-contracts';

export const TestLogSchemaVersion = 1;

export const TestLogScopeSchema = withParser(
  Schema.Literal('parent-agent', 'parent-portal', 'parent-cloudflare', 'parent-codex', 'parent-test')
);

export const RunTypeSchema = withParser(Schema.Literal('single', 'full', 'single-pool', 'single-threads'));

export const TestSuiteTypeSchema = withParser(Schema.Literal('unit', 'integration', 'e2e', 'contract', 'websocket'));

export const TestLogOriginSchema = withParser(Schema.Literal('test', 'worker', 'portal', 'agent-service', 'codex'));

export const NullableStringSchema = Schema.NullOr(Schema.String);
export const NullableIntegerSchema = Schema.NullOr(Schema.Number.pipe(Schema.int()));
export const StringArraySchema = Schema.Array(Schema.String);

export const StoredTestLogLineSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TestLogSchemaVersion),
    type: Schema.Literal('log'),
    scope: TestLogScopeSchema,
    runId: NonEmptyStringSchema,
    runType: RunTypeSchema,
    suiteType: Schema.NullOr(TestSuiteTypeSchema),
    testName: NonEmptyStringSchema,
    timestamp: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    level: LogLevelSchema,
    source: NullableStringSchema,
    context: NullableStringSchema,
    message: NonEmptyStringSchema,
    data: NullableStringSchema,
    file: NullableStringSchema,
    filePath: NullableStringSchema,
    line: NullableIntegerSchema,
    column: NullableIntegerSchema,
    correlationId: NullableStringSchema,
    tags: Schema.optionalWith(StringArraySchema, { default: () => [] }),
    stack: NullableStringSchema,
    origin: Schema.optionalWith(Schema.NullOr(TestLogOriginSchema), { default: () => null }),
    environment: Schema.optionalWith(NullableStringSchema, { default: () => null }),
  })
);

export const TestLogScope = {
  ParentAgent: TestLogScopeSchema.parse('parent-agent'),
  ParentPortal: TestLogScopeSchema.parse('parent-portal'),
  ParentCloudflare: TestLogScopeSchema.parse('parent-cloudflare'),
  ParentCodex: TestLogScopeSchema.parse('parent-codex'),
  ParentTest: TestLogScopeSchema.parse('parent-test'),
} as const;

export const RunType = {
  Single: RunTypeSchema.parse('single'),
  Full: RunTypeSchema.parse('full'),
  SinglePool: RunTypeSchema.parse('single-pool'),
  SingleThreads: RunTypeSchema.parse('single-threads'),
} as const;

export const TestSuiteType = {
  Unit: TestSuiteTypeSchema.parse('unit'),
  Integration: TestSuiteTypeSchema.parse('integration'),
  E2E: TestSuiteTypeSchema.parse('e2e'),
  Contract: TestSuiteTypeSchema.parse('contract'),
  Websocket: TestSuiteTypeSchema.parse('websocket'),
} as const;

export const TestLogOrigin = {
  Test: TestLogOriginSchema.parse('test'),
  Worker: TestLogOriginSchema.parse('worker'),
  Portal: TestLogOriginSchema.parse('portal'),
  AgentService: TestLogOriginSchema.parse('agent-service'),
  Codex: TestLogOriginSchema.parse('codex'),
} as const;

export type TestLogScope = Infer<typeof TestLogScopeSchema>;
export type RunType = Infer<typeof RunTypeSchema>;
export type TestSuiteType = Infer<typeof TestSuiteTypeSchema>;
export type TestLogOrigin = Infer<typeof TestLogOriginSchema>;
export type StoredTestLogLine = Infer<typeof StoredTestLogLineSchema>;

export interface TestLogStats {
  readonly totalLogs: number;
  readonly errorLogs: number;
  readonly warnLogs: number;
  readonly distinctRuns: number;
  readonly distinctTests: number;
  readonly newestTimestamp: number | null;
}

export function parseTestLogScopeOrDefault(
  value: string | null | undefined,
  fallback: TestLogScope = TestLogScope.ParentTest
): TestLogScope {
  if (value == null || value.trim().length === 0) {
    return fallback;
  }
  return TestLogScopeSchema.parse(value);
}

export function parseRunTypeOrDefault(value: string | null | undefined, fallback: RunType = RunType.Single): RunType {
  if (value == null || value.trim().length === 0) {
    return fallback;
  }
  return RunTypeSchema.parse(value);
}

export function parseSuiteTypeOrNull(value: string | null | undefined): TestSuiteType | null {
  if (value == null || value.trim().length === 0) {
    return null;
  }
  return TestSuiteTypeSchema.parse(value);
}
