import crypto from 'node:crypto';
import path from 'node:path';
import { durableReplaceLocalArtifact, readLocalArtifactText } from '../local-artifact-file';
import { withLocalArtifactLock } from '../local-artifact-lock';
import { ensureOwnedDirectory } from '../local-artifact-path';
import { applyLocalArtifactTransaction } from '../local-artifact-transaction';
import { normalizeWipeFileSelector } from '../test-log/wipeFileSelector';
import {
  emptyBridgeLifecycleState,
  parseBridgeLifecycleState,
  type PersistedBridgeLifecycleState,
} from './bridgeLifecycleStateCodec';

const MaximumLifecycleBytes = 256 * 1024;

export function bridgeLifecyclePath(rootDir: string): string {
  return path.join(rootDir, '.bridge', 'lifecycle-state.json');
}

export function serializeBridgeLifecycleState(state: PersistedBridgeLifecycleState): string {
  return `${JSON.stringify(parseBridgeLifecycleState(state))}\n`;
}

export function loadBridgeLifecycleState(rootDir: string): PersistedBridgeLifecycleState {
  return withLocalArtifactLock(rootDir, () => {
    const content = readLocalArtifactText(bridgeLifecyclePath(rootDir), rootDir, MaximumLifecycleBytes);
    if (content == null) {
      return emptyBridgeLifecycleState();
    }
    let value: unknown;
    try {
      value = JSON.parse(content) as unknown;
      return parseBridgeLifecycleState(value);
    } catch {
      return quarantineInvalidLifecycleState(rootDir, content, value);
    }
  });
}

export function replaceBridgeLifecycleState(rootDir: string, state: PersistedBridgeLifecycleState): void {
  durableReplaceLocalArtifact(bridgeLifecyclePath(rootDir), serializeBridgeLifecycleState(state), rootDir);
}

function quarantineInvalidLifecycleState(rootDir: string, raw: string, value: unknown): PersistedBridgeLifecycleState {
  const recordSha256 = crypto.createHash('sha256').update(raw).digest('hex');
  const quarantined = recoverInvalidLifecycleState(raw, value, recordSha256);
  const operatorState = quarantined.operatorState;
  if (operatorState == null) {
    throw new Error('invalid bridge lifecycle quarantine state');
  }
  const quarantineDirectory = ensureOwnedDirectory(path.join(rootDir, '.bridge', 'lifecycle-quarantine'));
  applyLocalArtifactTransaction(rootDir, [
    {
      kind: 'replace',
      filePath: path.join(quarantineDirectory, `${operatorState.recordSha256}.json`),
      payload: raw.endsWith('\n') ? raw : `${raw}\n`,
    },
    {
      kind: 'replace',
      filePath: bridgeLifecyclePath(rootDir),
      payload: serializeBridgeLifecycleState(quarantined),
    },
  ]);
  return quarantined;
}

function recoverInvalidLifecycleState(
  raw: string,
  value: unknown,
  recordSha256: string
): PersistedBridgeLifecycleState {
  try {
    return quarantineInvalidPendingSelector(raw, value);
  } catch {
    return {
      ...emptyBridgeLifecycleState(),
      operatorState: {
        status: 'manual-required',
        code: 'invalid-lifecycle-record',
        observedAt: Date.now(),
        recordSha256,
      },
    };
  }
}

function quarantineInvalidPendingSelector(raw: string, value: unknown): PersistedBridgeLifecycleState {
  const input = lifecycleRecord(value);
  const pendingInput = lifecycleRecord(input['pendingStart']);
  if (pendingInput['filePath'] == null || input['operatorState'] != null) {
    throw new Error('invalid bridge lifecycle state');
  }
  assertInvalidWipeSelector(pendingInput['filePath']);
  const recovered = parseBridgeLifecycleState({
    ...input,
    pendingStart: { ...pendingInput, filePath: null },
    operatorState: null,
  });
  return {
    ...recovered,
    pendingStart: null,
    operatorState: {
      status: 'manual-required',
      code: 'invalid-pending-start-selector',
      observedAt: Date.now(),
      recordSha256: crypto.createHash('sha256').update(raw).digest('hex'),
    },
  };
}

function lifecycleRecord(value: unknown): Record<string, unknown> {
  if (typeof value !== 'object' || value == null || Array.isArray(value)) {
    throw new Error('invalid bridge lifecycle state');
  }
  return value as Record<string, unknown>;
}

function assertInvalidWipeSelector(value: unknown): void {
  try {
    normalizeWipeFileSelector(value as string);
  } catch {
    return;
  }
  throw new Error('invalid bridge lifecycle state');
}
