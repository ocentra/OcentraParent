import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { validateAuthBoundaryRoute, type AuthState, type RouteAuditRule } from '../../src/auth/model.js';
import { ROUTE_MANIFEST } from '../../src/routes.js';

const EXPECTED_AUTH_ROUTES = new Map([
  ['GET /auth/billing/status', 'parent-session-required'],
  ['POST /auth/billing/checkout', 'parent-session-required'],
  ['POST /auth/billing/portal', 'parent-session-required'],
  ['GET /auth/billing/invoices', 'parent-session-required'],
  ['POST /auth/billing/change-plan', 'parent-session-required'],
  ['POST /auth/billing/cancel', 'parent-session-required'],
  ['GET /auth/billing/referrals', 'parent-session-required'],
  ['POST /auth/billing/referral-invite', 'parent-session-required'],
  ['GET /auth/billing/entitlement-snapshot', 'trusted-parent-device-required'],
  ['POST /auth/billing/license-check', 'trusted-parent-device-required'],
  ['POST /auth/billing/manual-invoice', 'support-required'],
] as const);

const EXPECTED_WEBHOOK_ROUTES = [
  'POST /webhooks/stripe',
  'POST /webhooks/razorpay',
  'POST /webhooks/paypal',
  'POST /webhooks/apple',
  'POST /webhooks/google',
] as const;

const EXPECTED_ADMIN_ROUTES = new Map([
  ['GET /admin/billing/accounts', 'support-required'],
  ['GET /admin/billing/invoices', 'support-required'],
  ['POST /admin/billing/refunds', 'admin-required'],
  ['GET /admin/billing/disputes', 'admin-required'],
  ['GET /admin/billing/referrals', 'admin-required'],
  ['POST /admin/billing/reconciliation', 'internal-queue-only'],
  ['GET /admin/billing/audit', 'admin-required'],
] as const);

const EXPECTED_ROUTE_KEYS = [
  'GET /health',
  'GET /public/pricing',
  ...EXPECTED_AUTH_ROUTES.keys(),
  ...EXPECTED_ADMIN_ROUTES.keys(),
  ...EXPECTED_WEBHOOK_ROUTES,
].sort();

const ALLOWED_ROUTE_PATTERNS = [
  /^\/health$/u,
  /^\/public\/pricing$/u,
  /^\/auth\/billing\/[a-z-]+$/u,
  /^\/webhooks\/[a-z]+$/u,
  /^\/admin\/billing\/[a-z-]+$/u,
] as const;

describe('ROUTE_MANIFEST', () => {
  it('keeps route and method pairs unique', () => {
    const routeKeys = ROUTE_MANIFEST.map((route) => `${route.method} ${route.path}`);
    assert.equal(new Set(routeKeys).size, routeKeys.length);
  });

  it('keeps the manifest route list exact instead of allowing extra worker-only route strings to drift in', () => {
    const routeKeys = ROUTE_MANIFEST.map((route) => `${route.method} ${route.path}`).sort();
    assert.deepEqual(routeKeys, EXPECTED_ROUTE_KEYS);
  });

  it('pins every public, auth, webhook, and admin billing route to the expected auth state', () => {
    const expectations = new Map<string, string>([
      ['GET /health', 'public'],
      ['GET /public/pricing', 'public'],
      ...EXPECTED_AUTH_ROUTES,
      ...EXPECTED_ADMIN_ROUTES,
      ...EXPECTED_WEBHOOK_ROUTES.map((routeKey) => [routeKey, 'provider-webhook-signature-required'] as const),
    ]);

    for (const [routeKey, authState] of expectations) {
      const route = ROUTE_MANIFEST.find((entry) => `${entry.method} ${entry.path}` === routeKey);
      assert.ok(route, `Expected route ${routeKey} to exist`);
      assert.equal(route?.authState, authState);
    }
  });

  it('requires every manifest entry to declare the core contract metadata fields', () => {
    for (const route of ROUTE_MANIFEST) {
      assert.match(route.path, /^\//u);
      assert.ok(route.method === 'GET' || route.method === 'POST');
      assert.match(route.authState, /.+/u);
      assert.match(route.handlerKey, /.+/u);
      assert.match(route.requestModel, /.+/u);
      assert.match(route.responseModel, /.+/u);
      assert.match(route.auditRule, /.+/u);
      assert.match(route.auditEvent, /.+/u);
      assert.match(route.proofIdFamily, /.+/u);
    }
  });

  it('never marks auth, admin, or webhook billing routes as public', () => {
    const privateRoutes = ROUTE_MANIFEST.filter(
      (route) =>
        route.path.startsWith('/auth/') || route.path.startsWith('/admin/') || route.path.startsWith('/webhooks/')
    );

    assert.ok(privateRoutes.length > 0);
    for (const route of privateRoutes) {
      assert.notEqual(route.authState, 'public', `${route.method} ${route.path} must not be public`);
    }
  });

  it('keeps route paths inside the declared Cloudflare worker groups', () => {
    for (const route of ROUTE_MANIFEST) {
      assert.ok(
        ALLOWED_ROUTE_PATTERNS.some((pattern) => pattern.test(route.path)),
        `Unexpected route outside declared groups: ${route.method} ${route.path}`
      );
    }
  });

  it('keeps every declared route inside the explicit auth-boundary guardrails', () => {
    for (const route of ROUTE_MANIFEST) {
      assert.equal(validateAuthBoundaryRoute(route), null, `${route.method} ${route.path} must remain boundary-valid`);
    }
  });

  it('rejects naked private routes and admin-support routes without explicit audit rules', () => {
    const supportRoute = ROUTE_MANIFEST.find((route) => route.path === '/admin/billing/accounts');
    assert.ok(supportRoute);

    const nakedPrivateRoute = {
      ...supportRoute,
      authState: 'public' as AuthState,
      auditRule: 'public-observability' as RouteAuditRule,
    };
    const missingAuditRuleRoute = {
      ...supportRoute,
      auditRule: 'public-observability' as RouteAuditRule,
    };

    assert.equal(validateAuthBoundaryRoute(nakedPrivateRoute), 'naked-private-route');
    assert.equal(validateAuthBoundaryRoute(missingAuditRuleRoute), 'admin-support-routes-require-audit-rule');
  });
});
