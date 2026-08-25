import { RunType, type RunType as RunTypeValue, type TestLogScope, type TestSuiteType } from '../test-log/types';
import {
  parseBridgeLifecycleOperatorState,
  parseBridgeRunCounter,
  parseBridgeRunInfo,
  parseBridgeRunStart,
} from './bridgeLifecycleStateParsing';

export const BridgeLifecycleSchemaVersion = 1;
export const MaximumTrackedBridgeRuns = 64;

export interface BridgeRunInfoState {
  readonly runId: string | null;
  readonly runType: RunTypeValue;
  readonly suiteType: TestSuiteType | null;
  readonly scope: TestLogScope | null;
  readonly startedAt: number | null;
}

export interface BridgeRunStartState {
  readonly runId: string;
  readonly runType: RunTypeValue;
  readonly suiteType: TestSuiteType | null;
  readonly scope: TestLogScope;
  readonly filePath: string | null;
  readonly wipeAll: boolean;
}

export interface BridgeRunCounter {
  readonly runId: string;
  readonly stored: number;
  readonly flushed: number;
  readonly updatedAt: number;
}

export interface BridgeLifecycleOperatorState {
  readonly status: 'manual-required';
  readonly code: 'invalid-pending-start-selector' | 'invalid-lifecycle-record';
  readonly observedAt: number;
  readonly recordSha256: string;
}

export interface PersistedBridgeLifecycleState {
  readonly schemaVersion: typeof BridgeLifecycleSchemaVersion;
  readonly activeRun: BridgeRunInfoState;
  readonly pendingStart: BridgeRunStartState | null;
  readonly runCounters: readonly BridgeRunCounter[];
  readonly operatorState: BridgeLifecycleOperatorState | null;
}

function invalid(): never {
  throw new Error('invalid bridge lifecycle state');
}

export function emptyBridgeLifecycleState(): PersistedBridgeLifecycleState {
  return {
    schemaVersion: BridgeLifecycleSchemaVersion,
    activeRun: { runId: null, runType: RunType.Single, suiteType: null, scope: null, startedAt: null },
    pendingStart: null,
    runCounters: [],
    operatorState: null,
  };
}

export function parseBridgeLifecycleState(value: unknown): PersistedBridgeLifecycleState {
  if (typeof value !== 'object' || value == null || Array.isArray(value)) {
    invalid();
  }
  const input = value as Record<string, unknown>;
  if (input['schemaVersion'] !== BridgeLifecycleSchemaVersion || !Array.isArray(input['runCounters'])) {
    invalid();
  }
  const runCounters = input['runCounters'].map(parseBridgeRunCounter);
  if (
    runCounters.length > MaximumTrackedBridgeRuns ||
    new Set(runCounters.map((item) => item.runId)).size !== runCounters.length
  ) {
    invalid();
  }
  return {
    schemaVersion: BridgeLifecycleSchemaVersion,
    activeRun: parseBridgeRunInfo(input['activeRun']),
    pendingStart: input['pendingStart'] == null ? null : parseBridgeRunStart(input['pendingStart']),
    runCounters,
    operatorState: input['operatorState'] == null ? null : parseBridgeLifecycleOperatorState(input['operatorState']),
  };
}
