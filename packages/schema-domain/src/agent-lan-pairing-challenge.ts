import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import {
  AgentDeviceIdSchema,
  AgentProtocolSchemaVersion,
  AgentTimestampSchema,
} from './event-primitives';
import { AgentLanPairingRouteIdSchema } from './agent-lan';

export const AgentLanPairingChallengeRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    childDeviceId: AgentDeviceIdSchema,
    parentDeviceId: AgentDeviceIdSchema,
    routeId: AgentLanPairingRouteIdSchema,
    origin: NonEmptyStringSchema,
    issuedAt: AgentTimestampSchema,
    expiresAt: AgentTimestampSchema,
  })
);

export type AgentLanPairingChallengeRequest = Infer<
  typeof AgentLanPairingChallengeRequestSchema
>;
