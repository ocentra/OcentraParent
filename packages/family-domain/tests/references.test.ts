import { describe, expect, it } from 'vitest';

import { FamilyReferenceSchema } from '../src/references';

describe('family-domain references', () => {
  it('parses shared family references through the canonical domain package', () => {
    const reference = FamilyReferenceSchema.parse({ familyId: 'family-alpha' });

    expect(reference.familyId).toBe('family-alpha');
  });
});
