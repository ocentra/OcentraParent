import { describe, expect, it } from 'vitest';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
import {
  createParentPortalAppGameDashboardIntent,
  type ParentPortalAppGameDashboardIntent,
} from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/app-game-dashboard-intent';

const log = Logger.instance;
log.register(import.meta.url);

describe('parent portal app/game source freshness intent', () => {
  it('projects fresh and manual source rows into counts, evidence, and grouped sections', () => {
    const dashboard = createParentPortalAppGameDashboardIntent(appUseReadModel(), gamesReadModel());

    expectDashboardMetrics(dashboard);
    expectDashboardRows(dashboard);
    expectDashboardEvidence(dashboard);
    expectDashboardSourceSections(dashboard);
    expectDashboardRedaction(dashboard);
    logDashboardProjection(dashboard);
  });
});

describe('parent portal app/game unavailable intent', () => {
  it('fails closed when either service read model is absent or unavailable', () => {
    const dashboard = createParentPortalAppGameDashboardIntent(null, { state: 'unavailable', rows: [] });

    expect(dashboard.state).toBe('unavailable');
    expect(dashboard.rows).toEqual([]);
    expect(dashboard.sourceStatusRows).toEqual([]);
    expect(dashboard.sourcePanelSections).toEqual([]);
    expect(dashboard.metricsState).toBe('unavailable');
    expect(dashboard.metrics).toEqual([]);
    expect(dashboard.metricsUnavailableMessage).toBe(
      'Measured totals are hidden until both app-use and games read models are reported by the local service.'
    );
    expect(dashboard.emptyMessage).toBe('No app/game read model rows reported by the local service.');
    expect(dashboard.evidenceRows).toEqual([
      { label: 'Evidence refs', value: 'No evidence refs reported', tone: 'gold' },
    ]);
    log.logInfo('app-game source freshness intent remains unavailable', getStackTrace(), { rows: 0 }, false);
  });

  it('shows measured zeroes only after both service read models report an empty result', () => {
    const dashboard = createParentPortalAppGameDashboardIntent(
      { state: 'ready', rows: [] },
      { state: 'ready', rows: [] }
    );

    expect(dashboard.state).toBe('ready');
    expect(dashboard.metricsState).toBe('reported');
    expect(dashboard.metricsUnavailableMessage).toBeNull();
    expect(metricPair(dashboard, 'Inventory')).toEqual(['Inventory', '0']);
    expect(metricPair(dashboard, 'Running')).toEqual(['Running', '0']);
    expect(metricPair(dashboard, 'Source rows')).toEqual(['Source rows', '0']);
  });
});

describe('parent portal app/game source review intent', () => {
  it('keeps unobserved and degraded source statuses reviewable instead of fresh', () => {
    const staleDashboard = createParentPortalAppGameDashboardIntent(
      sourceReadModel([sourceStatusRow('processSnapshot', 'ready', 1, null, 'ready', [])]),
      null
    );
    expect(sourceSectionValues(staleDashboard)).toEqual({
      title: 'App use sources',
      state: 'stale',
      tone: 'red',
      rowCount: 1,
      freshCount: 0,
      manualRequiredCount: 0,
      evidenceCount: 0,
    });
    expect(
      staleDashboard.sourcePanelSections[0]?.rows.map((row) => [
        row.sourceStatusLabel,
        row.freshnessLabel,
        row.lastObservedLabel,
      ])
    ).toEqual([['Process Snapshot', 'Needs review', 'not observed']]);

    const degradedDashboard = createParentPortalAppGameDashboardIntent(
      sourceReadModel([
        sourceStatusRow('foregroundWindow', 'offline', 1, '2026-06-01T15:00:00Z', 'degraded', ['degraded-ref']),
      ]),
      null
    );
    expect(sourceSectionValues(degradedDashboard)).toEqual({
      title: 'App use sources',
      state: 'manual-required',
      tone: 'gold',
      rowCount: 1,
      freshCount: 0,
      manualRequiredCount: 1,
      evidenceCount: 1,
    });
    expect(
      degradedDashboard.sourcePanelSections[0]?.rows.map((row) => [
        row.sourceStatusLabel,
        row.capabilityStatus,
        row.freshnessLabel,
      ])
    ).toEqual([['Foreground Window', 'degraded', 'Needs review']]);
    log.logInfo(
      'app-game source freshness intent retained review boundaries',
      getStackTrace(),
      {
        staleFreshSources: 0,
        degradedManualRequired: 1,
      },
      false
    );
  });
});

