import { RunTypeSchema, TestLogScopeSchema, TestSuiteTypeSchema } from '../test-log/types';
import type { BridgeRunCounter, BridgeRunInfoState, BridgeRunStartState } from './bridgeLifecycleStateCodec';

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
  requireValid(value == null || (typeof value === 'string' && value.length <= 4_096));
  return value == null ? null : (value as string);
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
