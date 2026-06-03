import { describe, expect, it } from 'vitest';
import { AppGamePerformanceHealthProofMatrix } from '../src/app-game-performance-health-proof';
import {
  AppGamePerformanceHealthMatrixSchema,
  AppGamePerformanceHealthRowSchema,
} from '../src/app-game-performance-health';

const rowFor = (healthCheckId: string) => {
  const row = AppGamePerformanceHealthProofMatrix.rows.find((candidate) => candidate.healthCheckId === healthCheckId);

  if (row === undefined) {
    throw new Error(`Missing app/game performance health row ${healthCheckId}`);
  }

  return row;
};

describe('app/game performance health contracts', () => {
  recordsRequiredPerformanceSurfaces();
  recordsScaleThresholdsForPlanBudgets();
  keepsPortalScaleProofScopedToExistingIntent();
  exposesDegradedStateAndRejectsHiddenAdapterFailures();
  rejectsLivePlatformAndAdapterClaims();
});

function recordsRequiredPerformanceSurfaces() {
  it('records every required app/game performance surface in one matrix', () => {
    const matrix = AppGamePerformanceHealthMatrixSchema.parse(AppGamePerformanceHealthProofMatrix);

    expect(matrix.matrixId).toBe('app-game-performance-health-proof');
    expect(matrix.rows.map((row) => row.surface)).toEqual([
      'inventory-scan-bounds',
      'runtime-polling-bounds',
      'foreground-debounce-bounds',
      'journal-write-volume',
      'session-replay-cost',
      'policy-compile-cost',
      'portal-render-bounds',
      'adapter-health-degraded-state',
    ]);
    expect(matrix.rows.every((row) => row.evidenceReferences.length > 0)).toBe(true);
  });
}

function recordsScaleThresholdsForPlanBudgets() {
  it('names the minimum scale counts required by the app and app/game workpacks', () => {
    expect(rowFor('inventory-scan-1000-budget').minimumEntityCount).toBe(1000);
    expect(rowFor('runtime-polling-500-budget').minimumEntityCount).toBe(500);
    expect(rowFor('foreground-debounce-500-budget').minimumEntityCount).toBe(500);
    expect(rowFor('journal-write-10000-budget').minimumEntityCount).toBe(10000);
    expect(rowFor('session-replay-100000-budget').minimumEntityCount).toBe(100000);
    expect(rowFor('policy-compile-1000-budget').minimumEntityCount).toBe(1000);
    expect(rowFor('portal-intent-500-row-budget').minimumEntityCount).toBe(500);
  });
}

function keepsPortalScaleProofScopedToExistingIntent() {
  it('keeps the 500-row portal scale proof scoped to existing dashboard intent, not browser DOM rendering', () => {
    const portal = rowFor('portal-intent-500-row-budget');

    expect(portal.measurementMode).toBe('existing-portal-intent-smoke');
    expect(portal.noClaimBoundaries).toContain('portal-intent-not-browser-dom-render-proof');
    expect(portal.parentVisibleState).toContain('without new portal edits');
    expect(
      AppGamePerformanceHealthRowSchema.safeParse({
        ...portal,
        noClaimBoundaries: portal.noClaimBoundaries.filter(
          (boundary) => boundary !== 'portal-intent-not-browser-dom-render-proof'
        ),
      }).success
    ).toBe(false);
  });
}

function exposesDegradedStateAndRejectsHiddenAdapterFailures() {
  it('requires degraded adapter and stale evidence states to remain parent-visible', () => {
    const degraded = rowFor('adapter-health-degraded-visible-state');

    expect(degraded.serviceHealthState).toBe('degraded');
    expect(degraded.degradedTriggers).toEqual(['adapter-error', 'stale-evidence', 'live-source-not-claimed']);
    expect(degraded.parentVisibleState).toContain('parent-visible degraded states');
    expect(
      AppGamePerformanceHealthRowSchema.safeParse({
        ...degraded,
        degradedTriggers: [],
      }).success
    ).toBe(false);
  });
}

function rejectsLivePlatformAndAdapterClaims() {
  it('rejects supported live platform or adapter execution claims in this proof row', () => {
    const policy = rowFor('policy-compile-1000-budget');
    const inventory = rowFor('inventory-scan-1000-budget');

    expect(policy.adapterExecutionClaim).toBe('not-claimed');
    expect(inventory.livePlatformClaim).toBe('not-claimed');
    expect(
      AppGamePerformanceHealthRowSchema.safeParse({
        ...policy,
        adapterExecutionClaim: 'supported',
      }).success
    ).toBe(false);
    expect(
      AppGamePerformanceHealthRowSchema.safeParse({
        ...inventory,
        livePlatformClaim: 'supported',
      }).success
    ).toBe(false);
  });
}
