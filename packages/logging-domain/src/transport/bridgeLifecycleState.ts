import fs from 'node:fs';
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
import { assertReadableLocalArtifactFile, durableReplaceLocalArtifact } from '../local-artifact-file';

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

function loadState(filePath: string): PersistedBridgeLifecycleState {
  if (!assertReadableLocalArtifactFile(filePath)) {
    return emptyBridgeLifecycleState();
  }
  if (fs.statSync(filePath).size > MaximumLifecycleBytes) {
    throw new Error('invalid bridge lifecycle state');
  }
  try {
    return parseBridgeLifecycleState(JSON.parse(fs.readFileSync(filePath, 'utf8')) as unknown);
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

export class BridgeLifecycleStateStore {
  private state: PersistedBridgeLifecycleState;
  private readonly filePath: string;

  constructor(rootDir: string) {
    this.filePath = lifecyclePath(rootDir);
    this.state = loadState(this.filePath);
  }

  runInfo(): BridgeRunInfoState {
    return { ...this.state.activeRun };
  }

  pendingStart(): BridgeRunStartState | null {
    return this.state.pendingStart == null ? null : { ...this.state.pendingStart };
  }

  prepareStart(run: BridgeRunStartState): void {
    this.commit({ ...this.state, pendingStart: run });
  }

  completeStart(): void {
    const run = this.state.pendingStart;
    if (run == null) {
      throw BridgeLifecycleConflictError;
    }
    const now = Date.now();
    this.commit({
      ...this.state,
      activeRun: {
        runId: run.runId,
        runType: run.runType,
        suiteType: run.suiteType,
        scope: run.scope,
        startedAt: now,
      },
      pendingStart: null,
      runCounters: upsertCounter(this.state.runCounters, run.runId, () => ({
        runId: run.runId,
        stored: 0,
        flushed: 0,
        updatedAt: now,
      })),
    });
  }

  recordStored(entries: readonly BridgeEntry[]): void {
    const counts = new Map<string, number>();
    for (const entry of entries) {
      counts.set(entry.runId, (counts.get(entry.runId) ?? 0) + 1);
    }
    let runCounters = [...this.state.runCounters];
    for (const [runId, count] of counts) {
      runCounters = upsertCounter(runCounters, runId, (counter) => ({
        ...counter,
        stored: counter.stored + count,
        updatedAt: Date.now(),
      }));
    }
    this.commit({ ...this.state, runCounters });
  }

  flush(runId: string): BridgeFlushState {
    const counter = this.state.runCounters.find((candidate) => candidate.runId === runId);
    if (counter == null) {
      throw BridgeLifecycleConflictError;
    }
    const result = { runId, flushed: counter.stored - counter.flushed, stored: counter.stored };
    this.commit({
      ...this.state,
      runCounters: upsertCounter(this.state.runCounters, runId, (current) => ({
        ...current,
        flushed: current.stored,
        updatedAt: Date.now(),
      })),
    });
    return result;
  }

  private commit(nextState: PersistedBridgeLifecycleState): void {
    durableReplaceLocalArtifact(this.filePath, `${JSON.stringify(nextState)}\n`);
    this.state = nextState;
  }
}
