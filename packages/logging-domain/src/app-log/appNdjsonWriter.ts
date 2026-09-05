import path from 'node:path';
import { AppLogEntrySchema, type AppLogEntry } from './types';
import type { TestLogScope } from '../test-log/types';
import { getAppLogScopeDir, getAppSessionFilePath, getDefaultLogRoot } from '../test-log/ndjsonPaths';
import { selectGeneratedPruneCandidates } from '../local-test-log';
import { durableAppendLocalArtifact, recoverLocalArtifactAppends } from '../local-artifact-append';
import { readLocalArtifactText, statLocalArtifact } from '../local-artifact-file';
import { withLocalArtifactLock } from '../local-artifact-lock';
import { applyLocalArtifactTransaction, type LocalArtifactMutation } from '../local-artifact-transaction';
import { providerList } from '../local-artifact-mutation-provider';
import { relativeLocalArtifactPath } from '../local-artifact-path';
import { MaximumBridgeBatchBytes, sanitizeAppLogBatchForCustody } from '../core/logCustody';
import { utf8Bytes } from '../core/logTextCustody';
import { sanitizeGeneratedPathSegment } from '../local-test-log-paths';

const MaximumAppLogFileBytes = 64 * 1024 * 1024;

export function appendAppLogEntries(
  scope: TestLogScope,
  sessionId: string,
  entries: readonly AppLogEntry[],
  rootDir?: string
): string {
  const normalizedEntries = sanitizeAppLogBatchForCustody(entries);
  if (normalizedEntries.length === 0) {
    throw new Error('app log append requires at least one custodied entry');
  }
  const normalizedSessionId = sanitizeGeneratedPathSegment(sessionId);
  if (normalizedEntries.some((entry) => entry.scope !== scope || entry.sessionId !== normalizedSessionId)) {
    throw new Error('app log append metadata does not match its declared target');
  }
  const resolvedRoot = rootDir ?? getDefaultLogRoot();
  const filePath = getAppSessionFilePath(scope, normalizedSessionId, resolvedRoot);
  const payload = normalizedEntries.map((entry) => JSON.stringify(entry)).join('\n');
  if (utf8Bytes(payload) > MaximumBridgeBatchBytes) {
    throw new Error('app log append batch exceeds its custody limit');
  }
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
    const relativeScope = relativeLocalArtifactPath(resolvedRoot, scopeDir).split(path.sep).join('/');
    return providerList(resolvedRoot, relativeScope)
      .map((entry) => {
        if (entry.is_directory) {
          throw new Error('app log scope contains an unexpected directory');
        }
        const filePath = path.join(scopeDir, entry.name);
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
      filesToDelete.map((filePath): LocalArtifactMutation => ({ kind: 'remove', filePath }))
    );
    return filesToDelete.length;
  });
}