function expectDashboardMetrics(dashboard: ParentPortalAppGameDashboardIntent): void {
  expect(dashboard.state).toBe('manual-required');
  expect(dashboard.metricsState).toBe('reported');
  expect(dashboard.metricsUnavailableMessage).toBeNull();
  expect(dashboard.summary).toContain('4 service-backed app/game rows');
  expect(metricPair(dashboard, 'Inventory')).toEqual(['Inventory', '3']);
  expect(metricPair(dashboard, 'Running')).toEqual(['Running', '4']);
  expect(metricPair(dashboard, 'Foreground')).toEqual(['Foreground', '2']);
  expect(metricPair(dashboard, 'Launcher')).toEqual(['Launcher', '1']);
  expect(metricPair(dashboard, 'Source rows')).toEqual(['Source rows', '6']);
  expect(metricPair(dashboard, 'Fresh sources')).toEqual(['Fresh sources', '4']);
  expect(metricPair(dashboard, 'Game budgets')).toEqual(['Game budgets', 'policy proof pending']);
  expect(metricPair(dashboard, 'Boundary rows')).toEqual(['Boundary rows', '10']);
  expect(metricPair(dashboard, 'AI classifier')).toEqual(['AI classifier', '2']);
  expect(metricPair(dashboard, 'Readiness blockers')).toEqual(['Readiness blockers', '8']);
}

function expectDashboardRows(dashboard: ParentPortalAppGameDashboardIntent): void {
  expect(rowValues(dashboard, 'app-row-study-timer')).toEqual({
    label: 'Study Timer',
    inventoryCount: 1,
    runningCount: 1,
    foregroundCount: 1,
    manualRequired: false,
    riskCandidate: false,
  });
  expect(rowValues(dashboard, 'game-row-launcher')).toEqual({
    label: 'Steam Launcher',
    inventoryCount: 1,
    runningCount: 1,
    foregroundCount: 0,
    manualRequired: true,
    launcherOnly: true,
    riskCandidate: false,
  });
  expect(rowValues(dashboard, 'app-row-malicious-name')).toEqual({
    label: 'VPN Proxy Portable <script>alert(1)</script> with a display name that is deliberately too long for one row',
    inventoryCount: 0,
    runningCount: 1,
    foregroundCount: 0,
    manualRequired: true,
    riskCandidate: true,
  });
}

function expectDashboardEvidence(dashboard: ParentPortalAppGameDashboardIntent): void {
  expect(
    dashboard.sourceStatusRows.map((row) => [row.parentLabel, row.sourceStatusKind, row.rowCount, row.evidenceCount])
  ).toContainEqual(['Study Timer', 'shortcut', 1, 1]);
  expect(
    dashboard.sourceStatusRows.map((row) => [row.parentLabel, row.sourceStatusLabel, row.lastObservedLabel])
  ).toContainEqual(['Steam Launcher', 'Launcher Manifest', '2026-06-01T14:57:00Z']);
  expect(dashboard.evidenceRows.map((row) => row.value).join(' ')).toContain('source rows');
  expect(dashboard.evidenceRows.map((row) => row.value).join(' ')).toContain('refs');
  expect(dashboard.evidenceRows.map((row) => row.label)).toEqual(
    expect.arrayContaining([
      'VPN Proxy Portable <script>alert(1)</script> with a display name that is deliberately too long for one row approval blocker',
      'VPN Proxy Portable <script>alert(1)</script> with a display name that is deliberately too long for one row AI review',
      'VPN Proxy Portable <script>alert(1)</script> with a display name that is deliberately too long for one row manual blocker',
      'Voxel Quest Candidate approval review',
    ])
  );
  expect(dashboard.evidenceRows.map((row) => row.value).join(' ')).toContain('evidence-only; no direct action');
  expect(dashboard.evidenceRows.map((row) => row.value).join(' ')).toContain('adapter dispatch not claimed');
}

