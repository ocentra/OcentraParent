import { describe, expect, it } from 'vitest';
import { ParentDomainPackage } from '../../src/package-info';

describe('parent-domain package info', () => {
  it('ParentDomainPackage: identifies the remaining parent runtime boundary', () => {
    expect(ParentDomainPackage.Boundary).toBe('parent-runtime-behavior');
  });
});
