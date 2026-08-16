/* generated from crates/logging-core/src/local_ndjson_log.rs */

import type { RunType, TestLogScope, TestSuiteType } from './test-log/types';
import {
  buildGeneratedLogsTreeKey as buildGeneratedLogsTreeKeyImpl,
  getGeneratedAppLogScopeDir as getGeneratedAppLogScopeDirImpl,
  getGeneratedAppSessionFilePath as getGeneratedAppSessionFilePathImpl,
  getGeneratedDbDir as getGeneratedDbDirImpl,
  getGeneratedManifestDir as getGeneratedManifestDirImpl,
  getGeneratedManifestPath as getGeneratedManifestPathImpl,
  getGeneratedRunDirPath as getGeneratedRunDirPathImpl,
  getGeneratedRunNdjsonFilePath as getGeneratedRunNdjsonFilePathImpl,
  getGeneratedTestLogScopeDir as getGeneratedTestLogScopeDirImpl,
  resolveGeneratedLocalLogRoot as resolveGeneratedLocalLogRootImpl,
  sanitizeGeneratedPathSegment as sanitizeGeneratedPathSegmentImpl,
  sanitizeGeneratedTestNameForNdjson as sanitizeGeneratedTestNameForNdjsonImpl,
} from './local-test-log-paths';
import {
  buildGeneratedManifest as buildGeneratedManifestImpl,
  classifyGeneratedManifestChanges as classifyGeneratedManifestChangesImpl,
  groupGeneratedTestLogEntriesByFilePath as groupGeneratedTestLogEntriesByFilePathImpl,
  splitGeneratedNdjsonContent as splitGeneratedNdjsonContentImpl,
} from './local-test-log-manifest';
import {
  matchesGeneratedWipeEntry as matchesGeneratedWipeEntryImpl,
  selectGeneratedPruneCandidates as selectGeneratedPruneCandidatesImpl,
} from './local-test-log-retention';
import { matchesGeneratedAppLogQuery as matchesGeneratedAppLogQueryImpl } from './local-test-log-query';

export const GeneratedLocalLogRootEnv = 'OCENTRA_PARENT_LOG_DIR';

export const GeneratedLocalLogDirs = {
  TestLogs: 'test-logs',
  AppLogs: 'app-logs',
  Db: 'db',
  Manifests: 'manifests',
} as const;

export interface GeneratedLogsTreeScope {
  readonly scope: TestLogScope | string;
  readonly runType: RunType | string;
  readonly suiteType: TestSuiteType | string | null;
}

export interface GeneratedWipeNdjsonScopeOptions {
  readonly scope: TestLogScope;
  readonly runType?: RunType | null;
  readonly suiteType?: TestSuiteType | null;
  readonly runId?: string | null;
  readonly filePath?: string | null;
}

export interface GeneratedPrunableFile {
  readonly filePath: string;
  readonly modifiedMs: number;
}

export interface GeneratedManifestEntry {
  readonly size: number;
  readonly modifiedMs: number;
  readonly sha256: string;
}

export interface GeneratedIngestManifest {
  readonly scope: TestLogScope;
  readonly updatedAt: number;
  readonly files: Record<string, GeneratedManifestEntry>;
}

export interface GeneratedObservedFileState {
  readonly resolvedPath: string;
  readonly size: number;
  readonly modifiedMs: number;
  readonly sha256: string;
}

export const sanitizeGeneratedPathSegment = sanitizeGeneratedPathSegmentImpl;
export const sanitizeGeneratedTestNameForNdjson = sanitizeGeneratedTestNameForNdjsonImpl;
export const resolveGeneratedLocalLogRoot = resolveGeneratedLocalLogRootImpl;
export const getGeneratedTestLogScopeDir = getGeneratedTestLogScopeDirImpl;
export const getGeneratedRunNdjsonFilePath = getGeneratedRunNdjsonFilePathImpl;
export const getGeneratedAppLogScopeDir = getGeneratedAppLogScopeDirImpl;
export const getGeneratedAppSessionFilePath = getGeneratedAppSessionFilePathImpl;
export const getGeneratedDbDir = getGeneratedDbDirImpl;
export const getGeneratedManifestDir = getGeneratedManifestDirImpl;
export const getGeneratedManifestPath = getGeneratedManifestPathImpl;
export const buildGeneratedLogsTreeKey = buildGeneratedLogsTreeKeyImpl;
export const getGeneratedRunDirPath = getGeneratedRunDirPathImpl;
export const groupGeneratedTestLogEntriesByFilePath = groupGeneratedTestLogEntriesByFilePathImpl;
export const splitGeneratedNdjsonContent = splitGeneratedNdjsonContentImpl;
export const matchesGeneratedWipeEntry = matchesGeneratedWipeEntryImpl;
export const selectGeneratedPruneCandidates = selectGeneratedPruneCandidatesImpl;
export const classifyGeneratedManifestChanges = classifyGeneratedManifestChangesImpl;
export const buildGeneratedManifest = buildGeneratedManifestImpl;
export const matchesGeneratedAppLogQuery = matchesGeneratedAppLogQueryImpl;
