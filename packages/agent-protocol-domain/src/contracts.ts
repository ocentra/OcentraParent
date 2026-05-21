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
  AgentPairingIdSchema,
  AgentPairingProofSchema,
  AgentPairingStateSchema,
  AgentPairingTokenHashSchema,
  AgentRouteSecurityPolicySchema,
  type AgentPairingId,
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
    'agent.browser.evidence.recent.get',
    'agent.browser.managed.bridge.poll'
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
    'agent.browser.evidence.recent.reported',
    'agent.browser.managed.status.reported'
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
  BrowserEvidenceRecentGet: AgentCommandNameSchema.parse('agent.browser.evidence.recent.get'),
  BrowserManagedBridgePoll: AgentCommandNameSchema.parse('agent.browser.managed.bridge.poll'),
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
  BrowserEvidenceRecentReported: AgentEventNameSchema.parse('agent.browser.evidence.recent.reported'),
  BrowserManagedStatusReported: AgentEventNameSchema.parse('agent.browser.managed.status.reported'),
} as const;
