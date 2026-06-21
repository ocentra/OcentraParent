import fs from 'node:fs';
import path from 'node:path';
import type {
  FileKey,
  NdjsonSummaryContent,
  TestName,
} from '@ocentra-parent/schema-domain/test-log/ndjsonBrands';
import { ensureDirectory, sanitizeTestNameForNdjson } from './ndjsonPaths';
import { getDirPath, refreshLogsTree, type LogsTreeScope } from './logsTree';

export function writeSummary(
  scope: LogsTreeScope,
  fileKey: FileKey,
  content: NdjsonSummaryContent
): void {
  const dirPath = ensureDirectory(getDirPath(scope, fileKey));
  const filePath = path.join(dirPath, `${fileKey}.ndjson`);
  try {
    fs.appendFileSync(filePath, content, 'utf8');
  } catch (err) {
    writeNdjsonError('writeSummary', filePath, err);
    throw err;
  } finally {
    refreshLogsTree();
  }
}

export function writeLogEntry(
  scope: LogsTreeScope,
  fileKey: FileKey,
  testName: TestName,
  lines: string
): void {
  const dirPath = ensureDirectory(getDirPath(scope, fileKey));
  const filePath = path.join(dirPath, `${sanitizeTestNameForNdjson(testName)}.ndjson`);
  try {
    if (!fs.existsSync(filePath)) fs.writeFileSync(filePath, '', 'utf8');
    if (lines.length > 0) fs.appendFileSync(filePath, lines, 'utf8');
  } catch (err) {
    writeNdjsonError('writeLogEntry', filePath, err);
    throw err;
  } finally {
    refreshLogsTree();
  }
}

function writeNdjsonError(op: string, filePath: string, err: unknown): void {
  const msg = err instanceof Error ? err.message : String(err);
  const line = `[ndjsonLogFileWriter] ${op} failed: ${filePath} - ${msg}\n`;
  process.stderr.write(line);
}

export function createEmptyTestNdjsonFiles(
  outputDir: string,
  scope: LogsTreeScope,
  fileKey: FileKey,
  testNames: TestName[]
): number {
  const baseDir = ensureDirectory(getDirPath(scope, fileKey, outputDir));
  let created = 0;
  for (const name of testNames) {
    const sanitized = sanitizeTestNameForNdjson(name);
    const filePath = path.join(baseDir, `${sanitized}.ndjson`);
    if (!fs.existsSync(filePath)) {
      fs.writeFileSync(filePath, '', 'utf8');
      created += 1;
    }
  }
  refreshLogsTree(outputDir);
  return created;
}
