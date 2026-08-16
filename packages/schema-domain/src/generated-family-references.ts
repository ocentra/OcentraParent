/* generated from crates/schema/src/family_references_ts.rs */

import { type Infer, Schema, withParser } from './effect';
import {
  ChildProfileDisplayNameSchema,
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentAccountIdSchema,
  ParentActionReferenceIdSchema,
  ParentActorIdSchema,
  ParentActorRoleSchema,
  ParentDeviceIdSchema,
  ParentDeviceLabelSchema,
  ParentEvidenceReferenceIdSchema,
  ParentEvidenceReferenceKindSchema,
  ParentPlatformSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from './generated-family-reference-primitives';

export const ParentActorReferenceSchema = withParser(
  Schema.Struct({
    actorId: ParentActorIdSchema,
    role: ParentActorRoleSchema,
  })
);

export const ParentAccountReferenceSchema = withParser(
  Schema.Struct({
    parentAccountId: ParentAccountIdSchema,
  })
);

export const FamilyReferenceSchema = withParser(
  Schema.Struct({
    familyId: FamilyIdSchema,
  })
);

export const ChildProfileReferenceSchema = withParser(
  Schema.Struct({
    childProfileId: ChildProfileIdSchema,
    displayName: ChildProfileDisplayNameSchema,
  })
);

export const ParentDeviceReferenceSchema = withParser(
  Schema.Struct({
    deviceId: ParentDeviceIdSchema,
    childProfileId: Schema.Union(ChildProfileIdSchema, Schema.Null),
    label: ParentDeviceLabelSchema,
    platform: ParentPlatformSchema,
  })
);

export const ParentEvidenceReferenceSchema = withParser(
  Schema.Struct({
    evidenceReferenceId: ParentEvidenceReferenceIdSchema,
    kind: ParentEvidenceReferenceKindSchema,
    observedAt: ParentTimestampSchema,
  })
);

export const ParentActionReferenceSchema = withParser(
  Schema.Struct({
    actionReferenceId: ParentActionReferenceIdSchema,
    actor: ParentActorReferenceSchema,
    policyVersion: ParentPolicyVersionSchema,
    createdAt: ParentTimestampSchema,
  })
);

export type ParentActorReference = Infer<typeof ParentActorReferenceSchema>;
export type ParentAccountReference = Infer<typeof ParentAccountReferenceSchema>;
export type FamilyReference = Infer<typeof FamilyReferenceSchema>;
export type ChildProfileReference = Infer<typeof ChildProfileReferenceSchema>;
export type ParentDeviceReference = Infer<typeof ParentDeviceReferenceSchema>;
export type ParentEvidenceReference = Infer<typeof ParentEvidenceReferenceSchema>;
export type ParentActionReference = Infer<typeof ParentActionReferenceSchema>;
