import { expect, it } from 'vitest';
import {
  AgentNetworkRemoteDeliveryRow10tRefs,
  type AgentNetworkRemoteDeliveryStatus,
} from '@ocentra-parent/schema-domain/network-remote-delivery-status';
import { AgentEvent } from '../../src/contracts';
import { AgentProtocolDefaults } from '../../src/defaults';
import {
  parseAgentNetworkRemoteDeliveryStatusEvent,
} from '../../src/network-remote-delivery-status';

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

it('rejects empty refs and invalid enum states', () => {
  expectInvalidPatch({ durableEnvelopeRef: '' });
  expectInvalidPatch({ statusRef: '' });
  expectInvalidPatch({ outboxRef: '' });
  expectInvalidPatch({ outboxHandoffRef: '' });
  expectInvalidPatch({ transportDispatchStateRef: '' });
  expectInvalidPatch({ blockedDispatchRef: '' });
  expectInvalidPatch({ futureTransportSeamRef: '' });
  expectInvalidPatch({ fixtureTransportRef: '' });
  expectInvalidPatch({ fixtureDispatchAttemptRef: '' });
  expectInvalidPatch({ fixtureAckRef: '' });
  expectInvalidPatch({ deleteExportPropagationRef: '' });
  expectInvalidPatch({ remoteDeleteReadinessRef: '' });
  expectInvalidPatch({ remoteExportReadinessRef: '' });
  expectInvalidPatch({ providerRouteRef: '' });
  expectInvalidPatch({ childDeviceRouteRef: '' });
  expectInvalidPatch({ providerDeliveryReadinessRef: '' });
  expectInvalidPatch({ childDeviceDeliveryReadinessRef: '' });
  expectInvalidPatch({ crossProcessCustodyStatusRef: '' });
  expectInvalidPatch({ crossProcessReplayReadinessRef: '' });
  expectInvalidPatch({ remoteRetentionReadinessRef: '' });
  expectInvalidPatch({ remoteDeleteCustodyReadinessRef: '' });
  expectInvalidPatch({ remoteExportCustodyReadinessRef: '' });
  expectInvalidPatch({ crossProcessReplayRef: '' });
  expectInvalidPatch({ crossProcessReplayStoreRef: '' });
  expectInvalidPatch({ crossProcessReplayCursorRef: '' });
  expectInvalidPatch({ externalCrossProcessTransportRef: '' });
  expectInvalidPatch({ externalCrossProcessTransportEnvelopeRef: '' });
  expectInvalidPatch({ externalCrossProcessTransportAckRef: '' });
  expectInvalidPatch({ brokerStatus: 'implemented' });
  expectInvalidPatch({ familyHubStatus: 'implemented' });
  expectInvalidPatch({ transportDispatchState: 'dispatch-ready' });
  expectInvalidPatch({ providerDeliveryReadinessState: 'available' });
  expectInvalidPatch({ childDeviceDeliveryReadinessState: 'available' });
  expectInvalidPatch({ crossProcessCustodyReadinessState: 'available' });
  expectInvalidPatch({ externalCrossProcessTransportState: 'manual-required' });
});

it('rejects negative or non-integer row10k, row10m, and candidate counts', () => {
  expectInvalidPatch({ preparedNotDispatchedCount: -1 });
  expectInvalidPatch({ sourceOutboxCandidateCount: 1.5 });
  expectInvalidPatch({ blockedDispatchRecordCount: -1 });
  expectInvalidPatch({ fixtureSourceOutboxCandidateCount: -1 });
  expectInvalidPatch({ fixtureDispatchAttemptCount: 1.5 });
  expectInvalidPatch({ fixtureRemoteAckCount: -1 });
  expectInvalidPatch({ dispatchReadyCandidateCount: 1 });
  expectInvalidPatch({ deleteExportReadinessRecordCount: -1 });
  expectInvalidPatch({ remoteDeleteReadyCount: 1.5 });
  expectInvalidPatch({ remoteExportReadyCount: -1 });
  expectInvalidPatch({ brokerMissingArtifactCount: 1.5 });
});

it('rejects row10p provider-child artifacts and row10q custody artifact claims', () => {
  expectInvalidPatch({ providerDeliveryReadinessRecordCount: -1 });
  expectInvalidPatch({ childDeviceDeliveryReadinessRecordCount: 1.5 });
  expectInvalidPatch({ providerDeliveryArtifactCount: 1 });
  expectInvalidPatch({ childDeviceDeliveryArtifactCount: 1 });
  expectInvalidPatch({ crossProcessReplayReadinessRecordCount: -1 });
  expectInvalidPatch({ remoteRetentionReadinessRecordCount: 1.5 });
  expectInvalidPatch({ remoteDeleteCustodyReadinessRecordCount: -1 });
  expectInvalidPatch({ remoteExportCustodyReadinessRecordCount: 1.5 });
  expectInvalidPatch({ crossProcessReplayArtifactCount: 1 });
  expectInvalidPatch({ remoteRetentionArtifactCount: 1 });
  expectInvalidPatch({ remoteDeleteCustodyArtifactCount: 1 });
  expectInvalidPatch({ remoteExportCustodyArtifactCount: 1 });
});

it('rejects row10r replay and row10t transport count domain violations', () => {
  expectInvalidPatch({ crossProcessReplayRecordCount: -1 });
  expectInvalidPatch({ crossProcessReplayStoreWriteCount: 1.5 });
  expectInvalidPatch({ crossProcessReplayCursorNextSequence: -1 });
  expectInvalidPatch({ externalCrossProcessTransportRecordCount: 1.5 });
  expectInvalidPatch({ externalCrossProcessTransportEnvelopeCount: -1 });
  expectInvalidPatch({ externalCrossProcessTransportAckCount: 1.5 });
  expectInvalidPatch({ dispatchAttemptCount: 1 });
  expectInvalidPatch({ remoteAckCount: 1 });
  expectInvalidPatch({ sequenceGapCount: 1 });
  expectInvalidPatch({ eventIdMismatchCount: 1 });
  expectInvalidPatch({ eventTypeMismatchCount: 1 });
  expectInvalidPatch({ correlationMismatchCount: 1 });
});

function expectInvalidPatch(patch: Record<string, unknown>) {
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
