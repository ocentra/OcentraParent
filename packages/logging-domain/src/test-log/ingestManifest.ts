import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import type { TestLogScope } from '@ocentra-parent/schema-domain/test-log/types';
import { ensureDirectory, getManifestDir, listNdjsonFiles } from './ndjsonPaths';

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
  return path.join(getManifestDir(rootDir), `${scope}-ingest-manifest.json`);
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
  const newFiles: string[] = [];
  const changedFiles: string[] = [];

  for (const filePath of files) {
    const resolvedPath = path.resolve(filePath);
    const currentStat = fs.statSync(resolvedPath);
    const existing = manifest.files[resolvedPath];

    if (existing == null) {
      newFiles.push(resolvedPath);
      continue;
    }

    if (existing.size === currentStat.size && existing.modifiedMs === currentStat.mtimeMs) {
      continue;
    }

    const sha256 = fileHash(resolvedPath);
    if (sha256 !== existing.sha256) {
      changedFiles.push(resolvedPath);
    }
  }

  return { newFiles, changedFiles, manifest };
}

export function updateManifest(scope: TestLogScope, logsDir: string, rootDir?: string): IngestManifest {
  const files = listNdjsonFiles(logsDir);
  const nextFiles: Record<string, ManifestEntry> = {};

  for (const filePath of files) {
    const resolvedPath = path.resolve(filePath);
    const stat = fs.statSync(resolvedPath);
    nextFiles[resolvedPath] = {
      size: stat.size,
      modifiedMs: stat.mtimeMs,
      sha256: fileHash(resolvedPath),
    };
  }

  const manifest: IngestManifest = {
    scope,
    updatedAt: Date.now(),
    files: nextFiles,
  };
  saveManifest(manifest, rootDir);
  return manifest;
}
