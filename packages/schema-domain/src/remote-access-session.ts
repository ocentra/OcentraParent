import {
  FamilyReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
} from './family-references';
import {
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import {
  type Infer,
  Schema,
  brandedNonEmptyStringSchema,
  withParser,
} from './effect';

export const RemoteAccessRequestIdSchema = brandedNonEmptyStringSchema('RemoteAccessRequestId');
export const RemoteAccessSessionIdSchema = brandedNonEmptyStringSchema('RemoteAccessSessionId');
export const RemoteAccessPurposeSchema = brandedNonEmptyStringSchema('RemoteAccessPurpose');

export const RemoteAccessConsentStateLiteral = {
  PendingChildConsent: 'pending-child-consent',
  ChildConsented: 'child-consented',
  Denied: 'denied',
} as const;

export const RemoteAccessTransportModeLiteral = {
  Relayed: 'relayed',
  LanDirect: 'lan-direct',
  Disabled: 'disabled',
} as const;

export const RemoteAccessDecisionStateLiteral = {
  Allowed: 'allowed',
  Blocked: 'blocked',
} as const;

export const RemoteAccessConsentStateSchema = withParser(
  Schema.Literal(
    RemoteAccessConsentStateLiteral.PendingChildConsent,
    RemoteAccessConsentStateLiteral.ChildConsented,
    RemoteAccessConsentStateLiteral.Denied
  )
);

export const RemoteAccessTransportModeSchema = withParser(
  Schema.Literal(
    RemoteAccessTransportModeLiteral.Relayed,
    RemoteAccessTransportModeLiteral.LanDirect,
    RemoteAccessTransportModeLiteral.Disabled
  )
);

export const RemoteAccessDecisionStateSchema = withParser(
  Schema.Literal(
    RemoteAccessDecisionStateLiteral.Allowed,
    RemoteAccessDecisionStateLiteral.Blocked
  )
);

export const RemoteAccessSessionRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    family: FamilyReferenceSchema,
    requestedBy: ParentActorReferenceSchema,
    targetDevice: ParentDeviceReferenceSchema,
    requestId: RemoteAccessRequestIdSchema,
    sessionId: RemoteAccessSessionIdSchema,
    purpose: RemoteAccessPurposeSchema,
    requestedAt: ParentTimestampSchema,
    consentState: RemoteAccessConsentStateSchema,
    transportMode: RemoteAccessTransportModeSchema,
  })
);

export const RemoteAccessSessionDecisionSchema = withParser(
  Schema.Struct({
    requestId: RemoteAccessRequestIdSchema,
    sessionId: RemoteAccessSessionIdSchema,
    decisionState: RemoteAccessDecisionStateSchema,
    consentState: RemoteAccessConsentStateSchema,
    transportMode: RemoteAccessTransportModeSchema,
  })
);

export type RemoteAccessConsentState = Infer<typeof RemoteAccessConsentStateSchema>;
export type RemoteAccessTransportMode = Infer<typeof RemoteAccessTransportModeSchema>;
export type RemoteAccessDecisionState = Infer<typeof RemoteAccessDecisionStateSchema>;
export type RemoteAccessSessionRequest = Infer<typeof RemoteAccessSessionRequestSchema>;
export type RemoteAccessSessionDecision = Infer<typeof RemoteAccessSessionDecisionSchema>;

export const RemoteAccessConsentState = {
  PendingChildConsent: RemoteAccessConsentStateSchema.parse(
    RemoteAccessConsentStateLiteral.PendingChildConsent
  ),
  ChildConsented: RemoteAccessConsentStateSchema.parse(RemoteAccessConsentStateLiteral.ChildConsented),
  Denied: RemoteAccessConsentStateSchema.parse(RemoteAccessConsentStateLiteral.Denied),
} as const;

export const RemoteAccessTransportMode = {
  Relayed: RemoteAccessTransportModeSchema.parse(RemoteAccessTransportModeLiteral.Relayed),
  LanDirect: RemoteAccessTransportModeSchema.parse(RemoteAccessTransportModeLiteral.LanDirect),
  Disabled: RemoteAccessTransportModeSchema.parse(RemoteAccessTransportModeLiteral.Disabled),
} as const;

export const RemoteAccessDecisionState = {
  Allowed: RemoteAccessDecisionStateSchema.parse(RemoteAccessDecisionStateLiteral.Allowed),
  Blocked: RemoteAccessDecisionStateSchema.parse(RemoteAccessDecisionStateLiteral.Blocked),
} as const;

export function decideRemoteAccessSession(input: RemoteAccessSessionRequest): RemoteAccessSessionDecision {
  const request = RemoteAccessSessionRequestSchema.parse(input);
  const allowed =
    request.consentState === RemoteAccessConsentState.ChildConsented &&
    request.transportMode !== RemoteAccessTransportMode.Disabled;

  return RemoteAccessSessionDecisionSchema.parse({
    requestId: request.requestId,
    sessionId: request.sessionId,
    decisionState: allowed ? RemoteAccessDecisionState.Allowed : RemoteAccessDecisionState.Blocked,
    consentState: request.consentState,
    transportMode: request.transportMode,
  });
}
