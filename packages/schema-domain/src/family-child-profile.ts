import { type Infer, Schema, withParser } from './effect';
import {
  ChildProfileDisplayNameSchema,
  ChildProfileIdSchema,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import {
  FamilyReferenceSchema,
  ParentActorReferenceSchema,
} from './family-references';

export const ChildProfileSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    childProfileId: ChildProfileIdSchema,
    family: FamilyReferenceSchema,
    displayName: ChildProfileDisplayNameSchema,
    createdBy: ParentActorReferenceSchema,
    createdAt: ParentTimestampSchema,
  })
);

export type ChildProfile = Infer<typeof ChildProfileSchema>;
