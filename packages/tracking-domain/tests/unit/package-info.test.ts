import { describe, expect, it } from 'vitest';
import { TrackingDomainPackageName } from '../../src/package-info';

describe('tracking-domain package info', () => {
  it('keeps the package name canonical', () => {
    expect(TrackingDomainPackageName).toBe('@ocentra-parent/tracking-domain');
  });
});
