import {
  AgentLogSnapshotSchema,
  LogFieldsSchema,
  LogLevelSchema,
  type AgentLogSnapshot,
  type LogFields,
  type LogLevel,
} from '@ocentra-parent/logging-domain/contracts';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AgentCorrelationIdSchema,
  AgentEventIdSchema,
  AgentMessageIdSchema,
  AgentMessageTargetSchema,
  AgentPeerSchema,
  AgentProtocolSchemaVersion,
  AgentTimestampSchema,
  AgentWebSocketUrlSchema,
  SerializedAgentMessageSchema,
} from './primitives';
export { AgentProtocolDefaults } from './defaults';

export {
  AgentCorrelationIdSchema,
  AgentDeviceIdSchema,
  AgentEventIdSchema,
  AgentMessageIdSchema,
  AgentMessageTargetSchema,
  AgentPeerIdSchema,
  AgentPeerRoleSchema,
  AgentPeerSchema,
  AgentPlatformSchema,
  AgentProtocolSchemaVersion,
  AgentRouteSchema,
  AgentTimestampSchema,
  AgentWebSocketUrlSchema,
  SerializedAgentMessageSchema,
  type AgentCorrelationId,
  type AgentDeviceId,
  type AgentEventId,
  type AgentMessageId,
  type AgentMessageTarget,
  type AgentPeer,
  type AgentPeerId,
  type AgentPeerRole,
  type AgentPlatform,
  type AgentRoute,
  type AgentTimestamp,
  type AgentWebSocketUrl,
  type SerializedAgentMessage,
} from './primitives';
export {
  AgentLanPairingAddressRefSchema,
  AgentLanPairingChallengeIdSchema,
  AgentLanPairingChallengeSchema,
  AgentLanChildAgentResponseSchema,
  AgentLanPairingDeviceRefSchema,
  AgentLanPairingDiscoveryDeviceSchema,
  AgentLanPairingIntentIdSchema,
  AgentLanPairingIntentKindSchema,
  AgentLanPairingNetworkModeSchema,
  AgentLanParentIntentEnvelopeSchema,
  AgentLanPairingProofDigestSchema,
  AgentLanPairingProofPreviewSchema,
  AgentLanPairingRejectionReasonSchema,
  AgentLanPairingResponseStateSchema,
  AgentLanPairingRouteIdSchema,
  AgentLanPairingRuntimeSupportStatusSchema,
  AgentPairingIdSchema,
  AgentLanSelectedDeviceReachabilitySchema,
  AgentPairingProofSchema,
  AgentPairingStateSchema,
  AgentPairingTokenHashSchema,
  AgentRouteSecurityPolicySchema,
  type AgentLanPairingAddressRef,
  type AgentLanPairingChallenge,
  type AgentLanPairingChallengeId,
  type AgentLanChildAgentResponse,
  type AgentLanPairingDeviceRef,
  type AgentLanPairingDiscoveryDevice,
  type AgentLanPairingIntentId,
  type AgentLanPairingIntentKind,
  type AgentLanPairingNetworkMode,
  type AgentLanParentIntentEnvelope,
  type AgentLanPairingProofDigest,
  type AgentLanPairingProofPreview,
  type AgentLanPairingRejectionReason,
  type AgentLanPairingResponseState,
  type AgentLanPairingRouteId,
  type AgentLanPairingRuntimeSupportStatus,
  type AgentPairingId,
  type AgentLanSelectedDeviceReachability,
  type AgentPairingProof,
  type AgentPairingState,
  type AgentPairingTokenHash,
  type AgentRouteSecurityPolicy,
} from './security';

export const AgentCommandNameSchema = withParser(
  Schema.Literal(
    'agent.health.check',
    'agent.log.snapshot.get',
    'agent.dev.echo',
    'agent.watch.status.get',
    'agent.activity.ingest.status.get',
    'agent.activity.recent.summary.get',
    'agent.activity.memory-graph.get',
    'agent.browser.evidence.recent.get',
    'agent.browser.managed.bridge.poll',
    'agent.browser.intervention.read-model.get',
    'agent.network.flow.read-model.get',
    'agent.local-ai.runtime.status.get',
    'agent.local-ai.chat.generate',
    'agent.policy.preview.read-model.get',
    'agent.lan-pairing.proof.submit',
    'agent.lan-pairing.route.select',
    'agent.lan-pairing.route.revoke',
    'agent.lan-pairing.status.get'
  )
);

export const AgentEventNameSchema = withParser(
  Schema.Literal(
    'agent.connection.ready',
    'agent.command.rejected',
    'agent.health.reported',
    'agent.log.snapshot.reported',
    'agent.dev.echoed',
    'agent.watch.status.reported',
    'agent.activity.ingest.status.reported',
    'agent.activity.recent.summary.reported',
    'agent.activity.memory-graph.reported',
    'agent.browser.evidence.recent.reported',
    'agent.browser.managed.status.reported',
    'agent.browser.intervention.read-model.reported',
    'agent.network.flow.read-model.reported',
    'agent.local-ai.runtime.status.reported',
    'agent.local-ai.chat.generation.reported',
    'agent.policy.preview.read-model.reported',
    'agent.lan-pairing.status.reported',
    'agent.lan-pairing.audit.reported'
  )
);

export const AgentCommandEnvelopeSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    messageId: AgentMessageIdSchema,
    sentAt: AgentTimestampSchema,
    source: AgentPeerSchema,
    target: AgentMessageTargetSchema,
    command: AgentCommandNameSchema,
    payload: LogFieldsSchema,
  })
);

