# payment-subscription-plan Event Architecture Instruction

## Owns

- billing product contracts;
- pricing, checkout, entitlement, invoice/refund/dispute, support/admin billing boundaries;
- billing-core and entitlement-core proof where local.

## Must not own

- Cloudflare worker runtime;
- account/session/trusted-device authority;
- parent portal broad UX unless assigned;
- Apple store proof without Apple host.

## Required chain

```text
checkout/billing command
-> billing owner validates product/entitlement contract
-> Cloudflare handoff artifact supplies worker truth
-> entitlement event/read model records state
-> parent/support surface consumes typed billing state
```

## Logging/proof

Log provider boundary, entitlement subject, household/device scope, webhook/idempotency result, redaction, support/admin access, and Cloudflare handoff reference.

## Tests

Billing-domain/core own unit/contract/security/property where applicable. Cloudflare tests remain Cloudflare-owned. Parent dashboard proof belongs in parent/portal integration.

## First architecture slice

Run payment worker/domain proof alpha locally. Final payment closure waits Cloudflare WP12 handoff artifact.
