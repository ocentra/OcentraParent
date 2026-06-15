import { describe, expect, it } from 'vitest';

import { NotificationDomainPackageName } from '../../src/package-info';

describe('notification-domain package', () => {
  it('exposes the package identity', () => {
    expect(NotificationDomainPackageName).toBe('@ocentra-parent/notification-domain');
  });
});