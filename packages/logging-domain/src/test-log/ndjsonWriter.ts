import fs from 'node:fs';
import path from 'node:path';
import { StoredTestLogLineSchema, type StoredTestLogLine } from '@ocentra-parent/schema-domain/test-log/types';
import { ensureDirectory, getRunNdjsonFilePath } from './ndjsonPaths';

export function appendTestLogEntries(entries: readonly StoredTestLogLine[], rootDir?: string): string[] {
  const grouped = new Map<string, StoredTestLogLine[]>();

  for (const rawEntry of entries) {
    const entry = StoredTestLogLineSchema.parse(rawEntry);
    const filePath = getRunNdjsonFilePath(entry.scope, entry.runType, entry.runId, entry.suiteType, rootDir);
    const existing = grouped.get(filePath);
    if (existing == null) {
      grouped.set(filePath, [entry]);
      continue;
    }
    existing.push(entry);
  }

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

  return content
    .split(/\r?\n/)
    .filter((line) => line.trim().length > 0)
    .map((line) => StoredTestLogLineSchema.parse(JSON.parse(line) as unknown));
}
