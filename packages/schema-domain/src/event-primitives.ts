/* generic helper for event and message branded primitives */

import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';

export const AgentProtocolSchemaVersion = 1;

export const AgentPeerIdSchema = brandedNonEmptyStringSchema('AgentPeerId');
export const AgentDeviceIdSchema = brandedNonEmptyStringSchema('AgentDeviceId');
export const AgentPlatformSchema = brandedNonEmptyStringSchema('AgentPlatform');
export const AgentMessageIdSchema = brandedNonEmptyStringSchema('AgentMessageId');
export const AgentEventIdSchema = brandedNonEmptyStringSchema('AgentEventId');
export const AgentEventNameSchema = brandedNonEmptyStringSchema('AgentEventName');
export const AgentCorrelationIdSchema = brandedNonEmptyStringSchema('AgentCorrelationId');
export const AgentTimestampSchema = brandedNonEmptyStringSchema('AgentTimestamp');
export const AgentWebSocketUrlSchema = brandedNonEmptyStringSchema('AgentWebSocketUrl');
export const SerializedAgentMessageSchema = brandedNonEmptyStringSchema('SerializedAgentMessage');

export const AgentPeerRoleLiteral = {
  Portal: 'portal',
  AgentService: 'agent-service',
  CloudRelay: 'cloud-relay',
} as const;

export const AgentRouteLiteral = {
  Localhost: 'localhost',
  LocalNetwork: 'local-network',
  CloudRelay: 'cloud-relay',
} as const;

export const AgentEventDeliveryModeLiteral = {
  FireAndForget: 'fire-and-forget',
  RequestResponse: 'request-response',
} as const;

export const AgentEventAcknowledgementStateLiteral = {
  Accepted: 'accepted',
  Rejected: 'rejected',
  TimedOut: 'timed-out',
} as const;

export const AgentPeerRoleSchema = withParser(
  Schema.Literal(AgentPeerRoleLiteral.Portal, AgentPeerRoleLiteral.AgentService, AgentPeerRoleLiteral.CloudRelay)
);

export const AgentRouteSchema = withParser(
  Schema.Literal(AgentRouteLiteral.Localhost, AgentRouteLiteral.LocalNetwork, AgentRouteLiteral.CloudRelay)
);

export const AgentEventDeliveryModeSchema = withParser(
  Schema.Literal(AgentEventDeliveryModeLiteral.FireAndForget, AgentEventDeliveryModeLiteral.RequestResponse)
);

export const AgentEventAcknowledgementStateSchema = withParser(
  Schema.Literal(
    AgentEventAcknowledgementStateLiteral.Accepted,
    AgentEventAcknowledgementStateLiteral.Rejected,
    AgentEventAcknowledgementStateLiteral.TimedOut
  )
);

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

export const AgentEventEnvelopeSchema = withParser(
  Schema.Struct({
    eventId: AgentEventIdSchema,
    eventName: AgentEventNameSchema,
    correlationId: AgentCorrelationIdSchema,
    occurredAt: AgentTimestampSchema,
    source: AgentPeerSchema,
    target: Schema.Union(AgentMessageTargetSchema, Schema.Null),
    deliveryMode: AgentEventDeliveryModeSchema,
  })
);

export const AgentEventAcknowledgementSchema = withParser(
  Schema.Struct({
    eventId: AgentEventIdSchema,
    correlationId: AgentCorrelationIdSchema,
    state: AgentEventAcknowledgementStateSchema,
    acknowledgedAt: AgentTimestampSchema,
    responder: AgentPeerSchema,
  })
);

export type AgentPeerRole = Infer<typeof AgentPeerRoleSchema>;
export type AgentRoute = Infer<typeof AgentRouteSchema>;
export type AgentEventDeliveryMode = Infer<typeof AgentEventDeliveryModeSchema>;
export type AgentEventAcknowledgementState = Infer<typeof AgentEventAcknowledgementStateSchema>;
export type AgentPeerId = Infer<typeof AgentPeerIdSchema>;
export type AgentDeviceId = Infer<typeof AgentDeviceIdSchema>;
export type AgentPlatform = Infer<typeof AgentPlatformSchema>;
export type AgentMessageId = Infer<typeof AgentMessageIdSchema>;
export type AgentEventId = Infer<typeof AgentEventIdSchema>;
export type AgentEventName = Infer<typeof AgentEventNameSchema>;
export type AgentCorrelationId = Infer<typeof AgentCorrelationIdSchema>;
export type AgentTimestamp = Infer<typeof AgentTimestampSchema>;
export type AgentWebSocketUrl = Infer<typeof AgentWebSocketUrlSchema>;
export type SerializedAgentMessage = Infer<typeof SerializedAgentMessageSchema>;
export type AgentPeer = Infer<typeof AgentPeerSchema>;
export type AgentMessageTarget = Infer<typeof AgentMessageTargetSchema>;
export type AgentEventEnvelope = Infer<typeof AgentEventEnvelopeSchema>;
export type AgentEventAcknowledgement = Infer<typeof AgentEventAcknowledgementSchema>;

export const AgentEventDeliveryMode = {
  FireAndForget: AgentEventDeliveryModeSchema.parse(AgentEventDeliveryModeLiteral.FireAndForget),
  RequestResponse: AgentEventDeliveryModeSchema.parse(AgentEventDeliveryModeLiteral.RequestResponse),
} as const;

export const AgentEventAcknowledgementState = {
  Accepted: AgentEventAcknowledgementStateSchema.parse(AgentEventAcknowledgementStateLiteral.Accepted),
  Rejected: AgentEventAcknowledgementStateSchema.parse(AgentEventAcknowledgementStateLiteral.Rejected),
  TimedOut: AgentEventAcknowledgementStateSchema.parse(AgentEventAcknowledgementStateLiteral.TimedOut),
} as const;
