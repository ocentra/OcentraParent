import {
  ChildProfileDisplayNameSchema,
  ChildProfileIdSchema,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
} from './references';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

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

export function toChildProfileReference(input: ChildProfile): Infer<typeof ChildProfileReferenceSchema> {
  const childProfile = ChildProfileSchema.parse(input);

  return ChildProfileReferenceSchema.parse({
    childProfileId: childProfile.childProfileId,
    displayName: childProfile.displayName,
  });
}

export function doesChildProfileMatchDeviceReference(
  childProfile: ChildProfile,
  device: Infer<typeof ParentDeviceReferenceSchema>
): boolean {
  const parsedChildProfile = ChildProfileSchema.parse(childProfile);
  const parsedDevice = ParentDeviceReferenceSchema.parse(device);

  return parsedDevice.childProfileId === parsedChildProfile.childProfileId;
}
