import { describe, expect, it } from 'vitest';
import {
  enforcementProofClaimFlagsAreUnset,
  enforcementProofEntriesHaveUniqueField,
  enforcementProofRequiredUniqueValuesAreCovered,
  enforcementProofRequiredValuesAreCovered,
  enforcementProofValuesAreUnique,
} from '@ocentra-parent/schema-domain/enforcement-proof-shape';

describe('enforcement proof shape predicates', () => {
  it('checks proof row uniqueness and required coverage without accepting duplicate required values', () => {
    const entries = [{ id: 'alpha' }, { id: 'beta' }];

    expect(enforcementProofEntriesHaveUniqueField(entries, (entry) => entry.id)).toBe(true);
    expect(enforcementProofEntriesHaveUniqueField([...entries, { id: 'alpha' }], (entry) => entry.id)).toBe(false);
    expect(enforcementProofValuesAreUnique(['alpha', 'beta'])).toBe(true);
    expect(enforcementProofValuesAreUnique(['alpha', 'alpha'])).toBe(false);
    expect(enforcementProofRequiredValuesAreCovered(['alpha', 'beta'], ['alpha'])).toBe(true);
    expect(enforcementProofRequiredValuesAreCovered(['alpha'], ['alpha', 'beta'])).toBe(false);
    expect(enforcementProofRequiredUniqueValuesAreCovered(['alpha', 'beta'], ['alpha', 'beta'])).toBe(true);
    expect(enforcementProofRequiredUniqueValuesAreCovered(['alpha', 'alpha'], ['alpha'])).toBe(false);
  });

  it('keeps proof claim flag checks explicit about unset boolean fields', () => {
    expect(enforcementProofClaimFlagsAreUnset([false, false, false])).toBe(true);
    expect(enforcementProofClaimFlagsAreUnset([false, true, false])).toBe(false);
  });
});
