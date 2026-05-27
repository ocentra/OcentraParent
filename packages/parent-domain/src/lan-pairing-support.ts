import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LanAiProviderRoutingStateSchema,
  LanPairingProductionDiscoveryStateSchema,
  LanPairingSchemaVersionSchema,
  LanPairingTrustStateSchema,
} from './lan-pairing-values';

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
export const LanPairingRuntimeSupportStatusSchema = withParser(
  Schema.Literal('planned-unsupported', 'websocket-direct')
);
export const LanPairingPersistenceModeSchema = withParser(
  Schema.Literal('in-memory-fail-closed', 'local-json-registry')
);
export const LanPairingRestartBehaviorSchema = withParser(
  Schema.Literal(
    'fail-closed-unpaired',
    'restore-trusted-registry-unselected',
    'restore-trusted-registry-selected-route'
  )
);
export const LanPairingProofModeSchema = withParser(Schema.Literal('direct-proof-submit'));
export const LanPairingRouteRequirementSchema = withParser(
  Schema.Literal(
    'paired-device',
    'allowed-origin',
    'target-device-match',
    'route-id-match',
    'unexpired-intent',
    'non-replayed-intent',
    'unrevoked-pairing',
    'active-controller-lease',
    'selected-device-reachable',
    'parent-write-authority',
    'lan-ai-job-authorized',
    'discovery-state-explicit',
    'route-recovery-persisted'
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
    discoveryStatus: LanPairingRuntimeSupportStatusSchema,
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    challengeStatus: LanPairingRuntimeSupportStatusSchema,
    proofPreviewStatus: LanPairingRuntimeSupportStatusSchema,
    lanAiProviderStatus: LanPairingRuntimeSupportStatusSchema,
    lanAiProviderRoutingState: LanAiProviderRoutingStateSchema,
    lanAiProviderCustodyLabel: NonEmptyLanPairingSupportText,
    lanAiJobStatus: LanPairingRuntimeSupportStatusSchema,
    persistenceMode: LanPairingPersistenceModeSchema,
    restartBehavior: LanPairingRestartBehaviorSchema,
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
export type LanPairingRuntimeSupportStatus = Infer<typeof LanPairingRuntimeSupportStatusSchema>;
export type LanPairingPersistenceMode = Infer<typeof LanPairingPersistenceModeSchema>;
export type LanPairingRestartBehavior = Infer<typeof LanPairingRestartBehaviorSchema>;
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

export const LanPairingRuntimeSupportStatus = {
  PlannedUnsupported: LanPairingRuntimeSupportStatusSchema.parse('planned-unsupported'),
  WebSocketDirect: LanPairingRuntimeSupportStatusSchema.parse('websocket-direct'),
} as const;

export const LanPairingPersistenceMode = {
  InMemoryFailClosed: LanPairingPersistenceModeSchema.parse('in-memory-fail-closed'),
  LocalJsonRegistry: LanPairingPersistenceModeSchema.parse('local-json-registry'),
} as const;

export const LanPairingRestartBehavior = {
  FailClosedUnpaired: LanPairingRestartBehaviorSchema.parse('fail-closed-unpaired'),
  RestoreTrustedRegistryUnselected: LanPairingRestartBehaviorSchema.parse('restore-trusted-registry-unselected'),
  RestoreTrustedRegistrySelectedRoute: LanPairingRestartBehaviorSchema.parse('restore-trusted-registry-selected-route'),
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
  ActiveControllerLease: LanPairingRouteRequirementSchema.parse('active-controller-lease'),
  SelectedDeviceReachable: LanPairingRouteRequirementSchema.parse('selected-device-reachable'),
  ParentWriteAuthority: LanPairingRouteRequirementSchema.parse('parent-write-authority'),
  LanAiJobAuthorized: LanPairingRouteRequirementSchema.parse('lan-ai-job-authorized'),
  DiscoveryStateExplicit: LanPairingRouteRequirementSchema.parse('discovery-state-explicit'),
  RouteRecoveryPersisted: LanPairingRouteRequirementSchema.parse('route-recovery-persisted'),
} as const;

export const LanPairingManualProofGap = {
  LanBind: LanPairingManualProofGapSchema.parse('manual-lan-bind-proof'),
  Firewall: LanPairingManualProofGapSchema.parse('manual-firewall-proof'),
  PhysicalDevice: LanPairingManualProofGapSchema.parse('manual-physical-device-proof'),
} as const;
