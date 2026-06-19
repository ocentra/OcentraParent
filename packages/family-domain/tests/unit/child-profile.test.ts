import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';
import {
  ChildProfileSchema,
  doesChildProfileMatchDeviceReference,
  toChildProfileReference,
} from '../../src/child-profile';
import { ParentMemberSchema } from '../../src/household-authority';
import { ParentDeviceReferenceSchema } from '../../src/references';

describe('child profile contracts', () => {
  const childProfileInput = {
    schemaVersion: 'v0.6',
    childProfileId: 'child-profile-main',
    family: { familyId: 'family-main' },
    displayName: 'Sam Profile',
    createdBy: { actorId: 'actor-owner', role: 'parent' },
    createdAt: '2026-06-13T15:50:00.000Z',
  } as const;

  it('parses a first-class child profile contract and derives its reusable reference', () => {
    const childProfile = ChildProfileSchema.parse(childProfileInput);

    expect(childProfile).toEqual(childProfileInput);
    expect(toChildProfileReference(childProfile)).toEqual({
      childProfileId: 'child-profile-main',
      displayName: 'Sam Profile',
    });
  });

  it('keeps child profile separate from child device and parent membership contracts', () => {
    const childProfile = ChildProfileSchema.parse(childProfileInput);
    const childDevice = ParentDeviceReferenceSchema.parse({
      deviceId: 'device-child-1',
      childProfileId: 'child-profile-main',
      label: 'Sam Android',
      platform: 'android',
    });

    expect(doesChildProfileMatchDeviceReference(childProfile, childDevice)).toBe(true);
    expect(
      doesChildProfileMatchDeviceReference(childProfile, {
        ...childDevice,
        childProfileId: null,
      })
    ).toBe(false);
    expect(ChildProfileSchema.safeParse(childDevice).success).toBe(false);
    expect(ParentDeviceReferenceSchema.safeParse(childProfile).success).toBe(false);
    expect(ParentMemberSchema.safeParse(childProfile).success).toBe(false);
  });

  it('keeps setup-domain on child-profile references instead of a second child-profile model', () => {
    const testDirectory = path.dirname(fileURLToPath(import.meta.url));
    const repoRoot = path.resolve(testDirectory, '..', '..', '..', '..');
    const setupDomainSourceDirectory = path.join(repoRoot, 'packages', 'setup-domain', 'src');
    const setupDomainChildProfileModulePath = path.join(setupDomainSourceDirectory, 'child-profile.ts');
    const childProfileConsumerFiles = ['family-setup-bridge.ts', 'pairing-intent.ts', 'readiness.ts'];

    expect(existsSync(setupDomainChildProfileModulePath)).toBe(false);
    expect(childProfileConsumerFiles).toEqual(
      expect.arrayContaining(['family-setup-bridge.ts', 'pairing-intent.ts', 'readiness.ts'])
    );

    for (const fileName of childProfileConsumerFiles) {
      const fileContents = readFileSync(path.join(setupDomainSourceDirectory, fileName), 'utf8');

      expect(fileContents.includes('ChildProfileReferenceSchema')).toBe(true);
      expect(fileContents.includes('ChildProfileSchema')).toBe(false);
    }
  });
});
