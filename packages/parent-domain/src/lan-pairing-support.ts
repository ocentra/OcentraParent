import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { LanPairingSchemaVersionSchema, LanPairingTrustStateSchema } from './lan-pairing-values';

const NonEmptyLanPairingSupportText = Schema.String.pipe(Schema.minLength(1));

export const LanPairingRuntimeCommandSchema = NonEmptyLanPairingSupportText.pipe(
  Schema.brand('LanPairingRuntimeCommand')
);
export const LanPairingHttpEndpointIdSchema = NonEmptyLanPairingSupportText.pipe(
  Schema.brand('LanPairingHttpEndpointId')
);
export const LanPairingHttpEndpointPathSchema = NonEmptyLanPairingSupportText.pipe(
  Schema.brand('LanPairingHttpEndpointPath')
);
export const LanPairingTransportSchema = withParser(Schema.Literal('websocket'));
export const LanPairingHttpEndpointSupportSchema = withParser(Schema.Literal('planned-unsupported'));
export const LanPairingPersistenceModeSchema = withParser(Schema.Literal('in-memory-fail-closed'));
export const LanPairingProofModeSchema = withParser(Schema.Literal('direct-proof-submit'));
export const LanPairingRouteRequirementSchema = withParser(
  Schema.Literal(
    'paired-device',
    'allowed-origin',
    'target-device-match',
    'route-id-match',
    'unexpired-intent',
    'non-replayed-intent',
    'unrevoked-pairing'
  )
);
export const LanPairingManualProofGapSchema = withParser(
  Schema.Literal('manual-lan-bind-proof', 'manual-firewall-proof', 'manual-physical-device-proof')
);

export const LanPairingUnsupportedHttpEndpointSchema = withParser(
  Schema.Struct({
    endpointId: LanPairingHttpEndpointIdSchema,
    path: LanPairingHttpEndpointPathSchema,
    support: LanPairingHttpEndpointSupportSchema,
  })
);

export const LanPairingRuntimeSupportSurfaceSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    transport: LanPairingTransportSchema,
    supportedWebSocketCommands: Schema.Array(LanPairingRuntimeCommandSchema),
    unsupportedHttpEndpoints: Schema.Array(LanPairingUnsupportedHttpEndpointSchema),
    pairingState: LanPairingTrustStateSchema,
    trustedDeviceCount: Schema.Number,
    persistenceMode: LanPairingPersistenceModeSchema,
    proofMode: LanPairingProofModeSchema,
    routeRequirements: Schema.Array(LanPairingRouteRequirementSchema),
    manualProofGaps: Schema.Array(LanPairingManualProofGapSchema),
  })
);

export type LanPairingRuntimeCommand = typeof LanPairingRuntimeCommandSchema.Type;
export type LanPairingHttpEndpointId = typeof LanPairingHttpEndpointIdSchema.Type;
export type LanPairingHttpEndpointPath = typeof LanPairingHttpEndpointPathSchema.Type;
export type LanPairingTransport = Infer<typeof LanPairingTransportSchema>;
export type LanPairingHttpEndpointSupport = Infer<typeof LanPairingHttpEndpointSupportSchema>;
export type LanPairingPersistenceMode = Infer<typeof LanPairingPersistenceModeSchema>;
export type LanPairingProofMode = Infer<typeof LanPairingProofModeSchema>;
export type LanPairingRouteRequirement = Infer<typeof LanPairingRouteRequirementSchema>;
export type LanPairingManualProofGap = Infer<typeof LanPairingManualProofGapSchema>;
export type LanPairingUnsupportedHttpEndpoint = Infer<typeof LanPairingUnsupportedHttpEndpointSchema>;
export type LanPairingRuntimeSupportSurface = Infer<typeof LanPairingRuntimeSupportSurfaceSchema>;

export const LanPairingTransport = {
  WebSocket: LanPairingTransportSchema.parse('websocket'),
} as const;

export const LanPairingHttpEndpointSupport = {
  PlannedUnsupported: LanPairingHttpEndpointSupportSchema.parse('planned-unsupported'),
} as const;

export const LanPairingPersistenceMode = {
  InMemoryFailClosed: LanPairingPersistenceModeSchema.parse('in-memory-fail-closed'),
} as const;

export const LanPairingProofMode = {
  DirectProofSubmit: LanPairingProofModeSchema.parse('direct-proof-submit'),
} as const;

export const LanPairingRouteRequirement = {
  PairedDevice: LanPairingRouteRequirementSchema.parse('paired-device'),
  AllowedOrigin: LanPairingRouteRequirementSchema.parse('allowed-origin'),
  TargetDeviceMatch: LanPairingRouteRequirementSchema.parse('target-device-match'),
  RouteIdMatch: LanPairingRouteRequirementSchema.parse('route-id-match'),
  UnexpiredIntent: LanPairingRouteRequirementSchema.parse('unexpired-intent'),
  NonReplayedIntent: LanPairingRouteRequirementSchema.parse('non-replayed-intent'),
  UnrevokedPairing: LanPairingRouteRequirementSchema.parse('unrevoked-pairing'),
} as const;

export const LanPairingManualProofGap = {
  LanBind: LanPairingManualProofGapSchema.parse('manual-lan-bind-proof'),
  Firewall: LanPairingManualProofGapSchema.parse('manual-firewall-proof'),
  PhysicalDevice: LanPairingManualProofGapSchema.parse('manual-physical-device-proof'),
} as const;
