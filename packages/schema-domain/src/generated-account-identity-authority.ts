/* generated from crates/schema/src/account_identity_authority_ts.rs */

import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import { ChildProfileIdSchema, FamilyIdSchema, ParentAccountIdSchema } from './generated-family-reference-primitives';

const PositiveSafeAuthorityGenerationSchema = Schema.Number.pipe(
  Schema.filter(
    (value) => (Number.isSafeInteger(value) && value > 0) || 'Expected a positive safe authority generation'
  )
);

export const AccountIdentityProviderSchema = withParser(Schema.Literal('authjs', 'firebase'));
export const AccountIdentityAuthoritySchemaVersionSchema = withParser(Schema.Literal('v0.7'));
export const AccountIdentityMemberAuthoritySchemaVersionSchema = withParser(Schema.Literal('v0.1'));
export const AccountIdentityMappingStatusSchema = withParser(Schema.Literal('active', 'revoked'));
export const AccountIdentityAccountStateSchema = withParser(Schema.Literal('active', 'suspended', 'disabled'));
export const AccountIdentityMembershipStateSchema = withParser(
  Schema.Literal('invited', 'pending', 'active', 'revoked', 'disabled')
);
export const AccountIdentityRoleSchema = withParser(
  Schema.Literal(
    'parent-owner',
    'co-parent-guardian',
    'observer',
    'child-profile',
    'child-device-agent',
    'support-admin'
  )
);
export const AccountIdentityDeviceTrustStateSchema = withParser(
  Schema.Literal('pending', 'trusted', 'revoked', 'reset-required', 'disabled')
);
export const AccountIdentitySessionFreshnessStateSchema = withParser(Schema.Literal('fresh', 'stale', 'expired'));
export const AccountIdentitySupportScopeSchema = withParser(Schema.Literal('read-only', 'household', 'device-control'));
export const AccountIdentitySupportReceiptRevocationStateSchema = withParser(Schema.Literal('active', 'revoked'));
export const AccountIdentityPairingStateSchema = withParser(Schema.Literal('pending', 'paired', 'unpaired'));
export const AccountIdentityInstallStateSchema = withParser(Schema.Literal('pending', 'installed', 'failed'));
export const AccountIdentitySelectedRouteSchema = withParser(
  Schema.Literal('local', 'lan', 'remote', 'manual-required')
);
export const AccountIdentityBindingLifecycleStateSchema = withParser(
  Schema.Literal('pending', 'active', 'suspended', 'removed')
);
export const AccountIdentityBindingRevocationStateSchema = withParser(Schema.Literal('active', 'revoked'));

export const AccountIdentityProviderSubjectSchema = brandedNonEmptyStringSchema('AccountIdentityProviderSubject');
export const AccountIdentityMemberIdSchema = brandedNonEmptyStringSchema('AccountIdentityMemberId');
export const AccountIdentitySessionIdSchema = brandedNonEmptyStringSchema('AccountIdentitySessionId');
export const AccountIdentityChildDeviceIdSchema = brandedNonEmptyStringSchema('AccountIdentityChildDeviceId');
export const AccountIdentityPairingIdSchema = brandedNonEmptyStringSchema('AccountIdentityPairingId');
export const AccountIdentityInstallationIdSchema = brandedNonEmptyStringSchema('AccountIdentityInstallationId');
export const AccountIdentityRouteIdSchema = brandedNonEmptyStringSchema('AccountIdentityRouteId');
export const AccountIdentityDeviceIdSchema = brandedNonEmptyStringSchema('AccountIdentityDeviceId');
export const AccountIdentitySupportReceiptIdSchema = brandedNonEmptyStringSchema('AccountIdentitySupportReceiptId');
export const AccountIdentitySupportIssuerIdSchema = brandedNonEmptyStringSchema('AccountIdentitySupportIssuerId');
export const AccountIdentityAuditIdentitySchema = brandedNonEmptyStringSchema('AccountIdentityAuditIdentity');

