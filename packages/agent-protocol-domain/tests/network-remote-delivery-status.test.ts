import { expect, it } from 'vitest';
import { AgentEvent } from '../src/contracts';
import { AgentProtocolDefaults } from '../src/defaults';
import {
  AgentNetworkRemoteDeliveryRow10tRefs,
  parseAgentNetworkRemoteDeliveryStatusEvent,
  type AgentNetworkRemoteDeliveryStatus,
} from '../src/network-remote-delivery-status';

const RemoteDeliveryStatusRefs = AgentProtocolDefaults.NetworkRemoteDeliveryStatus;

const RemoteDeliveryStatus = {
  statusRef: AgentNetworkRemoteDeliveryRow10tRefs.StatusRef,
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
  crossProcessCustodyStatusRef: RemoteDeliveryStatusRefs.CrossProcessCustodyStatusRef,
  crossProcessReplayReadinessRef: RemoteDeliveryStatusRefs.CrossProcessReplayReadinessRef,
  remoteRetentionReadinessRef: RemoteDeliveryStatusRefs.RemoteRetentionReadinessRef,
  remoteDeleteCustodyReadinessRef: RemoteDeliveryStatusRefs.RemoteDeleteCustodyReadinessRef,
  remoteExportCustodyReadinessRef: RemoteDeliveryStatusRefs.RemoteExportCustodyReadinessRef,
  crossProcessReplayRef: RemoteDeliveryStatusRefs.CrossProcessReplayRef,
  crossProcessReplayStoreRef: RemoteDeliveryStatusRefs.CrossProcessReplayStoreRef,
  crossProcessReplayCursorRef: RemoteDeliveryStatusRefs.CrossProcessReplayCursorRef,
  externalCrossProcessTransportRef: AgentNetworkRemoteDeliveryRow10tRefs.ExternalCrossProcessTransportRef,
  externalCrossProcessTransportEnvelopeRef:
    AgentNetworkRemoteDeliveryRow10tRefs.ExternalCrossProcessTransportEnvelopeRef,
  externalCrossProcessTransportAckRef: AgentNetworkRemoteDeliveryRow10tRefs.ExternalCrossProcessTransportAckRef,
  transportDispatchState: 'manual-required-blocked',
  providerDeliveryReadinessState: 'manual-required-unavailable',
  childDeviceDeliveryReadinessState: 'manual-required-unavailable',
  crossProcessCustodyReadinessState: 'manual-required-unavailable',
  externalCrossProcessTransportState: 'deterministic-envelope-ack-recorded',
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
  crossProcessReplayReadinessRecordCount: 3,
  remoteRetentionReadinessRecordCount: 3,
  remoteDeleteCustodyReadinessRecordCount: 3,
  remoteExportCustodyReadinessRecordCount: 3,
  crossProcessCustodyRecordsMatchProviderChildReadiness: true,
  crossProcessReplayArtifactCount: 0,
  remoteRetentionArtifactCount: 0,
  remoteDeleteCustodyArtifactCount: 0,
  remoteExportCustodyArtifactCount: 0,
  crossProcessReplayRecordCount: 3,
  crossProcessReplayStoreWriteCount: 3,
  crossProcessReplayCursorNextSequence: 4,
  crossProcessReplayRecordsMatchDurableEnvelopes: true,
  crossProcessReplayRecordsMatchCustodyReadiness: true,
  externalCrossProcessTransportRecordCount: 3,
  externalCrossProcessTransportEnvelopeCount: 3,
  externalCrossProcessTransportAckCount: 3,
  externalCrossProcessTransportRecordsMatchReplayRecords: true,
  externalCrossProcessTransportAckRecordsMatchEnvelopes: true,
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
  crossProcessReplayImplemented: true,
  externalCrossProcessTransportImplemented: true,
  remoteDeleteExportPropagationImplemented: false,
  productReadyRemoteDelivery: false,
  policyAuthority: false,
  sideEffectAuthority: false,
  hostFilteringClaimed: false,
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

it('parses row10t external cross-process transport status from a typed agent event', () => {
  const parsed = parseAgentNetworkRemoteDeliveryStatusEvent(
    eventWithPayload({
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify(RemoteDeliveryStatus),
    })
  );

  expect(parsed).toEqual({ ok: true, status: RemoteDeliveryStatus });
});

it('rejects live delivery, product-ready, adapter, and content claims', () => {
  expectInvalid({ ...RemoteDeliveryStatus, productReadyRemoteDelivery: true });
  expectInvalid({ ...RemoteDeliveryStatus, crossProcessReplayImplemented: false });
  expectInvalid({ ...RemoteDeliveryStatus, externalCrossProcessTransportImplemented: false });
  expectInvalid({ ...RemoteDeliveryStatus, remoteDeleteExportPropagationImplemented: true });
  expectInvalid({ ...RemoteDeliveryStatus, providerDeliveryImplemented: true });
  expectInvalid({ ...RemoteDeliveryStatus, childDeviceDeliveryImplemented: true });
  expectInvalid({ ...RemoteDeliveryStatus, dispatchAttemptCount: 1 });
  expectInvalid({ ...RemoteDeliveryStatus, remoteAckCount: 1 });
  expectInvalid({ ...RemoteDeliveryStatus, enforcementCommandEventCount: 1 });
  expectInvalid({ ...RemoteDeliveryStatus, adapterActionExecutedCount: 1 });
  expectInvalid({ ...RemoteDeliveryStatus, hostFilteringClaimed: true });
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
  expectInvalidPatch({ statusRef: 'network.remote-delivery.cross-process-custody-status.10q' });
  expectInvalidPatch({ statusRef: 'network.remote-delivery.outbox-status-bridge.10h' });
  expectInvalidPatch({ statusRef: 'network.remote-delivery.transport-dispatch-state.10k' });
  expectInvalidPatch({ statusRef: 'network.remote-delivery.delete-export-status-bridge.10n' });
  expectInvalidPatch({ statusRef: 'network.remote-delivery.cross-process-custody-status.10p' });
  expectInvalidPatch({
    statusRef: 'wrong.network.remote-delivery.cross-process-custody-status.10q',
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
  expectInvalidPatch({
    crossProcessCustodyStatusRef: 'network.remote-delivery.cross-process-custody-status.10p',
  });
  expectInvalidPatch({
    crossProcessReplayReadinessRef: 'network.remote-delivery.cross-process-replay-readiness.10p',
  });
  expectInvalidPatch({
    remoteRetentionReadinessRef: 'network.remote-delivery.remote-retention-readiness.10p',
  });
  expectInvalidPatch({
    remoteDeleteCustodyReadinessRef: 'network.remote-delivery.remote-delete-custody-readiness.10p',
  });
  expectInvalidPatch({
    remoteExportCustodyReadinessRef: 'network.remote-delivery.remote-export-custody-readiness.10p',
  });
  expectInvalidPatch({ crossProcessReplayRef: 'network.remote-delivery.cross-process-replay.10q' });
  expectInvalidPatch({
    crossProcessReplayStoreRef: 'network.remote-delivery.cross-process-replay-store.10q',
  });
  expectInvalidPatch({
    crossProcessReplayCursorRef: 'network.remote-delivery.cross-process-replay-cursor.10q',
  });
  expectInvalidPatch({
    externalCrossProcessTransportRef: 'network.remote-delivery.external-cross-process-transport.10s',
  });
  expectInvalidPatch({
    externalCrossProcessTransportEnvelopeRef: 'network.remote-delivery.external-cross-process-transport-envelope.10s',
  });
  expectInvalidPatch({
    externalCrossProcessTransportAckRef: 'network.remote-delivery.external-cross-process-transport-ack.10s',
  });
});

it('rejects row10k dispatch, row10m readiness, and candidate-count mismatches', () => {
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
  expectInvalidPatch({ dispatchReadyCandidateCount: 1 });
  expectInvalidPatch({ transportDispatchState: 'dispatch-ready' });
  expectInvalidPatch({ brokerMissingArtifactCount: 1 });
});

it('rejects row10p provider/child readiness and row10q custody mismatches', () => {
  expectInvalidPatch({ providerDeliveryReadinessRecordCount: 2 });
  expectInvalidPatch({ childDeviceDeliveryReadinessRecordCount: 2 });
  expectInvalidPatch({ providerDeliveryArtifactCount: 1 });
  expectInvalidPatch({ childDeviceDeliveryArtifactCount: 1 });
  expectInvalidPatch({ providerDeliveryRecordsMatchFixtureAcks: false });
  expectInvalidPatch({ childDeviceDeliveryRecordsMatchFixtureAcks: false });
  expectInvalidPatch({ providerDeliveryReadinessState: 'available' });
  expectInvalidPatch({ childDeviceDeliveryReadinessState: 'available' });
  expectInvalidPatch({ crossProcessReplayReadinessRecordCount: 2 });
  expectInvalidPatch({ remoteRetentionReadinessRecordCount: 2 });
  expectInvalidPatch({ remoteDeleteCustodyReadinessRecordCount: 2 });
  expectInvalidPatch({ remoteExportCustodyReadinessRecordCount: 2 });
  expectInvalidPatch({ crossProcessCustodyRecordsMatchProviderChildReadiness: false });
  expectInvalidPatch({ crossProcessReplayArtifactCount: 1 });
  expectInvalidPatch({ remoteRetentionArtifactCount: 1 });
  expectInvalidPatch({ remoteDeleteCustodyArtifactCount: 1 });
  expectInvalidPatch({ remoteExportCustodyArtifactCount: 1 });
  expectInvalidPatch({ crossProcessCustodyReadinessState: 'available' });
});

it('rejects row10r replay and row10t transport mismatches', () => {
  expectInvalidPatch({ crossProcessReplayRecordCount: 2 });
  expectInvalidPatch({ crossProcessReplayStoreWriteCount: 2 });
  expectInvalidPatch({ crossProcessReplayCursorNextSequence: 3 });
  expectInvalidPatch({ crossProcessReplayRecordsMatchDurableEnvelopes: false });
  expectInvalidPatch({ crossProcessReplayRecordsMatchCustodyReadiness: false });
  expectInvalidPatch({ externalCrossProcessTransportRecordCount: 2 });
  expectInvalidPatch({ externalCrossProcessTransportEnvelopeCount: 2 });
  expectInvalidPatch({ externalCrossProcessTransportAckCount: 2 });
  expectInvalidPatch({ externalCrossProcessTransportRecordsMatchReplayRecords: false });
  expectInvalidPatch({ externalCrossProcessTransportAckRecordsMatchEnvelopes: false });
  expectInvalidPatch({ externalCrossProcessTransportState: 'manual-required' });
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
