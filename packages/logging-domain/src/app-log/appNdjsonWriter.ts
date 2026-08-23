import fs from 'node:fs';
import path from 'node:path';
import { AppLogEntrySchema, type AppLogEntry } from './types';
import type { TestLogScope } from '../test-log/types';
import { getAppLogScopeDir, getAppSessionFilePath, getDefaultLogRoot } from '../test-log/ndjsonPaths';
import { selectGeneratedPruneCandidates } from '../local-test-log';
import { durableAppendLocalArtifact, recoverLocalArtifactAppends } from '../local-artifact-append';
import { readLocalArtifactText, statLocalArtifact } from '../local-artifact-file';
import { withLocalArtifactLock } from '../local-artifact-lock';
import { applyLocalArtifactTransaction } from '../local-artifact-transaction';
import { assertExistingOwnedPath } from '../local-artifact-path';

const MaximumAppLogFileBytes = 64 * 1024 * 1024;

export function appendAppLogEntries(
  scope: TestLogScope,
  sessionId: string,
  entries: readonly AppLogEntry[],
  rootDir?: string
): string {
  const resolvedRoot = rootDir ?? getDefaultLogRoot();
  const filePath = getAppSessionFilePath(scope, sessionId, resolvedRoot);
  const payload = entries.map((entry) => JSON.stringify(AppLogEntrySchema.parse(entry))).join('\n');
  durableAppendLocalArtifact(filePath, `${payload}\n`, resolvedRoot);
  return filePath;
}

export function readAppLogEntries(filePath: string, rootDir?: string): AppLogEntry[] {
  const content = readLocalArtifactText(filePath, rootDir, MaximumAppLogFileBytes);
  if (content == null) {
    return [];
  }

  const normalized = content.trim();
  if (normalized.length === 0) {
    return [];
  }

  return normalized
    .split(/\r?\n/)
    .filter((line) => line.trim().length > 0)
    .map((line) => AppLogEntrySchema.parse(JSON.parse(line) as unknown));
}

export function listAppLogSessionFiles(scope: TestLogScope, rootDir?: string): string[] {
  const resolvedRoot = rootDir ?? getDefaultLogRoot();
  return withLocalArtifactLock(resolvedRoot, () => {
    recoverLocalArtifactAppends(resolvedRoot);
    const scopeDir = getAppLogScopeDir(scope, resolvedRoot);
    return fs
      .readdirSync(scopeDir, { withFileTypes: true })
      .map((entry) => {
        const filePath = path.join(scopeDir, entry.name);
        assertExistingOwnedPath(filePath, 'file');
        return filePath;
      })
      .filter((filePath) => filePath.endsWith('.ndjson'))
      .sort((left, right) => left.localeCompare(right));
  });
}

export function pruneAppLogSessions(scope: TestLogScope, keepNewest: number, rootDir?: string): number {
  const resolvedRoot = rootDir ?? getDefaultLogRoot();
  return withLocalArtifactLock(resolvedRoot, () => {
    const files = listAppLogSessionFiles(scope, resolvedRoot)
      .map((filePath) => {
        const stat = statLocalArtifact(filePath, resolvedRoot);
        if (stat == null) {
          throw new Error('app log disappeared during retention planning');
        }
        return { filePath, modifiedMs: stat.modifiedMs };
      })
      .sort((left, right) => right.modifiedMs - left.modifiedMs);
    const filesToDelete = selectGeneratedPruneCandidates(files, keepNewest);
    applyLocalArtifactTransaction(
      resolvedRoot,
      filesToDelete.map((filePath) => ({ kind: 'remove' as const, filePath }))
    );
    return filesToDelete.length;
  });
}
