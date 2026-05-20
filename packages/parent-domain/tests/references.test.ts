import { describe, expect, it } from 'vitest';
import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from '../src/references';

describe('parent reference contracts', () => {
  it('reference schemas: parse family, actor, child, device, and evidence references exactly', () => {
    expect(FamilyReferenceSchema.parse({ familyId: 'family-main' })).toEqual({ familyId: 'family-main' });
    expect(ParentActorReferenceSchema.parse({ actorId: 'parent-1', role: 'parent' })).toEqual({
      actorId: 'parent-1',
      role: 'parent',
    });
    expect(
      ChildProfileReferenceSchema.parse({
        childProfileId: 'child-1',
        displayName: 'Sam',
      })
    ).toEqual({
      childProfileId: 'child-1',
      displayName: 'Sam',
    });
    expect(
      ParentDeviceReferenceSchema.parse({
        deviceId: 'device-1',
        childProfileId: 'child-1',
        label: 'Sam Windows PC',
        platform: 'windows',
      })
    ).toEqual({
      deviceId: 'device-1',
      childProfileId: 'child-1',
      label: 'Sam Windows PC',
      platform: 'windows',
    });
    expect(
      ParentEvidenceReferenceSchema.parse({
        evidenceReferenceId: 'evidence-1',
        kind: 'journal-event',
        observedAt: '2026-05-20T20:45:00.000Z',
      })
    ).toEqual({
      evidenceReferenceId: 'evidence-1',
      kind: 'journal-event',
      observedAt: '2026-05-20T20:45:00.000Z',
    });
  });

  it('device references: reject unsupported platform claims at the schema boundary', () => {
    const result = ParentDeviceReferenceSchema.safeParse({
      deviceId: 'device-1',
      childProfileId: 'child-1',
      label: 'Sam Console',
      platform: 'game-console',
    });

    expect(result.success).toBe(false);
    if (!result.success) {
      expect([...new Set(result.error.issues.map((issue) => issue.path.join('.')))]).toEqual(['platform']);
    }
  });
});
