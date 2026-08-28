import { describe, expect, it } from 'vitest';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
import {
  createParentPortalActivityUiIntent,
  type ParentPortalActivityStateLike,
} from '../../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent';
import { resolveSnapshotLiveActivityState } from '../../src/route-live-activity-state';
import type { ParentRouteLiveActivitySnapshot } from '../../generated/parent-ui-bridge';

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
    providerDispatchTarget: 'must-not-reach-dashboard',
    rawPlatformDiagnostics: 'must-not-reach-dashboard',
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
  const activityState: ParentPortalActivityStateLike = {
    activityAppUseReadModel:
      state.activityAppUseReadModel as ParentPortalActivityStateLike['activityAppUseReadModel'],
    // The generated bridge intentionally exposes this newly-added extension as
    // an unknown record; this assertion narrows only at the existing adapter
    // boundary, which performs the runtime ok/value shape check.
    activityAppGamePlatformExtensionReadModel:
      state.activityAppGamePlatformExtensionReadModel as ParentPortalActivityStateLike['activityAppGamePlatformExtensionReadModel'],
    activityGamesReadModel: state.activityGamesReadModel as ParentPortalActivityStateLike['activityGamesReadModel'],
  };
  return createParentPortalActivityUiIntent(activityState, 3).appGameDashboard;
}

describe('app-game dashboard platform capability intent', () => {
  it('renders four honest limitation rows with proof counts and no provider internals', () => {
    const snapshot: ParentRouteLiveActivitySnapshot = {
      activityAppUseReadModel: {
        ok: true,
        state: 'ready',
        value: {
          rows: [
            {
              rowId: 'app-row-1',
              appName: 'Focus App',
              deviceId: 'child-device-1',
              state: 'ready',
              productKind: 'native-app',
              classificationState: 'known',
              inventoryState: 'installed',
              runtimeState: 'not-running',
              foregroundState: 'not-foreground',
              capabilityStatus: 'ready',
              lastObservedAt: '2026-08-28T12:00:00Z',
              launchCount: 1,
              evidence: [{ evidenceId: 'app-evidence-1' }],
            },
          ],
        },
      },
      activityAppGamePlatformExtensionReadModel: {
        ok: true,
        state: 'manual-required',
        value: {
          rows: platformRows(),
          providerDispatchTarget: 'must-not-reach-dashboard',
          rawPlatformDiagnostics: 'must-not-reach-dashboard',
        },
      },
      activityGamesReadModel: {
        ok: true,
        state: 'ready',
        value: {
          rows: [
            {
              rowId: 'game-row-1',
              displayName: 'Focus Game',
              deviceId: 'child-device-1',
              state: 'ready',
              productKind: 'native-game',
              classificationState: 'known',
              inventoryState: 'installed',
              runtimeState: 'not-running',
              foregroundState: 'not-foreground',
              capabilityStatus: 'ready',
              lastObservedAt: '2026-08-28T12:00:00Z',
              sessionCount: 1,
              evidence: [{ evidenceId: 'game-evidence-1' }],
            },
          ],
        },
      },
    };

    const dashboard = dashboardForSnapshot(snapshot);

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
    expect(exactMetric(dashboard.metrics, 'Platform gaps')).toEqual({
      label: 'Platform gaps',
      value: '4',
      tone: 'gold',
    });
    expect(exactMetric(dashboard.metrics, 'Adapter executed')).toEqual({
      label: 'Adapter executed',
      value: '0',
      tone: 'gold',
    });
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
    expect(JSON.stringify(dashboard)).not.toContain('providerDispatchTarget');
    expect(JSON.stringify(dashboard)).not.toContain('rawPlatformDiagnostics');
    log.logInfo(
      'app-game platform limitation rows rendered',
      getStackTrace(),
      { platformCount: dashboard.platformCapabilityRows.length, adapterExecuted: 0 },
      false
    );
  });

  it('fails closed when the platform adapter result is unavailable', () => {
    const snapshot: ParentRouteLiveActivitySnapshot = {
      activityAppGamePlatformExtensionReadModel: {
        ok: false,
        state: 'unavailable',
        reason: 'platform proof read model unavailable',
      },
    };

    const dashboard = dashboardForSnapshot(snapshot);

    expect(dashboard.platformCapabilityRows).toEqual([]);
    expect(exactMetric(dashboard.metrics, 'Platform gaps')).toEqual({
      label: 'Platform gaps',
      value: '0',
      tone: 'cyan',
    });
    expect(exactMetric(dashboard.metrics, 'Adapter executed')).toEqual({
      label: 'Adapter executed',
      value: '0',
      tone: 'gold',
    });
    expect(JSON.stringify(dashboard)).not.toContain('providerDispatchTarget');
    expect(JSON.stringify(dashboard)).not.toContain('rawPlatformDiagnostics');
    log.logInfo('app-game platform adapter remains unavailable', getStackTrace(), { adapterExecuted: 0 }, false);
  });
});
