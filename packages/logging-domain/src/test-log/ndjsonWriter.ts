import fs from 'node:fs';
import path from 'node:path';
import { StoredTestLogLineSchema, type StoredTestLogLine } from './types';
import { ensureDirectory, getDefaultLogRoot } from './ndjsonPaths';
import { groupGeneratedTestLogEntriesByFilePath, splitGeneratedNdjsonContent } from '../local-test-log';

export function appendTestLogEntries(entries: readonly StoredTestLogLine[], rootDir?: string): string[] {
  const normalizedEntries = entries.map((rawEntry) => StoredTestLogLineSchema.parse(rawEntry));
  const grouped = groupGeneratedTestLogEntriesByFilePath(normalizedEntries, rootDir ?? getDefaultLogRoot());

  for (const [filePath, fileEntries] of grouped.entries()) {
    ensureDirectory(path.dirname(filePath));
    const serialized = fileEntries.map((entry) => JSON.stringify(entry)).join('\n');
    fs.appendFileSync(filePath, `${serialized}\n`, 'utf8');
  }

  return [...grouped.keys()].sort((left, right) => left.localeCompare(right));
}

export function readTestLogEntriesFromFile(filePath: string): StoredTestLogLine[] {
  if (!fs.existsSync(filePath)) {
    return [];
  }

  const content = fs.readFileSync(filePath, 'utf8').trim();
  if (content.length === 0) {
    return [];
  }

  return splitGeneratedNdjsonContent(content).map((line) => StoredTestLogLineSchema.parse(JSON.parse(line) as unknown));
}
