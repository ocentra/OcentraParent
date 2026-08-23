import { StoredTestLogLineSchema, type StoredTestLogLine } from './types';
import { getDefaultLogRoot } from './ndjsonPaths';
import { groupGeneratedTestLogEntriesByFilePath, splitGeneratedNdjsonContent } from '../local-test-log';
import { durableAppendLocalArtifact } from '../local-artifact-append';
import { readLocalArtifactText } from '../local-artifact-file';

const MaximumNdjsonFileBytes = 64 * 1024 * 1024;

export function appendTestLogEntries(entries: readonly StoredTestLogLine[], rootDir?: string): string[] {
  const resolvedRoot = rootDir ?? getDefaultLogRoot();
  const normalizedEntries = entries.map((rawEntry) => StoredTestLogLineSchema.parse(rawEntry));
  const grouped = groupGeneratedTestLogEntriesByFilePath(normalizedEntries, resolvedRoot);

  for (const [filePath, fileEntries] of grouped.entries()) {
    const serialized = fileEntries.map((entry) => JSON.stringify(entry)).join('\n');
    durableAppendLocalArtifact(filePath, `${serialized}\n`, resolvedRoot);
  }

  return [...grouped.keys()].sort((left, right) => left.localeCompare(right));
}

export function readTestLogEntriesFromFile(filePath: string, rootDir?: string): StoredTestLogLine[] {
  const content = readLocalArtifactText(filePath, rootDir, MaximumNdjsonFileBytes);
  if (content == null) {
    return [];
  }

  const normalized = content.trim();
  if (normalized.length === 0) {
    return [];
  }

  return splitGeneratedNdjsonContent(normalized).map((line) =>
    StoredTestLogLineSchema.parse(JSON.parse(line) as unknown)
  );
}
