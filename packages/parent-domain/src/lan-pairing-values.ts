import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyLanPairingText = Schema.String.pipe(Schema.minLength(1));

export const LanPairingSchemaVersionSchema = withParser(Schema.Literal('v0.9'));

export const LanPairingIdSchema = NonEmptyLanPairingText.pipe(Schema.brand('LanPairingId'));
export const LanPairingChallengeIdSchema = NonEmptyLanPairingText.pipe(Schema.brand('LanPairingChallengeId'));
export const LanPairingProofDigestSchema = NonEmptyLanPairingText.pipe(Schema.brand('LanPairingProofDigest'));
export const LanPairingIntentIdSchema = NonEmptyLanPairingText.pipe(Schema.brand('LanPairingIntentId'));
export const LanPairingAuditEventIdSchema = NonEmptyLanPairingText.pipe(Schema.brand('LanPairingAuditEventId'));
export const LanPairingRouteIdSchema = NonEmptyLanPairingText.pipe(Schema.brand('LanPairingRouteId'));
export const LanPairingOriginSchema = NonEmptyLanPairingText.pipe(Schema.brand('LanPairingOrigin'));
export const LanPairingTimestampSchema = NonEmptyLanPairingText.pipe(Schema.brand('LanPairingTimestamp'));
export const LanPairingAgentPeerIdSchema = NonEmptyLanPairingText.pipe(Schema.brand('LanPairingAgentPeerId'));
export const LanPairingAddressRefSchema = NonEmptyLanPairingText.pipe(Schema.brand('LanPairingAddressRef'));

export const LanPairingNetworkModeSchema = withParser(Schema.Literal('loopback', 'local-network'));
export const LanPairingEnablementStateSchema = withParser(
  Schema.Literal('loopback-only', 'lan-disabled', 'lan-enabled')
);
export const LanPairingTrustStateSchema = withParser(
  Schema.Literal('unpaired', 'pairing', 'paired', 'revoked', 'expired')
);
export const LanPairingDeviceReachabilitySchema = withParser(Schema.Literal('online', 'offline', 'stale'));
export const LanPairingIntentKindSchema = withParser(
  Schema.Literal('health-query', 'rule-query', 'rule-update', 'approval-decision', 'configuration-update')
);
export const LanPairingResponseStateSchema = withParser(Schema.Literal('accepted', 'rejected', 'queued', 'completed'));
export const LanPairingRejectionReasonSchema = withParser(
  Schema.Literal(
    'anonymous',
    'wrong-origin',
    'wrong-device',
    'expired',
    'replayed',
    'malformed',
    'stale',
    'revoked',
    'local-network-disabled',
    'unsupported-route'
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
    'pairing-revoked',
    'selected-device-changed'
  )
);

export type LanPairingSchemaVersion = Infer<typeof LanPairingSchemaVersionSchema>;
export type LanPairingId = typeof LanPairingIdSchema.Type;
export type LanPairingChallengeId = typeof LanPairingChallengeIdSchema.Type;
export type LanPairingProofDigest = typeof LanPairingProofDigestSchema.Type;
export type LanPairingIntentId = typeof LanPairingIntentIdSchema.Type;
export type LanPairingAuditEventId = typeof LanPairingAuditEventIdSchema.Type;
export type LanPairingRouteId = typeof LanPairingRouteIdSchema.Type;
export type LanPairingOrigin = typeof LanPairingOriginSchema.Type;
export type LanPairingTimestamp = typeof LanPairingTimestampSchema.Type;
export type LanPairingAgentPeerId = typeof LanPairingAgentPeerIdSchema.Type;
export type LanPairingAddressRef = typeof LanPairingAddressRefSchema.Type;
export type LanPairingNetworkMode = Infer<typeof LanPairingNetworkModeSchema>;
export type LanPairingEnablementState = Infer<typeof LanPairingEnablementStateSchema>;
export type LanPairingTrustState = Infer<typeof LanPairingTrustStateSchema>;
export type LanPairingDeviceReachability = Infer<typeof LanPairingDeviceReachabilitySchema>;
export type LanPairingIntentKind = Infer<typeof LanPairingIntentKindSchema>;
export type LanPairingResponseState = Infer<typeof LanPairingResponseStateSchema>;
export type LanPairingRejectionReason = Infer<typeof LanPairingRejectionReasonSchema>;
export type LanPairingAuditEventType = Infer<typeof LanPairingAuditEventTypeSchema>;

export const LanPairingSchemaVersion = {
  V0_9: LanPairingSchemaVersionSchema.parse('v0.9'),
} as const;

export const LanPairingNetworkMode = {
  Loopback: LanPairingNetworkModeSchema.parse('loopback'),
  LocalNetwork: LanPairingNetworkModeSchema.parse('local-network'),
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
  Revoked: LanPairingRejectionReasonSchema.parse('revoked'),
  LocalNetworkDisabled: LanPairingRejectionReasonSchema.parse('local-network-disabled'),
  UnsupportedRoute: LanPairingRejectionReasonSchema.parse('unsupported-route'),
} as const;
