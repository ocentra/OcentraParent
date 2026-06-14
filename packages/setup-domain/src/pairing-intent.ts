import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentAccountReferenceSchema,
  ParentDeviceReferenceSchema,
} from '@ocentra-parent/family-domain/references';
import {
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

function brandedNonEmptyStringSchema<const Brand extends string>(brand: Brand) {
  return Schema.String.pipe(Schema.minLength(1), Schema.brand(brand));
}

export const SetupPairingIntentIdSchema = brandedNonEmptyStringSchema('SetupPairingIntentId');
export const SetupPairingCodeSchema = brandedNonEmptyStringSchema('SetupPairingCode');
export const SetupStepIdSchema = brandedNonEmptyStringSchema('SetupStepId');
export const SetupPairingReplayNonceSchema = brandedNonEmptyStringSchema('SetupPairingReplayNonce');

export const SetupPairingStateLiteral = {
  Generated: 'generated',
  Displayed: 'displayed',
  Accepted: 'accepted',
  Expired: 'expired',
  Revoked: 'revoked',
  Replayed: 'replayed',
  WrongHousehold: 'wrong-household',
  WrongDevice: 'wrong-device',
  AnonymousDevice: 'anonymous-device',
  ParentRoleRequired: 'parent-role-required',
  StaleSignedHello: 'stale-signed-hello',
  Trusted: 'trusted',
  Untrusted: 'untrusted',
  Recovered: 'recovered',
} as const;

export const SetupPairingTransportLiteral = {
  LanQr: 'lan-qr',
  ManualCode: 'manual-code',
  SignedRelay: 'signed-relay',
} as const;

export const SetupPairingFailureReasonLiteral = {
  StaleCode: 'stale-code',
  ReplayRejected: 'replay-rejected',
  WrongHousehold: 'wrong-household',
  WrongDevice: 'wrong-device',
  AnonymousDevice: 'anonymous-device',
  ParentRoleRequired: 'parent-role-required',
  StaleSignedHello: 'stale-signed-hello',
  RevokedDevice: 'revoked-device',
  OfflineChild: 'offline-child',
  WrongAccount: 'wrong-account',
  PermissionLoss: 'permission-loss',
} as const;

export const SetupPairingStateSchema = withParser(
  Schema.Literal(
    SetupPairingStateLiteral.Generated,
    SetupPairingStateLiteral.Displayed,
    SetupPairingStateLiteral.Accepted,
    SetupPairingStateLiteral.Expired,
    SetupPairingStateLiteral.Revoked,
    SetupPairingStateLiteral.Replayed,
    SetupPairingStateLiteral.WrongHousehold,
    SetupPairingStateLiteral.WrongDevice,
    SetupPairingStateLiteral.AnonymousDevice,
    SetupPairingStateLiteral.ParentRoleRequired,
    SetupPairingStateLiteral.StaleSignedHello,
    SetupPairingStateLiteral.Trusted,
    SetupPairingStateLiteral.Untrusted,
    SetupPairingStateLiteral.Recovered
  )
);

export const SetupPairingTransportSchema = withParser(
  Schema.Literal(
    SetupPairingTransportLiteral.LanQr,
    SetupPairingTransportLiteral.ManualCode,
    SetupPairingTransportLiteral.SignedRelay
  )
);

export const SetupPairingFailureReasonSchema = withParser(
  Schema.Literal(
    SetupPairingFailureReasonLiteral.StaleCode,
    SetupPairingFailureReasonLiteral.ReplayRejected,
    SetupPairingFailureReasonLiteral.WrongHousehold,
    SetupPairingFailureReasonLiteral.WrongDevice,
    SetupPairingFailureReasonLiteral.AnonymousDevice,
    SetupPairingFailureReasonLiteral.ParentRoleRequired,
    SetupPairingFailureReasonLiteral.StaleSignedHello,
    SetupPairingFailureReasonLiteral.RevokedDevice,
    SetupPairingFailureReasonLiteral.OfflineChild,
    SetupPairingFailureReasonLiteral.WrongAccount,
    SetupPairingFailureReasonLiteral.PermissionLoss
  )
);

export const SetupPairingIntentSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    family: FamilyReferenceSchema,
    parentAccount: ParentAccountReferenceSchema,
    parentDevice: ParentDeviceReferenceSchema,
    childProfile: ChildProfileReferenceSchema,
    childDevice: Schema.Union(ParentDeviceReferenceSchema, Schema.Null),
    pairingIntentId: SetupPairingIntentIdSchema,
    activeStepId: SetupStepIdSchema,
    pairingCode: SetupPairingCodeSchema,
    replayNonce: SetupPairingReplayNonceSchema,
    transport: SetupPairingTransportSchema,
    createdAt: ParentTimestampSchema,
    displayedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    acceptedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    trustedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    recoveredAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    revokedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    expiresAt: ParentTimestampSchema,
    state: SetupPairingStateSchema,
    failureReason: Schema.Union(SetupPairingFailureReasonSchema, Schema.Null),
  })
);