export const AgentEventEnvelopeSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    eventId: AgentEventIdSchema,
    correlationId: AgentCorrelationIdSchema,
    sentAt: AgentTimestampSchema,
    source: AgentPeerSchema,
    target: AgentPeerSchema,
    event: AgentEventNameSchema,
    severity: LogLevelSchema,
    payload: LogFieldsSchema,
    snapshot: Schema.Union(AgentLogSnapshotSchema, Schema.Null),
  })
);

export type AgentCommandName = Infer<typeof AgentCommandNameSchema>;
export type AgentEventName = Infer<typeof AgentEventNameSchema>;
export type AgentCommandEnvelope = Infer<typeof AgentCommandEnvelopeSchema>;
export type AgentEventEnvelope = Infer<typeof AgentEventEnvelopeSchema>;
export type AgentProtocolLogFields = LogFields;
export type AgentProtocolLogLevel = LogLevel;
export type AgentProtocolSnapshot = AgentLogSnapshot;

export const decodeAgentMessageId = Schema.decodeUnknownSync(AgentMessageIdSchema);
export const decodeAgentTimestamp = Schema.decodeUnknownSync(AgentTimestampSchema);
export const decodeAgentWebSocketUrl = Schema.decodeUnknownSync(AgentWebSocketUrlSchema);
export const decodeSerializedAgentMessage = Schema.decodeUnknownSync(SerializedAgentMessageSchema);

export const AgentCommand = {
  HealthCheck: AgentCommandNameSchema.parse('agent.health.check'),
  LogSnapshotGet: AgentCommandNameSchema.parse('agent.log.snapshot.get'),
  DevEcho: AgentCommandNameSchema.parse('agent.dev.echo'),
  WatchStatusGet: AgentCommandNameSchema.parse('agent.watch.status.get'),
  ActivityIngestStatusGet: AgentCommandNameSchema.parse('agent.activity.ingest.status.get'),
  ActivityRecentSummaryGet: AgentCommandNameSchema.parse('agent.activity.recent.summary.get'),
  ActivityMemoryGraphGet: AgentCommandNameSchema.parse('agent.activity.memory-graph.get'),
  BrowserEvidenceRecentGet: AgentCommandNameSchema.parse('agent.browser.evidence.recent.get'),
  BrowserManagedBridgePoll: AgentCommandNameSchema.parse('agent.browser.managed.bridge.poll'),
  BrowserInterventionReadModelGet: AgentCommandNameSchema.parse('agent.browser.intervention.read-model.get'),
  NetworkFlowReadModelGet: AgentCommandNameSchema.parse('agent.network.flow.read-model.get'),
  LocalAiRuntimeStatusGet: AgentCommandNameSchema.parse('agent.local-ai.runtime.status.get'),
  LocalAiChatGenerate: AgentCommandNameSchema.parse('agent.local-ai.chat.generate'),
  PolicyPreviewReadModelGet: AgentCommandNameSchema.parse('agent.policy.preview.read-model.get'),
  LanPairingProofSubmit: AgentCommandNameSchema.parse('agent.lan-pairing.proof.submit'),
  LanPairingRouteSelect: AgentCommandNameSchema.parse('agent.lan-pairing.route.select'),
  LanPairingRouteRevoke: AgentCommandNameSchema.parse('agent.lan-pairing.route.revoke'),
  LanPairingStatusGet: AgentCommandNameSchema.parse('agent.lan-pairing.status.get'),
} as const;

export const AgentLanPairingSupportedWebSocketCommand = {
  ProofSubmit: AgentCommand.LanPairingProofSubmit,
  RouteSelect: AgentCommand.LanPairingRouteSelect,
  RouteRevoke: AgentCommand.LanPairingRouteRevoke,
  StatusGet: AgentCommand.LanPairingStatusGet,
} as const;

export const AgentEvent = {
  ConnectionReady: AgentEventNameSchema.parse('agent.connection.ready'),
  CommandRejected: AgentEventNameSchema.parse('agent.command.rejected'),
  HealthReported: AgentEventNameSchema.parse('agent.health.reported'),
  LogSnapshotReported: AgentEventNameSchema.parse('agent.log.snapshot.reported'),
  DevEchoed: AgentEventNameSchema.parse('agent.dev.echoed'),
  WatchStatusReported: AgentEventNameSchema.parse('agent.watch.status.reported'),
  ActivityIngestStatusReported: AgentEventNameSchema.parse('agent.activity.ingest.status.reported'),
  ActivityRecentSummaryReported: AgentEventNameSchema.parse('agent.activity.recent.summary.reported'),
  ActivityMemoryGraphReported: AgentEventNameSchema.parse('agent.activity.memory-graph.reported'),
  BrowserEvidenceRecentReported: AgentEventNameSchema.parse('agent.browser.evidence.recent.reported'),
  BrowserManagedStatusReported: AgentEventNameSchema.parse('agent.browser.managed.status.reported'),
  BrowserInterventionReadModelReported: AgentEventNameSchema.parse('agent.browser.intervention.read-model.reported'),
  NetworkFlowReadModelReported: AgentEventNameSchema.parse('agent.network.flow.read-model.reported'),
  LocalAiRuntimeStatusReported: AgentEventNameSchema.parse('agent.local-ai.runtime.status.reported'),
  LocalAiChatGenerationReported: AgentEventNameSchema.parse('agent.local-ai.chat.generation.reported'),
  PolicyPreviewReadModelReported: AgentEventNameSchema.parse('agent.policy.preview.read-model.reported'),
  LanPairingStatusReported: AgentEventNameSchema.parse('agent.lan-pairing.status.reported'),
  LanPairingAuditReported: AgentEventNameSchema.parse('agent.lan-pairing.audit.reported'),
} as const;
