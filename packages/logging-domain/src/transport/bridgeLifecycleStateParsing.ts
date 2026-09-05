import { RunTypeSchema, TestLogScopeSchema, TestSuiteTypeSchema } from '../test-log/types';
import type {
  BridgeLifecycleOperatorState,
  BridgeRunCounter,
  BridgeRunInfoState,
  BridgeRunStartState,
} from './bridgeLifecycleStateCodec';
import { normalizeWipeFileSelector } from '../test-log/wipeFileSelector';

function invalid(): never {
  throw new Error('invalid bridge lifecycle state');
}

function requireValid(condition: boolean): void {
  if (!condition) {
    invalid();
  }
}

function record(value: unknown): Record<string, unknown> {
  return typeof value === 'object' && value != null && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : invalid();
}

function requiredRunId(value: unknown): string {
  requireValid(typeof value === 'string' && value.trim().length > 0 && value.length <= 256);
  return value as string;
}

function nonNegativeInteger(value: unknown): number {
  requireValid(Number.isSafeInteger(value) && (value as number) >= 0);
  return value as number;
}

function optional<T>(value: unknown, parse: (input: unknown) => T): T | null {
  return value == null ? null : parse(value);
}

function optionalFilePath(value: unknown): string | null {
  requireValid(value == null || typeof value === 'string');
  try {
    return value == null ? null : normalizeWipeFileSelector(value as string);
  } catch {
    return invalid();
  }
}

export function parseBridgeRunStart(value: unknown): BridgeRunStartState {
  const input = record(value);
  requireValid(typeof input['wipeAll'] === 'boolean');
  return {
    runId: requiredRunId(input['runId']),
    runType: RunTypeSchema.parse(input['runType']),
    suiteType: optional(input['suiteType'], TestSuiteTypeSchema.parse),
    scope: TestLogScopeSchema.parse(input['scope']),
    filePath: optionalFilePath(input['filePath']),
    wipeAll: input['wipeAll'] as boolean,
  };
}

export function parseBridgeRunInfo(value: unknown): BridgeRunInfoState {
  const input = record(value);
  const parsed: BridgeRunInfoState = {
    runId: optional(input['runId'], requiredRunId),
    runType: RunTypeSchema.parse(input['runType']),
    suiteType: optional(input['suiteType'], TestSuiteTypeSchema.parse),
    scope: optional(input['scope'], TestLogScopeSchema.parse),
    startedAt: optional(input['startedAt'], nonNegativeInteger),
  };
  const emptyState =
    parsed.runId == null && parsed.suiteType == null && parsed.scope == null && parsed.startedAt == null;
  const activeState = parsed.runId != null && parsed.scope != null && parsed.startedAt != null;
  requireValid(emptyState || activeState);
  return parsed;
}

export function parseBridgeRunCounter(value: unknown): BridgeRunCounter {
  const input = record(value);
  const stored = nonNegativeInteger(input['stored']);
  const flushed = nonNegativeInteger(input['flushed']);
  requireValid(flushed <= stored);
  return {
    runId: requiredRunId(input['runId']),
    stored,
    flushed,
    updatedAt: nonNegativeInteger(input['updatedAt']),
  };
}

export function parseBridgeLifecycleOperatorState(value: unknown): BridgeLifecycleOperatorState {
  const input = record(value);
  requireValid(
    input['status'] === 'manual-required' &&
      (input['code'] === 'invalid-pending-start-selector' || input['code'] === 'invalid-lifecycle-record') &&
      typeof input['recordSha256'] === 'string' &&
      /^[0-9a-f]{64}$/u.test(input['recordSha256'])
  );
  return {
    status: 'manual-required',
    code: input['code'] as BridgeLifecycleOperatorState['code'],
    observedAt: nonNegativeInteger(input['observedAt']),
    recordSha256: input['recordSha256'] as string,
  };
}
