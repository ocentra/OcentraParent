type AppGamePerformanceSurfaceRuleInput =
  | 'inventory-scan-bounds'
  | 'runtime-polling-bounds'
  | 'foreground-debounce-bounds'
  | 'journal-write-volume'
  | 'session-replay-cost'
  | 'policy-compile-cost'
  | 'portal-render-bounds'
  | 'adapter-health-degraded-state';

type AppGamePerformanceMeasurementModeRuleInput =
  | 'contract-budget-target'
  | 'generated-scale-smoke'
  | 'existing-portal-intent-smoke'
  | 'degraded-state-contract';

type AppGamePerformanceServiceHealthRuleInput = 'healthy' | 'degraded' | 'manual-required' | 'not-claimed';

type AppGamePerformanceCapabilityStateRuleInput =
  | 'supported'
  | 'unavailable'
  | 'degraded'
  | 'dry-run'
  | 'observe-only'
  | 'manual-required';

interface AppGamePerformanceHealthRuleInput {
  readonly surface: AppGamePerformanceSurfaceRuleInput;
  readonly measurementMode: AppGamePerformanceMeasurementModeRuleInput;
  readonly minimumEntityCount: number;
  readonly warningThresholdMs: number;
  readonly budgetMs: number;
  readonly serviceHealthState: AppGamePerformanceServiceHealthRuleInput;
  readonly capabilityState: AppGamePerformanceCapabilityStateRuleInput;
  readonly degradedTriggers: readonly string[];
  readonly evidenceReferences: readonly unknown[];
  readonly parentVisibleState: unknown;
  readonly noClaimBoundaries: readonly string[];
  readonly proofPackRefs: readonly unknown[];
  readonly adapterExecutionClaim: 'not-claimed';
  readonly livePlatformClaim: 'not-claimed';
}

const requiredSurfaces: readonly AppGamePerformanceSurfaceRuleInput[] = [
  'inventory-scan-bounds',
  'runtime-polling-bounds',
  'foreground-debounce-bounds',
  'journal-write-volume',
  'session-replay-cost',
  'policy-compile-cost',
  'portal-render-bounds',
  'adapter-health-degraded-state',
];

const surfaceMinimums: Readonly<Record<AppGamePerformanceSurfaceRuleInput, number>> = {
  'inventory-scan-bounds': 1000,
  'runtime-polling-bounds': 500,
  'foreground-debounce-bounds': 500,
  'journal-write-volume': 10000,
  'session-replay-cost': 100000,
  'policy-compile-cost': 1000,
  'portal-render-bounds': 500,
  'adapter-health-degraded-state': 1,
};

export function appGamePerformanceHealthRowIsHonest(row: AppGamePerformanceHealthRuleInput): boolean {
  return (
    row.evidenceReferences.length > 0 &&
    row.proofPackRefs.length > 0 &&
    row.minimumEntityCount >= surfaceMinimums[row.surface] &&
    thresholdsAreBounded(row) &&
    degradedStatesAreVisible(row) &&
    portalScaleProofIsScoped(row) &&
    noClaimBoundariesArePresent(row) &&
    row.adapterExecutionClaim === 'not-claimed' &&
    row.livePlatformClaim === 'not-claimed'
  );
}

export function appGamePerformanceHealthMatrixCoversRequiredSurfaces(
  rows: readonly AppGamePerformanceHealthRuleInput[]
): boolean {
  const surfaces = new Set(rows.map((row) => row.surface));
  return requiredSurfaces.every((surface) => surfaces.has(surface));
}

function thresholdsAreBounded(row: AppGamePerformanceHealthRuleInput): boolean {
  return (
    Number.isFinite(row.minimumEntityCount) &&
    Number.isFinite(row.warningThresholdMs) &&
    Number.isFinite(row.budgetMs) &&
    row.minimumEntityCount > 0 &&
    row.warningThresholdMs > 0 &&
    row.budgetMs >= row.warningThresholdMs
  );
}

function degradedStatesAreVisible(row: AppGamePerformanceHealthRuleInput): boolean {
  if (row.serviceHealthState !== 'degraded' && row.capabilityState !== 'degraded') {
    return true;
  }

  return row.degradedTriggers.length > 0 && row.parentVisibleState !== null;
}

function portalScaleProofIsScoped(row: AppGamePerformanceHealthRuleInput): boolean {
  if (row.surface !== 'portal-render-bounds') {
    return true;
  }

  return (
    row.measurementMode === 'existing-portal-intent-smoke' &&
    row.noClaimBoundaries.includes('portal-intent-not-browser-dom-render-proof')
  );
}

function noClaimBoundariesArePresent(row: AppGamePerformanceHealthRuleInput): boolean {
  return (
    row.noClaimBoundaries.includes('fixture-scale-not-live-load-test') &&
    row.noClaimBoundaries.includes('no-platform-adapter-execution') &&
    row.noClaimBoundaries.includes('not-product-complete-performance-claim')
  );
}
