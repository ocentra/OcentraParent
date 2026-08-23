import crypto from 'node:crypto';
import type { TestLogScope } from './types';
import { getDefaultLogRoot, listNdjsonFiles } from './ndjsonPaths';
import {
  buildGeneratedManifest,
  classifyGeneratedManifestChanges,
  getGeneratedManifestPath,
  type GeneratedIngestManifest,
  type GeneratedObservedFileState,
} from '../local-test-log';
import {
  durableRemoveLocalArtifact,
  durableReplaceLocalArtifact,
  readLocalArtifactTextSnapshot,
} from '../local-artifact-file';
import { withLocalArtifactLock } from '../local-artifact-lock';
import { resolveContainedLocalArtifactPath } from '../local-artifact-path';

const MaximumManifestBytes = 256 * 1024;
const MaximumObservedLogBytes = 64 * 1024 * 1024;

export interface ManifestEntry {
  readonly size: number;
  readonly modifiedMs: number;
  readonly sha256: string;
}

export interface IngestManifest {
  readonly scope: TestLogScope;
  readonly updatedAt: number;
  readonly files: Record<string, ManifestEntry>;
}

function parseManifest(scope: TestLogScope, input: unknown, rootDir: string): IngestManifest {
  const isRecord = [typeof input === 'object', input != null, !Array.isArray(input)].every(Boolean);
  if (!isRecord) {
    throw new Error('invalid local log ingest manifest');
  }
  const record = input as Record<string, unknown>;
  const updatedAt = record['updatedAt'];
  const rawFiles = record['files'];
  const validHeader = [
    record['scope'] === scope,
    typeof updatedAt === 'number',
    Number.isFinite(updatedAt as number),
    (updatedAt as number) >= 0,
    typeof rawFiles === 'object',
    rawFiles != null,
    !Array.isArray(rawFiles),
  ].every(Boolean);
  if (!validHeader) {
    throw new Error('invalid local log ingest manifest');
  }

  const files: Record<string, ManifestEntry> = {};
  for (const [filePath, value] of Object.entries(rawFiles as Record<string, unknown>)) {
    const validRecord = [filePath.length > 0, typeof value === 'object', value != null, !Array.isArray(value)].every(
      Boolean
    );
    if (!validRecord) {
      throw new Error('invalid local log ingest manifest');
    }
    resolveContainedLocalArtifactPath(rootDir, filePath);
    const entry = value as Record<string, unknown>;
    const size = entry['size'];
    const modifiedMs = entry['modifiedMs'];
    const sha256 = entry['sha256'];
    const validEntry = [
      typeof size === 'number',
      Number.isFinite(size as number),
      (size as number) >= 0,
      typeof modifiedMs === 'number',
      Number.isFinite(modifiedMs as number),
      (modifiedMs as number) >= 0,
      typeof sha256 === 'string',
      /^[0-9a-f]{64}$/u.test(sha256 as string),
    ].every(Boolean);
    if (!validEntry) {
      throw new Error('invalid local log ingest manifest');
    }
    files[filePath] = {
      size: size as number,
      modifiedMs: modifiedMs as number,
      sha256: sha256 as string,
    };
  }
  return { scope, updatedAt: updatedAt as number, files };
}

export function getManifestPath(scope: TestLogScope, rootDir?: string): string {
  return getGeneratedManifestPath(scope, rootDir ?? getDefaultLogRoot());
}

export function loadManifest(scope: TestLogScope, rootDir?: string): IngestManifest {
  const resolvedRoot = rootDir ?? getDefaultLogRoot();
  const manifestPath = getManifestPath(scope, resolvedRoot);
  const snapshot = readLocalArtifactTextSnapshot(manifestPath, resolvedRoot, MaximumManifestBytes);
  if (snapshot == null) {
    return { scope, updatedAt: 0, files: {} };
  }

  try {
    return parseManifest(scope, JSON.parse(snapshot.content) as unknown, resolvedRoot);
  } catch {
    throw new Error('invalid local log ingest manifest');
  }
}

export function saveManifest(manifest: IngestManifest, rootDir?: string): void {
  const resolvedRoot = rootDir ?? getDefaultLogRoot();
  const manifestPath = getManifestPath(manifest.scope, resolvedRoot);
  const validated = parseManifest(manifest.scope, manifest, resolvedRoot);
  durableReplaceLocalArtifact(manifestPath, `${JSON.stringify(validated, null, 2)}\n`, resolvedRoot);
}

export function removeManifest(scope: TestLogScope, rootDir?: string): void {
  const resolvedRoot = rootDir ?? getDefaultLogRoot();
  durableRemoveLocalArtifact(getManifestPath(scope, resolvedRoot), resolvedRoot);
}

export function getChangedFiles(
  scope: TestLogScope,
  logsDir: string,
  rootDir?: string
): { readonly newFiles: string[]; readonly changedFiles: string[]; readonly manifest: IngestManifest } {
  const resolvedRoot = rootDir ?? getDefaultLogRoot();
  return withLocalArtifactLock(resolvedRoot, () => {
    resolveContainedLocalArtifactPath(resolvedRoot, logsDir);
    const manifest = loadManifest(scope, resolvedRoot);
    const observedFiles = listNdjsonFiles(logsDir).map((resolvedPath): GeneratedObservedFileState => {
      const snapshot = readLocalArtifactTextSnapshot(resolvedPath, resolvedRoot, MaximumObservedLogBytes);
      if (snapshot == null) {
        throw new Error('test log disappeared during manifest observation');
      }
      const existing = manifest.files[resolvedPath];
      const sha256 =
        existing != null && existing.size === snapshot.stat.size && existing.modifiedMs === snapshot.stat.modifiedMs
          ? existing.sha256
          : crypto.createHash('sha256').update(snapshot.content, 'utf8').digest('hex');
      return {
        resolvedPath,
        size: snapshot.stat.size,
        modifiedMs: snapshot.stat.modifiedMs,
        sha256,
      };
    });
    const { newFiles, changedFiles } = classifyGeneratedManifestChanges(
      manifest as GeneratedIngestManifest,
      observedFiles
    );
    return { newFiles, changedFiles, manifest };
  });
}

export function updateManifest(scope: TestLogScope, logsDir: string, rootDir?: string): IngestManifest {
  const resolvedRoot = rootDir ?? getDefaultLogRoot();
  return withLocalArtifactLock(resolvedRoot, () => {
    resolveContainedLocalArtifactPath(resolvedRoot, logsDir);
    const observedFiles = listNdjsonFiles(logsDir).map((resolvedPath): GeneratedObservedFileState => {
      const snapshot = readLocalArtifactTextSnapshot(resolvedPath, resolvedRoot, MaximumObservedLogBytes);
      if (snapshot == null) {
        throw new Error('test log disappeared during manifest observation');
      }
      return {
        resolvedPath,
        size: snapshot.stat.size,
        modifiedMs: snapshot.stat.modifiedMs,
        sha256: crypto.createHash('sha256').update(snapshot.content, 'utf8').digest('hex'),
      };
    });
    const manifest = buildGeneratedManifest(scope, Date.now(), observedFiles) as IngestManifest;
    saveManifest(manifest, resolvedRoot);
    return manifest;
  });
}
