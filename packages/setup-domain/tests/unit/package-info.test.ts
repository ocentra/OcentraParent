import { describe, expect, it } from 'vitest';
import { SetupDomainPackageName } from '../../src/package-info';

describe('setup domain package identity', () => {
  it('keeps the package name canonical', () => {
    expect(SetupDomainPackageName).toBe('@ocentra-parent/setup-domain');
  });
});
