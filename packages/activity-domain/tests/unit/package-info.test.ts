import { describe, expect, it } from 'vitest';
import { ActivityDomainPackage } from '../../src/package-info';

describe('activity-domain package info', () => {
  it('ActivityDomainPackage: identifies the device activity boundary', () => {
    expect(ActivityDomainPackage.Boundary).toBe('device-activity-contracts');
  });
});
