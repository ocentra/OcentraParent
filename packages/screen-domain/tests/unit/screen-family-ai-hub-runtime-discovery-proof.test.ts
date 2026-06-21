import { describe, expect, it } from 'vitest';
import { ScreenFamilyAiHubRuntimeDiscoveryReadModelSchema } from '@ocentra-parent/schema-domain/screen-family-ai-hub-runtime-discovery-proof';

const ReadModel = {
  schemaVersion: 'screen-family-ai-hub-runtime-discovery-proof',
  lanSchemaVersion: 'v0.9',
  discovery: {
    runtimeState: 'runtime-discovered',
    householdLanState: 'loopback-runtime-proof',
    cloudRelayState: 'not-implemented',
    discoveredAt: '2026-06-05T18:05:00.000Z',
    runtimeEndpointRef: 'loopback-family-ai-hub-runtime',
    discoveryEvidence: [
      discoveryEvidence('screen-family-hub-hello', 'child-agent-hello', 'child-agent-presence'),
      discoveryEvidence('screen-family-hub-heartbeat', 'child-agent-heartbeat', 'child-agent-presence'),
      discoveryEvidence('screen-family-hub-route', 'local-service', 'route'),
    ],
  },
  route: {
    routeId: 'screen-family-hub-proof-selected',
    lanRouteId: 'household-lan-screen-family-hub-route',
    routeExecutionState: 'selected',
    destinationCustodyState: 'live-lan-child-agent',
    localProviderAttempted: true,
    parentApprovedFamilyHub: true,
    remoteApiFallbackAllowed: false,
    rawImageRetentionAllowed: false,
    ocentraHostedProcessingAllowed: false,
  },
  exchange: {
    exchangeState: 'completed',
    transferMode: 'redactedCrop',
    requestEvidenceRef: 'screen-family-hub-runtime-request',
    responseEvidenceRef: 'screen-family-hub-runtime-response',
    rawFullScreenshotTransferred: false,
    rawImageRetained: false,
    remoteProviderUsed: false,
    ocentraHostedProcessingUsed: false,
  },
  claimBoundaries: [
    'loopback runtime discovery is proved by this branch',
    'physical household LAN proof remains manual-required',
    'cloud relay remains not implemented',
    'production model quality is not claimed',
  ],
  updatedAt: '2026-06-05T18:05:01.000Z',
} as const;

describe('screen family AI hub runtime discovery proof contracts', () => {
  it('accepts a loopback runtime discovery proof linked to the selected screen family hub route', () => {
    const parsed = ScreenFamilyAiHubRuntimeDiscoveryReadModelSchema.parse(ReadModel);

    expect(parsed.discovery.runtimeState).toBe('runtime-discovered');
    expect(parsed.discovery.discoveryEvidence.map((record) => record.source)).toEqual([
      'child-agent-hello',
      'child-agent-heartbeat',
      'local-service',
    ]);
    expect(parsed.route.destinationCustodyState).toBe('live-lan-child-agent');
    expect(parsed.exchange.rawFullScreenshotTransferred).toBe(false);
  });

  it('rejects discovery without hello, heartbeat, and route evidence', () => {
    const missingRouteEvidence = {
      ...ReadModel,
      discovery: {
        ...ReadModel.discovery,
        discoveryEvidence: [
          discoveryEvidence('screen-family-hub-hello', 'child-agent-hello', 'child-agent-presence'),
          discoveryEvidence('screen-family-hub-heartbeat', 'child-agent-heartbeat', 'child-agent-presence'),
        ],
      },
    };

    expect(ScreenFamilyAiHubRuntimeDiscoveryReadModelSchema.safeParse(missingRouteEvidence).success).toBe(false);
  });

  it('rejects raw image transfer or retention upgrades', () => {
    const rawTransfer = {
      ...ReadModel,
      exchange: {
        ...ReadModel.exchange,
        rawFullScreenshotTransferred: true,
      },
    };
    const rawRetention = {
      ...ReadModel,
      exchange: {
        ...ReadModel.exchange,
        rawImageRetained: true,
      },
    };

    expect(ScreenFamilyAiHubRuntimeDiscoveryReadModelSchema.safeParse(rawTransfer).success).toBe(false);
    expect(ScreenFamilyAiHubRuntimeDiscoveryReadModelSchema.safeParse(rawRetention).success).toBe(false);
  });

  it('rejects remote API or Ocentra-hosted processing upgrades', () => {
    const remoteApi = {
      ...ReadModel,
      exchange: {
        ...ReadModel.exchange,
        remoteProviderUsed: true,
      },
    };
    const hosted = {
      ...ReadModel,
      route: {
        ...ReadModel.route,
        ocentraHostedProcessingAllowed: true,
      },
    };

    expect(ScreenFamilyAiHubRuntimeDiscoveryReadModelSchema.safeParse(remoteApi).success).toBe(false);
    expect(ScreenFamilyAiHubRuntimeDiscoveryReadModelSchema.safeParse(hosted).success).toBe(false);
  });
});

function discoveryEvidence(
  evidenceId: string,
  source: 'child-agent-hello' | 'child-agent-heartbeat' | 'local-service',
  evidenceKind: 'child-agent-presence' | 'route'
) {
  return {
    schemaVersion: 'v0.9',
    evidenceId,
    source,
    evidenceKind,
    deviceId: 'family-ai-hub-device',
    value: `${source}:${evidenceKind}`,
    normalizedValue: `${source}:${evidenceKind}`,
    firstSeenAt: '2026-06-05T18:04:00.000Z',
    lastSeenAt: '2026-06-05T18:05:00.000Z',
    expiresAt: '2026-06-05T18:10:00.000Z',
    confidence: 'confirmed',
    mergeKey: `family-ai-hub:${source}:${evidenceKind}`,
    note: null,
  } as const;
}
