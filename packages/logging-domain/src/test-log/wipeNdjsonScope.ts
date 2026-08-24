import type { RunType, TestLogScope, TestSuiteType } from './types';
import { getDefaultLogRoot, getTestLogScopeDir, listNdjsonFiles } from './ndjsonPaths';
import { readTestLogEntriesFromFile } from './ndjsonWriter';
import { matchesGeneratedWipeEntry } from '../local-test-log';
import { withLocalArtifactLock } from '../local-artifact-lock';
import type { LocalArtifactMutation } from '../local-artifact-transaction';
import { applyTestLogMutation, replacementMapEntry } from './testLogMutation';
import { normalizeWipeFileSelector } from './wipeFileSelector';

export interface WipeNdjsonScopeOptions {
  readonly scope: TestLogScope;
  readonly runType?: RunType | null;
  readonly suiteType?: TestSuiteType | null;
  readonly runId?: string | null;
  readonly filePath?: string | null;
  readonly rootDir?: string;
}

export interface WipeNdjsonScopeResult {
  readonly deletedEntries: number;
  readonly deletedFiles: string[];
  readonly rewrittenFiles: string[];
}

interface PlannedWipeMutation {
  readonly filePath: string;
  readonly deletedEntries: number;
  readonly originalEntries: ReturnType<typeof readTestLogEntriesFromFile>;
  readonly keptEntries: ReturnType<typeof readTestLogEntriesFromFile>;
}

function entryMatchesFile(entryFilePath: string | null, selector: string | null): boolean {
  if (selector == null) {
    return true;
  }
  return entryFilePath != null && normalizeWipeFileSelector(entryFilePath) === selector;
}

function matchesWipeEntry(
  entry: ReturnType<typeof readTestLogEntriesFromFile>[number],
  options: WipeNdjsonScopeOptions,
  fileSelector: string | null
): boolean {
  return (
    matchesGeneratedWipeEntry(entry, { ...options, filePath: null }) && entryMatchesFile(entry.filePath, fileSelector)
  );
}

function planWipe(options: WipeNdjsonScopeOptions, rootDir: string): PlannedWipeMutation[] {
  const scopeDir = getTestLogScopeDir(options.scope, rootDir);
  const mutations: PlannedWipeMutation[] = [];
  const selector = options.filePath == null ? null : normalizeWipeFileSelector(options.filePath);
  for (const filePath of listNdjsonFiles(scopeDir)) {
    const entries = readTestLogEntriesFromFile(filePath, rootDir);
    const keptEntries = entries.filter((entry) => !matchesWipeEntry(entry, options, selector));
    if (keptEntries.length === entries.length) {
      continue;
    }
    mutations.push({
      filePath,
      deletedEntries: entries.length - keptEntries.length,
      originalEntries: entries,
      keptEntries,
    });
  }
  return mutations;
}

function addAffectedRunIds(item: PlannedWipeMutation, affectedRunIds: Set<string>): void {
  const keptRunCounts = new Map<string, number>();
  for (const entry of item.keptEntries) {
    keptRunCounts.set(entry.runId, (keptRunCounts.get(entry.runId) ?? 0) + 1);
  }
  const seenRunCounts = new Map<string, number>();
  for (const entry of item.originalEntries) {
    const seen = (seenRunCounts.get(entry.runId) ?? 0) + 1;
    seenRunCounts.set(entry.runId, seen);
    if (seen > (keptRunCounts.get(entry.runId) ?? 0)) {
      affectedRunIds.add(entry.runId);
    }
  }
}

function logMutation(item: PlannedWipeMutation): LocalArtifactMutation {
  return item.keptEntries.length === 0
    ? { kind: 'remove', filePath: item.filePath }
    : {
        kind: 'replace',
        filePath: item.filePath,
        payload: `${item.keptEntries.map((entry) => JSON.stringify(entry)).join('\n')}\n`,
      };
}

export function wipeNdjsonScope(options: WipeNdjsonScopeOptions): WipeNdjsonScopeResult {
  const rootDir = options.rootDir ?? getDefaultLogRoot();
  return withLocalArtifactLock(rootDir, () => {
    const mutations = planWipe(options, rootDir);
    const affectedRunIds = new Set<string>();
    const replacements = new Map(mutations.map((item) => replacementMapEntry(item.filePath, item.keptEntries)));
    mutations.forEach((item) => addAffectedRunIds(item, affectedRunIds));
    const logMutations = mutations.map(logMutation);
    applyTestLogMutation(options.scope, rootDir, logMutations, replacements, affectedRunIds);

    return {
      deletedEntries: mutations.reduce((sum, mutation) => sum + mutation.deletedEntries, 0),
      deletedFiles: mutations
        .filter((mutation) => mutation.keptEntries.length === 0)
        .map((mutation) => mutation.filePath)
        .sort((left, right) => left.localeCompare(right)),
      rewrittenFiles: mutations
        .filter((mutation) => mutation.keptEntries.length > 0)
        .map((mutation) => mutation.filePath)
        .sort((left, right) => left.localeCompare(right)),
    };
  });
}
