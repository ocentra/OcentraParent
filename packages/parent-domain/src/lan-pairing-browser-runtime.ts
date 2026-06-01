import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentDeviceIdSchema } from './reference-primitives';
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

export type LanBrowserDiscoveryScanRequest = Infer<typeof LanBrowserDiscoveryScanRequestSchema>;
export type LanBrowserAddDeviceRuntimeRequest = Infer<typeof LanBrowserAddDeviceRuntimeRequestSchema>;
