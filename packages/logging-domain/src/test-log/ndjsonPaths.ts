import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import type {
  RunType,
  TestLogScope,
  TestSuiteType,
} from '@ocentra-parent/schema-domain/test-log/types';

const LOG_ROOT_ENV = 'OCENTRA_PARENT_LOG_DIR';
const TEST_LOG_DIR = 'test-logs';
const APP_LOG_DIR = 'app-logs';
const DB_DIR = 'db';
const MANIFEST_DIR = 'manifests';

function getPackageRoot(): string {
  const filename = fileURLToPath(import.meta.url);
  return path.resolve(path.dirname(filename), '..', '..');
}

function getWorkspaceRoot(): string {
  return path.resolve(getPackageRoot(), '..', '..');
}

function sanitizePathSegment(value: string): string {
  return value.replace(/[^a-zA-Z0-9._-]+/g, '-').replace(/^-+/, '').replace(/-+$/, '') || 'default';
}

export function sanitizeTestNameForNdjson(testName: string): string {
  const sanitized = testName
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+/, '')
    .replace(/-+$/, '')
    .substring(0, 100);
  return sanitized || 'unnamed-test';
}

export function ensureDirectory(dirPath: string): string {
  fs.mkdirSync(dirPath, { recursive: true });
  return dirPath;
}

export function getDefaultLogRoot(): string {
  const fromEnv = process.env[LOG_ROOT_ENV];
  if (fromEnv != null && fromEnv.trim().length > 0) {
    return path.resolve(fromEnv);
  }
  return path.join(getWorkspaceRoot(), 'output', 'logging-domain');
}

export function getTestLogScopeDir(scope: TestLogScope, rootDir?: string): string {
  return ensureDirectory(path.join(rootDir ?? getDefaultLogRoot(), TEST_LOG_DIR, scope));
}

export function getRunNdjsonFilePath(
  scope: TestLogScope,
  runType: RunType,
  runId: string,
  suiteType: TestSuiteType | null,
  rootDir?: string
): string {
  const scopeDir = getTestLogScopeDir(scope, rootDir);
  const suiteSegment = suiteType ?? 'unspecified';
  const fileName = `${sanitizePathSegment(runId)}.ndjson`;
  return path.join(scopeDir, runType, suiteSegment, fileName);
}

export function getAppLogScopeDir(scope: TestLogScope, rootDir?: string): string {
  return ensureDirectory(path.join(rootDir ?? getDefaultLogRoot(), APP_LOG_DIR, scope));
}

export function getAppSessionFilePath(
  scope: TestLogScope,
  sessionId: string,
  rootDir?: string
): string {
  return path.join(getAppLogScopeDir(scope, rootDir), `${sanitizePathSegment(sessionId)}.ndjson`);
}

export function getDbDir(rootDir?: string): string {
  return ensureDirectory(path.join(rootDir ?? getDefaultLogRoot(), DB_DIR));
}

export function getManifestDir(rootDir?: string): string {
  return ensureDirectory(path.join(rootDir ?? getDefaultLogRoot(), MANIFEST_DIR));
}

export function listNdjsonFiles(rootPath: string): string[] {
  if (!fs.existsSync(rootPath)) {
    return [];
  }

  const entries = fs.readdirSync(rootPath, { withFileTypes: true });
  const files: string[] = [];

  for (const entry of entries) {
    const fullPath = path.join(rootPath, entry.name);
    if (entry.isDirectory()) {
      files.push(...listNdjsonFiles(fullPath));
      continue;
    }
    if (entry.name.endsWith('.ndjson')) {
      files.push(fullPath);
    }
  }

  return files.sort((left, right) => left.localeCompare(right));
}

export function clearDirectory(targetDir: string): void {
  fs.rmSync(targetDir, { force: true, recursive: true });
}
