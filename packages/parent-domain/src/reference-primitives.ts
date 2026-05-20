import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyParentText = Schema.String.pipe(Schema.minLength(1));

export const ParentContractSchemaVersionSchema = withParser(Schema.Literal('v0.6'));

export const ParentAccountIdSchema = NonEmptyParentText.pipe(Schema.brand('ParentAccountId'));
export const FamilyIdSchema = NonEmptyParentText.pipe(Schema.brand('FamilyId'));
export const ChildProfileIdSchema = NonEmptyParentText.pipe(Schema.brand('ChildProfileId'));
export const ChildProfileDisplayNameSchema = NonEmptyParentText.pipe(Schema.brand('ChildProfileDisplayName'));
export const ParentDeviceIdSchema = NonEmptyParentText.pipe(Schema.brand('ParentDeviceId'));
export const ParentDeviceLabelSchema = NonEmptyParentText.pipe(Schema.brand('ParentDeviceLabel'));
export const ParentActorIdSchema = NonEmptyParentText.pipe(Schema.brand('ParentActorId'));
export const ParentPolicyVersionSchema = NonEmptyParentText.pipe(Schema.brand('ParentPolicyVersion'));
export const ParentEvidenceReferenceIdSchema = NonEmptyParentText.pipe(Schema.brand('ParentEvidenceReferenceId'));
export const ParentActionReferenceIdSchema = NonEmptyParentText.pipe(Schema.brand('ParentActionReferenceId'));
export const ParentTimestampSchema = NonEmptyParentText.pipe(Schema.brand('ParentTimestamp'));

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
