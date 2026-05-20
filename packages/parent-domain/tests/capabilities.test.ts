import { describe, expect, it } from 'vitest';
import {
  ParentControlCapabilityName,
  ParentControlCapabilityStatus,
  ParentControlPlatformCapabilities,
  ParentControlPlatformCapabilitySchema,
} from '../src/capabilities';

describe('parent control platform capabilities', () => {
  it('ParentControlPlatformCapabilities: keeps platform claims explicit and schema-valid', () => {
    const parsed = ParentControlPlatformCapabilities.map((entry) => ParentControlPlatformCapabilitySchema.parse(entry));

    expect(parsed.map((entry) => entry.platform)).toEqual(['windows', 'linux', 'macos', 'android', 'ios']);
  });

  it('ParentControlPlatformCapabilities: does not claim mobile policy or store distribution support yet', () => {
    const mobileCapabilities = ParentControlPlatformCapabilities.filter((entry) =>
      ['android', 'ios'].includes(entry.platform)
    ).flatMap((entry) => entry.capabilities);

    expect(
      mobileCapabilities.every((capability) => {
        if (
          capability.capability === ParentControlCapabilityName.DeviceOwnerPolicy ||
          capability.capability === ParentControlCapabilityName.FamilyControlsEntitlement ||
          capability.capability === ParentControlCapabilityName.StoreDistribution
        ) {
          return capability.status === ParentControlCapabilityStatus.Planned;
        }
        return true;
      })
    ).toBe(true);
  });
});
