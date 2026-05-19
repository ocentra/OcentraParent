import { describe, expect, it } from 'vitest';
import { ParentDomainPackage } from '../src/package-info';

describe('parent-domain package info', () => {
  it('ParentDomainPackage: identifies the parent contract boundary', () => {
    expect(ParentDomainPackage.Boundary).toBe('parent-product-contracts');
  });
});
