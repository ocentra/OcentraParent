import { AgentProtocolSchemaVersion } from '@ocentra-parent/schema-domain/event-primitives';
import type { AppGamePlatformProofStatusReadModel } from '@ocentra-parent/schema-domain/app-game-platform-proof-status';
import { describe, expect, it } from 'vitest';
import { AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { parseAgentAppGamePlatformProofStatusEvent } from '../../src/app-game-platform-proof-status';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const PlatformProofStatusReadModel = {
  schemaVersion: 'app-game-platform-proof-status',
  readModelId: 'app-game-platform-proof-status',
  generatedAt: '2026-06-08T17:10:00.000Z',
  rows: [
    {
      platform: 'windows',
      proofState: 'windows-policy-preflight-observed',
      authorityState: 'visibility-only',
      parentVisibleSummary: 'Windows policy preflight observed.',
      packageVisibilityCount: 2,
      runtimeVisibilityCount: 1,
      ownerProofAttached: true,
      mechanismProofAttached: true,
      rollbackProofAttached: false,
      auditProofAttached: false,
      adapterDispatchClaimed: false,
      broadBlockingClaimed: false,
      platformEnforcementClaimed: false,
      childDeliveryClaimed: false,
      proofRefs: ['windows-local-policy-evidence-proof-ref', 'app-game-platform-proof-status-ref'],
      openGaps: ['windows-broad-blocking-not-proved', 'cross-platform-child-delivery-not-proved'],
    },
    {
      platform: 'android',
      proofState: 'physical-device-observed',
      authorityState: 'visibility-only',
      parentVisibleSummary: 'Android physical device observed.',
      packageVisibilityCount: 1,
      runtimeVisibilityCount: 1,
      ownerProofAttached: false,
      mechanismProofAttached: true,
      rollbackProofAttached: false,
      auditProofAttached: false,
      adapterDispatchClaimed: false,
      broadBlockingClaimed: false,
      platformEnforcementClaimed: false,
      childDeliveryClaimed: false,
      proofRefs: [
        'android-physical-device-proof-ref',
        'android-authority-preflight-ref',
        'android-usage-events-replay-ref',
      ],
      openGaps: [
        'android-device-owner-not-proved',
        'android-durable-usage-events-replay-not-proved',
        'cross-platform-child-delivery-not-proved',
      ],
    },
  ],
  platformProofObservedCount: 2,
  visibilityOnlyCount: 2,
  enforcementReadyCount: 0,
  openGapCount: 5,
  productClaim: 'visibility-only-platform-proof',
} satisfies AppGamePlatformProofStatusReadModel;

describe('agent app-game platform proof status parser', () => {
  it('parses the platform proof status read-model event payload', () => {
    const parsed = parseAgentAppGamePlatformProofStatusEvent(
      platformProofStatusEvent(JSON.stringify(PlatformProofStatusReadModel))
    );

    expect(parsed).toEqual({
      ok: true,
      value: PlatformProofStatusReadModel,
    });
  });

  it('rejects invalid platform proof status payloads and claim upgrades', () => {
    expect(
      parseAgentAppGamePlatformProofStatusEvent({
        ...platformProofStatusEvent(JSON.stringify(PlatformProofStatusReadModel)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentAppGamePlatformProofStatusEvent(platformProofStatusEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentAppGamePlatformProofStatusEvent(
        platformProofStatusEvent(
          JSON.stringify({
            ...PlatformProofStatusReadModel,
            rows: [
              {
                ...PlatformProofStatusReadModel.rows[0],
                platformEnforcementClaimed: true,
              },
            ],
          })
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
    expect(
      parseAgentAppGamePlatformProofStatusEvent(
        platformProofStatusEvent(
          JSON.stringify({
            ...PlatformProofStatusReadModel,
            visibilityOnlyCount: 1,
          })
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

function platformProofStatusEvent(serializedReadModel: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-platform-proof-status-event',
    correlationId: 'app-game-platform-proof-status-command',
    sentAt: '2026-06-08T17:10:01.000Z',
    source: Source,
    target: Target,
    event: AgentEvent.ActivityAppGamePlatformProofStatusReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityAppGamePlatformProofStatusReadModel]: serializedReadModel,
    },
    snapshot: null,
  };
}
