/* generated from crates/schema/src/family_reference_primitives_ts.rs */

import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';

export const ParentContractSchemaVersionSchema = withParser(Schema.Literal('v0.6'));

export const ParentAccountIdSchema = brandedNonEmptyStringSchema('ParentAccountId');
export const FamilyIdSchema = brandedNonEmptyStringSchema('FamilyId');
export const ChildProfileIdSchema = brandedNonEmptyStringSchema('ChildProfileId');
export const ChildProfileDisplayNameSchema = brandedNonEmptyStringSchema('ChildProfileDisplayName');
export const ParentDeviceIdSchema = brandedNonEmptyStringSchema('ParentDeviceId');
export const ParentDeviceLabelSchema = brandedNonEmptyStringSchema('ParentDeviceLabel');
export const ParentActorIdSchema = brandedNonEmptyStringSchema('ParentActorId');
export const ParentPolicyVersionSchema = brandedNonEmptyStringSchema('ParentPolicyVersion');
export const ParentEvidenceReferenceIdSchema = brandedNonEmptyStringSchema('ParentEvidenceReferenceId');
export const ParentActionReferenceIdSchema = brandedNonEmptyStringSchema('ParentActionReferenceId');
export const ParentTimestampSchema = brandedNonEmptyStringSchema('ParentTimestamp');

export const ParentPlatformSchema = withParser(Schema.Literal('windows', 'linux', 'macos', 'android', 'ios'));
export const ParentActorRoleSchema = withParser(Schema.Literal('parent', 'guardian', 'system'));
export const ParentEvidenceReferenceKindSchema = withParser(
  Schema.Literal('journal-event', 'query-store-summary', 'activity-event', 'policy-decision', 'local-ai-result')
);

export type ParentContractSchemaVersion = Infer<typeof ParentContractSchemaVersionSchema>;
export type ParentAccountId = typeof ParentAccountIdSchema.Type;
export type FamilyId = typeof FamilyIdSchema.Type;
export type ChildProfileId = typeof ChildProfileIdSchema.Type;
export type ChildProfileDisplayName = typeof ChildProfileDisplayNameSchema.Type;
export type ParentDeviceId = typeof ParentDeviceIdSchema.Type;
export type ParentDeviceLabel = typeof ParentDeviceLabelSchema.Type;
export type ParentActorId = typeof ParentActorIdSchema.Type;
export type ParentPolicyVersion = typeof ParentPolicyVersionSchema.Type;
export type ParentEvidenceReferenceId = typeof ParentEvidenceReferenceIdSchema.Type;
export type ParentActionReferenceId = typeof ParentActionReferenceIdSchema.Type;
export type ParentTimestamp = typeof ParentTimestampSchema.Type;
export type ParentPlatform = Infer<typeof ParentPlatformSchema>;
export type ParentActorRole = Infer<typeof ParentActorRoleSchema>;
export type ParentEvidenceReferenceKind = Infer<typeof ParentEvidenceReferenceKindSchema>;

export const ParentContractSchemaVersion = {
  V0_6: ParentContractSchemaVersionSchema.parse('v0.6'),
} as const;

export const ParentPlatform = {
  Windows: ParentPlatformSchema.parse('windows'),
  Linux: ParentPlatformSchema.parse('linux'),
  Macos: ParentPlatformSchema.parse('macos'),
  Android: ParentPlatformSchema.parse('android'),
  Ios: ParentPlatformSchema.parse('ios'),
} as const;

export const ParentActorRole = {
  Parent: ParentActorRoleSchema.parse('parent'),
  Guardian: ParentActorRoleSchema.parse('guardian'),
  System: ParentActorRoleSchema.parse('system'),
} as const;

export const ParentEvidenceReferenceKind = {
  JournalEvent: ParentEvidenceReferenceKindSchema.parse('journal-event'),
  QueryStoreSummary: ParentEvidenceReferenceKindSchema.parse('query-store-summary'),
  ActivityEvent: ParentEvidenceReferenceKindSchema.parse('activity-event'),
  PolicyDecision: ParentEvidenceReferenceKindSchema.parse('policy-decision'),
  LocalAiResult: ParentEvidenceReferenceKindSchema.parse('local-ai-result'),
} as const;
