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
import { AgentPairingStateSchema, AgentRouteSecurityPolicySchema } from './security';

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
  Schema.Literal('agent.health.check', 'agent.log.snapshot.get', 'agent.dev.echo', 'agent.watch.status.get')
);

export const AgentEventNameSchema = withParser(
  Schema.Literal(
    'agent.connection.ready',
    'agent.command.rejected',
    'agent.health.reported',
    'agent.log.snapshot.reported',
    'agent.dev.echoed',
    'agent.watch.status.reported'
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
} as const;

export const AgentEvent = {
  ConnectionReady: AgentEventNameSchema.parse('agent.connection.ready'),
  CommandRejected: AgentEventNameSchema.parse('agent.command.rejected'),
  HealthReported: AgentEventNameSchema.parse('agent.health.reported'),
  LogSnapshotReported: AgentEventNameSchema.parse('agent.log.snapshot.reported'),
  DevEchoed: AgentEventNameSchema.parse('agent.dev.echoed'),
  WatchStatusReported: AgentEventNameSchema.parse('agent.watch.status.reported'),
} as const;

export const AgentProtocolDefaults = {
  SchemaVersion: AgentProtocolSchemaVersion,
  WebSocketUrl: decodeAgentWebSocketUrl('ws://127.0.0.1:4477/api/dev/ws'),
  MessageIdPrefix: 'cmd-',
  Peer: {
    PortalDev: AgentPeerSchema.parse({
      peerId: 'portal-dev',
      role: 'portal',
    }),
  },
  Target: {
    LocalhostWindowsAgent: AgentMessageTargetSchema.parse({
      deviceId: 'local-dev-agent',
      platform: 'windows',
      route: 'localhost',
    }),
    LocalNetworkWindowsAgent: AgentMessageTargetSchema.parse({
      deviceId: 'local-dev-agent',
      platform: 'windows',
      route: 'local-network',
    }),
  },
  PairingState: {
    Unpaired: AgentPairingStateSchema.parse('unpaired'),
    Pairing: AgentPairingStateSchema.parse('pairing'),
    Paired: AgentPairingStateSchema.parse('paired'),
    Revoked: AgentPairingStateSchema.parse('revoked'),
  },
  RouteSecurity: {
    Localhost: AgentRouteSecurityPolicySchema.parse({
      route: 'localhost',
      requiresPairing: false,
      allowsAnonymousControl: true,
    }),
    LocalNetwork: AgentRouteSecurityPolicySchema.parse({
      route: 'local-network',
      requiresPairing: true,
      allowsAnonymousControl: false,
    }),
    CloudRelay: AgentRouteSecurityPolicySchema.parse({
      route: 'cloud-relay',
      requiresPairing: true,
      allowsAnonymousControl: false,
    }),
  },
  Host: {
    LoopbackIp: '127.0.0.1',
    LocalhostName: 'localhost',
  },
  Field: {
    Available: 'available',
    Message: 'message',
  },
  Primitive: {
    String: 'string',
  },
} as const;
