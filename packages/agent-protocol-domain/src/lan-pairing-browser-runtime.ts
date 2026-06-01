import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentDeviceIdSchema, AgentProtocolSchemaVersion, AgentTimestampSchema } from './primitives';
import { AgentLanPairingDiscoverySourceSchema } from './lan-pairing-browser-add-device-state';
import { AgentLanPairingRouteIdSchema } from './security';

const NonEmptyLanRuntimeText = Schema.String.pipe(Schema.minLength(1));

export const AgentLanBrowserRuntimeCommandSchema = withParser(
  Schema.Literal('agent.lan-pairing.browser-discovery.scan', 'agent.lan-pairing.add-device.request')
);

export const AgentLanBrowserRuntimeEventSchema = withParser(
  Schema.Literal('agent.lan-pairing.browser-discovery.reported', 'agent.lan-pairing.add-device.reported')
);

export const AgentLanBrowserDiscoveryScanRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    requestedDiscoverySource: AgentLanPairingDiscoverySourceSchema,
  })
);

export const AgentLanBrowserAddDeviceRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    childDeviceId: AgentDeviceIdSchema,
    parentDeviceId: AgentDeviceIdSchema,
    routeId: AgentLanPairingRouteIdSchema,
    origin: NonEmptyLanRuntimeText,
    issuedAt: AgentTimestampSchema,
    expiresAt: AgentTimestampSchema,
  })
);

export type AgentLanBrowserRuntimeCommand = Infer<typeof AgentLanBrowserRuntimeCommandSchema>;
export type AgentLanBrowserRuntimeEvent = Infer<typeof AgentLanBrowserRuntimeEventSchema>;
export type AgentLanBrowserDiscoveryScanRequest = Infer<typeof AgentLanBrowserDiscoveryScanRequestSchema>;
export type AgentLanBrowserAddDeviceRequest = Infer<typeof AgentLanBrowserAddDeviceRequestSchema>;
