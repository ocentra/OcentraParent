import {
  AgentLogSnapshotSchema,
  LogFieldsSchema,
  LogLevelSchema,
  type AgentLogSnapshot,
  type LogFields,
  type LogLevel,
} from '@ocentra-parent/logging-domain/contracts';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

export const AgentProtocolSchemaVersion = 1;

const NonEmptyProtocolText = Schema.String.pipe(Schema.minLength(1));

export const AgentPeerIdSchema = NonEmptyProtocolText.pipe(Schema.brand('AgentPeerId'));
export const AgentDeviceIdSchema = NonEmptyProtocolText.pipe(Schema.brand('AgentDeviceId'));
export const AgentPlatformSchema = NonEmptyProtocolText.pipe(Schema.brand('AgentPlatform'));
export const AgentMessageIdSchema = NonEmptyProtocolText.pipe(Schema.brand('AgentMessageId'));
export const AgentEventIdSchema = NonEmptyProtocolText.pipe(Schema.brand('AgentEventId'));
export const AgentCorrelationIdSchema = NonEmptyProtocolText.pipe(Schema.brand('AgentCorrelationId'));
export const AgentTimestampSchema = NonEmptyProtocolText.pipe(Schema.brand('AgentTimestamp'));
export const AgentWebSocketUrlSchema = NonEmptyProtocolText.pipe(Schema.brand('AgentWebSocketUrl'));
export const SerializedAgentMessageSchema = NonEmptyProtocolText.pipe(Schema.brand('SerializedAgentMessage'));

export const AgentPeerRoleSchema = withParser(Schema.Literal('portal', 'agent-service', 'cloud-relay'));

export const AgentRouteSchema = withParser(Schema.Literal('localhost', 'local-network', 'cloud-relay'));

export const AgentPeerSchema = withParser(
  Schema.Struct({
    peerId: AgentPeerIdSchema,
    role: AgentPeerRoleSchema,
  })
);

export const AgentMessageTargetSchema = withParser(
  Schema.Struct({
    deviceId: AgentDeviceIdSchema,
    platform: AgentPlatformSchema,
    route: AgentRouteSchema,
  })
);

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

export type AgentPeerRole = Infer<typeof AgentPeerRoleSchema>;
export type AgentRoute = Infer<typeof AgentRouteSchema>;
export type AgentPeerId = typeof AgentPeerIdSchema.Type;
export type AgentDeviceId = typeof AgentDeviceIdSchema.Type;
export type AgentPlatform = typeof AgentPlatformSchema.Type;
export type AgentMessageId = typeof AgentMessageIdSchema.Type;
export type AgentEventId = typeof AgentEventIdSchema.Type;
export type AgentCorrelationId = typeof AgentCorrelationIdSchema.Type;
export type AgentTimestamp = typeof AgentTimestampSchema.Type;
export type AgentWebSocketUrl = typeof AgentWebSocketUrlSchema.Type;
export type SerializedAgentMessage = typeof SerializedAgentMessageSchema.Type;
export type AgentPeer = Infer<typeof AgentPeerSchema>;
export type AgentMessageTarget = Infer<typeof AgentMessageTargetSchema>;
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
