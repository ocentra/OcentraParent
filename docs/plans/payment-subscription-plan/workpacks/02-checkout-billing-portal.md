# Workpack 02: Checkout and Billing Portal

Purpose: define the Cloudflare Worker/API boundary for checkout session creation, hosted portal sessions, and billing entry points.

## Owns

- `CLOUDFLARE_BILLING_CONTROL_PLANE.md`
- `BILLING_API_BOUNDARY.md`
- `CHECKOUT_BILLING_PORTAL_MODEL.md`
- PSP-001, PSP-002, PSP-004, and PSP-006

## Must prove

- Checkout sessions are created server-side.
- Portal sessions are created server-side.
- Missing provider config fails safely.
- Checkout success is not treated as payment proof.
- Browser callers never see provider secrets.

## Proof path

- Use `docs/proof/payment-subscription-plan/wp02/` or the owning crate's local proof directory.

## Failure conditions

- The workpack fails if the browser can complete payment without the server boundary.
- The workpack fails if checkout or portal flows leak secrets.
- The workpack fails if a redirect is treated as final entitlement proof.
