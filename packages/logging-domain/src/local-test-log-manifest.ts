/* generated from crates/logging-core/src/local_ndjson_log.rs */

import type { GeneratedIngestManifest, GeneratedObservedFileState } from './local-test-log';
import { getGeneratedRunNdjsonFilePath } from './local-test-log-paths';
import type { StoredTestLogLine, TestLogScope } from './test-log/types';

export function groupGeneratedTestLogEntriesByFilePath(
  entries: readonly StoredTestLogLine[],
  rootDir: string
): Map<string, StoredTestLogLine[]> {
  const grouped = new Map<string, StoredTestLogLine[]>();
  for (const entry of entries) {
    const filePath = getGeneratedRunNdjsonFilePath(entry.scope, entry.runType, entry.runId, entry.suiteType, rootDir);
    const existing = grouped.get(filePath);
    if (existing == null) {
      grouped.set(filePath, [entry]);
      continue;
    }
    existing.push(entry);
  }
  return grouped;
}

export function splitGeneratedNdjsonContent(content: string): string[] {
  const trimmed = content.trim();
  if (trimmed.length === 0) {
    return [];
  }
  return trimmed
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => line.length > 0);
}

export function classifyGeneratedManifestChanges(
  manifest: GeneratedIngestManifest,
  observedFiles: readonly GeneratedObservedFileState[]
): { readonly newFiles: string[]; readonly changedFiles: string[] } {
  const newFiles: string[] = [];
  const changedFiles: string[] = [];

  for (const observed of observedFiles) {
    const existing = manifest.files[observed.resolvedPath];
    if (existing == null) {
      newFiles.push(observed.resolvedPath);
      continue;
    }
    if (
      existing.size !== observed.size ||
      existing.modifiedMs !== observed.modifiedMs ||
      existing.sha256 !== observed.sha256
    ) {
      changedFiles.push(observed.resolvedPath);
    }
  }

  return { newFiles, changedFiles };
}

export function buildGeneratedManifest(
  scope: TestLogScope,
  updatedAt: number,
  observedFiles: readonly GeneratedObservedFileState[]
): GeneratedIngestManifest {
  const files = Object.fromEntries(
    observedFiles.map((observed) => [
      observed.resolvedPath,
      {
        size: observed.size,
        modifiedMs: observed.modifiedMs,
        sha256: observed.sha256,
      } satisfies GeneratedManifestEntry,
    ])
  );

  return {
    scope,
    updatedAt,
    files,
  };
}

type GeneratedManifestEntry = GeneratedIngestManifest['files'][string];
