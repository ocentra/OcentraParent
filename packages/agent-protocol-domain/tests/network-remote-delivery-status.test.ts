import { describe, expect, it } from 'vitest';
import { AgentEvent } from '../src/contracts';
import { AgentProtocolDefaults } from '../src/defaults';
import {
  parseAgentNetworkRemoteDeliveryStatusEvent,
  type AgentNetworkRemoteDeliveryStatus,
} from '../src/network-remote-delivery-status';

const RemoteDeliveryStatusRefs = AgentProtocolDefaults.NetworkRemoteDeliveryStatus;

const RemoteDeliveryStatus = {
  statusRef: RemoteDeliveryStatusRefs.StatusRef,
  brokerStatus: 'fixture-requirements-recorded-but-not-implemented',
  familyHubStatus: 'fixture-requirements-recorded-but-not-implemented',
  custodyProofRef: 'broker.network.custody-proof.1',
  publisherAuthRef: 'broker.network.publisher-auth.1',
  subscriberAuthRef: 'broker.network.subscriber-auth.1',
  encryptionRef: 'broker.network.encryption.1',
  retentionPolicyRef: 'broker.network.retention-policy.1',
  replayPlanRef: 'broker.network.replay-plan.1',
  deletionPlanRef: 'broker.network.deletion-plan.1',
  offsetPolicyRef: 'broker.network.offset-policy.1',
  dedupePolicyRef: 'broker.network.dedupe-policy.1',
  transportConfigRef: 'broker.network.config.1',
  relayIdentityRef: 'family-hub.network.identity.1',
  relayPolicyRef: 'family-hub.network.relay-policy.1',
  brokerMissingArtifactCount: 0,
  familyHubMissingArtifactCount: 0,
  acceptedEventTypeCount: 3,
  localIdempotencyQueueProved: true,
  droppedEventDeadLetterCount: 1,
  queuedDuplicateRejected: true,
  completedDuplicateRejected: true,
  eventChainJournalRef: RemoteDeliveryStatusRefs.EventChainJournalRef,
  receiptLedgerRef: RemoteDeliveryStatusRefs.ReceiptLedgerRef,
  localReceiptAckRef: RemoteDeliveryStatusRefs.LocalReceiptAckRef,
  durableEnvelopeRef: RemoteDeliveryStatusRefs.DurableEnvelopeRef,
  durableStoreRef: RemoteDeliveryStatusRefs.DurableStoreRef,
  durableReplayRef: RemoteDeliveryStatusRefs.DurableReplayRef,
  durableDeleteExportRef: RemoteDeliveryStatusRefs.DurableDeleteExportRef,
  durableSupportStatusRef: RemoteDeliveryStatusRefs.DurableSupportStatusRef,
  durableEnvelopeReady: true,
  durableEnvelopeMissingArtifactCount: 0,
  brokerDeliveryImplemented: false,
  familyHubDeliveryImplemented: false,
  remoteDeliveryAckImplemented: false,
  providerDeliveryImplemented: false,
  childDeviceDeliveryImplemented: false,
  crossProcessReplayImplemented: false,
  remoteDeleteExportPropagationImplemented: false,
  productReadyRemoteDelivery: false,
  policyAuthority: false,
  sideEffectAuthority: false,
  enforcementCommandEventCount: 0,
  adapterActionExecutedCount: 0,
  rawPcapAvailableCount: 0,
  exactUrlAvailableCount: 0,
  decryptedPayloadAvailableCount: 0,
  pageContentAvailableCount: 0,
  videoContentAvailableCount: 0,
  privateMessageContentAvailableCount: 0,
  searchQueryAvailableCount: 0,
} satisfies AgentNetworkRemoteDeliveryStatus;

describe('agent network remote delivery status contract', () => {
  it('parses row10f status from a typed agent event', () => {
    const parsed = parseAgentNetworkRemoteDeliveryStatusEvent(
      eventWithPayload({
        [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify(RemoteDeliveryStatus),
      })
    );

    expect(parsed).toEqual({ ok: true, status: RemoteDeliveryStatus });
  });

  it('rejects live delivery, product-ready, adapter, and content claims', () => {
    expectInvalid({ ...RemoteDeliveryStatus, productReadyRemoteDelivery: true });
    expectInvalid({ ...RemoteDeliveryStatus, providerDeliveryImplemented: true });
    expectInvalid({ ...RemoteDeliveryStatus, childDeviceDeliveryImplemented: true });
    expectInvalid({ ...RemoteDeliveryStatus, enforcementCommandEventCount: 1 });
    expectInvalid({ ...RemoteDeliveryStatus, adapterActionExecutedCount: 1 });
    expectInvalid({ ...RemoteDeliveryStatus, exactUrlAvailableCount: 1 });
    expectInvalid({ ...RemoteDeliveryStatus, searchQueryAvailableCount: 1 });
  });

  it('rejects missing fields, malformed JSON, and stale row refs', () => {
    expect(parseAgentNetworkRemoteDeliveryStatusEvent(eventWithPayload({}))).toEqual({
      ok: false,
      reason: 'missing-remote-delivery-status',
    });
    expect(
      parseAgentNetworkRemoteDeliveryStatusEvent(
        eventWithPayload({ [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: '{' })
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-remote-delivery-status-json',
    });
    expectInvalid({
      ...RemoteDeliveryStatus,
      durableEnvelopeRef: 'network.remote-delivery.durable-envelope.10d',
    });
    expectInvalid({
      ...RemoteDeliveryStatus,
      statusRef: 'wrong.network.remote-delivery.status-bridge.10f',
    });
    expectInvalid({
      ...RemoteDeliveryStatus,
      durableEnvelopeRef: 'wrong.network.remote-delivery.durable-envelope.10e',
    });
    expectInvalid({
      ...RemoteDeliveryStatus,
      brokerMissingArtifactCount: 1,
    });
  });
});

function expectInvalid(value: unknown) {
  expect(
    parseAgentNetworkRemoteDeliveryStatusEvent(
      eventWithPayload({ [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify(value) })
    )
  ).toEqual({ ok: false, reason: 'invalid-remote-delivery-status' });
}

function eventWithPayload(payload: Record<string, unknown>) {
  return {
    schemaVersion: 1,
    eventId: 'network-remote-delivery-status-reported',
    correlationId: 'cmd-network-remote-delivery-status',
    sentAt: '2026-06-07T08:24:00Z',
    source: { peerId: 'local-dev-agent', role: 'agent-service' },
    target: { peerId: 'portal-dev', role: 'portal' },
    event: AgentEvent.NetworkRemoteDeliveryStatusReported,
    severity: 'info',
    payload,
    snapshot: null,
  } as const;
}
