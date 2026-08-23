import crypto from 'node:crypto';
import fs from 'node:fs';
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
  assertReadableLocalArtifactFile,
  durableRemoveLocalArtifact,
  durableReplaceLocalArtifact,
} from '../local-artifact-file';

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

function fileHash(filePath: string): string {
  assertReadableLocalArtifactFile(filePath);
  return crypto.createHash('sha256').update(fs.readFileSync(filePath)).digest('hex');
}

function parseManifest(scope: TestLogScope, input: unknown): IngestManifest {
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
  const manifestPath = getManifestPath(scope, rootDir);
  if (!assertReadableLocalArtifactFile(manifestPath)) {
    return { scope, updatedAt: 0, files: {} };
  }

  try {
    return parseManifest(scope, JSON.parse(fs.readFileSync(manifestPath, 'utf8')) as unknown);
  } catch {
    throw new Error('invalid local log ingest manifest');
  }
}

export function saveManifest(manifest: IngestManifest, rootDir?: string): void {
  const manifestPath = getManifestPath(manifest.scope, rootDir);
  const validated = parseManifest(manifest.scope, manifest);
  durableReplaceLocalArtifact(manifestPath, `${JSON.stringify(validated, null, 2)}\n`);
}

export function removeManifest(scope: TestLogScope, rootDir?: string): void {
  durableRemoveLocalArtifact(getManifestPath(scope, rootDir));
}

export function getChangedFiles(
  scope: TestLogScope,
  logsDir: string,
  rootDir?: string
): { readonly newFiles: string[]; readonly changedFiles: string[]; readonly manifest: IngestManifest } {
  const manifest = loadManifest(scope, rootDir);
  const files = listNdjsonFiles(logsDir);
  const observedFiles: GeneratedObservedFileState[] = [];

  for (const filePath of files) {
    const resolvedPath = filePath;
    assertReadableLocalArtifactFile(resolvedPath);
    const currentStat = fs.statSync(resolvedPath);
    const existing = manifest.files[resolvedPath];
    const sha256 =
      existing != null && existing.size === currentStat.size && existing.modifiedMs === currentStat.mtimeMs
        ? existing.sha256
        : fileHash(resolvedPath);
    observedFiles.push({
      resolvedPath,
      size: currentStat.size,
      modifiedMs: currentStat.mtimeMs,
      sha256,
    });
  }

  const { newFiles, changedFiles } = classifyGeneratedManifestChanges(
    manifest as GeneratedIngestManifest,
    observedFiles
  );
  return { newFiles, changedFiles, manifest };
}

export function updateManifest(scope: TestLogScope, logsDir: string, rootDir?: string): IngestManifest {
  const files = listNdjsonFiles(logsDir);
  const observedFiles: GeneratedObservedFileState[] = [];

  for (const filePath of files) {
    const resolvedPath = filePath;
    assertReadableLocalArtifactFile(resolvedPath);
    const stat = fs.statSync(resolvedPath);
    observedFiles.push({
      resolvedPath,
      size: stat.size,
      modifiedMs: stat.mtimeMs,
      sha256: fileHash(resolvedPath),
    });
  }

  const manifest = buildGeneratedManifest(scope, Date.now(), observedFiles) as IngestManifest;
  saveManifest(manifest, rootDir);
  return manifest;
}
