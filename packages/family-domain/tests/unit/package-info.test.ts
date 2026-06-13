import { describe, expect, it } from 'vitest';
import { packageName } from '../../src/package-info';

describe('family-domain package info', () => {
  it('keeps the package name canonical', () => {
    expect(packageName()).toBe('@ocentra-parent/family-domain');
  });
});
