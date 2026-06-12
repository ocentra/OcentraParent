import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
  AgentProtocolSchemaVersion,
  type AgentEventEnvelope,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { createAppGamePlatformProofStatusPanelIntent } from '@ocentra-parent/portal-domain/app-game-platform-proof-status-panel';
import { resolveLiveActivityState } from '../src/live-activity-state';
import { shouldRenderAppGamePlatformProofStatusRoute } from '../src/AppGamePlatformProofStatusRoutePanel';

const AppGameSchemaVersion = 1;

const PlatformProofStatusReadModel = {
  schemaVersion: AppGameSchemaVersion,
  readModelId: 'app-game-platform-proof-status',
  generatedAt: '2026-06-08T17:15:00.000Z',
  sourceReadModelIds: ['v0-8-supported-adapter-runtime-proof'],
  custodyLabel: 'app-game-platform-proof-status',
  capabilityStatus: 'app-game-platform-proof-status-partial',
  returned: 2,
  hostVisibleCount: 1,
  hostNotDetectedCount: 0,
  localRuntimeNotApplicableCount: 1,
  enforcementReadyCount: 0,
  openGapCount: 4,
  adapterDispatchClaimed: false,
  broadInstalledAppBlockingClaimed: false,
  platformEnforcementClaimed: false,
  providerDeliveryClaimed: false,
  childDeviceDeliveryClaimed: false,
  privateDiagnosticsClaimed: false,
  rows: [
    platformProofStatusRow('windows', 'scoped-windows-execution-proved', 'available'),
    platformProofStatusRow('ios', 'local-runtime-not-applicable', 'not-applicable'),
  ],
} as const;

describe('app-game platform proof status portal route panel', () => {
  it('attaches the renderer only to App/Game Sessions', () => {
    expect(shouldRenderAppGamePlatformProofStatusRoute(PortalRoute.AppGameSessions)).toBe(true);
    expect(shouldRenderAppGamePlatformProofStatusRoute(PortalRoute.Overview)).toBe(false);
  });

  it('uses the latest service-backed platform proof status event for the route intent', () => {
    const event = platformProofStatusEvent(JSON.stringify(PlatformProofStatusReadModel));
    const liveActivity = resolveLiveActivityState([event]);

    expect(liveActivity.appGamePlatformProofStatusReadModel).toMatchObject({
      ok: true,
      value: {
        returned: 2,
        enforcementReadyCount: 0,
        platformEnforcementClaimed: false,
      },
    });

    const intent = createAppGamePlatformProofStatusPanelIntent(
      liveActivity.appGamePlatformProofStatusReadModel?.ok === true
        ? liveActivity.appGamePlatformProofStatusReadModel.value
        : null
    );

    expect(intent.summaryDetails).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Platform proofs', value: '2' }),
        expect.objectContaining({ label: 'Host-visible rows', value: '1' }),
        expect.objectContaining({ label: 'Not-applicable rows', value: '1' }),
        expect.objectContaining({ label: 'Enforcement-ready rows', value: '0' }),
      ])
    );
    expect(intent.rows.map((row) => row.title)).toEqual(['windows', 'ios']);
    expect(intent.rows[0]?.details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Host capability', value: 'available' }),
        expect.objectContaining({ label: 'Adapter dispatch', value: 'Not claimed' }),
        expect.objectContaining({ label: 'Platform state', value: 'Not claimed' }),
      ])
    );
  });
});

function platformProofStatusRow(platform: string, proofState: string, hostCapabilityState: string) {
  return {
    schemaVersion: AppGameSchemaVersion,
    rowId: `app-game-platform-proof-status-${platform}`,
    platform,
    proofState,
    authorityState: platform === 'windows' ? 'scoped-execution-only' : 'not-locally-provable',
    hostCapabilityState,
    hostCapabilityEvidenceRefs: hostCapabilityState === 'available' ? ['adapter-capability-state-ref'] : [],
    hostCapabilityProbeRefs: hostCapabilityState === 'available' ? [`${platform}-host-local-probe-ref`] : [],
    productMeanings: ['native-app', 'native-game'],
    proofRefs: [`${platform}-platform-proof-ref`],
    openGaps: ['platform-enforcement-not-proved', 'child-device-delivery-not-proved'],
    adapterDispatchClaimed: false,
    broadInstalledAppBlockingClaimed: false,
    platformEnforcementClaimed: false,
    providerDeliveryClaimed: false,
    childDeliveryClaimed: false,
    childDeviceDeliveryClaimed: false,
    privateDiagnosticsClaimed: false,
    lastCheckedAt: '2026-06-08T17:15:00.000Z',
  };
}

function platformProofStatusEvent(serializedReadModel: string): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-platform-proof-status-event',
    correlationId: 'app-game-platform-proof-status-command',
    sentAt: '2026-06-08T17:15:01.000Z',
    source: {
      peerId: 'agent-service',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.ActivityAppGamePlatformProofStatusReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityAppGamePlatformProofStatusReadModel]: serializedReadModel,
    },
    snapshot: null,
  });
}
