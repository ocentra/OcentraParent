/* generated from crates/schema/src/account_identity_authority_ts.rs */

import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentAccountIdSchema,
  ParentDeviceIdSchema,
  ParentContractSchemaVersionSchema,
} from './generated-family-reference-primitives';

export const AccountIdentityProviderSchema = withParser(Schema.Literal('authjs', 'firebase'));
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

export const AccountIdentityProviderSubjectSchema = brandedNonEmptyStringSchema('AccountIdentityProviderSubject');
export const AccountIdentityMemberIdSchema = brandedNonEmptyStringSchema('AccountIdentityMemberId');
export const AccountIdentitySessionIdSchema = brandedNonEmptyStringSchema('AccountIdentitySessionId');

export const AccountIdentityProviderSubjectMappingSchema = withParser(
  Schema.Struct({
    accountId: ParentAccountIdSchema,
    provider: AccountIdentityProviderSchema,
    providerSubject: AccountIdentityProviderSubjectSchema,
    status: AccountIdentityMappingStatusSchema,
  })
);

export const AccountIdentityAuthoritySnapshotSchema = withParser(
  Schema.Struct({
    accountId: ParentAccountIdSchema,
    accountState: AccountIdentityAccountStateSchema,
    householdId: Schema.Union(FamilyIdSchema, Schema.Null),
    memberId: Schema.Union(AccountIdentityMemberIdSchema, Schema.Null),
    membershipState: Schema.Union(AccountIdentityMembershipStateSchema, Schema.Null),
    role: Schema.Union(AccountIdentityRoleSchema, Schema.Null),
    childProfileId: Schema.Union(ChildProfileIdSchema, Schema.Null),
    deviceId: Schema.Union(ParentDeviceIdSchema, Schema.Null),
    deviceTrustState: Schema.Union(AccountIdentityDeviceTrustStateSchema, Schema.Null),
    sessionId: Schema.Union(AccountIdentitySessionIdSchema, Schema.Null),
    sessionFreshnessState: Schema.Union(AccountIdentitySessionFreshnessStateSchema, Schema.Null),
  })
);

export const AccountIdentityAuthorityHandoffSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    mapping: AccountIdentityProviderSubjectMappingSchema,
    authority: Schema.Union(AccountIdentityAuthoritySnapshotSchema, Schema.Null),
  })
);

export type AccountIdentityProvider = Infer<typeof AccountIdentityProviderSchema>;
export type AccountIdentityMappingStatus = Infer<typeof AccountIdentityMappingStatusSchema>;
export type AccountIdentityAccountState = Infer<typeof AccountIdentityAccountStateSchema>;
export type AccountIdentityMembershipState = Infer<typeof AccountIdentityMembershipStateSchema>;
export type AccountIdentityRole = Infer<typeof AccountIdentityRoleSchema>;
export type AccountIdentityDeviceTrustState = Infer<typeof AccountIdentityDeviceTrustStateSchema>;
export type AccountIdentitySessionFreshnessState = Infer<typeof AccountIdentitySessionFreshnessStateSchema>;
export type AccountIdentityProviderSubject = typeof AccountIdentityProviderSubjectSchema.Type;
export type AccountIdentityMemberId = typeof AccountIdentityMemberIdSchema.Type;
export type AccountIdentitySessionId = typeof AccountIdentitySessionIdSchema.Type;
export type AccountIdentityProviderSubjectMapping = Infer<typeof AccountIdentityProviderSubjectMappingSchema>;
export type AccountIdentityAuthoritySnapshot = Infer<typeof AccountIdentityAuthoritySnapshotSchema>;
export type AccountIdentityAuthorityHandoff = Infer<typeof AccountIdentityAuthorityHandoffSchema>;