export const AccountIdentityProviderSubjectMappingSchema = withParser(
  Schema.Struct({
    accountId: ParentAccountIdSchema,
    provider: AccountIdentityProviderSchema,
    providerSubject: AccountIdentityProviderSubjectSchema,
    status: AccountIdentityMappingStatusSchema,
  })
);

export const AccountIdentityHouseholdChildDeviceBindingSchema = withParser(
  Schema.Struct({
    accountId: ParentAccountIdSchema,
    householdId: FamilyIdSchema,
    childProfileId: ChildProfileIdSchema,
    childDeviceId: AccountIdentityChildDeviceIdSchema,
    pairingId: AccountIdentityPairingIdSchema,
    installationId: AccountIdentityInstallationIdSchema,
    selectedRouteId: AccountIdentityRouteIdSchema,
    pairingState: AccountIdentityPairingStateSchema,
    installState: AccountIdentityInstallStateSchema,
    selectedRoute: AccountIdentitySelectedRouteSchema,
    lifecycleState: AccountIdentityBindingLifecycleStateSchema,
    revocationState: AccountIdentityBindingRevocationStateSchema,
    authorityGeneration: PositiveSafeAuthorityGenerationSchema,
  })
);

export const AccountIdentitySupportAuthorityReceiptSchema = withParser(
  Schema.Struct({
    receiptId: AccountIdentitySupportReceiptIdSchema,
    providerSubject: AccountIdentityProviderSubjectSchema,
    accountId: ParentAccountIdSchema,
    memberId: AccountIdentityMemberIdSchema,
    householdId: FamilyIdSchema,
    deviceId: AccountIdentityDeviceIdSchema,
    childProfileId: ChildProfileIdSchema,
    childDeviceId: AccountIdentityChildDeviceIdSchema,
    scope: AccountIdentitySupportScopeSchema,
    issuer: AccountIdentitySupportIssuerIdSchema,
    issuedAt: brandedNonEmptyStringSchema('AccountIdentitySupportIssuedAt'),
    expiresAt: brandedNonEmptyStringSchema('AccountIdentitySupportExpiresAt'),
    revocationState: AccountIdentitySupportReceiptRevocationStateSchema,
    auditIdentity: AccountIdentityAuditIdentitySchema,
  })
);

/** Legacy v0.7 evidence DTO; never use this schema as authority. */
export const AccountIdentityAuthorityHandoffSchema = withParser(
  Schema.Struct({
    schemaVersion: AccountIdentityAuthoritySchemaVersionSchema,
    mapping: AccountIdentityProviderSubjectMappingSchema,
    binding: AccountIdentityHouseholdChildDeviceBindingSchema,
  })
);

export const AccountIdentityCurrentMemberDeviceAuthoritySchema = withParser(
  Schema.Struct({
    accountId: ParentAccountIdSchema,
    householdId: FamilyIdSchema,
    memberId: AccountIdentityMemberIdSchema,
    role: AccountIdentityRoleSchema,
    accountState: AccountIdentityAccountStateSchema,
    membershipState: AccountIdentityMembershipStateSchema,
    deviceId: AccountIdentityDeviceIdSchema,
    deviceTrustState: AccountIdentityDeviceTrustStateSchema,
    sessionFreshnessState: AccountIdentitySessionFreshnessStateSchema,
    sessionId: AccountIdentitySessionIdSchema,
    sessionGeneration: PositiveSafeAuthorityGenerationSchema,
    sessionExpiresAt: brandedNonEmptyStringSchema('AccountIdentitySessionExpiresAt'),
    supportReceipt: Schema.NullOr(AccountIdentitySupportAuthorityReceiptSchema),
    authorityGeneration: PositiveSafeAuthorityGenerationSchema,
  })
);

export const AccountIdentityCurrentMemberDeviceAuthorityHandoffSchema = withParser(
  Schema.Struct({
    schemaVersion: AccountIdentityMemberAuthoritySchemaVersionSchema,
    mapping: AccountIdentityProviderSubjectMappingSchema,
    member: AccountIdentityCurrentMemberDeviceAuthoritySchema,
    binding: AccountIdentityHouseholdChildDeviceBindingSchema,
  })
);

