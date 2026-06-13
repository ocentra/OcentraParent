import { describe, expect, it } from 'vitest';
import { TextDomainPackage } from '../../src/package-info';

describe('text-domain package info', () => {
  it('TextDomainPackage: identifies the schema-backed display text boundary', () => {
    expect(TextDomainPackage.Boundary).toBe('schema-backed-display-text');
  });
});