function expectDashboardSourceSections(dashboard: ParentPortalAppGameDashboardIntent): void {
  expect(
    dashboard.sourcePanelSections.map((section) => [
      section.title,
      section.rowCount,
      section.freshCount,
      section.manualRequiredCount,
      section.evidenceCount,
      section.state,
    ])
  ).toEqual([
    ['App use sources', 4, 3, 1, 4, 'manual-required'],
    ['Game sources', 2, 1, 1, 2, 'manual-required'],
  ]);
  expect(dashboard.sourcePanelSections.map((section) => section.subtitle)).toEqual([
    '3 fresh of 4 source rows; 1 manual-required',
    '1 fresh of 2 source rows; 1 manual-required',
  ]);
  const panelRows = dashboard.sourcePanelSections.flatMap((section) =>
    section.rows.map((row) => [section.title, row.parentLabel, row.sourceStatusLabel, row.freshnessLabel])
  );
  expect(panelRows).toContainEqual(['App use sources', 'Study Timer', 'Foreground Window', 'Fresh source']);
  expect(panelRows).toContainEqual(['Game sources', 'Steam Launcher', 'Launcher Manifest', 'Needs review']);
}

function expectDashboardRedaction(dashboard: ParentPortalAppGameDashboardIntent): void {
  const serialized = JSON.stringify(dashboard);
  expect(serialized).not.toContain('C:\\Users\\child\\AppData\\Local\\Study Timer\\study-timer.exe');
  expect(serialized).not.toContain('C:\\Program Files\\VoxelQuest\\VoxelQuest.exe');
  expect(serialized).not.toContain('executablePathRef');
}

function logDashboardProjection(dashboard: ParentPortalAppGameDashboardIntent): void {
  log.logInfo(
    'app-game source freshness intent projected service rows',
    getStackTrace(),
    {
      appGameRows: dashboard.rows.length,
      sourceRows: dashboard.sourceStatusRows.length,
      freshSources: 4,
    },
    false
  );
}

function metricPair(dashboard: ParentPortalAppGameDashboardIntent, label: string): [string, string] {
  const metric = dashboard.metrics.find((candidate) => candidate.label === label);
  if (metric === undefined) throw new Error(`missing app/game dashboard metric: ${label}`);
  return [metric.label, metric.value];
}

function rowValues(dashboard: ParentPortalAppGameDashboardIntent, rowId: string): Record<string, unknown> {
  const row = dashboard.rows.find((candidate) => candidate.rowId === rowId);
  if (row === undefined) throw new Error(`missing app/game dashboard row: ${rowId}`);
  return {
    label: row.label,
    inventoryCount: row.inventoryCount,
    runningCount: row.runningCount,
    foregroundCount: row.foregroundCount,
    manualRequired: row.manualRequired,
    ...(row.launcherOnly ? { launcherOnly: true } : {}),
    riskCandidate: row.riskCandidate,
  };
}

function sourceSectionValues(dashboard: ParentPortalAppGameDashboardIntent): Record<string, unknown> {
  const section = dashboard.sourcePanelSections[0];
  if (section === undefined) throw new Error('missing app/game source panel section');
  return {
    title: section.title,
    state: section.state,
    tone: section.tone,
    rowCount: section.rowCount,
    freshCount: section.freshCount,
    manualRequiredCount: section.manualRequiredCount,
    evidenceCount: section.evidenceCount,
  };
}

