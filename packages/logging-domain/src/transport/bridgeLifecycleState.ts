import path from 'node:path';
import type { BridgeEntry } from './bridgeLogPayload';
import {
  MaximumTrackedBridgeRuns,
  emptyBridgeLifecycleState,
  parseBridgeLifecycleState,
  type BridgeRunCounter,
  type BridgeRunInfoState,
  type BridgeRunStartState,
  type PersistedBridgeLifecycleState,
} from './bridgeLifecycleStateCodec';
import { durableReplaceLocalArtifact, readLocalArtifactText } from '../local-artifact-file';
import { withLocalArtifactLock } from '../local-artifact-lock';
import type { LocalArtifactMutation } from '../local-artifact-transaction';

const MaximumLifecycleBytes = 256 * 1024;

export interface BridgeFlushState {
  readonly runId: string;
  readonly flushed: number;
  readonly stored: number;
}

export const BridgeLifecycleConflictError = new Error('bridge lifecycle conflict');

function lifecyclePath(rootDir: string): string {
  return path.join(rootDir, '.bridge', 'lifecycle-state.json');
}

function loadState(rootDir: string): PersistedBridgeLifecycleState {
  const content = readLocalArtifactText(lifecyclePath(rootDir), rootDir, MaximumLifecycleBytes);
  if (content == null) {
    return emptyBridgeLifecycleState();
  }
  try {
    return parseBridgeLifecycleState(JSON.parse(content) as unknown);
  } catch {
    throw new Error('invalid bridge lifecycle state');
  }
}

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

function serializeState(state: PersistedBridgeLifecycleState): string {
  return `${JSON.stringify(parseBridgeLifecycleState(state))}\n`;
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
  const filePath = lifecyclePath(rootDir);
  const state = loadState(rootDir);
  const next = { ...state, runCounters: reconcileCounters(state, remainingCounts, affectedRunIds) };
  return { kind: 'replace', filePath, payload: serializeState(next) };
}

export function bridgeLifecycleClearCountersMutation(rootDir: string): LocalArtifactMutation {
  const state = loadState(rootDir);
  return {
    kind: 'replace',
    filePath: lifecyclePath(rootDir),
    payload: serializeState({ ...state, runCounters: [] }),
  };
}

export class BridgeLifecycleStateStore {
  private readonly rootDir: string;
  private readonly filePath: string;

  constructor(rootDir: string) {
    this.rootDir = rootDir;
    this.filePath = lifecyclePath(rootDir);
    loadState(rootDir);
  }

  runInfo(): BridgeRunInfoState {
    return { ...loadState(this.rootDir).activeRun };
  }

  pendingStart(): BridgeRunStartState | null {
    const pending = loadState(this.rootDir).pendingStart;
    return pending == null ? null : { ...pending };
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
      const nextState = parseBridgeLifecycleState(update(loadState(this.rootDir)));
      durableReplaceLocalArtifact(this.filePath, serializeState(nextState), this.rootDir);
    });
  }
}
