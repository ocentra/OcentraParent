import { describe, expect, it } from 'vitest';
import { BrowserDomainPackageName } from '../../src/package-info';

describe('browser domain package boundary', () => {
  it('declares the canonical browser domain package', () => {
    expect(BrowserDomainPackageName).toBe('@ocentra-parent/browser-domain');
  });
});

