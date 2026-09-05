import { describe, expect, it } from 'vitest';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
import {
  createParentPortalActivityUiIntent,
  type ParentPortalActivityStateLike,
} from '../../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent';
import { resolveSnapshotLiveActivityState } from '../../src/route-live-activity-state';
import type {
  ParentAgentActivityAppUseReadModelRow,
  ParentAgentActivityGamesReadModelRow,
  ParentAgentActivitySurfaceRequest,
  ParentRouteLiveActivitySnapshot,
} from '../../generated/parent-ui-bridge';

const log = Logger.instance;
log.register(import.meta.url);

const platforms = ['macos', 'ios', 'android', 'linux'] as const;

function platformRows(): readonly Record<string, unknown>[] {
  return platforms.map((platform) => ({
    platform,
    state: 'manual-required',
    setupState: 'manual-required',
    proofPackState: 'manual-proof-pack-required',
    authorityTier: 'visibility-only',
    adapterExecutionClaim: 'not-executed',
    broadBlockingClaimed: false,
    privilegedMobileClaimed: false,
    childDeviceDeliveryClaimed: false,
    requiredProofRefs: [
      `${platform}-setup-proof`,
      `${platform}-inventory-proof`,
      `${platform}-blocking-proof`,
      `${platform}-rollback-proof`,
    ],
  }));
}

function exactMetric(
  metrics: readonly { readonly label: string; readonly value: string; readonly tone: string }[],
  label: string
) {
  const matches = metrics.filter((metric) => metric.label === label);
  expect(matches).toHaveLength(1);
  return matches[0];
}

function dashboardForSnapshot(snapshot: ParentRouteLiveActivitySnapshot) {
  const state = resolveSnapshotLiveActivityState(snapshot);
  const appUseReadModel = state.activityAppUseReadModel;
  const gamesReadModel = state.activityGamesReadModel;
  const activityState: ParentPortalActivityStateLike = {
    ...(appUseReadModel === undefined ? {} : { activityAppUseReadModel: appUseReadModel }),
    // The generated bridge exposes this extension as an unknown record. The
    // route decoder has already validated and stripped everything except the
    // current Rust-owned adapter result before this UI boundary consumes it.
    activityAppGamePlatformExtensionReadModel: parentPortalActivityAdapterResult(
      state.activityAppGamePlatformExtensionReadModel
    ),
    ...(gamesReadModel === undefined ? {} : { activityGamesReadModel: gamesReadModel }),
  };
  return createParentPortalActivityUiIntent(activityState, 3).appGameDashboard;
}

type ParentPortalActivityAdapterResult = NonNullable<
  ParentPortalActivityStateLike['activityAppGamePlatformExtensionReadModel']
>;

function parentPortalActivityAdapterResult(value: unknown): ParentPortalActivityAdapterResult | null {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) {
    return null;
  }
  return value;
}

function platformCapabilitySnapshot(): ParentRouteLiveActivitySnapshot {
  return {
    activityAppUseReadModel: activityAppUseAdapterSnapshot([
      activityAppUseRow({
        rowId: 'app-row-1',
        appName: 'Focus App',
        classificationState: 'known',
        inventoryState: 'installed',
        runtimeState: 'not-running',
        foregroundState: 'not-foreground',
        capabilityStatus: 'ready',
        launchCount: 1,
        inventoryRowCount: 1,
        runningRowCount: 0,
        foregroundRowCount: 0,
        evidenceId: 'app-evidence-1',
      }),
    ]),
    activityAppGamePlatformExtensionReadModel: {
      ok: true,
      value: {
        schemaVersion: 1,
        state: 'manual-required',
        generatedAt: '2026-08-28T12:00:00Z',
        summary: 'App/game platform extension proof-pack readiness from service projection',
        rows: platformRows(),
      },
    },
    activityGamesReadModel: activityGamesAdapterSnapshot([
      activityGamesRow({
        rowId: 'game-row-1',
        displayName: 'Focus Game',
        classificationState: 'known',
        inventoryState: 'installed',
        runtimeState: 'not-running',
        foregroundState: 'not-foreground',
        capabilityStatus: 'ready',
        sessionCount: 1,
        inventoryRowCount: 1,
        launcherRowCount: 0,
        runningRowCount: 0,
        foregroundRowCount: 0,
        evidenceId: 'game-evidence-1',
      }),
    ]),
  };
}

type AppGameDashboard = ReturnType<typeof dashboardForSnapshot>;

function expectPlatformCapabilityRows(dashboard: AppGameDashboard): void {
  expect(
    dashboard.platformCapabilityRows.map((row) => [
      row.platform,
      row.state,
      row.setupState,
      row.proofPackState,
      row.adapterExecutionClaim,
      row.requiredProofCount,
      row.tone,
    ])
  ).toEqual([
    ['macos', 'manual-required', 'manual-required', 'manual-proof-pack-required', 'not-executed', 4, 'gold'],
    ['ios', 'manual-required', 'manual-required', 'manual-proof-pack-required', 'not-executed', 4, 'gold'],
    ['android', 'manual-required', 'manual-required', 'manual-proof-pack-required', 'not-executed', 4, 'gold'],
    ['linux', 'manual-required', 'manual-required', 'manual-proof-pack-required', 'not-executed', 4, 'gold'],
  ]);
}

