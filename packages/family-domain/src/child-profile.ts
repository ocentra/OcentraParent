import { type Infer } from '@ocentra-parent/schema-domain/effect';
import {
  type ChildProfile,
  ChildProfileSchema,
} from '@ocentra-parent/schema-domain/family-child-profile';
import {
  ChildProfileReferenceSchema,
  ParentDeviceReferenceSchema,
} from '@ocentra-parent/schema-domain/family-references';

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
