import { EventingEventTypeSchema } from './eventing';
import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';

export const LanPairingSchemaVersionSchema = withParser(Schema.Literal(1));

export const LanPairingIdSchema = brandedNonEmptyStringSchema('LanPairingId');
export const LanPairingChallengeIdSchema = brandedNonEmptyStringSchema('LanPairingChallengeId');
export const LanPairingProofDigestSchema = brandedNonEmptyStringSchema('LanPairingProofDigest');
export const LanPairingIntentIdSchema = brandedNonEmptyStringSchema('LanPairingIntentId');
export const LanPairingAuditEventIdSchema = brandedNonEmptyStringSchema('LanPairingAuditEventId');
export const LanPairingRouteIdSchema = brandedNonEmptyStringSchema('LanPairingRouteId');
export const LanPairingControllerLeaseIdSchema = brandedNonEmptyStringSchema('LanPairingControllerLeaseId');
export const LanPairingOriginSchema = brandedNonEmptyStringSchema('LanPairingOrigin');
export const LanPairingTimestampSchema = brandedNonEmptyStringSchema('LanPairingTimestamp');
export const LanPairingAgentPeerIdSchema = brandedNonEmptyStringSchema('LanPairingAgentPeerId');
export const LanPairingAddressRefSchema = brandedNonEmptyStringSchema('LanPairingAddressRef');

