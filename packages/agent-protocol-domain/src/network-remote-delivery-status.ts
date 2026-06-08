import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';
import { AgentProtocolDefaults } from './defaults';

const NetworkRemoteDeliveryText = Schema.String.pipe(Schema.minLength(1));
const NetworkRemoteDeliveryCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const NetworkRemoteDeliveryRefs = AgentProtocolDefaults.NetworkRemoteDeliveryStatus;

export const AgentNetworkRemoteDeliveryStatusStateSchema = withParser(
  Schema.Literal('fixture-requirements-recorded-but-not-implemented', 'manual-required')
);

export const AgentNetworkRemoteDeliveryTransportDispatchStateSchema = withParser(
  Schema.Literal('manual-required-blocked')
);

const AgentNetworkRemoteDeliveryStatusFields = Schema.Struct({
  statusRef: NetworkRemoteDeliveryText,
  brokerStatus: AgentNetworkRemoteDeliveryStatusStateSchema,
  familyHubStatus: AgentNetworkRemoteDeliveryStatusStateSchema,
  custodyProofRef: NetworkRemoteDeliveryText,
  publisherAuthRef: NetworkRemoteDeliveryText,
  subscriberAuthRef: NetworkRemoteDeliveryText,
  encryptionRef: NetworkRemoteDeliveryText,
  retentionPolicyRef: NetworkRemoteDeliveryText,
  replayPlanRef: NetworkRemoteDeliveryText,
  deletionPlanRef: NetworkRemoteDeliveryText,
  offsetPolicyRef: NetworkRemoteDeliveryText,
  dedupePolicyRef: NetworkRemoteDeliveryText,
  transportConfigRef: NetworkRemoteDeliveryText,
  relayIdentityRef: NetworkRemoteDeliveryText,
  relayPolicyRef: NetworkRemoteDeliveryText,
  brokerMissingArtifactCount: NetworkRemoteDeliveryCount,
  familyHubMissingArtifactCount: NetworkRemoteDeliveryCount,
  acceptedEventTypeCount: NetworkRemoteDeliveryCount,
  localIdempotencyQueueProved: Schema.Boolean,
  droppedEventDeadLetterCount: NetworkRemoteDeliveryCount,
  queuedDuplicateRejected: Schema.Boolean,
  completedDuplicateRejected: Schema.Boolean,
  eventChainJournalRef: NetworkRemoteDeliveryText,
  receiptLedgerRef: NetworkRemoteDeliveryText,
  localReceiptAckRef: NetworkRemoteDeliveryText,
  durableEnvelopeRef: NetworkRemoteDeliveryText,
  durableStoreRef: NetworkRemoteDeliveryText,
  durableReplayRef: NetworkRemoteDeliveryText,
  durableDeleteExportRef: NetworkRemoteDeliveryText,
  durableSupportStatusRef: NetworkRemoteDeliveryText,
  durableEnvelopeReady: Schema.Boolean,
  durableEnvelopeMissingArtifactCount: NetworkRemoteDeliveryCount,
  outboxRef: NetworkRemoteDeliveryText,
  outboxHandoffRef: NetworkRemoteDeliveryText,
  outboxReplayRef: NetworkRemoteDeliveryText,
  outboxSupportStatusRef: NetworkRemoteDeliveryText,
  transportDispatchStateRef: NetworkRemoteDeliveryText,
  blockedDispatchRef: NetworkRemoteDeliveryText,
  futureTransportSeamRef: NetworkRemoteDeliveryText,
  deleteExportPropagationRef: NetworkRemoteDeliveryText,
  remoteDeleteReadinessRef: NetworkRemoteDeliveryText,
  remoteExportReadinessRef: NetworkRemoteDeliveryText,
  transportDispatchState: AgentNetworkRemoteDeliveryTransportDispatchStateSchema,
  outboxCandidateCount: NetworkRemoteDeliveryCount,
  sourceOutboxCandidateCount: NetworkRemoteDeliveryCount,
  preparedNotDispatchedCount: NetworkRemoteDeliveryCount,
  blockedDispatchRecordCount: NetworkRemoteDeliveryCount,
  blockedDispatchRecordsMatchOutboxCandidates: Schema.Boolean,
  deleteExportReadinessRecordCount: NetworkRemoteDeliveryCount,
  remoteDeleteReadyCount: NetworkRemoteDeliveryCount,
  remoteExportReadyCount: NetworkRemoteDeliveryCount,
  deleteExportRecordsMatchFixtureAcks: Schema.Boolean,
  dispatchReadyCandidateCount: Schema.Literal(0),
  dispatchAttemptCount: Schema.Literal(0),
  remoteAckCount: Schema.Literal(0),
  duplicateDurableEnvelopeRejected: Schema.Boolean,
  outboxCandidatesMatchDurableEnvelopes: Schema.Boolean,
  outboxCandidatesMatchReceipts: Schema.Boolean,
  sequenceGapCount: Schema.Literal(0),
  eventIdMismatchCount: Schema.Literal(0),
  eventTypeMismatchCount: Schema.Literal(0),
  correlationMismatchCount: Schema.Literal(0),
  brokerDeliveryImplemented: Schema.Literal(false),
  familyHubDeliveryImplemented: Schema.Literal(false),
  remoteDeliveryAckImplemented: Schema.Literal(false),
  providerDeliveryImplemented: Schema.Literal(false),
  childDeviceDeliveryImplemented: Schema.Literal(false),
  crossProcessReplayImplemented: Schema.Literal(false),
  remoteDeleteExportPropagationImplemented: Schema.Literal(false),
  productReadyRemoteDelivery: Schema.Literal(false),
  policyAuthority: Schema.Literal(false),
  sideEffectAuthority: Schema.Literal(false),
  enforcementCommandEventCount: Schema.Literal(0),
  adapterActionExecutedCount: Schema.Literal(0),
  rawPcapAvailableCount: Schema.Literal(0),
  exactUrlAvailableCount: Schema.Literal(0),
  decryptedPayloadAvailableCount: Schema.Literal(0),
  pageContentAvailableCount: Schema.Literal(0),
  videoContentAvailableCount: Schema.Literal(0),
  privateMessageContentAvailableCount: Schema.Literal(0),
  searchQueryAvailableCount: Schema.Literal(0),
});

