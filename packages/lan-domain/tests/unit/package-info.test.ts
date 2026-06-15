import { describe, expect, it } from 'vitest';
import { LanDomainPackageName } from '../../src/package-info';

describe('lan domain package boundary', () => {
  it('declares the canonical LAN domain package', () => {
    expect(LanDomainPackageName).toBe('@ocentra-parent/lan-domain');
  });
});

