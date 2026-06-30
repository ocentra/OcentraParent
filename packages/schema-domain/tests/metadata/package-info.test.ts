import { describe, expect, it } from 'vitest';
import { packageName, packageRole } from '../../src/package-info';

describe('schema-domain package info', () => {
  it('keeps the package name canonical', () => {
    expect(packageName()).toBe('@ocentra-parent/schema-domain');
  });

  it('keeps the package role generated-thin-only', () => {
    expect(packageRole()).toBe('generated-thin-only');
  });
});
