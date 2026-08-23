import fs from 'node:fs';
import { StoredTestLogLineSchema, type StoredTestLogLine } from './types';
import { getDefaultLogRoot } from './ndjsonPaths';
import { groupGeneratedTestLogEntriesByFilePath, splitGeneratedNdjsonContent } from '../local-test-log';
import { assertReadableLocalArtifactFile, durableAppendLocalArtifact } from '../local-artifact-file';

export function appendTestLogEntries(entries: readonly StoredTestLogLine[], rootDir?: string): string[] {
  const normalizedEntries = entries.map((rawEntry) => StoredTestLogLineSchema.parse(rawEntry));
  const grouped = groupGeneratedTestLogEntriesByFilePath(normalizedEntries, rootDir ?? getDefaultLogRoot());

  for (const [filePath, fileEntries] of grouped.entries()) {
    const serialized = fileEntries.map((entry) => JSON.stringify(entry)).join('\n');
    durableAppendLocalArtifact(filePath, `${serialized}\n`);
  }

  return [...grouped.keys()].sort((left, right) => left.localeCompare(right));
}

export function readTestLogEntriesFromFile(filePath: string): StoredTestLogLine[] {
  if (!assertReadableLocalArtifactFile(filePath)) {
    return [];
  }

  const content = fs.readFileSync(filePath, 'utf8').trim();
  if (content.length === 0) {
    return [];
  }

  return splitGeneratedNdjsonContent(content).map((line) => StoredTestLogLineSchema.parse(JSON.parse(line) as unknown));
}
