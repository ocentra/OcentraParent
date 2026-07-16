/* generated from crates/logging-core/src/parent_log_runtime.rs */

import { normalizeGeneratedDebugPath } from './parent-log-runtime-parsing';

function normalizeGeneratedSource(source: string | null | undefined): string | null {
  if (source == null || source.trim().length === 0) {
    return null;
  }
  return source.trim().toLowerCase();
}

export function matchesGeneratedDebugSelection(
  debugSources: readonly string[],
  debugFiles: readonly string[],
  debugRuns: readonly string[],
  source: string | null | undefined,
  filePath?: string | null,
  runId?: string | null,
  requestDebugSources?: readonly string[]
): boolean {
  const normalizedSource = normalizeGeneratedSource(source);
  if (
    normalizedSource != null &&
    (debugSources.includes(normalizedSource) ||
      requestDebugSources?.some((entry) => entry.trim().toLowerCase() === normalizedSource) === true)
  ) {
    return true;
  }

  if (filePath != null && filePath.trim().length > 0) {
    const normalizedFile = normalizeGeneratedDebugPath(filePath);
    if (debugFiles.some((entry) => normalizedFile.includes(entry))) {
      return true;
    }
  }

  if (runId != null && runId.trim().length > 0 && debugRuns.includes(runId.trim())) {
    return true;
  }

  return false;
}
