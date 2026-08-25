import type { BridgeEntry } from './bridgeLogPayload';
import {
  MaximumTrackedBridgeRuns,
  parseBridgeLifecycleState,
  type BridgeRunCounter,
  type BridgeLifecycleOperatorState,
  type BridgeRunInfoState,
  type BridgeRunStartState,
  type PersistedBridgeLifecycleState,
} from './bridgeLifecycleStateCodec';
import { withLocalArtifactLock } from '../local-artifact-lock';
import type { LocalArtifactMutation } from '../local-artifact-transaction';
import {
  bridgeLifecyclePath,
  loadBridgeLifecycleState,
  replaceBridgeLifecycleState,
  serializeBridgeLifecycleState,
} from './bridgeLifecycleRecovery';

export interface BridgeFlushState {
  readonly runId: string;
  readonly flushed: number;
  readonly stored: number;
}

export const BridgeLifecycleConflictError = new Error('bridge lifecycle conflict');
export const BridgeLifecycleManualRequiredError = new Error('bridge lifecycle requires operator resolution');

function upsertCounter(
  counters: readonly BridgeRunCounter[],
  runId: string,
  update: (counter: BridgeRunCounter) => BridgeRunCounter
): BridgeRunCounter[] {
  const current = counters.find((counter) => counter.runId === runId) ?? {
    runId,
    stored: 0,
    flushed: 0,
    updatedAt: 0,
  };
  return [...counters.filter((counter) => counter.runId !== runId), update(current)]
    .sort((left, right) => left.updatedAt - right.updatedAt)
    .slice(-MaximumTrackedBridgeRuns);
}

function reconcileCounters(
  state: PersistedBridgeLifecycleState,
  remainingCounts: ReadonlyMap<string, number>,
  affectedRunIds: ReadonlySet<string>
): BridgeRunCounter[] {
  const now = Date.now();
  let counters = state.runCounters.filter((counter) => !affectedRunIds.has(counter.runId));
  for (const runId of affectedRunIds) {
    const stored = remainingCounts.get(runId) ?? 0;
    const mustRemain = state.activeRun.runId === runId || state.pendingStart?.runId === runId || stored > 0;
    if (mustRemain) {
      counters = upsertCounter(counters, runId, () => ({ runId, stored, flushed: 0, updatedAt: now }));
    }
  }
  return counters;
}

export function bridgeLifecycleReconciliationMutation(
  rootDir: string,
  remainingCounts: ReadonlyMap<string, number>,
  affectedRunIds: ReadonlySet<string>
): LocalArtifactMutation | null {
  if (affectedRunIds.size === 0) {
    return null;
  }
  const filePath = bridgeLifecyclePath(rootDir);
  const state = loadBridgeLifecycleState(rootDir);
  const next = { ...state, runCounters: reconcileCounters(state, remainingCounts, affectedRunIds) };
  return { kind: 'replace', filePath, payload: serializeBridgeLifecycleState(next) };
}

export function bridgeLifecycleClearCountersMutation(rootDir: string): LocalArtifactMutation {
  const state = loadBridgeLifecycleState(rootDir);
  return {
    kind: 'replace',
    filePath: bridgeLifecyclePath(rootDir),
    payload: serializeBridgeLifecycleState({ ...state, runCounters: [] }),
  };
}

export class BridgeLifecycleStateStore {
  private readonly rootDir: string;

  constructor(rootDir: string) {
    this.rootDir = rootDir;
    loadBridgeLifecycleState(rootDir);
  }

  runInfo(): BridgeRunInfoState {
    return { ...loadBridgeLifecycleState(this.rootDir).activeRun };
  }

  pendingStart(): BridgeRunStartState | null {
    const pending = loadBridgeLifecycleState(this.rootDir).pendingStart;
    return pending == null ? null : { ...pending };
  }

  operatorState(): BridgeLifecycleOperatorState | null {
    const state = loadBridgeLifecycleState(this.rootDir).operatorState;
    return state == null ? null : { ...state };
  }

  prepareStart(run: BridgeRunStartState): void {
    this.mutate((state) => ({ ...state, pendingStart: run }));
  }

  completeStart(): void {
    this.mutate((state) => {
      const run = state.pendingStart;
      if (run == null) {
        throw BridgeLifecycleConflictError;
      }
      const now = Date.now();
      return {
        ...state,
        activeRun: {
          runId: run.runId,
          runType: run.runType,
          suiteType: run.suiteType,
          scope: run.scope,
          startedAt: now,
        },
        pendingStart: null,
        runCounters: upsertCounter(state.runCounters, run.runId, () => ({
          runId: run.runId,
          stored: 0,
          flushed: 0,
          updatedAt: now,
        })),
      };
    });
  }

  recordStored(entries: readonly BridgeEntry[]): void {
    this.mutate((state) => {
      const counts = new Map<string, number>();
      for (const entry of entries) {
        counts.set(entry.runId, (counts.get(entry.runId) ?? 0) + 1);
      }
      let runCounters = [...state.runCounters];
      for (const [runId, count] of counts) {
        runCounters = upsertCounter(runCounters, runId, (counter) => ({
          ...counter,
          stored: counter.stored + count,
          updatedAt: Date.now(),
        }));
      }
      return { ...state, runCounters };
    });
  }

  flush(runId: string, actualStored?: number): BridgeFlushState {
    let result: BridgeFlushState | null = null;
    this.mutate((state) => {
      const counter = state.runCounters.find((candidate) => candidate.runId === runId);
      const stored = actualStored ?? counter?.stored;
      const known = counter != null || state.activeRun.runId === runId || (stored ?? 0) > 0;
      if (!known || stored == null || !Number.isSafeInteger(stored) || stored < 0) {
        throw BridgeLifecycleConflictError;
      }
      const previouslyFlushed = Math.min(counter?.flushed ?? 0, stored);
      result = { runId, flushed: stored - previouslyFlushed, stored };
      return {
        ...state,
        runCounters: upsertCounter(state.runCounters, runId, () => ({
          runId,
          stored,
          flushed: stored,
          updatedAt: Date.now(),
        })),
      };
    });
    if (result == null) {
      throw new Error('bridge lifecycle flush was not committed');
    }
    return result;
  }

  private mutate(update: (state: PersistedBridgeLifecycleState) => PersistedBridgeLifecycleState): void {
    withLocalArtifactLock(this.rootDir, () => {
      const current = loadBridgeLifecycleState(this.rootDir);
      if (current.operatorState != null) {
        throw BridgeLifecycleManualRequiredError;
      }
      const nextState = parseBridgeLifecycleState(update(current));
      replaceBridgeLifecycleState(this.rootDir, nextState);
    });
  }
}
