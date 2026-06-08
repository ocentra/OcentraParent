import { expect, it } from 'vitest';
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
  outboxRef: RemoteDeliveryStatusRefs.OutboxRef,
  outboxHandoffRef: RemoteDeliveryStatusRefs.OutboxHandoffRef,
  outboxReplayRef: RemoteDeliveryStatusRefs.OutboxReplayRef,
  outboxSupportStatusRef: RemoteDeliveryStatusRefs.OutboxSupportStatusRef,
  transportDispatchStateRef: RemoteDeliveryStatusRefs.TransportDispatchStateRef,
  blockedDispatchRef: RemoteDeliveryStatusRefs.BlockedDispatchRef,
  futureTransportSeamRef: RemoteDeliveryStatusRefs.FutureTransportSeamRef,
  fixtureTransportRef: RemoteDeliveryStatusRefs.FixtureTransportRef,
  fixtureDispatchAttemptRef: RemoteDeliveryStatusRefs.FixtureDispatchAttemptRef,
  fixtureAckRef: RemoteDeliveryStatusRefs.FixtureAckRef,
  deleteExportPropagationRef: RemoteDeliveryStatusRefs.DeleteExportPropagationRef,
  remoteDeleteReadinessRef: RemoteDeliveryStatusRefs.RemoteDeleteReadinessRef,
  remoteExportReadinessRef: RemoteDeliveryStatusRefs.RemoteExportReadinessRef,
  providerRouteRef: RemoteDeliveryStatusRefs.ProviderRouteRef,
  childDeviceRouteRef: RemoteDeliveryStatusRefs.ChildDeviceRouteRef,
  providerDeliveryReadinessRef: RemoteDeliveryStatusRefs.ProviderDeliveryReadinessRef,
  childDeviceDeliveryReadinessRef: RemoteDeliveryStatusRefs.ChildDeviceDeliveryReadinessRef,
  transportDispatchState: 'manual-required-blocked',
  providerDeliveryReadinessState: 'manual-required-unavailable',
  childDeviceDeliveryReadinessState: 'manual-required-unavailable',
  outboxCandidateCount: 3,
  sourceOutboxCandidateCount: 3,
  preparedNotDispatchedCount: 3,
  blockedDispatchRecordCount: 3,
  blockedDispatchRecordsMatchOutboxCandidates: true,
  fixtureSourceOutboxCandidateCount: 3,
  fixtureDispatchAttemptCount: 3,
  fixtureRemoteAckCount: 3,
  fixtureRecordsMatchOutboxCandidates: true,
  deleteExportReadinessRecordCount: 3,
  remoteDeleteReadyCount: 3,
  remoteExportReadyCount: 3,
  deleteExportRecordsMatchFixtureAcks: true,
  providerDeliveryReadinessRecordCount: 3,
  childDeviceDeliveryReadinessRecordCount: 3,
  providerDeliveryArtifactCount: 0,
  childDeviceDeliveryArtifactCount: 0,
  providerDeliveryRecordsMatchFixtureAcks: true,
  childDeviceDeliveryRecordsMatchFixtureAcks: true,
  dispatchReadyCandidateCount: 0,
  dispatchAttemptCount: 0,
  remoteAckCount: 0,
  duplicateDurableEnvelopeRejected: true,
  outboxCandidatesMatchDurableEnvelopes: true,
  outboxCandidatesMatchReceipts: true,
  sequenceGapCount: 0,
  eventIdMismatchCount: 0,
  eventTypeMismatchCount: 0,
  correlationMismatchCount: 0,
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

it('parses row10n delete export status with row10k blocked dispatch refs from a typed agent event', () => {
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
  expectInvalid({ ...RemoteDeliveryStatus, dispatchAttemptCount: 1 });
  expectInvalid({ ...RemoteDeliveryStatus, remoteAckCount: 1 });
  expectInvalid({ ...RemoteDeliveryStatus, enforcementCommandEventCount: 1 });
  expectInvalid({ ...RemoteDeliveryStatus, adapterActionExecutedCount: 1 });
  expectInvalid({ ...RemoteDeliveryStatus, exactUrlAvailableCount: 1 });
  expectInvalid({ ...RemoteDeliveryStatus, searchQueryAvailableCount: 1 });
});

it('rejects missing fields and malformed JSON', () => {
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
});

