import { AppGameSchemaVersion } from '@ocentra-parent/app-game-domain/app-game';
import { describe, expect, it } from 'vitest';
import { AgentEvent, type AgentEventEnvelope } from '../../src/contracts';
import { AgentProtocolSchemaVersion } from '@ocentra-parent/schema-domain/event-primitives';
import {
  AgentAppGamePlatformProofStatusHostCapability,
  AgentAppGamePlatformProofStatusPayloadField,
  AgentAppGamePlatformProofStatusState,
  parseAgentAppGamePlatformProofStatusEvent,
} from '../../src/app-game-platform-proof-status';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const PlatformProofStatusReadModel = {
  schemaVersion: AppGameSchemaVersion,
  readModelId: 'app-game-platform-proof-status',
  generatedAt: '2026-06-08T17:10:00.000Z',
  sourceReadModelIds: ['v0-8-supported-adapter-runtime-proof'],
  custodyLabel: 'app-game-platform-proof-status',
  capabilityStatus: 'app-game-platform-proof-status-partial',
  returned: 2,
  hostVisibleCount: 1,
  hostNotDetectedCount: 1,
  localRuntimeNotApplicableCount: 0,
  enforcementReadyCount: 0,
  openGapCount: 7,
  adapterDispatchClaimed: false,
  broadInstalledAppBlockingClaimed: false,
  platformEnforcementClaimed: false,
  providerDeliveryClaimed: false,
  childDeviceDeliveryClaimed: false,
  privateDiagnosticsClaimed: false,
  rows: [
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: 'app-game-platform-proof-status-windows',
      platform: 'windows',
      proofState: AgentAppGamePlatformProofStatusState.ScopedWindowsExecutionProved,
      authorityState: 'scoped-execution-only',
      hostCapabilityState: AgentAppGamePlatformProofStatusHostCapability.Available,
      hostCapabilityEvidenceRefs: ['adapter-capability-state-ref'],
      hostCapabilityProbeRefs: ['windows-host-local-probe-ref'],
      productMeanings: ['native-app', 'native-game'],
      proofRefs: ['app-game-session-evidence-ref'],
      openGaps: [
        'broad-installed-app-blocking-not-proved',
        'platform-enforcement-not-proved',
        'child-device-delivery-not-proved',
      ],
      adapterDispatchClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      childDeviceDeliveryClaimed: false,
      childDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      lastCheckedAt: '2026-06-08T17:10:00.000Z',
    },
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: 'app-game-platform-proof-status-android',
      platform: 'android',
      proofState: AgentAppGamePlatformProofStatusState.AndroidHostNotDetected,
      authorityState: 'visibility-only',
      hostCapabilityState: AgentAppGamePlatformProofStatusHostCapability.NotDetected,
      hostCapabilityEvidenceRefs: [],
      hostCapabilityProbeRefs: [],
      productMeanings: ['native-app', 'native-game'],
      proofRefs: [
        'android-adb-host-toolchain-ref',
        'android-physical-device-proof-ref',
        'android-usage-events-foreground-ref',
      ],
      openGaps: [
        'android-device-owner-not-proved',
        'android-durable-usage-events-replay-not-proved',
        'platform-enforcement-not-proved',
        'child-device-delivery-not-proved',
      ],
      adapterDispatchClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      childDeviceDeliveryClaimed: false,
      childDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      lastCheckedAt: '2026-06-08T17:10:00.000Z',
    },
  ],
} as const;

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
            hostVisibleCount: 0,
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
      [AgentAppGamePlatformProofStatusPayloadField]: serializedReadModel,
    },
    snapshot: null,
  };
}