export type AgentNetworkRemoteDeliveryStatus = Infer<typeof AgentNetworkRemoteDeliveryStatusFields>;

export const AgentNetworkRemoteDeliveryStatusSchema = withParser(
  AgentNetworkRemoteDeliveryStatusFields.pipe(
    Schema.filter(
      (status: AgentNetworkRemoteDeliveryStatus) =>
        (remoteRequirementCountsMatch(status) &&
          durableEnvelopeRefsMatch(status) &&
          outboxHandoffRefsMatch(status) &&
          transportDispatchStateMatches(status) &&
          deleteExportReadinessMatches(status) &&
          localDeliveryProofMatches(status)) ||
        'Network remote delivery status must preserve row10n status identity, row10g outbox refs, row10k blocked dispatch refs, and row10m delete/export readiness refs without live delivery or content claims'
    )
  )
);

export type AgentNetworkRemoteDeliveryStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkRemoteDeliveryStatus;
    }
  | {
      readonly ok: false;
      readonly reason:
        | 'wrong-event'
        | 'missing-remote-delivery-status'
        | 'invalid-remote-delivery-status-json'
        | 'invalid-remote-delivery-status';
    };

export function parseAgentNetworkRemoteDeliveryStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkRemoteDeliveryStatusParseResult {
  if (event.event !== AgentEvent.NetworkRemoteDeliveryStatusReported) {
    return { ok: false, reason: 'wrong-event' };
  }

  const raw = event.payload[AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus];
  if (!isAgentProtocolLogText(raw)) {
    return { ok: false, reason: 'missing-remote-delivery-status' };
  }

  let value: unknown;
  try {
    value = JSON.parse(raw) as unknown;
  } catch {
    return { ok: false, reason: 'invalid-remote-delivery-status-json' };
  }

  const parsed = AgentNetworkRemoteDeliveryStatusSchema.safeParse(value);
  if (!parsed.success) {
    return { ok: false, reason: 'invalid-remote-delivery-status' };
  }

  return { ok: true, status: parsed.data };
}

