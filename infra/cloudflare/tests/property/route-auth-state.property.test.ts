import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { validateAuthBoundaryRoute } from '../../src/auth/model.js';
import { ROUTE_MANIFEST } from '../../src/routes.js';

describe('route auth-state properties', () => {
  it('keeps path families aligned with auth boundary expectations', () => {
    for (const route of ROUTE_MANIFEST) {
      if (route.path === '/health' || route.path.startsWith('/public/')) {
        assert.equal(route.authState, 'public');
        continue;
      }

      if (route.path.startsWith('/webhooks/')) {
        assert.equal(route.authState, 'provider-webhook-signature-required');
        continue;
      }

      if (route.path === '/admin/billing/reconciliation') {
        assert.equal(route.authState, 'internal-queue-only');
        continue;
      }

      if (
        route.path === '/admin/billing/refunds' ||
        route.path === '/admin/billing/disputes' ||
        route.path === '/admin/billing/referrals' ||
        route.path === '/admin/billing/audit'
      ) {
        assert.equal(route.authState, 'admin-required');
        continue;
      }

      if (route.path.startsWith('/admin/')) {
        assert.equal(route.authState, 'support-required');
        continue;
      }

      if (route.path === '/auth/billing/entitlement-snapshot' || route.path === '/auth/billing/license-check') {
        assert.equal(route.authState, 'trusted-parent-device-required');
        continue;
      }

      if (route.path === '/auth/billing/manual-invoice') {
        assert.equal(route.authState, 'support-required');
        continue;
      }

      if (route.path.startsWith('/auth/')) {
        assert.equal(route.authState, 'parent-session-required');
      }
    }
  });

  it('keeps manual-invoice as the only elevated auth exception inside the /auth/billing route family', () => {
    const elevatedAuthBillingRoutes = ROUTE_MANIFEST.filter(
      (route) =>
        route.path.startsWith('/auth/billing/') &&
        route.authState !== 'parent-session-required' &&
        route.authState !== 'trusted-parent-device-required'
    ).map((route) => `${route.method} ${route.path}`);

    assert.deepEqual(elevatedAuthBillingRoutes, ['POST /auth/billing/manual-invoice']);
  });

  it('keeps audit rules aligned with the stronger admin and support route families', () => {
    for (const route of ROUTE_MANIFEST) {
      assert.equal(validateAuthBoundaryRoute(route), null);

      if (route.authState === 'admin-required') {
        assert.match(route.auditRule, /^admin-/u);
      }

      if (route.authState === 'support-required') {
        assert.match(route.auditRule, /^support-/u);
      }
    }
  });
});