export const LanPairingNetworkModeSchema = withParser(Schema.Literal('loopback', 'local-network'));
export const LanPairingParentAuthoritySchema = withParser(Schema.Literal('active-controller', 'observer'));
export const LanPairingEnablementStateSchema = withParser(
  Schema.Literal('loopback-only', 'lan-disabled', 'lan-enabled')
);
export const LanPairingTrustStateSchema = withParser(
  Schema.Literal('unpaired', 'pairing', 'paired', 'revoked', 'expired')
);
export const LanPairingDeviceReachabilitySchema = withParser(Schema.Literal('online', 'offline', 'stale'));
export const LanPairingProductionDiscoveryStateSchema = withParser(
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
export const LanPairingDiscoverySourceSchema = withParser(
  Schema.Literal('local-service', 'physical-household-lan', 'cloud-relay')
);
export const LanPairingIntentKindSchema = withParser(
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
export const LanPairingResponseStateSchema = withParser(
  Schema.Literal('accepted', 'rejected', 'queued', 'completed', 'degraded')
);
export const LanAiProviderRoutingStateSchema = withParser(
  Schema.Literal('authorized-result', 'busy', 'degraded', 'unavailable', 'unsupported-capability')
);
export const LanPairingRejectionReasonSchema = withParser(
  Schema.Literal(
    'anonymous',
    'wrong-origin',
    'wrong-device',
    'expired',
    'replayed',
    'malformed',
    'stale',
    'offline',
    'revoked',
    'local-network-disabled',
    'unsupported-route',
    'unselected-device',
    'controller-lease-missing',
    'controller-lease-expired',
    'wrong-controller',
    'observer-read-only',
    'takeover-denied',
    'lan-ai-provider-unavailable',
    'lan-ai-job-unauthorized'
  )
);
export const LanPairingAuditEventTypeSchema = withParser(
  Schema.Literal(
    'discovery-advertised',
    'pairing-challenge-issued',
    'pairing-proof-accepted',
    'pairing-proof-rejected',
    'control-accepted',
    'control-rejected',
    'route-selected',
    'pairing-revoked',
    'selected-device-changed',
    'controller-lease-renewed',
    'controller-lease-released',
    'controller-lease-takeover-accepted',
    'controller-lease-takeover-rejected',
    'lan-ai-provider-advertised',
    'lan-ai-job-accepted',
    'lan-ai-job-rejected',
    'lan-ai-job-completed',
    'lan-ai-job-degraded'
  ).pipe(
    Schema.filter(
      (eventType) =>
        EventingEventTypeSchema.safeParse(eventType).success ||
        'Expected LAN audit event type to satisfy the shared eventing taxonomy'
    )
  )
);

export type LanPairingSchemaVersion = Infer<typeof LanPairingSchemaVersionSchema>;
export type LanPairingId = typeof LanPairingIdSchema.Type;
export type LanPairingChallengeId = typeof LanPairingChallengeIdSchema.Type;
export type LanPairingProofDigest = typeof LanPairingProofDigestSchema.Type;
export type LanPairingIntentId = typeof LanPairingIntentIdSchema.Type;
export type LanPairingAuditEventId = typeof LanPairingAuditEventIdSchema.Type;
export type LanPairingRouteId = typeof LanPairingRouteIdSchema.Type;
export type LanPairingControllerLeaseId = typeof LanPairingControllerLeaseIdSchema.Type;
export type LanPairingOrigin = typeof LanPairingOriginSchema.Type;
export type LanPairingTimestamp = typeof LanPairingTimestampSchema.Type;
export type LanPairingAgentPeerId = typeof LanPairingAgentPeerIdSchema.Type;
export type LanPairingAddressRef = typeof LanPairingAddressRefSchema.Type;
export type LanPairingNetworkMode = Infer<typeof LanPairingNetworkModeSchema>;
export type LanPairingParentAuthority = Infer<typeof LanPairingParentAuthoritySchema>;
export type LanPairingEnablementState = Infer<typeof LanPairingEnablementStateSchema>;
export type LanPairingTrustState = Infer<typeof LanPairingTrustStateSchema>;
export type LanPairingDeviceReachability = Infer<typeof LanPairingDeviceReachabilitySchema>;
export type LanPairingProductionDiscoveryState = Infer<typeof LanPairingProductionDiscoveryStateSchema>;
export type LanPairingDiscoverySource = Infer<typeof LanPairingDiscoverySourceSchema>;
export type LanPairingIntentKind = Infer<typeof LanPairingIntentKindSchema>;
export type LanPairingResponseState = Infer<typeof LanPairingResponseStateSchema>;
export type LanAiProviderRoutingState = Infer<typeof LanAiProviderRoutingStateSchema>;
export type LanPairingRejectionReason = Infer<typeof LanPairingRejectionReasonSchema>;
export type LanPairingAuditEventType = Infer<typeof LanPairingAuditEventTypeSchema>;

export const LanPairingSchemaVersion = {
  V1: LanPairingSchemaVersionSchema.parse(1),
} as const;

export const LanPairingNetworkMode = {
  Loopback: LanPairingNetworkModeSchema.parse('loopback'),
  LocalNetwork: LanPairingNetworkModeSchema.parse('local-network'),
} as const;

export const LanPairingParentAuthority = {
  ActiveController: LanPairingParentAuthoritySchema.parse('active-controller'),
  Observer: LanPairingParentAuthoritySchema.parse('observer'),
} as const;

export const LanPairingEnablementState = {
  LoopbackOnly: LanPairingEnablementStateSchema.parse('loopback-only'),
  LanDisabled: LanPairingEnablementStateSchema.parse('lan-disabled'),
  LanEnabled: LanPairingEnablementStateSchema.parse('lan-enabled'),
} as const;

export const LanPairingTrustState = {
  Unpaired: LanPairingTrustStateSchema.parse('unpaired'),
  Pairing: LanPairingTrustStateSchema.parse('pairing'),
  Paired: LanPairingTrustStateSchema.parse('paired'),
  Revoked: LanPairingTrustStateSchema.parse('revoked'),
  Expired: LanPairingTrustStateSchema.parse('expired'),
} as const;

export const LanPairingRejectionReason = {
  Anonymous: LanPairingRejectionReasonSchema.parse('anonymous'),
  WrongOrigin: LanPairingRejectionReasonSchema.parse('wrong-origin'),
  WrongDevice: LanPairingRejectionReasonSchema.parse('wrong-device'),
  Expired: LanPairingRejectionReasonSchema.parse('expired'),
  Replayed: LanPairingRejectionReasonSchema.parse('replayed'),
  Malformed: LanPairingRejectionReasonSchema.parse('malformed'),
  Stale: LanPairingRejectionReasonSchema.parse('stale'),
  Offline: LanPairingRejectionReasonSchema.parse('offline'),
  Revoked: LanPairingRejectionReasonSchema.parse('revoked'),
  LocalNetworkDisabled: LanPairingRejectionReasonSchema.parse('local-network-disabled'),
  UnsupportedRoute: LanPairingRejectionReasonSchema.parse('unsupported-route'),
  UnselectedDevice: LanPairingRejectionReasonSchema.parse('unselected-device'),
  ControllerLeaseMissing: LanPairingRejectionReasonSchema.parse('controller-lease-missing'),
  ControllerLeaseExpired: LanPairingRejectionReasonSchema.parse('controller-lease-expired'),
  WrongController: LanPairingRejectionReasonSchema.parse('wrong-controller'),
  ObserverReadOnly: LanPairingRejectionReasonSchema.parse('observer-read-only'),
  TakeoverDenied: LanPairingRejectionReasonSchema.parse('takeover-denied'),
  LanAiProviderUnavailable: LanPairingRejectionReasonSchema.parse('lan-ai-provider-unavailable'),
  LanAiJobUnauthorized: LanPairingRejectionReasonSchema.parse('lan-ai-job-unauthorized'),
} as const;

export const LanPairingProductionDiscoveryStates = {
  Discovered: LanPairingProductionDiscoveryStateSchema.parse('discovered'),
  Pending: LanPairingProductionDiscoveryStateSchema.parse('pending'),
  Paired: LanPairingProductionDiscoveryStateSchema.parse('paired'),
  Rejected: LanPairingProductionDiscoveryStateSchema.parse('rejected'),
  Expired: LanPairingProductionDiscoveryStateSchema.parse('expired'),
  Revoked: LanPairingProductionDiscoveryStateSchema.parse('revoked'),
  Stale: LanPairingProductionDiscoveryStateSchema.parse('stale'),
  Offline: LanPairingProductionDiscoveryStateSchema.parse('offline'),
  ManualRequired: LanPairingProductionDiscoveryStateSchema.parse('manual-required'),
  Unavailable: LanPairingProductionDiscoveryStateSchema.parse('unavailable'),
} as const;

export const LanPairingDiscoverySources = {
  LocalService: LanPairingDiscoverySourceSchema.parse('local-service'),
  PhysicalHouseholdLan: LanPairingDiscoverySourceSchema.parse('physical-household-lan'),
  CloudRelay: LanPairingDiscoverySourceSchema.parse('cloud-relay'),
} as const;
