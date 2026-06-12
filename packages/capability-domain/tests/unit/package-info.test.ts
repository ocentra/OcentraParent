import { describe, expect, it } from 'vitest';

import { CapabilityDomainPackageName } from '../../src/package-info';

describe('capability domain package', () => {
  it('exposes the package identity', () => {
    expect(CapabilityDomainPackageName).toBe('@ocentra-parent/capability-domain');
  });
});