export type AccountIdentityProvider = Infer<typeof AccountIdentityProviderSchema>;
export type AccountIdentityAuthoritySchemaVersion = Infer<typeof AccountIdentityAuthoritySchemaVersionSchema>;
export type AccountIdentityMemberAuthoritySchemaVersion = Infer<
  typeof AccountIdentityMemberAuthoritySchemaVersionSchema
>;
export type AccountIdentityMappingStatus = Infer<typeof AccountIdentityMappingStatusSchema>;
export type AccountIdentityAccountState = Infer<typeof AccountIdentityAccountStateSchema>;
export type AccountIdentityMembershipState = Infer<typeof AccountIdentityMembershipStateSchema>;
export type AccountIdentityRole = Infer<typeof AccountIdentityRoleSchema>;
export type AccountIdentityDeviceTrustState = Infer<typeof AccountIdentityDeviceTrustStateSchema>;
export type AccountIdentitySessionFreshnessState = Infer<typeof AccountIdentitySessionFreshnessStateSchema>;
export type AccountIdentitySupportScope = Infer<typeof AccountIdentitySupportScopeSchema>;
export type AccountIdentitySupportReceiptRevocationState = Infer<
  typeof AccountIdentitySupportReceiptRevocationStateSchema
>;
export type AccountIdentityPairingState = Infer<typeof AccountIdentityPairingStateSchema>;
export type AccountIdentityInstallState = Infer<typeof AccountIdentityInstallStateSchema>;
export type AccountIdentitySelectedRouteKind = Infer<typeof AccountIdentitySelectedRouteSchema>;
export type AccountIdentityBindingLifecycleState = Infer<typeof AccountIdentityBindingLifecycleStateSchema>;
export type AccountIdentityBindingRevocationState = Infer<typeof AccountIdentityBindingRevocationStateSchema>;
export type AccountIdentityProviderSubject = typeof AccountIdentityProviderSubjectSchema.Type;
export type AccountIdentityMemberId = typeof AccountIdentityMemberIdSchema.Type;
export type AccountIdentitySessionId = typeof AccountIdentitySessionIdSchema.Type;
export type AccountIdentityChildDeviceId = typeof AccountIdentityChildDeviceIdSchema.Type;
export type AccountIdentityPairingId = typeof AccountIdentityPairingIdSchema.Type;
export type AccountIdentityInstallationId = typeof AccountIdentityInstallationIdSchema.Type;
export type AccountIdentityRouteId = typeof AccountIdentityRouteIdSchema.Type;
export type AccountIdentityDeviceId = typeof AccountIdentityDeviceIdSchema.Type;
export type AccountIdentitySupportReceiptId = typeof AccountIdentitySupportReceiptIdSchema.Type;
export type AccountIdentitySupportIssuerId = typeof AccountIdentitySupportIssuerIdSchema.Type;
export type AccountIdentityAuditIdentity = typeof AccountIdentityAuditIdentitySchema.Type;
export type AccountIdentityProviderSubjectMapping = Infer<typeof AccountIdentityProviderSubjectMappingSchema>;
export type AccountIdentityHouseholdChildDeviceBinding = Infer<typeof AccountIdentityHouseholdChildDeviceBindingSchema>;
export type AccountIdentitySupportAuthorityReceipt = Infer<typeof AccountIdentitySupportAuthorityReceiptSchema>;
export type AccountIdentityAuthorityHandoff = Infer<typeof AccountIdentityAuthorityHandoffSchema>;
export type AccountIdentityCurrentMemberDeviceAuthority = Infer<
  typeof AccountIdentityCurrentMemberDeviceAuthoritySchema
>;
export type AccountIdentityCurrentMemberDeviceAuthorityHandoff = Infer<
  typeof AccountIdentityCurrentMemberDeviceAuthorityHandoffSchema
>;
