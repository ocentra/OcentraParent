/* generated from crates/logging-core/src/parent_log_runtime.rs */

export function generatedStaleRunInfoWarning(
  runId: string | null,
  startedAt: number | null,
  now = Date.now()
): string | null {
  if (runId == null || startedAt == null) {
    return null;
  }
  return now - startedAt > 5 * 60 * 1000 ? 'previous run info was stale and has been replaced' : null;
}

export function generatedHasRunInfoConflict(
  runInfo: { readonly runId: string | null; readonly scope: string | null },
  entries: readonly { readonly runId?: string | null; readonly consumer: string | null }[]
): boolean {
  return (
    runInfo.runId != null &&
    runInfo.scope != null &&
    entries.some((entry) => entry.consumer === runInfo.scope && entry.runId !== runInfo.runId)
  );
}

export function buildGeneratedRunStartedPayload(payload: {
  readonly runId: string;
  readonly runType?: string;
  readonly suiteType?: string | null;
  readonly scope?: string | null;
  readonly filePath?: string | null;
  readonly wipeAll?: boolean;
}) {
  return {
    runId: payload.runId,
    runType: payload.runType ?? 'single',
    suiteType: payload.suiteType ?? null,
    scope: payload.scope ?? null,
    filePath: payload.filePath ?? null,
    wipeAll: payload.wipeAll ?? false,
  };
}