function remoteRequirementCountsMatch(status: AgentNetworkRemoteDeliveryStatus): boolean {
  return (
    (status.brokerStatus !== 'fixture-requirements-recorded-but-not-implemented' ||
      status.brokerMissingArtifactCount === 0) &&
    (status.familyHubStatus !== 'fixture-requirements-recorded-but-not-implemented' ||
      status.familyHubMissingArtifactCount === 0)
  );
}

function durableEnvelopeRefsMatch(status: AgentNetworkRemoteDeliveryStatus): boolean {
  return (
    status.durableEnvelopeReady &&
    status.durableEnvelopeMissingArtifactCount === 0 &&
    status.statusRef === NetworkRemoteDeliveryRefs.StatusRef &&
    status.durableEnvelopeRef === NetworkRemoteDeliveryRefs.DurableEnvelopeRef &&
    status.durableStoreRef === NetworkRemoteDeliveryRefs.DurableStoreRef &&
    status.durableReplayRef === NetworkRemoteDeliveryRefs.DurableReplayRef &&
    status.durableDeleteExportRef === NetworkRemoteDeliveryRefs.DurableDeleteExportRef &&
    status.durableSupportStatusRef === NetworkRemoteDeliveryRefs.DurableSupportStatusRef
  );
}

function outboxHandoffRefsMatch(status: AgentNetworkRemoteDeliveryStatus): boolean {
  return (
    status.outboxRef === NetworkRemoteDeliveryRefs.OutboxRef &&
    status.outboxHandoffRef === NetworkRemoteDeliveryRefs.OutboxHandoffRef &&
    status.outboxReplayRef === NetworkRemoteDeliveryRefs.OutboxReplayRef &&
    status.outboxSupportStatusRef === NetworkRemoteDeliveryRefs.OutboxSupportStatusRef &&
    status.outboxCandidateCount > 0 &&
    status.sourceOutboxCandidateCount === status.outboxCandidateCount &&
    status.preparedNotDispatchedCount === status.outboxCandidateCount &&
    status.duplicateDurableEnvelopeRejected &&
    status.outboxCandidatesMatchDurableEnvelopes &&
    status.outboxCandidatesMatchReceipts
  );
}

function transportDispatchStateMatches(status: AgentNetworkRemoteDeliveryStatus): boolean {
  return (
    status.transportDispatchStateRef === NetworkRemoteDeliveryRefs.TransportDispatchStateRef &&
    status.blockedDispatchRef === NetworkRemoteDeliveryRefs.BlockedDispatchRef &&
    status.futureTransportSeamRef === NetworkRemoteDeliveryRefs.FutureTransportSeamRef &&
    status.transportDispatchState === 'manual-required-blocked' &&
    status.blockedDispatchRecordCount === status.outboxCandidateCount &&
    status.blockedDispatchRecordsMatchOutboxCandidates
  );
}

function deleteExportReadinessMatches(status: AgentNetworkRemoteDeliveryStatus): boolean {
  return (
    status.deleteExportPropagationRef === NetworkRemoteDeliveryRefs.DeleteExportPropagationRef &&
    status.remoteDeleteReadinessRef === NetworkRemoteDeliveryRefs.RemoteDeleteReadinessRef &&
    status.remoteExportReadinessRef === NetworkRemoteDeliveryRefs.RemoteExportReadinessRef &&
    status.deleteExportReadinessRecordCount === status.outboxCandidateCount &&
    status.remoteDeleteReadyCount === status.outboxCandidateCount &&
    status.remoteExportReadyCount === status.outboxCandidateCount &&
    status.deleteExportRecordsMatchFixtureAcks
  );
}

function localDeliveryProofMatches(status: AgentNetworkRemoteDeliveryStatus): boolean {
  return (
    status.acceptedEventTypeCount > 0 &&
    status.localIdempotencyQueueProved &&
    status.droppedEventDeadLetterCount > 0 &&
    status.queuedDuplicateRejected &&
    status.completedDuplicateRejected &&
    status.eventChainJournalRef === NetworkRemoteDeliveryRefs.EventChainJournalRef &&
    status.receiptLedgerRef === NetworkRemoteDeliveryRefs.ReceiptLedgerRef &&
    status.localReceiptAckRef === NetworkRemoteDeliveryRefs.LocalReceiptAckRef
  );
}
