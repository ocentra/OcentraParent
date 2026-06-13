import { NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentDeviceIdSchema, AgentProtocolSchemaVersion, AgentTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
import { AgentLanPairingRouteIdSchema } from './security';

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
