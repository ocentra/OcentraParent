export type AuthState =
  | 'public'
  | 'parent-session-required'
  | 'trusted-parent-device-required'
  | 'admin-required'
  | 'support-required'
  | 'provider-webhook-signature-required'
  | 'internal-queue-only';

export type AuthAdapterMethod =
  | 'verifyPublic'
  | 'verifyParentSession'
  | 'verifyTrustedParentDevice'
  | 'verifyAdmin'
  | 'verifySupport'
  | 'verifyProviderWebhook'
  | 'verifyInternalQueue';

export type RouteAuditRule =
  | 'public-observability'
  | 'parent-session-read'
  | 'parent-session-write'
  | 'trusted-parent-device-read'
  | 'trusted-parent-device-write'
  | 'support-read'
  | 'support-write'
  | 'admin-read'
  | 'admin-write'
  | 'provider-webhook'
  | 'internal-queue';

export interface AuthStateModel {
  state: AuthState;
  adapterMethod: AuthAdapterMethod;
  privateRoute: boolean;
  manualRequiredOwner:
    | 'not-applicable'
    | 'account-identity-family-plan'
    | 'provider-webhook-proof'
    | 'cloudflare-control-plane-plan';
}

export const AUTH_STATE_MODELS: Record<AuthState, AuthStateModel> = {
  public: {
    state: 'public',
    adapterMethod: 'verifyPublic',
    privateRoute: false,
    manualRequiredOwner: 'not-applicable',
  },
  'parent-session-required': {
    state: 'parent-session-required',
    adapterMethod: 'verifyParentSession',
    privateRoute: true,
    manualRequiredOwner: 'account-identity-family-plan',
  },
  'trusted-parent-device-required': {
    state: 'trusted-parent-device-required',
    adapterMethod: 'verifyTrustedParentDevice',
    privateRoute: true,
    manualRequiredOwner: 'account-identity-family-plan',
  },
  'admin-required': {
    state: 'admin-required',
    adapterMethod: 'verifyAdmin',
    privateRoute: true,
    manualRequiredOwner: 'account-identity-family-plan',
  },
  'support-required': {
    state: 'support-required',
    adapterMethod: 'verifySupport',
    privateRoute: true,
    manualRequiredOwner: 'account-identity-family-plan',
  },
  'provider-webhook-signature-required': {
    state: 'provider-webhook-signature-required',
    adapterMethod: 'verifyProviderWebhook',
    privateRoute: true,
    manualRequiredOwner: 'provider-webhook-proof',
  },
  'internal-queue-only': {
    state: 'internal-queue-only',
    adapterMethod: 'verifyInternalQueue',
    privateRoute: true,
    manualRequiredOwner: 'cloudflare-control-plane-plan',
  },
};

export interface AuthBoundaryRouteLike {
  path: string;
  method: string;
  authState: AuthState;
  auditEvent: string;
  auditRule: RouteAuditRule;
}

export type AuthBoundaryViolationReason =
  | 'naked-private-route'
  | 'admin-support-route-without-elevated-state'
  | 'admin-support-routes-require-audit-rule'
  | 'webhook-route-auth-state-mismatch'
  | 'internal-queue-route-auth-state-mismatch';

const ADMIN_SUPPORT_AUDIT_RULES = new Set<RouteAuditRule>([
  'support-read',
  'support-write',
  'admin-read',
  'admin-write',
]);

export function getAuthStateModel(authState: AuthState): AuthStateModel {
  return AUTH_STATE_MODELS[authState];
}

export function validateAuthBoundaryRoute(route: AuthBoundaryRouteLike): AuthBoundaryViolationReason | null {
  const isPrivateRoute =
    route.path.startsWith('/auth/') || route.path.startsWith('/admin/') || route.path.startsWith('/webhooks/');
  if (isPrivateRoute && !getAuthStateModel(route.authState).privateRoute) {
    return 'naked-private-route';
  }

  const isAdminSupportSurface = route.path.startsWith('/admin/') || route.path === '/auth/billing/manual-invoice';
  if (isAdminSupportSurface) {
    if (route.path === '/admin/billing/reconciliation') {
      return route.authState === 'internal-queue-only' ? null : 'internal-queue-route-auth-state-mismatch';
    }

    if (route.authState !== 'admin-required' && route.authState !== 'support-required') {
      return 'admin-support-route-without-elevated-state';
    }

    if (!ADMIN_SUPPORT_AUDIT_RULES.has(route.auditRule)) {
      return 'admin-support-routes-require-audit-rule';
    }

    if (route.authState === 'admin-required' && !route.auditRule.startsWith('admin-')) {
      return 'admin-support-routes-require-audit-rule';
    }

    if (route.authState === 'support-required' && !route.auditRule.startsWith('support-')) {
      return 'admin-support-routes-require-audit-rule';
    }

    return null;
  }

  if (route.path.startsWith('/webhooks/')) {
    return route.authState === 'provider-webhook-signature-required' ? null : 'webhook-route-auth-state-mismatch';
  }

  return null;
}
