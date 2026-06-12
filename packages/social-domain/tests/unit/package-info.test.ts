import { describe, expect, it } from 'vitest';
import { SocialDomainPackageName } from '../../src/package-info';

describe('social domain package boundary', () => {
  it('declares the canonical social domain package', () => {
    expect(SocialDomainPackageName).toBe('@ocentra-parent/social-domain');
  });
});