export type SetupPairingState = Infer<typeof SetupPairingStateSchema>;
export type SetupPairingTransport = Infer<typeof SetupPairingTransportSchema>;
export type SetupPairingFailureReason = Infer<typeof SetupPairingFailureReasonSchema>;
export type SetupPairingIntent = Infer<typeof SetupPairingIntentSchema>;

export const SetupPairingState = {
  Generated: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Generated),
  Displayed: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Displayed),
  Accepted: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Accepted),
  Expired: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Expired),
  Revoked: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Revoked),
  Replayed: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Replayed),
  WrongHousehold: SetupPairingStateSchema.parse(SetupPairingStateLiteral.WrongHousehold),
  WrongDevice: SetupPairingStateSchema.parse(SetupPairingStateLiteral.WrongDevice),
  AnonymousDevice: SetupPairingStateSchema.parse(SetupPairingStateLiteral.AnonymousDevice),
  ParentRoleRequired: SetupPairingStateSchema.parse(SetupPairingStateLiteral.ParentRoleRequired),
  StaleSignedHello: SetupPairingStateSchema.parse(SetupPairingStateLiteral.StaleSignedHello),
  Trusted: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Trusted),
  Untrusted: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Untrusted),
  Recovered: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Recovered),
} as const;

export const SetupPairingTransport = {
  LanQr: SetupPairingTransportSchema.parse(SetupPairingTransportLiteral.LanQr),
  ManualCode: SetupPairingTransportSchema.parse(SetupPairingTransportLiteral.ManualCode),
  SignedRelay: SetupPairingTransportSchema.parse(SetupPairingTransportLiteral.SignedRelay),
} as const;

export const SetupPairingFailureReason = {
  StaleCode: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.StaleCode),
  ReplayRejected: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.ReplayRejected),
  WrongHousehold: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.WrongHousehold),
  WrongDevice: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.WrongDevice),
  AnonymousDevice: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.AnonymousDevice),
  ParentRoleRequired: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.ParentRoleRequired),
  StaleSignedHello: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.StaleSignedHello),
  RevokedDevice: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.RevokedDevice),
  OfflineChild: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.OfflineChild),
  WrongAccount: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.WrongAccount),
  PermissionLoss: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.PermissionLoss),
} as const;

export function isSetupPairingIntentActive(input: SetupPairingIntent): boolean {
  const intent = SetupPairingIntentSchema.parse(input);

  return (
    intent.state === SetupPairingState.Generated ||
    intent.state === SetupPairingState.Displayed ||
    intent.state === SetupPairingState.Accepted ||
    intent.state === SetupPairingState.Trusted ||
    intent.state === SetupPairingState.Recovered
  );
}

export function isSetupPairingTrustEstablished(input: SetupPairingIntent): boolean {
  const intent = SetupPairingIntentSchema.parse(input);

  return intent.state === SetupPairingState.Trusted || intent.state === SetupPairingState.Recovered;
}

export function requiresSetupPairingRecovery(input: SetupPairingIntent): boolean {
  const intent = SetupPairingIntentSchema.parse(input);

  return (
    intent.state === SetupPairingState.Expired ||
    intent.state === SetupPairingState.Revoked ||
    intent.state === SetupPairingState.Replayed ||
    intent.state === SetupPairingState.WrongHousehold ||
    intent.state === SetupPairingState.WrongDevice ||
    intent.state === SetupPairingState.AnonymousDevice ||
    intent.state === SetupPairingState.ParentRoleRequired ||
    intent.state === SetupPairingState.StaleSignedHello ||
    intent.state === SetupPairingState.Untrusted ||
    intent.failureReason !== null
  );
}
