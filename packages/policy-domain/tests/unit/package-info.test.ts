import { describe, expect, it } from 'vitest';

import { PolicyDomainPackageName } from '../../src/package-info';

describe('policy domain package', () => {
  it('exposes the package identity', () => {
    expect(PolicyDomainPackageName).toBe('@ocentra-parent/policy-domain');
  });
});
