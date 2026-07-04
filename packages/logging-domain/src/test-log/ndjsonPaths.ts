import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import type { RunType, TestLogScope, TestSuiteType } from './types';
import {
  GeneratedLocalLogRootEnv,
  getGeneratedAppLogScopeDir,
  getGeneratedAppSessionFilePath,
  getGeneratedDbDir,
  getGeneratedManifestDir,
  getGeneratedRunNdjsonFilePath,
  getGeneratedTestLogScopeDir,
  resolveGeneratedLocalLogRoot,
  sanitizeGeneratedTestNameForNdjson,
} from '../local-test-log';

function getPackageRoot(): string {
  const filename = fileURLToPath(import.meta.url);
  return path.resolve(path.dirname(filename), '..', '..');
}

function getWorkspaceRoot(): string {
  return path.resolve(getPackageRoot(), '..', '..');
}

export function sanitizeTestNameForNdjson(testName: string): string {
  return sanitizeGeneratedTestNameForNdjson(testName);
}

export function ensureDirectory(dirPath: string): string {
  fs.mkdirSync(dirPath, { recursive: true });
  return dirPath;
}

export function getDefaultLogRoot(): string {
  return resolveGeneratedLocalLogRoot(process.env[GeneratedLocalLogRootEnv], getWorkspaceRoot());
}

export function getTestLogScopeDir(scope: TestLogScope, rootDir?: string): string {
  return ensureDirectory(getGeneratedTestLogScopeDir(scope, rootDir ?? getDefaultLogRoot()));
}

export function getRunNdjsonFilePath(
  scope: TestLogScope,
  runType: RunType,
  runId: string,
  suiteType: TestSuiteType | null,
  rootDir?: string
): string {
  return getGeneratedRunNdjsonFilePath(scope, runType, runId, suiteType, rootDir ?? getDefaultLogRoot());
}

export function getAppLogScopeDir(scope: TestLogScope, rootDir?: string): string {
  return ensureDirectory(getGeneratedAppLogScopeDir(scope, rootDir ?? getDefaultLogRoot()));
}

export function getAppSessionFilePath(scope: TestLogScope, sessionId: string, rootDir?: string): string {
  return getGeneratedAppSessionFilePath(scope, sessionId, rootDir ?? getDefaultLogRoot());
}

export function getDbDir(rootDir?: string): string {
  return ensureDirectory(getGeneratedDbDir(rootDir ?? getDefaultLogRoot()));
}

export function getManifestDir(rootDir?: string): string {
  return ensureDirectory(getGeneratedManifestDir(rootDir ?? getDefaultLogRoot()));
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
