import { describe, expect, it } from 'vitest';

import { ProductionDomainPackageName } from '../../src/package-info';

describe('production-domain package', () => {
  it('exposes the package identity', () => {
    expect(ProductionDomainPackageName).toBe('@ocentra-parent/production-domain');
  });
});
