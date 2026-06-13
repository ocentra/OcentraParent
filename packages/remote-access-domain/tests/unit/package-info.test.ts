import { describe, expect, it } from 'vitest';
import { RemoteAccessDomainPackageName } from '../../src/package-info';

describe('remote access domain package identity', () => {
  it('keeps the package name canonical', () => {
    expect(RemoteAccessDomainPackageName).toBe('@ocentra-parent/remote-access-domain');
  });
});
