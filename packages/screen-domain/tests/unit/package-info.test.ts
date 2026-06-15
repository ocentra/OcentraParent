import { describe, expect, it } from 'vitest';
import { ScreenDomainPackageName } from '../../src/package-info';

describe('screen domain package boundary', () => {
  it('declares the canonical screen domain package', () => {
    expect(ScreenDomainPackageName).toBe('@ocentra-parent/screen-domain');
  });
});