function appUseReadModel(): Record<string, unknown> {
  return {
    state: 'ready',
    rows: [
      appRow('app-row-study-timer', 'Study Timer', {
        inventoryState: 'installed',
        runtimeState: 'running',
        foregroundState: 'foreground',
        launchCount: 2,
        inventoryRowCount: 1,
        runningRowCount: 1,
        foregroundRowCount: 1,
        evidence: [evidence('app-ev-1'), evidence('app-ev-2')],
        sourceStatusRows: [
          sourceStatusRow('shortcut', 'ready', 1, '2026-06-01T15:00:00Z', 'available', ['app-source-1']),
          sourceStatusRow('processSnapshot', 'ready', 1, '2026-06-01T15:00:00Z', 'available', ['app-source-2']),
          sourceStatusRow('foregroundWindow', 'ready', 1, '2026-06-01T15:00:00Z', 'available', ['app-source-3']),
        ],
      }),
      appRow(
        'app-row-malicious-name',
        'VPN Proxy Portable <script>alert(1)</script> with a display name that is deliberately too long for one row',
        {
          inventoryState: 'stale',
          runtimeState: 'running',
          capabilityStatus: 'manual-required',
          runningRowCount: 1,
          evidence: [evidence('app-ev-3')],
          executablePathRef: 'C:\\Users\\child\\AppData\\Local\\Study Timer\\study-timer.exe',
          evidenceClaimRowCount: 1,
          identityRowCount: 1,
          approvalAuthorityRowCount: 1,
          platformAuthorityRowCount: 1,
          aiClassifierResultRowCount: 1,
          sourceStatusRows: [
            sourceStatusRow('processSnapshot', 'stale', 1, '2026-06-01T14:58:00Z', 'manual-required', ['app-source-4']),
          ],
        }
      ),
    ],
  };
}

function gamesReadModel(): Record<string, unknown> {
  return {
    state: 'ready',
    rows: [
      gameRow('game-row-launcher', 'Steam Launcher', {
        productKind: 'launcher',
        inventoryState: 'installed',
        capabilityStatus: 'manual-required',
        launcherRowCount: 1,
        runningRowCount: 1,
        evidence: [evidence('game-ev-1')],
        sourceStatusRows: [
          sourceStatusRow('launcherManifest', 'permission-required', 1, '2026-06-01T14:57:00Z', 'manual-required', [
            'game-source-1',
          ]),
        ],
      }),
      gameRow('game-row-candidate', 'Voxel Quest Candidate', {
        classificationState: 'possible-game',
        inventoryState: 'detected',
        runtimeState: 'running',
        foregroundState: 'foreground',
        capabilityStatus: 'permission-required',
        sessionCount: 1,
        runningRowCount: 1,
        foregroundRowCount: 1,
        evidence: [evidence('game-ev-2')],
        executablePathRef: 'C:\\Program Files\\VoxelQuest\\VoxelQuest.exe',
        evidenceClaimRowCount: 1,
        identityRowCount: 1,
        approvalAuthorityRowCount: 1,
        platformAuthorityRowCount: 1,
        aiClassifierResultRowCount: 1,
        sourceStatusRows: [
          sourceStatusRow('foregroundWindow', 'ready', 1, '2026-06-01T14:56:00Z', 'available', ['game-source-2']),
        ],
      }),
    ],
  };
}

function sourceReadModel(sourceStatusRows: readonly Record<string, unknown>[]): Record<string, unknown> {
  return {
    state: 'ready',
    rows: [appRow('app-row-source-freshness', 'Source freshness sample', { sourceStatusRows })],
  };
}

function appRow(rowId: string, appName: string, overrides: Record<string, unknown> = {}): Record<string, unknown> {
  return { rowId, appName, deviceId: 'child-device-1', state: 'ready', productKind: 'native-app', ...overrides };
}

function gameRow(rowId: string, displayName: string, overrides: Record<string, unknown> = {}): Record<string, unknown> {
  return { rowId, displayName, deviceId: 'child-device-2', state: 'ready', productKind: 'native-game', ...overrides };
}

function sourceStatusRow(
  sourceKind: string,
  state: string,
  rowCount: number,
  lastObservedAt: string | null,
  capabilityStatus: string,
  evidenceIds: readonly string[]
): Record<string, unknown> {
  return { sourceKind, state, rowCount, lastObservedAt, capabilityStatus, evidence: evidenceIds.map(evidence) };
}

function evidence(evidenceId: string): Record<string, unknown> {
  return { evidenceId, sourceId: 'activity-store', capturedAt: '2026-06-01T15:00:00Z' };
}