function expectPlatformCapabilityMetrics(dashboard: AppGameDashboard): void {
  expect(exactMetric(dashboard.metrics, 'Platform gaps')).toEqual({ label: 'Platform gaps', value: '4', tone: 'gold' });
  expect(exactMetric(dashboard.metrics, 'Adapter executed')).toEqual({
    label: 'Adapter executed',
    value: '0',
    tone: 'gold',
  });
}

function expectPlatformCapabilityEvidence(dashboard: AppGameDashboard): void {
  expect(dashboard.evidenceRows).toContainEqual({
    label: 'macos platform proof',
    value:
      'manual-required; manual-proof-pack-required; 4 proof refs; adapter not-executed; broad block not claimed; delivery not claimed',
    tone: 'gold',
  });
  expect(dashboard.evidenceRows).toContainEqual({
    label: 'linux platform proof',
    value:
      'manual-required; manual-proof-pack-required; 4 proof refs; adapter not-executed; broad block not claimed; delivery not claimed',
    tone: 'gold',
  });
}

function expectProviderInternalsExcluded(dashboard: AppGameDashboard): void {
  expect(JSON.stringify(dashboard)).not.toContain('providerDispatchTarget');
  expect(JSON.stringify(dashboard)).not.toContain('rawPlatformDiagnostics');
}

describe('app-game dashboard platform capability intent', () => {
  it('renders four honest limitation rows with proof counts and no provider internals', () => {
    const dashboard = dashboardForSnapshot(platformCapabilitySnapshot());

    expectPlatformCapabilityRows(dashboard);
    expectPlatformCapabilityMetrics(dashboard);
    expectPlatformCapabilityEvidence(dashboard);
    expectProviderInternalsExcluded(dashboard);
    log.logInfo(
      'app-game platform limitation rows rendered',
      getStackTrace(),
      { platformCount: dashboard.platformCapabilityRows.length, adapterExecuted: 0 },
      false
    );
  });
});

describe('app-game dashboard unavailable platform intent', () => {
  it('fails closed when the platform adapter result is unavailable', () => {
    const snapshot: ParentRouteLiveActivitySnapshot = { activityAppGamePlatformExtensionReadModel: null };

    const dashboard = dashboardForSnapshot(snapshot);

    expect(dashboard.platformCapabilityRows).toEqual([]);
    expect(dashboard.metrics).toEqual([]);
    expect(dashboard.metricsState).toBe('unavailable');
    expect(dashboard.metricsUnavailableMessage).toContain('both app-use and games read models');
    expectProviderInternalsExcluded(dashboard);
    log.logInfo('app-game platform adapter remains unavailable', getStackTrace(), { adapterExecuted: 0 }, false);
  });
});

describe('app-game dashboard hostile metadata intent', () => {
  it('keeps hostile long metadata in the exported state matrix without unsafe promotion', () => {
    const longHostileName = `${'<script>alert(1)</script>'.repeat(20)}${'x'.repeat(400)}`;
    const snapshot: ParentRouteLiveActivitySnapshot = {
      activityAppUseReadModel: activityAppUseAdapterSnapshot([
        activityAppUseRow({
          rowId: 'app-hostile-1',
          appName: longHostileName,
          classificationState: 'unknown',
          inventoryState: 'installed',
          runtimeState: 'running',
          foregroundState: 'foreground',
          capabilityStatus: 'manual-required',
          launchCount: 2,
          inventoryRowCount: 1,
          runningRowCount: 1,
          foregroundRowCount: 1,
          evidenceId: 'evidence-1',
        }),
      ]),
      activityGamesReadModel: activityGamesAdapterSnapshot([
        activityGamesRow({
          rowId: 'game-launcher-1',
          displayName: 'Launcher-only game',
          classificationState: 'launcher-only',
          inventoryState: 'installed',
          runtimeState: 'not-running',
          foregroundState: 'not-foreground',
          capabilityStatus: 'ready',
          sessionCount: 0,
          inventoryRowCount: 1,
          launcherRowCount: 1,
          runningRowCount: 0,
          foregroundRowCount: 0,
          evidenceId: null,
        }),
      ]),
    };

    const dashboard = dashboardForSnapshot(snapshot);
    const hostileRow = dashboard.appRows[0];
    const launcherRow = dashboard.gameRows[0];

    if (hostileRow === undefined) {
      throw new Error('Expected the hostile app row to be rendered.');
    }
    if (launcherRow === undefined) {
      throw new Error('Expected the launcher-only game row to be rendered.');
    }

    expect(hostileRow.label).toBe(longHostileName);
    expect(hostileRow.unknownApproval).toBe(true);
    expect(hostileRow.manualRequired).toBe(true);
    expect(hostileRow.foregroundCount).toBe(1);
    expect(hostileRow.tone).toBe('gold');
    expect(launcherRow.launcherOnly).toBe(true);
    expect(launcherRow.foregroundCount).toBe(0);
    expectProviderInternalsExcluded(dashboard);
  });
});

