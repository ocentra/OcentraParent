import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import type { TestLogScope } from './types';
import { ensureDirectory, getDefaultLogRoot, listNdjsonFiles } from './ndjsonPaths';
import {
  buildGeneratedManifest,
  classifyGeneratedManifestChanges,
  getGeneratedManifestPath,
  type GeneratedIngestManifest,
  type GeneratedObservedFileState,
} from '../local-test-log';

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
  return crypto.createHash('sha256').update(fs.readFileSync(filePath)).digest('hex');
}

export function getManifestPath(scope: TestLogScope, rootDir?: string): string {
  return getGeneratedManifestPath(scope, rootDir ?? getDefaultLogRoot());
}

export function loadManifest(scope: TestLogScope, rootDir?: string): IngestManifest {
  const manifestPath = getManifestPath(scope, rootDir);
  if (!fs.existsSync(manifestPath)) {
    return { scope, updatedAt: 0, files: {} };
  }

  try {
    return JSON.parse(fs.readFileSync(manifestPath, 'utf8')) as IngestManifest;
  } catch {
    return { scope, updatedAt: 0, files: {} };
  }
}

export function saveManifest(manifest: IngestManifest, rootDir?: string): void {
  const manifestPath = getManifestPath(manifest.scope, rootDir);
  ensureDirectory(path.dirname(manifestPath));
  fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2), 'utf8');
}

export function removeManifest(scope: TestLogScope, rootDir?: string): void {
  fs.rmSync(getManifestPath(scope, rootDir), { force: true });
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
