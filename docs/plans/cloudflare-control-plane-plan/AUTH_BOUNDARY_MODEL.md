# Auth Boundary Model

Purpose: define the Cloudflare-side trust states without hardcoding the final account provider.

## Auth states

| State | Meaning | Examples |
| --- | --- | --- |
| `public` | No caller identity required. | `/health`, `/public/pricing`, `/auth/session/login` |
| `browser-session-required` | A provider-issued browser access session and its verified session identity are required. | Browser session identity established by `/auth/session/login` and consumed by authenticated browser requests |
| `browser-refresh-required` | The protected browser refresh cookie, CSRF token, and current session binding are required. | `/auth/session/refresh`, `/auth/session/logout`, `/auth/session/revoke` |
| `parent-session-required` | Parent auth session required. | `/auth/billing/status`, `/auth/billing/invoices` |
| `trusted-parent-device-required` | Parent session plus trusted-device proof required. | `/auth/billing/entitlement-snapshot`, `/auth/billing/license-check` |
| `admin-required` | Elevated admin role required. | `/admin/billing/refunds`, `/admin/billing/reconciliation` |
| `support-required` | Limited support role required, with audit logging. | Account lookup and safe support views where admin is not required |
| `provider-webhook-signature-required` | Provider signature must verify before any processing. | `/webhooks/stripe`, `/webhooks/paypal` |
| `internal-queue-only` | Queue or cron caller only. | retry, reconciliation, dead-letter handlers |

## Required route rules

- `/public/pricing` may be public.
- `/auth/session/login` accepts the public login request and must establish the
  provider-owned session boundary; it is not proof of device trust.
- `/auth/session/refresh`, `/auth/session/logout`, and `/auth/session/revoke`
  are explicitly `browser-refresh-required`. The protected refresh cookie,
  CSRF token, and current browser-session binding are the authority inputs;
  callers cannot promote a DTO or caller-supplied user id.
- All `/auth/billing/*` routes require `parent-session-required` unless the route is one of the explicit stronger exceptions below.
- `/auth/billing/entitlement-snapshot` and `/auth/billing/license-check` additionally require `trusted-parent-device-required` or an explicit manual-required blocker.
- `/auth/billing/manual-invoice` is the only support-owned exception inside the `/auth/billing/*` family and must remain `support-required` plus `support-write`.
- All `/admin/*` routes require `admin-required` or `support-required` plus audit logging.
- All `/webhooks/*` routes require `provider-webhook-signature-required`.
- Queue and cron routes are `internal-queue-only`.

## Adapter interface

```text
AuthVerifier
  verifyParentSession(request)
  verifyTrustedParentDevice(request)
  verifyAdmin(request)
  verifySupport(request)
  verifyProviderWebhook(provider, request)
```

Adapter mode contract:

- `local-safe-fixture` is the only test/local fixture mode that may satisfy parent/admin/support checks today.
- `account-auth-adapter-manual-required` keeps parent, trusted-device, admin, and support routes in `manual-required` until `account-identity-family-plan` chooses the real provider.
- Unknown future adapter modes must also return `manual-required`; do not silently assume a provider.

## Audit rule contract

Admin and support surfaces must declare an explicit audit rule in the route manifest, not just a non-empty audit event string.

| Route family | Auth state | Required audit rule family |
| --- | --- | --- |
| support review surfaces | `support-required` | `support-read` |
| support mutation surfaces | `support-required` | `support-write` |
| admin review surfaces | `admin-required` | `admin-read` |
| admin mutation surfaces | `admin-required` | `admin-write` |

## Current blocker handling

- Concrete provider choice is owned by `account-identity-family-plan`.
- Until that decision lands, use `account-auth-adapter-manual-required` in proof and source matrices.
