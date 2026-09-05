import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { durableAppendLocalArtifact } from '../../src/local-artifact-append';
import {
  durableRemoveLocalArtifact,
  durableReplaceLocalArtifact,
  readLocalArtifactTextSnapshot,
  statLocalArtifact,
} from '../../src/local-artifact-file';
import {
  beginLocalArtifactProviderLease,
  closeLocalArtifactMutationProvider,
  endLocalArtifactProviderLease,
  providerReplace,
  providerRootIdentity,
} from '../../src/local-artifact-mutation-provider';
import { currentProviderSession } from '../../src/local-artifact-provider-registry';
import { ensureLocalArtifactRoot, ensureOwnedDirectory } from '../../src/local-artifact-path';
import { applyLocalArtifactTransaction } from '../../src/local-artifact-transaction';
import { clearLoggingArtifactRoot } from '../../src/local-artifact-tree';

const temporaryRoots: string[] = [];

function temporaryArtifactRoot(): string {
  const parent = fs.mkdtempSync(path.join(os.tmpdir(), 'ocentra-local-artifact-provider-'));
  const root = path.join(parent, 'artifacts');
  temporaryRoots.push(root);
  return root;
}

afterEach(async () => {
  for (const root of temporaryRoots.splice(0)) {
    await closeLocalArtifactMutationProvider(root);
    fs.rmSync(path.dirname(root), { force: true, recursive: true });
  }
});

function mapsAbsentStatTargetToNull(): void {
  const root = ensureLocalArtifactRoot(temporaryArtifactRoot());
  const missingArtifact = path.join(root, 'app-logs', 'missing.ndjson');

  expect(statLocalArtifact(missingArtifact, root)).toBeNull();
}

async function ownsProviderOperationsAndRestart(): Promise<void> {
  const root = ensureLocalArtifactRoot(temporaryArtifactRoot());
  expect(durableRemoveLocalArtifact(path.join(root, 'missing', 'artifact.ndjson'), root)).toBe(false);
  const nestedReplacement = path.join(root, 'manifests', 'nested', 'artifact.json');
  durableReplaceLocalArtifact(nestedReplacement, '{"ready":true}\n', root);
  expect(readLocalArtifactTextSnapshot(nestedReplacement, root)?.content).toBe('{"ready":true}\n');
  const appDirectory = ensureOwnedDirectory(path.join(root, 'app-logs', 'sessions'));
  const appLog = path.join(appDirectory, 'session.ndjson');
  durableAppendLocalArtifact(appLog, '{"sequence":1}\n', root);
  durableAppendLocalArtifact(appLog, '{"sequence":2}\n', root);

  const appended = readLocalArtifactTextSnapshot(appLog, root);
  expect(appended?.content).toBe('{"sequence":1}\n{"sequence":2}\n');
  expect(appended?.stat.size).toBe(Buffer.byteLength(appended?.content ?? '', 'utf8'));
  expect(appended?.stat.identity.device).toMatch(/^(?:0|[1-9]\d*)$/u);
  expect(appended?.stat.identity.inode).toMatch(/^(?:0|[1-9]\d*)$/u);

  durableReplaceLocalArtifact(appLog, '{"sequence":3}\n', root);
  expect(readLocalArtifactTextSnapshot(appLog, root)?.content).toBe('{"sequence":3}\n');

  const removableDirectory = ensureOwnedDirectory(path.join(root, 'test-logs', 'scope'));
  const removableLog = path.join(removableDirectory, 'run.ndjson');
  durableReplaceLocalArtifact(removableLog, '{"run":1}\n', root);
  const lifecycle = path.join(root, '.bridge', 'lifecycle-state.json');
  applyLocalArtifactTransaction(root, [
    { kind: 'removeTree', filePath: removableDirectory },
    { kind: 'replace', filePath: lifecycle, payload: '{"owner":"application"}\n' },
  ]);
  expect(statLocalArtifact(removableDirectory, root)).toBeNull();
  expect(readLocalArtifactTextSnapshot(lifecycle, root)?.content).toBe('{"owner":"application"}\n');

  ensureOwnedDirectory(path.join(root, 'manifests'));
  clearLoggingArtifactRoot(root);
  expect(statLocalArtifact(path.join(root, 'app-logs'), root)).toBeNull();
  expect(statLocalArtifact(path.join(root, 'test-logs'), root)).toBeNull();
  expect(statLocalArtifact(path.join(root, 'manifests'), root)).toBeNull();
  expect(readLocalArtifactTextSnapshot(lifecycle, root)?.content).toBe('{"owner":"application"}\n');

  await closeLocalArtifactMutationProvider(root);
  expect(readLocalArtifactTextSnapshot(lifecycle, root)?.content).toBe('{"owner":"application"}\n');
}

