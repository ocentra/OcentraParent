import { describe, expect, it } from 'vitest';
import { PortalDomainPackage } from '../../src/package-info';

describe('portal-domain package info', () => {
  it('PortalDomainPackage: identifies the portal DOM contract boundary', () => {
    expect(PortalDomainPackage.Boundary).toBe('portal-route-dom-contracts');
  });
});
