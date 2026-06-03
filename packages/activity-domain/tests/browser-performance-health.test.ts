import { describe, expect, it } from 'vitest';
import { BrowserPerformanceHealthReadModelSchema } from '../src/browser';

describe('browser performance health contracts', () => {
  it('accepts measured browser budgets while keeping future routes manual', acceptsPerformanceMatrix);
  it('rejects degraded rows without a reason', rejectsDegradedRowsWithoutReason);
  it('rejects healthy read models that contain degraded rows', rejectsHealthyModelWithDegradedRows);
  it('rejects manual or future rows that claim runtime coverage', rejectsFutureRuntimeClaims);
});

function acceptsPerformanceMatrix() {
  const parsed = BrowserPerformanceHealthReadModelSchema.safeParse(performanceReadModel());

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.rows).toHaveLength(13);
    expect(parsed.data.rows.filter((row) => row.state === 'within-budget')).toHaveLength(8);
    expect(parsed.data.rows.filter((row) => row.state === 'manual-required')).toHaveLength(5);
  }
}

function rejectsDegradedRowsWithoutReason() {
  expect(
    BrowserPerformanceHealthReadModelSchema.safeParse({
      ...performanceReadModel(),
      healthState: 'degraded',
      rows: [
        {
          ...withinBudgetRow('inventory-scan', 251, 250, 1),
          state: 'degraded',
        },
      ],
    }).success
  ).toBe(false);
}

function rejectsHealthyModelWithDegradedRows() {
  expect(
    BrowserPerformanceHealthReadModelSchema.safeParse({
      ...performanceReadModel(),
      rows: [
        {
          ...withinBudgetRow('inventory-scan', 251, 250, 1),
          state: 'degraded',
          degradedReason: 'fixture exceeded budget',
        },
      ],
    }).success
  ).toBe(false);
}

function rejectsFutureRuntimeClaims() {
  expect(
    BrowserPerformanceHealthReadModelSchema.safeParse({
      ...performanceReadModel(),
      rows: [
        {
          ...manualRow('browser-game-runtime-signal-collection', 100, 'runtime signal adapter not implemented'),
          runtimeClaimed: true,
        },
      ],
    }).success
  ).toBe(false);
}

function performanceReadModel() {
  return {
    schemaVersion: 1,
    generatedAt: '2026-06-03T01:35:00Z',
    sourceId: 'browser-performance-health-proof',
    healthState: 'healthy',
    rows: [...measuredBudgetRows(), ...manualRequiredBudgetRows()],
  };
}

function measuredBudgetRows() {
  return [
    withinBudgetRow('inventory-scan', 120, 250, 1),
    withinBudgetRow('support-matrix-derivation', 20, 50, 1),
    withinBudgetRow('cdp-target-mapping-100-tabs', 80, 100, 100),
    withinBudgetRow('journal-write-per-event', 5, 20, 1),
    withinBudgetRow('sqlite-replay-10000-events', 1_200, 2_000, 10_000),
    withinBudgetRow('unmanaged-process-scan', 100, 250, 1),
    withinBudgetRow('rapid-bridge-reconnect', 200, 500, 2),
    withinBudgetRow('memory-cache-lookup-invalidation', 8, 20, 1),
  ];
}

function manualRequiredBudgetRows() {
  return [
    manualRow('portal-render-100-tabs', 500, 'portal stress proof needs UI runtime ownership'),
    manualRow('url-shape-metadata-extraction', 100, 'URL/video intelligence has not started'),
    manualRow('local-ai-queue-timeout', 30_000, 'local AI queue priority is future browser intelligence work'),
    manualRow('browser-game-runtime-signal-collection', 100, 'runtime signal collection is not implemented'),
    manualRow('cloud-gaming-heuristic-timeout', 1_000, 'cloud-gaming heuristics are not implemented'),
  ];
}

function withinBudgetRow(budgetId: string, observedMs: number, budgetMs: number, sampleSize: number) {
  return {
    budgetId,
    state: 'within-budget',
    proofSource: 'fixture',
    observedMs,
    budgetMs,
    sampleSize,
    degradedReason: null,
    manualRequiredReason: null,
    runtimeClaimed: false,
  };
}

function manualRow(budgetId: string, budgetMs: number, manualRequiredReason: string) {
  return {
    budgetId,
    state: 'manual-required',
    proofSource: 'future-work',
    observedMs: null,
    budgetMs,
    sampleSize: 1,
    degradedReason: null,
    manualRequiredReason,
    runtimeClaimed: false,
  };
}
