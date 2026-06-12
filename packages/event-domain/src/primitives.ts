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
