import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import { AgentDeviceIdSchema, AgentPlatformSchema, AgentRouteSchema } from './event-primitives';

export const AgentPairingIdSchema = NonEmptyStringSchema.pipe(Schema.brand('AgentPairingId'));
export const AgentPairingTokenHashSchema = NonEmptyStringSchema.pipe(Schema.brand('AgentPairingTokenHash'));
export const AgentLanPairingAddressRefSchema = NonEmptyStringSchema.pipe(Schema.brand('AgentLanPairingAddressRef'));
export const AgentLanPairingProofDigestSchema = NonEmptyStringSchema.pipe(Schema.brand('AgentLanPairingProofDigest'));
export const AgentLanPairingRouteIdSchema = NonEmptyStringSchema.pipe(Schema.brand('AgentLanPairingRouteId'));

export const AgentLanSelectedDeviceReachabilitySchema = withParser(Schema.Literal('online', 'offline', 'stale'));
export const AgentLanPairingNetworkModeSchema = withParser(Schema.Literal('loopback', 'local-network'));
export const AgentLanPairingParentAuthoritySchema = withParser(Schema.Literal('active-controller', 'observer'));
export const AgentLanPairingProductionDiscoveryStateSchema = withParser(
  Schema.Literal(
    'discovered',
    'pending',
    'paired',
    'rejected',
    'expired',
    'revoked',
    'stale',
    'offline',
    'manual-required',
    'unavailable'
  )
);
export const AgentLanPairingRuntimeSupportStatusSchema = withParser(
  Schema.Literal('planned-unsupported', 'websocket-direct', 'network-neighbor')
);
export const AgentLanPairingIntentKindSchema = withParser(
  Schema.Literal(
    'health-query',
    'rule-query',
    'rule-update',
    'approval-decision',
    'configuration-update',
    'controller-lease-renew',
    'controller-lease-release',
    'controller-lease-takeover',
    'lan-ai-provider-status',
    'lan-ai-job-submit'
  )
);
export const AgentLanPairingResponseStateSchema = withParser(
  Schema.Literal('accepted', 'rejected', 'queued', 'completed', 'degraded')
);
export const AgentLanAiProviderRoutingStateSchema = withParser(
  Schema.Literal('authorized-result', 'busy', 'degraded', 'unavailable', 'unsupported-capability')
);

export const AgentLanPairingDeviceRefSchema = withParser(
  Schema.Struct({
    deviceId: AgentDeviceIdSchema,
    childProfileId: Schema.Union(NonEmptyStringSchema, Schema.Null),
    label: NonEmptyStringSchema,
    platform: AgentPlatformSchema,
    ipAddress: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
    macAddress: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
    hostname: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
    networkInterface: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
    agentStatus: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
    hardwareProfile: Schema.optionalWith(
      Schema.Union(
        Schema.Struct({
          manufacturer: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          model: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          cpuModel: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          cpuCores: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          memoryTotal: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          gpuModel: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          gpuDriver: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          gpuMemory: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          nvidiaSmi: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
        }),
        Schema.Null
      ),
      { default: () => null }
    ),
  })
);

export const AgentRouteSecurityPolicySchema = withParser(
  Schema.Struct({
    route: AgentRouteSchema,
    requiresPairing: Schema.Boolean,
    allowsAnonymousControl: Schema.Boolean,
  })
);

export type AgentPairingId = typeof AgentPairingIdSchema.Type;
export type AgentPairingTokenHash = typeof AgentPairingTokenHashSchema.Type;
export type AgentLanPairingAddressRef = typeof AgentLanPairingAddressRefSchema.Type;
export type AgentLanPairingDeviceRef = Infer<typeof AgentLanPairingDeviceRefSchema>;
export type AgentLanPairingIntentKind = Infer<typeof AgentLanPairingIntentKindSchema>;
export type AgentLanPairingNetworkMode = Infer<typeof AgentLanPairingNetworkModeSchema>;
export type AgentLanPairingParentAuthority = Infer<typeof AgentLanPairingParentAuthoritySchema>;
export type AgentLanPairingProductionDiscoveryState = Infer<typeof AgentLanPairingProductionDiscoveryStateSchema>;
export type AgentLanPairingProofDigest = typeof AgentLanPairingProofDigestSchema.Type;
export type AgentLanPairingResponseState = Infer<typeof AgentLanPairingResponseStateSchema>;
export type AgentLanPairingRouteId = typeof AgentLanPairingRouteIdSchema.Type;
export type AgentLanPairingRuntimeSupportStatus = Infer<typeof AgentLanPairingRuntimeSupportStatusSchema>;
export type AgentLanAiProviderRoutingState = Infer<typeof AgentLanAiProviderRoutingStateSchema>;
export type AgentLanSelectedDeviceReachability = Infer<typeof AgentLanSelectedDeviceReachabilitySchema>;
export type AgentRouteSecurityPolicy = Infer<typeof AgentRouteSecurityPolicySchema>;
