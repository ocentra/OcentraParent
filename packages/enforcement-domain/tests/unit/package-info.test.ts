import { describe, expect, it } from 'vitest';

import { EnforcementDomainPackageName } from '../../src/package-info';

describe('enforcement domain package', () => {
  it('exposes the package identity', () => {
    expect(EnforcementDomainPackageName).toBe('@ocentra-parent/enforcement-domain');
  });
});
