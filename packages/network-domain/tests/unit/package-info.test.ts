import { describe, expect, it } from 'vitest';
import { NetworkDomainPackageName } from '../../src/package-info';

describe('network domain package boundary', () => {
  it('declares the canonical network domain package', () => {
    expect(NetworkDomainPackageName).toBe('@ocentra-parent/network-domain');
  });
});

