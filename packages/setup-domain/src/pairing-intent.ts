import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentAccountReferenceSchema,
} from '@ocentra-parent/family-domain/references';
import {
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import {
  type Infer,
  Schema,
  brandedNonEmptyStringSchema,
  withParser,
} from '@ocentra-parent/schema-domain/effect';

export const SetupPairingIntentIdSchema = brandedNonEmptyStringSchema('SetupPairingIntentId');
export const SetupPairingCodeSchema = brandedNonEmptyStringSchema('SetupPairingCode');
export const SetupStepIdSchema = brandedNonEmptyStringSchema('SetupStepId');

export const SetupPairingStateLiteral = {
  Pending: 'pending',
  Verified: 'verified',
  Expired: 'expired',
  Revoked: 'revoked',
} as const;

export const SetupPairingTransportLiteral = {
  LanQr: 'lan-qr',
  ManualCode: 'manual-code',
  SignedRelay: 'signed-relay',
} as const;

export const SetupPairingStateSchema = withParser(
  Schema.Literal(
    SetupPairingStateLiteral.Pending,
    SetupPairingStateLiteral.Verified,
    SetupPairingStateLiteral.Expired,
    SetupPairingStateLiteral.Revoked
  )
);

export const SetupPairingTransportSchema = withParser(
  Schema.Literal(
    SetupPairingTransportLiteral.LanQr,
    SetupPairingTransportLiteral.ManualCode,
    SetupPairingTransportLiteral.SignedRelay
  )
);

export const SetupPairingIntentSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    family: FamilyReferenceSchema,
    parentAccount: ParentAccountReferenceSchema,
    childProfile: ChildProfileReferenceSchema,
    pairingIntentId: SetupPairingIntentIdSchema,
    activeStepId: SetupStepIdSchema,
    pairingCode: SetupPairingCodeSchema,
    transport: SetupPairingTransportSchema,
    expiresAt: ParentTimestampSchema,
    state: SetupPairingStateSchema,
  })
);

export type SetupPairingState = Infer<typeof SetupPairingStateSchema>;
export type SetupPairingTransport = Infer<typeof SetupPairingTransportSchema>;
export type SetupPairingIntent = Infer<typeof SetupPairingIntentSchema>;

export const SetupPairingState = {
  Pending: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Pending),
  Verified: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Verified),
  Expired: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Expired),
  Revoked: SetupPairingStateSchema.parse(SetupPairingStateLiteral.Revoked),
} as const;

export const SetupPairingTransport = {
  LanQr: SetupPairingTransportSchema.parse(SetupPairingTransportLiteral.LanQr),
  ManualCode: SetupPairingTransportSchema.parse(SetupPairingTransportLiteral.ManualCode),
  SignedRelay: SetupPairingTransportSchema.parse(SetupPairingTransportLiteral.SignedRelay),
} as const;

export function isSetupPairingIntentActive(input: SetupPairingIntent): boolean {
  const intent = SetupPairingIntentSchema.parse(input);

  return intent.state === SetupPairingState.Pending || intent.state === SetupPairingState.Verified;
}
