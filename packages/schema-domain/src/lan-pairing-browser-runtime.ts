import { type Infer, Schema, withParser } from './effect';
import { ParentDeviceIdSchema } from './family-reference-primitives';
import {
  LanPairingDiscoverySourceSchema,
  LanPairingOriginSchema,
  LanPairingRouteIdSchema,
  LanPairingSchemaVersionSchema,
  LanPairingTimestampSchema,
} from './lan-pairing-values';

export const LanBrowserDiscoveryScanRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    requestedDiscoverySource: LanPairingDiscoverySourceSchema,
  })
);

export const LanBrowserAddDeviceRuntimeRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    childDeviceId: ParentDeviceIdSchema,
    parentDeviceId: ParentDeviceIdSchema,
    routeId: LanPairingRouteIdSchema,
    origin: LanPairingOriginSchema,
    issuedAt: LanPairingTimestampSchema,
    expiresAt: LanPairingTimestampSchema,
  })
);

export const AgentLanBrowserRuntimeCommandNameLiteral = {
  BrowserDiscoveryScan: 'agent.lan-pairing.browser-discovery.scan',
  AddDeviceRequest: 'agent.lan-pairing.add-device.request',
} as const;

export const AgentLanBrowserRuntimeEventNameLiteral = {
  BrowserDiscoveryReported: 'agent.lan-pairing.browser-discovery.reported',
  AddDeviceReported: 'agent.lan-pairing.add-device.reported',
} as const;

export const AgentLanBrowserRuntimeCommandSchema = withParser(
  Schema.Literal(
    AgentLanBrowserRuntimeCommandNameLiteral.BrowserDiscoveryScan,
    AgentLanBrowserRuntimeCommandNameLiteral.AddDeviceRequest
  )
);

export const AgentLanBrowserRuntimeEventSchema = withParser(
  Schema.Literal(
    AgentLanBrowserRuntimeEventNameLiteral.BrowserDiscoveryReported,
    AgentLanBrowserRuntimeEventNameLiteral.AddDeviceReported
  )
);

export const AgentLanBrowserDiscoveryScanRequestSchema = LanBrowserDiscoveryScanRequestSchema;
export const AgentLanBrowserAddDeviceRequestSchema = LanBrowserAddDeviceRuntimeRequestSchema;

export type LanBrowserDiscoveryScanRequest = Infer<typeof LanBrowserDiscoveryScanRequestSchema>;
export type LanBrowserAddDeviceRuntimeRequest = Infer<typeof LanBrowserAddDeviceRuntimeRequestSchema>;
export type AgentLanBrowserRuntimeCommand = Infer<typeof AgentLanBrowserRuntimeCommandSchema>;
export type AgentLanBrowserRuntimeEvent = Infer<typeof AgentLanBrowserRuntimeEventSchema>;
export type AgentLanBrowserDiscoveryScanRequest = Infer<typeof AgentLanBrowserDiscoveryScanRequestSchema>;
export type AgentLanBrowserAddDeviceRequest = Infer<typeof AgentLanBrowserAddDeviceRequestSchema>;
