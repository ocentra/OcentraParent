import { LogLevelSchema, type LogLevel } from '../logging-contracts';
import {
  createRuntimeSchema,
  parseLiteral,
  parseNonEmptyString,
  parseNonNegativeInteger,
  parseNullableInteger,
  parseNullableString,
  parseRecord,
  parseStringArray,
} from '../contracts/runtime-schema';

export const TestLogSchemaVersion = 1;

const TestLogScopeValues = [
  'parent-agent',
  'parent-portal',
  'parent-cloudflare',
  'parent-codex',
  'parent-test',
] as const;
const RunTypeValues = ['single', 'full', 'single-pool', 'single-threads'] as const;
const TestSuiteTypeValues = ['unit', 'integration', 'e2e', 'contract', 'websocket'] as const;
const TestLogOriginValues = ['test', 'worker', 'portal', 'agent-service', 'codex'] as const;

export type TestLogScope = (typeof TestLogScopeValues)[number];
export type RunType = (typeof RunTypeValues)[number];
export type TestSuiteType = (typeof TestSuiteTypeValues)[number];
export type TestLogOrigin = (typeof TestLogOriginValues)[number];

export interface StoredTestLogLine {
  readonly schemaVersion: typeof TestLogSchemaVersion;
  readonly type: 'log';
  readonly scope: TestLogScope;
  readonly runId: string;
  readonly runType: RunType;
  readonly suiteType: TestSuiteType | null;
  readonly testName: string;
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
  readonly tags: readonly string[];
  readonly stack: string | null;
  readonly origin: TestLogOrigin | null;
  readonly environment: string | null;
}

export interface TestLogStats {
  readonly totalLogs: number;
  readonly errorLogs: number;
  readonly warnLogs: number;
  readonly distinctRuns: number;
  readonly distinctTests: number;
  readonly newestTimestamp: number | null;
}

export const TestLogScopeSchema = createRuntimeSchema((input) =>
  parseLiteral(input, TestLogScopeValues, 'test log scope')
);
export const RunTypeSchema = createRuntimeSchema((input) => parseLiteral(input, RunTypeValues, 'run type'));
export const TestSuiteTypeSchema = createRuntimeSchema((input) =>
  parseLiteral(input, TestSuiteTypeValues, 'test suite type')
);
export const TestLogOriginSchema = createRuntimeSchema((input) =>
  parseLiteral(input, TestLogOriginValues, 'test log origin')
);

export const NullableStringSchema = createRuntimeSchema((input) => parseNullableString(input, 'nullable string'));
export const NullableIntegerSchema = createRuntimeSchema((input) => parseNullableInteger(input, 'nullable integer'));
export const StringArraySchema = createRuntimeSchema((input) => parseStringArray(input, 'string array'));

export const StoredTestLogLineSchema = createRuntimeSchema((input): StoredTestLogLine => {
  const record = parseRecord(input, 'stored test log line');
  return {
    schemaVersion: parseLiteral(record.schemaVersion, [TestLogSchemaVersion], 'schemaVersion'),
    type: parseLiteral(record.type, ['log'], 'type'),
    scope: TestLogScopeSchema.parse(record.scope),
    runId: parseNonEmptyString(record.runId, 'runId'),
    runType: RunTypeSchema.parse(record.runType),
    suiteType: record.suiteType === null ? null : TestSuiteTypeSchema.parse(record.suiteType),
    testName: parseNonEmptyString(record.testName, 'testName'),
    timestamp: parseNonNegativeInteger(record.timestamp, 'timestamp'),
    level: LogLevelSchema.parse(record.level),
    source: NullableStringSchema.parse(record.source),
    context: NullableStringSchema.parse(record.context),
    message: parseNonEmptyString(record.message, 'message'),
    data: NullableStringSchema.parse(record.data),
    file: NullableStringSchema.parse(record.file),
    filePath: NullableStringSchema.parse(record.filePath),
    line: NullableIntegerSchema.parse(record.line),
    column: NullableIntegerSchema.parse(record.column),
    correlationId: NullableStringSchema.parse(record.correlationId),
    tags: StringArraySchema.parse(record.tags ?? []),
    stack: NullableStringSchema.parse(record.stack),
    origin: record.origin === null || record.origin === undefined ? null : TestLogOriginSchema.parse(record.origin),
    environment: NullableStringSchema.parse(record.environment),
  };
});

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

export function parseTestLogScopeOrDefault(
  value: string | null | undefined,
  fallback: TestLogScope = TestLogScope.ParentTest
): TestLogScope {
  return value == null || value.trim().length === 0 ? fallback : TestLogScopeSchema.parse(value);
}

export function parseRunTypeOrDefault(value: string | null | undefined, fallback: RunType = RunType.Single): RunType {
  return value == null || value.trim().length === 0 ? fallback : RunTypeSchema.parse(value);
}

export function parseSuiteTypeOrNull(value: string | null | undefined): TestSuiteType | null {
  return value == null || value.trim().length === 0 ? null : TestSuiteTypeSchema.parse(value);
}
