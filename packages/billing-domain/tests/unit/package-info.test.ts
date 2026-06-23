import { describe, expect, it } from 'vitest';

import { BillingDomainPackageName } from '../../src/package-info';

describe('billing-domain package', () => {
  it('exposes the package identity', () => {
    expect(BillingDomainPackageName).toBe('@ocentra-parent/billing-domain');
  });
});
