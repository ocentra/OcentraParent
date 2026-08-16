import fs from 'node:fs';
import path from 'node:path';
import type { FileKey, NdjsonSummaryContent, TestName } from './ndjsonBrands';
import { ensureDirectory, getDefaultLogRoot, sanitizeTestNameForNdjson } from './ndjsonPaths';
import { refreshLogsTree, type LogsTreeScope } from './logsTree';
import { getGeneratedRunDirPath } from '../local-test-log';

function hasErrorCode(error: unknown, code: string): boolean {
  return typeof error === 'object' && error !== null && 'code' in error && (error as { code?: unknown }).code === code;
}

function touchFile(filePath: string): void {
  const handle = fs.openSync(filePath, 'a');
  fs.closeSync(handle);
}

function createFileIfMissing(filePath: string): boolean {
  try {
    const handle = fs.openSync(filePath, 'wx');
    fs.closeSync(handle);
    return true;
  } catch (error) {
    if (hasErrorCode(error, 'EEXIST')) {
      return false;
    }
    throw error;
  }
}

export function writeSummary(scope: LogsTreeScope, fileKey: FileKey, content: NdjsonSummaryContent): void {
  const dirPath = ensureDirectory(getGeneratedRunDirPath(scope, fileKey, getDefaultLogRoot()));
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

export function writeLogEntry(scope: LogsTreeScope, fileKey: FileKey, testName: TestName, lines: string): void {
  const dirPath = ensureDirectory(getGeneratedRunDirPath(scope, fileKey, getDefaultLogRoot()));
  const filePath = path.join(dirPath, `${sanitizeTestNameForNdjson(testName)}.ndjson`);
  try {
    if (lines.length > 0) {
      fs.appendFileSync(filePath, lines, 'utf8');
    } else {
      touchFile(filePath);
    }
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
  const baseDir = ensureDirectory(getGeneratedRunDirPath(scope, fileKey, outputDir));
  let created = 0;
  for (const name of testNames) {
    const sanitized = sanitizeTestNameForNdjson(name);
    const filePath = path.join(baseDir, `${sanitized}.ndjson`);
    if (createFileIfMissing(filePath)) {
      created += 1;
    }
  }
  refreshLogsTree(outputDir);
  return created;
}
