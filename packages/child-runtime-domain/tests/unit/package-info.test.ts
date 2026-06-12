import { describe, expect, it } from 'vitest';

import { ChildRuntimeDomainPackageName } from '../../src/package-info';

describe('child-runtime-domain package', () => {
  it('exposes the package identity', () => {
    expect(ChildRuntimeDomainPackageName).toBe('@ocentra-parent/child-runtime-domain');
  });
});