const ActivityRequest = {
  schemaVersion: 1,
  scope: { scopeKind: 'family', familyId: 'family-1', deviceId: null },
  requestedAt: '2026-08-28T11:59:00Z',
  rangeStart: '2026-08-27T12:00:00Z',
  rangeEnd: '2026-08-28T12:00:00Z',
} as const satisfies ParentAgentActivitySurfaceRequest;

type AppUseRowInput = Readonly<{
  rowId: string;
  appName: string;
  classificationState: string;
  inventoryState: string;
  runtimeState: string;
  foregroundState: string;
  capabilityStatus: string;
  launchCount: number;
  inventoryRowCount: number;
  runningRowCount: number;
  foregroundRowCount: number;
  evidenceId: string | null;
}>;

function activityAppUseRow(input: AppUseRowInput): ParentAgentActivityAppUseReadModelRow {
  return {
    rowId: input.rowId,
    appName: input.appName,
    deviceId: 'child-device-1',
    state: 'ready',
    productKind: 'native-app',
    classificationState: input.classificationState,
    inventoryState: input.inventoryState,
    runtimeState: input.runtimeState,
    foregroundState: input.foregroundState,
    capabilityStatus: input.capabilityStatus,
    lastObservedAt: '2026-08-28T12:00:00Z',
    totalMs: 60_000,
    launchCount: input.launchCount,
    inventoryRowCount: input.inventoryRowCount,
    runningRowCount: input.runningRowCount,
    foregroundRowCount: input.foregroundRowCount,
    dailyRollupCount: 0,
    evidenceClaimRowCount: 0,
    identityRowCount: 0,
    approvalAuthorityRowCount: 0,
    approvalActionResultRowCount: 0,
    platformAuthorityMatrixCount: 0,
    platformAuthorityRowCount: 0,
    aiClassifierResultRowCount: 0,
    sourceStatusRows: [],
    evidence: input.evidenceId === null ? [] : [activityEvidence(input.evidenceId)],
  };
}

type GamesRowInput = Readonly<{
  rowId: string;
  displayName: string;
  classificationState: string;
  inventoryState: string;
  runtimeState: string;
  foregroundState: string;
  capabilityStatus: string;
  sessionCount: number;
  inventoryRowCount: number;
  launcherRowCount: number;
  runningRowCount: number;
  foregroundRowCount: number;
  evidenceId: string | null;
}>;

function activityGamesRow(input: GamesRowInput): ParentAgentActivityGamesReadModelRow {
  return {
    rowId: input.rowId,
    displayName: input.displayName,
    deviceId: 'child-device-1',
    state: 'ready',
    productKind: 'native-game',
    classificationState: input.classificationState,
    inventoryState: input.inventoryState,
    runtimeState: input.runtimeState,
    foregroundState: input.foregroundState,
    capabilityStatus: input.capabilityStatus,
    lastObservedAt: '2026-08-28T12:00:00Z',
    totalMs: 60_000,
    sessionCount: input.sessionCount,
    launcherRowCount: input.launcherRowCount,
    runningRowCount: input.runningRowCount,
    foregroundRowCount: input.foregroundRowCount,
    dailyRollupCount: 0,
    evidenceClaimRowCount: 0,
    identityRowCount: 0,
    approvalAuthorityRowCount: 0,
    approvalActionResultRowCount: 0,
    platformAuthorityMatrixCount: 0,
    platformAuthorityRowCount: 0,
    aiClassifierResultRowCount: 0,
    sourceStatusRows: [],
    evidence: input.evidenceId === null ? [] : [activityEvidence(input.evidenceId)],
  };
}

function activityAppUseAdapterSnapshot(rows: readonly ParentAgentActivityAppUseReadModelRow[]) {
  return {
    ok: true,
    state: 'ready',
    value: {
      schemaVersion: 1,
      request: ActivityRequest,
      state: 'ready',
      generatedAt: '2026-08-28T12:00:00Z',
      summary: `${rows.length} app-use rows`,
      rows,
    },
  } as const;
}

function activityGamesAdapterSnapshot(rows: readonly ParentAgentActivityGamesReadModelRow[]) {
  return {
    ok: true,
    state: 'ready',
    value: {
      schemaVersion: 1,
      request: ActivityRequest,
      state: 'ready',
      generatedAt: '2026-08-28T12:00:00Z',
      summary: `${rows.length} games rows`,
      rows,
    },
  } as const;
}

function activityEvidence(evidenceId: string) {
  return {
    evidenceId,
    kind: 'journal-entry',
    digest: null,
    uri: null,
  } as const;
}
