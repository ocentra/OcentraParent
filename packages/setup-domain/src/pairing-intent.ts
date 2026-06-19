import {
  DeviceAuthorityActionSchema,
  ParentStepUpAssertionSchema,
  ParentStepUpMethodSchema,
  type ParentStepUpAssertion,
} from '@ocentra-parent/family-domain/household-authority';
import {
  ChildProfileReferenceSchema as ChildProfileReferenceContractSchema,
  FamilyReferenceSchema as FamilyReferenceContractSchema,
  ParentAccountReferenceSchema as ParentAccountReferenceContractSchema,
  ParentDeviceReferenceSchema as ParentDeviceReferenceContractSchema,
} from '@ocentra-parent/family-domain/references';
import {
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

function brandedNonEmptyStringSchema<const Brand extends string>(brand: Brand) {
  return Schema.String.pipe(Schema.minLength(1), Schema.brand(brand));
}

export const SetupPairingIntentIdSchema = brandedNonEmptyStringSchema('SetupPairingIntentId');
export const SetupPairingCodeSchema = brandedNonEmptyStringSchema('SetupPairingCode');
export const SetupStepIdSchema = brandedNonEmptyStringSchema('SetupStepId');
export const SetupPairingReplayNonceSchema = brandedNonEmptyStringSchema('SetupPairingReplayNonce');
export const SetupPairingApprovalChallengeIdSchema = brandedNonEmptyStringSchema('SetupPairingApprovalChallengeId');
export const SetupPairingApprovalResponseIdSchema = brandedNonEmptyStringSchema('SetupPairingApprovalResponseId');
export const SetupPairingDesktopSessionIdSchema = brandedNonEmptyStringSchema('SetupPairingDesktopSessionId');
export const SetupPairingApprovalNonceSchema = brandedNonEmptyStringSchema('SetupPairingApprovalNonce');

export const SetupPairingStateLiteral = {
  Generated: 'generated',
  Displayed: 'displayed',
  Accepted: 'accepted',
  Expired: 'expired',
  Revoked: 'revoked',
  Replayed: 'replayed',
  WrongHousehold: 'wrong-household',
  WrongDevice: 'wrong-device',
  WrongTarget: 'wrong-target',
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
  WrongTarget: 'wrong-target',
  AnonymousDevice: 'anonymous-device',
  ParentRoleRequired: 'parent-role-required',
  StaleSignedHello: 'stale-signed-hello',
  RevokedDevice: 'revoked-device',
  OfflineChild: 'offline-child',
  WrongAccount: 'wrong-account',
  ApprovalExpired: 'approval-expired',
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
    SetupPairingStateLiteral.WrongTarget,
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
    SetupPairingFailureReasonLiteral.WrongTarget,
    SetupPairingFailureReasonLiteral.AnonymousDevice,
    SetupPairingFailureReasonLiteral.ParentRoleRequired,
    SetupPairingFailureReasonLiteral.StaleSignedHello,
    SetupPairingFailureReasonLiteral.RevokedDevice,
    SetupPairingFailureReasonLiteral.OfflineChild,
    SetupPairingFailureReasonLiteral.WrongAccount,
    SetupPairingFailureReasonLiteral.ApprovalExpired,
    SetupPairingFailureReasonLiteral.PermissionLoss
  )
);

export const SetupPairingIntentSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    family: FamilyReferenceContractSchema,
    parentAccount: ParentAccountReferenceContractSchema,
    parentDevice: ParentDeviceReferenceContractSchema,
    childProfile: ChildProfileReferenceContractSchema,
    childDevice: Schema.Union(ParentDeviceReferenceContractSchema, Schema.Null),
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

export const SetupPairingApprovalChallengeSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    approvalChallengeId: SetupPairingApprovalChallengeIdSchema,
    pairingIntentId: SetupPairingIntentIdSchema,
    family: FamilyReferenceContractSchema,
    parentAccount: ParentAccountReferenceContractSchema,
    actionDevice: ParentDeviceReferenceContractSchema,
    desktopSessionId: SetupPairingDesktopSessionIdSchema,
    childProfile: ChildProfileReferenceContractSchema,
    action: DeviceAuthorityActionSchema,
    challengeNonce: SetupPairingApprovalNonceSchema,
    createdAt: ParentTimestampSchema,
    expiresAt: ParentTimestampSchema,
  })
);

export const SetupPairingApprovalResponseSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    approvalResponseId: SetupPairingApprovalResponseIdSchema,
    approvalChallengeId: SetupPairingApprovalChallengeIdSchema,
    pairingIntentId: SetupPairingIntentIdSchema,
    family: FamilyReferenceContractSchema,
    parentAccount: ParentAccountReferenceContractSchema,
    actionDevice: ParentDeviceReferenceContractSchema,
    desktopSessionId: SetupPairingDesktopSessionIdSchema,
    approvalDevice: ParentDeviceReferenceContractSchema,
    childProfile: ChildProfileReferenceContractSchema,
    action: DeviceAuthorityActionSchema,
    challengeNonce: SetupPairingApprovalNonceSchema,
    approvalMethod: ParentStepUpMethodSchema,
    approvedAt: ParentTimestampSchema,
  })
);

