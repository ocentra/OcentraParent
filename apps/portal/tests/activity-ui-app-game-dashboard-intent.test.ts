import { describe, expect, it } from 'vitest';
import { ActivitySurfaceSchemaVersion } from '@ocentra-parent/activity-domain/activity-surface';
import { createParentPortalActivityUiIntent } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent';

const ActivityRequest = {
  schemaVersion: ActivitySurfaceSchemaVersion,
  scope: {
    scopeKind: 'device',
    familyId: null,
    deviceId: 'child-device-1',
  },
  requestedAt: '2026-06-01T15:00:00Z',
  rangeStart: '2026-06-01T00:00:00Z',
  rangeEnd: '2026-06-01T15:00:00Z',
} as const;

describe('parent portal app/game dashboard intent', () => {
  it('separates inventory, running, foreground, launcher, unknown, manual, and evidence state', () => {
    const intent = createParentPortalActivityUiIntent(
      {
        activityAppUseReadModel: adapterResult(appUseReadModel()),
        activityGamesReadModel: adapterResult(gamesReadModel()),
      },
      3
    );

    expect(intent.appGameDashboard.summary).toContain('4 service-backed app/game rows');
    expect(intent.appGameDashboard.metrics.map((metric) => [metric.label, metric.value])).toContainEqual([
      'Inventory',
      '3',
    ]);
    expect(intent.appGameDashboard.metrics.map((metric) => [metric.label, metric.value])).toContainEqual([
      'Running',
      '4',
    ]);
    expect(intent.appGameDashboard.metrics.map((metric) => [metric.label, metric.value])).toContainEqual([
      'Foreground',
      '2',
    ]);
    expect(intent.appGameDashboard.metrics.map((metric) => [metric.label, metric.value])).toContainEqual([
      'Launcher',
      '1',
    ]);
    expect(intent.appGameDashboard.rows.map((row) => [row.label, row.inventoryCount, row.runningCount])).toContainEqual(
      ['Study Timer', 1, 1]
    );
    expect(
      intent.appGameDashboard.rows.map((row) => [row.label, row.foregroundCount, row.evidenceCount])
    ).toContainEqual(['Study Timer', 1, 2]);
    expect(
      intent.appGameDashboard.rows.map((row) => [row.label, row.launcherOnly, row.unknownApproval])
    ).toContainEqual(['Steam Launcher', true, false]);
    expect(
      intent.appGameDashboard.rows.map((row) => [row.label, row.manualRequired, row.riskCandidate])
    ).toContainEqual([
      'VPN Proxy Portable <script>alert(1)</script> with a display name that is deliberately too long for one row',
      true,
      true,
    ]);
    expect(intent.appGameDashboard.rows.some((row) => row.label.includes('<script>alert(1)</script>'))).toBe(true);
    expect(intent.appGameDashboard.capabilityRows.map((row) => row.label)).toContain('manual-required');
    expect(intent.appGameDashboard.evidenceRows.map((row) => row.value).join(' ')).toContain('refs');
  });

  it('does not create dashboard rows when app/game adapter data is absent or failed', () => {
    const intent = createParentPortalActivityUiIntent(
      {
        activityAppUseReadModel: { ok: false, reason: 'invalid-json', state: 'unavailable' },
        activityGamesReadModel: null,
      },
      2
    );

    expect(intent.appGameDashboard.rows).toEqual([]);
    expect(intent.appGameDashboard.emptyMessage).toContain('No app/game read model rows');
    expect(intent.appGameDashboard.metrics.map((metric) => [metric.label, metric.value])).toContainEqual([
      'Manual required',
      '0',
    ]);
  });
});

function appUseReadModel() {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: ActivityRequest,
    state: 'ready',
    generatedAt: '2026-06-01T15:00:01Z',
    summary: 'App-use read model from service projection',
    rows: [
      {
        rowId: 'app-row-study-timer',
        appName: 'Study Timer',
        deviceId: 'child-device-1',
        state: 'ready',
        productKind: 'native-app',
        classificationState: 'known-app',
        inventoryState: 'installed',
        runtimeState: 'running',
        foregroundState: 'foreground',
        capabilityStatus: 'ready',
        lastObservedAt: '2026-06-01T15:00:00Z',
        totalMs: 900000,
        launchCount: 2,
        inventoryRowCount: 1,
        runningRowCount: 1,
        foregroundRowCount: 1,
        dailyRollupCount: 1,
        evidence: [
          { evidenceId: 'app-ev-1', sourceId: 'journal', capturedAt: '2026-06-01T15:00:00Z' },
          { evidenceId: 'app-ev-2', sourceId: 'sqlite', capturedAt: '2026-06-01T15:00:00Z' },
        ],
      },
      {
        rowId: 'app-row-malicious-name',
        appName:
          'VPN Proxy Portable <script>alert(1)</script> with a display name that is deliberately too long for one row',
        deviceId: 'child-device-2',
        state: 'manual-required',
        productKind: 'native-app',
        classificationState: 'unknown-process',
        inventoryState: 'stale',
        runtimeState: 'running',
        foregroundState: 'not-foreground',
        capabilityStatus: 'manual-required',
        lastObservedAt: '2026-06-01T14:58:00Z',
        totalMs: 120000,
        launchCount: 1,
        inventoryRowCount: 0,
        runningRowCount: 1,
        foregroundRowCount: 0,
        dailyRollupCount: 0,
        evidence: [{ evidenceId: 'app-ev-3', sourceId: 'sqlite', capturedAt: '2026-06-01T14:58:00Z' }],
      },
    ],
  } as const;
}

function gamesReadModel() {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: ActivityRequest,
    state: 'manual-required',
    generatedAt: '2026-06-01T15:00:01Z',
    summary: 'Games read model from service projection',
    rows: [
      {
        rowId: 'game-row-launcher',
        displayName: 'Steam Launcher',
        deviceId: 'child-device-2',
        state: 'manual-required',
        productKind: 'launcher',
        classificationState: 'known-launcher',
        inventoryState: 'installed',
        runtimeState: 'running',
        foregroundState: 'not-foreground',
        capabilityStatus: 'manual-required',
        lastObservedAt: '2026-06-01T14:57:00Z',
        totalMs: 600000,
        sessionCount: 1,
        launcherRowCount: 1,
        runningRowCount: 1,
        foregroundRowCount: 0,
        dailyRollupCount: 1,
        evidence: [{ evidenceId: 'game-ev-1', sourceId: 'journal', capturedAt: '2026-06-01T14:57:00Z' }],
      },
      {
        rowId: 'game-row-candidate',
        displayName: 'Voxel Quest Candidate',
        deviceId: 'child-device-2',
        state: 'manual-required',
        productKind: 'native-game',
        classificationState: 'possible-game',
        inventoryState: 'detected',
        runtimeState: 'running',
        foregroundState: 'foreground',
        capabilityStatus: 'permission-required',
        lastObservedAt: '2026-06-01T14:56:00Z',
        totalMs: 300000,
        sessionCount: 1,
        launcherRowCount: 0,
        runningRowCount: 1,
        foregroundRowCount: 1,
        dailyRollupCount: 0,
        evidence: [{ evidenceId: 'game-ev-2', sourceId: 'sqlite', capturedAt: '2026-06-01T14:56:00Z' }],
      },
    ],
  } as const;
}

function adapterResult(value: Record<string, unknown>) {
  return {
    ok: true,
    state: value['state'] ?? 'ready',
    value,
  } as const;
}
