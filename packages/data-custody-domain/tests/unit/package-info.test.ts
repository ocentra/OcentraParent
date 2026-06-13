import { describe, expect, it } from 'vitest';
import { DataCustodyDomainPackageName } from '../../src/package-info';

describe('data custody domain package identity', () => {
  it('keeps the package name canonical', () => {
    expect(DataCustodyDomainPackageName).toBe('@ocentra-parent/data-custody-domain');
  });
});
