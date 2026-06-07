import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const NetworkRemoteDeliveryText = Schema.String.pipe(Schema.minLength(1));
const NetworkRemoteDeliveryCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const NetworkRemoteDeliveryStatusField = 'networkRemoteDeliveryStatus';

export const AgentNetworkRemoteDeliveryStatusStateSchema = withParser(
  Schema.Literal('fixture-requirements-recorded-but-not-implemented', 'manual-required')
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
          localDeliveryProofMatches(status)) ||
        'Network remote delivery status must preserve row10e durable-envelope refs without live delivery or content claims'
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

  const raw = event.payload[NetworkRemoteDeliveryStatusField];
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
  const durableRefs = [
    status.durableEnvelopeRef,
    status.durableStoreRef,
    status.durableReplayRef,
    status.durableDeleteExportRef,
    status.durableSupportStatusRef,
  ];
  return (
    status.durableEnvelopeReady &&
    status.durableEnvelopeMissingArtifactCount === 0 &&
    durableRefs.every((ref) => ref.includes('10e'))
  );
}

function localDeliveryProofMatches(status: AgentNetworkRemoteDeliveryStatus): boolean {
  return (
    status.acceptedEventTypeCount > 0 &&
    status.localIdempotencyQueueProved &&
    status.droppedEventDeadLetterCount > 0 &&
    status.queuedDuplicateRejected &&
    status.completedDuplicateRejected &&
    status.eventChainJournalRef.includes('10c') &&
    status.receiptLedgerRef.includes('10d') &&
    status.localReceiptAckRef.includes('10d')
  );
}