export const SetupPairingApprovalResolutionSchema = withParser(
  Schema.Struct({
    assertion: Schema.Union(ParentStepUpAssertionSchema, Schema.Null),
    failureReason: Schema.Union(SetupPairingFailureReasonSchema, Schema.Null),
  })
);

export type SetupPairingState = Infer<typeof SetupPairingStateSchema>;
export type SetupPairingTransport = Infer<typeof SetupPairingTransportSchema>;
export type SetupPairingFailureReason = Infer<typeof SetupPairingFailureReasonSchema>;
export type SetupPairingIntent = Infer<typeof SetupPairingIntentSchema>;
export type SetupPairingApprovalChallenge = Infer<typeof SetupPairingApprovalChallengeSchema>;
export type SetupPairingApprovalResponse = Infer<typeof SetupPairingApprovalResponseSchema>;
export type SetupPairingApprovalResolution = Infer<typeof SetupPairingApprovalResolutionSchema>;

export const SetupPairingState = {
  Generated: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Generated),
  Displayed: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Displayed),
  Accepted: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Accepted),
  Expired: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Expired),
  Revoked: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Revoked),
  Replayed: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Replayed),
  WrongHousehold: SetupPairingStateSchema.parse(SetupPairingStateLiteral.WrongHousehold),
  WrongDevice: SetupPairingStateSchema.parse(SetupPairingStateLiteral.WrongDevice),
  WrongTarget: SetupPairingStateSchema.parse(SetupPairingStateLiteral.WrongTarget),
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
  WrongTarget: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.WrongTarget),
  AnonymousDevice: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.AnonymousDevice),
  ParentRoleRequired: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.ParentRoleRequired),
  StaleSignedHello: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.StaleSignedHello),
  RevokedDevice: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.RevokedDevice),
  OfflineChild: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.OfflineChild),
  WrongAccount: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.WrongAccount),
  ApprovalExpired: SetupPairingFailureReasonSchema.parse(SetupPairingFailureReasonLiteral.ApprovalExpired),
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

export function deriveParentStepUpAssertionFromSetupPairingApproval(input: {
  challenge: SetupPairingApprovalChallenge;
  response: SetupPairingApprovalResponse;
  observedAt: Infer<typeof ParentTimestampSchema>;
}): SetupPairingApprovalResolution {
  const challenge = SetupPairingApprovalChallengeSchema.parse(input.challenge);
  const response = SetupPairingApprovalResponseSchema.parse(input.response);
  const observedAt = Schema.decodeUnknownSync(ParentTimestampSchema)(input.observedAt);

  if (challenge.expiresAt < observedAt || response.approvedAt > challenge.expiresAt) {
    return SetupPairingApprovalResolutionSchema.parse({
      assertion: null,
      failureReason: SetupPairingFailureReason.ApprovalExpired,
    });
  }

  if (
    response.approvalChallengeId !== challenge.approvalChallengeId ||
    response.pairingIntentId !== challenge.pairingIntentId ||
    response.challengeNonce !== challenge.challengeNonce
  ) {
    return SetupPairingApprovalResolutionSchema.parse({
      assertion: null,
      failureReason: SetupPairingFailureReason.ReplayRejected,
    });
  }

  if (response.family.familyId !== challenge.family.familyId) {
    return SetupPairingApprovalResolutionSchema.parse({
      assertion: null,
      failureReason: SetupPairingFailureReason.WrongHousehold,
    });
  }

  if (response.parentAccount.parentAccountId !== challenge.parentAccount.parentAccountId) {
    return SetupPairingApprovalResolutionSchema.parse({
      assertion: null,
      failureReason: SetupPairingFailureReason.WrongAccount,
    });
  }

  if (
    response.action !== challenge.action ||
    response.desktopSessionId !== challenge.desktopSessionId ||
    response.childProfile.childProfileId !== challenge.childProfile.childProfileId
  ) {
    return SetupPairingApprovalResolutionSchema.parse({
      assertion: null,
      failureReason: SetupPairingFailureReason.WrongTarget,
    });
  }

  if (
    response.actionDevice.deviceId !== challenge.actionDevice.deviceId ||
    response.actionDevice.childProfileId !== challenge.actionDevice.childProfileId
  ) {
    return SetupPairingApprovalResolutionSchema.parse({
      assertion: null,
      failureReason: SetupPairingFailureReason.WrongDevice,
    });
  }

  const assertion: ParentStepUpAssertion = ParentStepUpAssertionSchema.parse({
    schemaVersion: response.schemaVersion,
    stepUpAssertionId: response.approvalResponseId,
    family: response.family,
    parentAccount: response.parentAccount,
    actionDevice: response.actionDevice,
    approverDevice: response.approvalDevice,
    targetChildProfile: response.childProfile,
    action: response.action,
    method: response.approvalMethod,
    nonce: response.challengeNonce,
    issuedAt: response.approvedAt,
    expiresAt: challenge.expiresAt,
  });

  return SetupPairingApprovalResolutionSchema.parse({
    assertion,
    failureReason: null,
  });
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
    intent.state === SetupPairingState.WrongTarget ||
    intent.state === SetupPairingState.AnonymousDevice ||
    intent.state === SetupPairingState.ParentRoleRequired ||
    intent.state === SetupPairingState.StaleSignedHello ||
    intent.state === SetupPairingState.Untrusted ||
    intent.failureReason !== null
  );
}
