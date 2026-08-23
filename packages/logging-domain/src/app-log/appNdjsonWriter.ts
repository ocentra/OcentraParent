import fs from 'node:fs';
import path from 'node:path';
import { AppLogEntrySchema, type AppLogEntry } from './types';
import type { TestLogScope } from '../test-log/types';
import { getAppLogScopeDir, getAppSessionFilePath } from '../test-log/ndjsonPaths';
import { selectGeneratedPruneCandidates } from '../local-test-log';
import {
  assertReadableLocalArtifactFile,
  durableAppendLocalArtifact,
  durableRemoveLocalArtifact,
} from '../local-artifact-file';

export function appendAppLogEntries(
  scope: TestLogScope,
  sessionId: string,
  entries: readonly AppLogEntry[],
  rootDir?: string
): string {
  const filePath = getAppSessionFilePath(scope, sessionId, rootDir);
  const payload = entries.map((entry) => JSON.stringify(AppLogEntrySchema.parse(entry))).join('\n');
  durableAppendLocalArtifact(filePath, `${payload}\n`);
  return filePath;
}

export function readAppLogEntries(filePath: string): AppLogEntry[] {
  if (!assertReadableLocalArtifactFile(filePath)) {
    return [];
  }

  const content = fs.readFileSync(filePath, 'utf8').trim();
  if (content.length === 0) {
    return [];
  }

  return content
    .split(/\r?\n/)
    .filter((line) => line.trim().length > 0)
    .map((line) => AppLogEntrySchema.parse(JSON.parse(line) as unknown));
}

export function listAppLogSessionFiles(scope: TestLogScope, rootDir?: string): string[] {
  const scopeDir = getAppLogScopeDir(scope, rootDir);
  if (!fs.existsSync(scopeDir)) {
    return [];
  }

  return fs
    .readdirSync(scopeDir, { withFileTypes: true })
    .filter((entry) => entry.isFile() && entry.name.endsWith('.ndjson'))
    .map((entry) => path.join(scopeDir, entry.name))
    .filter((filePath) => assertReadableLocalArtifactFile(filePath))
    .sort((left, right) => left.localeCompare(right));
}

export function pruneAppLogSessions(scope: TestLogScope, keepNewest: number, rootDir?: string): number {
  const files = listAppLogSessionFiles(scope, rootDir)
    .map((filePath) => ({
      filePath,
      modifiedMs: fs.statSync(filePath).mtimeMs,
    }))
    .sort((left, right) => right.modifiedMs - left.modifiedMs);

  const filesToDelete = selectGeneratedPruneCandidates(files, keepNewest);
  for (const file of filesToDelete) {
    durableRemoveLocalArtifact(file);
  }

  return filesToDelete.length;
}