async function retainsRootIdentityAcrossPathCaseVariants(): Promise<void> {
  const root = ensureLocalArtifactRoot(temporaryArtifactRoot());
  providerRootIdentity(root);
  await closeLocalArtifactMutationProvider(root);

  const retainedRoot = `${root}-retained`;
  fs.renameSync(root, retainedRoot);
  fs.mkdirSync(root);

  const caseVariantRoot = path.join(path.dirname(root), path.basename(root).toUpperCase());
  expect(() => providerRootIdentity(caseVariantRoot)).toThrowError(
    'provider root identity changed across process restart'
  );
}

async function rejectsOverlappingSessionDuringTeardown(): Promise<void> {
  const root = ensureLocalArtifactRoot(temporaryArtifactRoot());
  const initialIdentity = providerRootIdentity(root);

  const closing = closeLocalArtifactMutationProvider(root);
  expect(() => providerRootIdentity(root)).toThrowError('provider session disposal is still in progress');
  await closing;

  expect(providerRootIdentity(root)).toEqual(initialIdentity);
}

async function awaitsWorkerAndChildTeardownAfterProcessLoss(): Promise<void> {
  const root = ensureLocalArtifactRoot(temporaryArtifactRoot());
  providerRootIdentity(root);
  const session = currentProviderSession(root);
  if (session == null) throw new Error('provider session was not retained');
  const releasedRoot = `${root}-released`;
  expect(session.process.kill()).toBe(true);

  await closeLocalArtifactMutationProvider(root);
  fs.renameSync(root, releasedRoot);

  expect(session.worker.threadId).toBe(-1);
  expect(currentProviderSession(root) ?? null).toBeNull();
  expect(fs.statSync(releasedRoot).isDirectory()).toBe(true);
}

function rejectsMutationOutsideLeaseContext(): void {
  const root = ensureLocalArtifactRoot(temporaryArtifactRoot());
  const leaseId = beginLocalArtifactProviderLease(root);
  try {
    expect(() =>
      providerReplace(root, 'app-logs/unowned.ndjson', Buffer.from('{"unowned":true}\n', 'utf8'))
    ).toThrowError('provider mutation requires the current root lease');
  } finally {
    endLocalArtifactProviderLease(root, leaseId);
  }
  expect(statLocalArtifact(path.join(root, 'app-logs', 'unowned.ndjson'), root)).toBeNull();
}

describe.skipIf(process.platform !== 'win32')('Rust local-artifact mutation provider', () => {
  it('maps an absent stat target to null at the filesystem adapter', mapsAbsentStatTargetToNull);
  it('owns append, snapshot, replacement, transaction, recursive clear, and restart', ownsProviderOperationsAndRestart);
  it('retains root identity across Windows path-case variants', retainsRootIdentityAcrossPathCaseVariants);
  it(
    'keeps a closing session unavailable until worker and child teardown complete',
    rejectsOverlappingSessionDuringTeardown
  );
  it(
    'awaits worker and child teardown when the provider exits before graceful shutdown',
    awaitsWorkerAndChildTeardownAfterProcessLoss
  );
  it(
    'rejects mutation when a caller is outside the asynchronous lease owner context',
    rejectsMutationOutsideLeaseContext
  );
});