it('rejects stale row refs', () => {
  expectInvalidPatch({ durableEnvelopeRef: 'network.remote-delivery.durable-envelope.10d' });
  expectInvalidPatch({ statusRef: 'network.remote-delivery.outbox-status-bridge.10h' });
  expectInvalidPatch({ statusRef: 'network.remote-delivery.transport-dispatch-state.10k' });
  expectInvalidPatch({
    statusRef: 'wrong.network.remote-delivery.delete-export-status-bridge.10n',
  });
  expectInvalidPatch({ durableEnvelopeRef: 'wrong.network.remote-delivery.durable-envelope.10e' });
  expectInvalidPatch({ outboxRef: 'network.remote-delivery.outbox.10f' });
  expectInvalidPatch({ outboxHandoffRef: 'wrong.network.remote-delivery.outbox-handoff.10g' });
  expectInvalidPatch({
    transportDispatchStateRef: 'network.remote-delivery.transport-dispatch-state.10j',
  });
  expectInvalidPatch({
    blockedDispatchRef: 'network.remote-delivery.dispatch-blocked-manual-required.10j',
  });
  expectInvalidPatch({
    futureTransportSeamRef: 'network.remote-delivery.future-transport-seam.10j',
  });
  expectInvalidPatch({ fixtureTransportRef: 'network.remote-delivery.fixture-transport.10k' });
  expectInvalidPatch({
    fixtureDispatchAttemptRef: 'network.remote-delivery.fixture-dispatch-attempt.10k',
  });
  expectInvalidPatch({ fixtureAckRef: 'network.remote-delivery.fixture-ack.10k' });
  expectInvalidPatch({
    deleteExportPropagationRef: 'network.remote-delivery.delete-export-propagation-readiness.10l',
  });
  expectInvalidPatch({
    remoteDeleteReadinessRef: 'network.remote-delivery.remote-delete-readiness.10l',
  });
  expectInvalidPatch({
    remoteExportReadinessRef: 'network.remote-delivery.remote-export-readiness.10l',
  });
  expectInvalidPatch({ providerRouteRef: 'network.remote-delivery.provider-route.10o' });
  expectInvalidPatch({ childDeviceRouteRef: 'network.remote-delivery.child-device-route.10o' });
  expectInvalidPatch({ providerDeliveryReadinessRef: 'network.remote-delivery.provider-readiness.10o' });
  expectInvalidPatch({
    childDeviceDeliveryReadinessRef: 'network.remote-delivery.child-device-readiness.10o',
  });
});

it('rejects row10k dispatch, row10m readiness, row10p readiness, and candidate-count mismatches', () => {
  expectInvalidPatch({ preparedNotDispatchedCount: 2 });
  expectInvalidPatch({ sourceOutboxCandidateCount: 2 });
  expectInvalidPatch({ blockedDispatchRecordCount: 2 });
  expectInvalidPatch({ blockedDispatchRecordsMatchOutboxCandidates: false });
  expectInvalidPatch({ fixtureSourceOutboxCandidateCount: 2 });
  expectInvalidPatch({ fixtureDispatchAttemptCount: 2 });
  expectInvalidPatch({ fixtureRemoteAckCount: 2 });
  expectInvalidPatch({ fixtureRecordsMatchOutboxCandidates: false });
  expectInvalidPatch({ dispatchReadyCandidateCount: 1 });
  expectInvalidPatch({ deleteExportReadinessRecordCount: 2 });
  expectInvalidPatch({ remoteDeleteReadyCount: 2 });
  expectInvalidPatch({ remoteExportReadyCount: 2 });
  expectInvalidPatch({ deleteExportRecordsMatchFixtureAcks: false });
  expectInvalidPatch({ providerDeliveryReadinessRecordCount: 2 });
  expectInvalidPatch({ childDeviceDeliveryReadinessRecordCount: 2 });
  expectInvalidPatch({ providerDeliveryArtifactCount: 1 });
  expectInvalidPatch({ childDeviceDeliveryArtifactCount: 1 });
  expectInvalidPatch({ providerDeliveryRecordsMatchFixtureAcks: false });
  expectInvalidPatch({ childDeviceDeliveryRecordsMatchFixtureAcks: false });
  expectInvalidPatch({ providerDeliveryReadinessState: 'available' });
  expectInvalidPatch({ childDeviceDeliveryReadinessState: 'available' });
  expectInvalidPatch({ transportDispatchState: 'dispatch-ready' });
  expectInvalidPatch({ brokerMissingArtifactCount: 1 });
});

function expectInvalidPatch(patch: Partial<AgentNetworkRemoteDeliveryStatus>) {
  expectInvalid({ ...RemoteDeliveryStatus, ...patch });
}

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
