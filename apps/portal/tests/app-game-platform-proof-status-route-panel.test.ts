import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  type AgentEventEnvelope,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { AgentProtocolSchemaVersion } from '@ocentra-parent/schema-domain/event-primitives';
import { PortalRoute } from '@ocentra-parent/portal-domain/routes';
import { createAppGamePlatformProofStatusPanelIntent } from '@ocentra-parent/portal-domain/app-game-platform-proof-status-panel';
import { resolveLiveActivityState } from '../src/live-activity-state';
import {
  normalizeAppGamePlatformProofStatusReadModel,
  shouldRenderAppGamePlatformProofStatusRoute,
} from '../src/AppGamePlatformProofStatusRoutePanel';

const PlatformProofStatusReadModel = {
  schemaVersion: 'app-game-platform-proof-status',
  readModelId: 'app-game-platform-proof-status',
  generatedAt: '2026-06-08T17:15:00.000Z',
  platformProofObservedCount: 2,
  visibilityOnlyCount: 2,
  enforcementReadyCount: 0,
  openGapCount: 4,
  productClaim: 'Platform proof rows are visibility-only.',
  rows: [
    platformProofStatusRow('windows', 'windows-policy-preflight-observed', 1, 1),
    platformProofStatusRow('ios', 'apple-ci-artifacts-required', 0, 0),
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
        platformProofObservedCount: 2,
        enforcementReadyCount: 0,
        openGapCount: 4,
      },
    });

    const platformProofStatusReadModel =
      liveActivity.appGamePlatformProofStatusReadModel !== null &&
      liveActivity.appGamePlatformProofStatusReadModel.ok
        ? normalizeAppGamePlatformProofStatusReadModel(liveActivity.appGamePlatformProofStatusReadModel.value)
        : null;

    const intent = createAppGamePlatformProofStatusPanelIntent(platformProofStatusReadModel);

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

function platformProofStatusRow(
  platform: 'windows' | 'ios',
  proofState: 'windows-policy-preflight-observed' | 'apple-ci-artifacts-required',
  packageVisibilityCount: number,
  runtimeVisibilityCount: number
) {
  return {
    platform,
    proofState,
    authorityState: 'visibility-only',
    parentVisibleSummary: platform === 'windows' ? 'Local Windows proof observed' : 'Apple CI artifacts required',
    packageVisibilityCount,
    runtimeVisibilityCount,
    ownerProofAttached: false,
    mechanismProofAttached: false,
    rollbackProofAttached: false,
    auditProofAttached: false,
    adapterDispatchClaimed: false,
    broadBlockingClaimed: false,
    platformEnforcementClaimed: false,
    childDeliveryClaimed: false,
    proofRefs:
      platform === 'windows'
        ? ['windows-broad-blocking-authority-preflight-ref']
        : ['apple-ci-platform-proof-preflight-ref'],
    openGaps: ['cross-platform-child-delivery-not-proved', 'windows-applocker-enforce-not-proved'],
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
