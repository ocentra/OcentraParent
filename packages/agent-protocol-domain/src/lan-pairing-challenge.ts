import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentDeviceIdSchema, AgentProtocolSchemaVersion, AgentTimestampSchema } from './primitives';
import { AgentLanPairingRouteIdSchema } from './security';

const NonEmptyLanChallengeText = Schema.String.pipe(Schema.minLength(1));

export const AgentLanPairingChallengeRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    childDeviceId: AgentDeviceIdSchema,
    parentDeviceId: AgentDeviceIdSchema,
    routeId: AgentLanPairingRouteIdSchema,
    origin: NonEmptyLanChallengeText,
    issuedAt: AgentTimestampSchema,
    expiresAt: AgentTimestampSchema,
  })
);
