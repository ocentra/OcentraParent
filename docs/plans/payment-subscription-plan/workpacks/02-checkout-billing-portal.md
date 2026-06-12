# Workpack 02: Checkout Billing Portal

Goal: define the payment frontend/server boundary.

Expected shape:

- Family site or portal calls an authenticated app API.
- Server/Worker creates Stripe Checkout Session or Customer Portal Session.
- Browser receives only a short-lived Stripe-hosted URL/session result.
- Stripe secrets, webhook secrets, and provider admin actions never run in client code.
- Checkout creation includes bot/abuse controls and strict return/cancel URL validation.

Expected proof:

- Authenticated checkout route proof.
- Invalid product and unauthorized role rejection.
- Origin/redirect validation.
- Secret-not-in-client scan.
- Billing portal session proof.

Failure: direct browser payment flow that bypasses app-owned auth, product validation, and entitlement ledger.

## Execution Detail

Minimum context:

- `E:\ocentra-games\infra\cloudflare\src\handlers\payments.ts`
- `E:\ocentra-games\infra\cloudflare\src\flows\payment-checkout-flow.ts`
- `E:\ocentra-games\infra\cloudflare\src\utils\turnstile.ts`
- Official Stripe Checkout and Customer Portal docs for current behavior.

Research required:

- Verify whether Parent should deploy billing API under the family web Worker, a shared API Worker, or an app-local backend during MVP.
- Discuss with Sujan whether custom payment UI is needed; default should be Stripe-hosted Checkout for lower PCI scope.

Required decisions:

- API route owner.
- Success/cancel URL allowlist.
- Customer creation timing.
- Billing portal entry point.
- Bot/abuse control.
- Test-mode and production-mode separation.

Expected tests/proof names:

- `checkout.auth-required`
- `checkout.turnstile-or-abuse-gate`
- `checkout.invalid-product-rejected`
- `checkout.redirect-allowlist`
- `checkout.no-client-secret-exposure`
- `billing-portal.short-lived-session`

Proof artifact expectations:

- Route matrix.
- Secret scan.
- Checkout/portal request-response proof.
- Explicit direct-browser rejection note.
