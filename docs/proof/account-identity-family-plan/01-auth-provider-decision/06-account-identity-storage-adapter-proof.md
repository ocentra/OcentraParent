# WP01 account-identity storage-adapter proof

## Scope

- Plan: `account-identity-family-plan`
- Workpack: `WP01 Auth Provider Decision`
- Owner: `infra/cloudflare`
- Slice: minimal verified-provider-subject to Ocentra account persistence boundary
- Branch: `codex/lane-account-identity`
- Base: `c00434c16`
- Evidence date: `2026-08-06`

This is a bounded adapter/persistence proof. It does not close WP01 or claim
that authentication, session routes, household authority, or Cloudflare
deployment is ready.

## Authority boundary proved

`infra/cloudflare/src/storage/account-identity-store.ts` stores only the
minimal mapping required after an external provider has already verified a
subject:

```text
provider + provider subject -> Ocentra account id + active/revoked status
```

The optional `ACCOUNT_IDENTITY_D1` binding owns this mapping. The adapter:

- initializes a D1-shaped SQLite table with provider-subject uniqueness;
- persists a new mapping and updates status for the same account;
- returns a conflict without overwriting when a subject is linked to another
  account;
- returns `manual-required` when the optional binding is absent; and
- rejects unsupported providers, malformed identifiers, control characters,
  overlong values, invalid status, and invalid timestamps before writes.

No provider token, raw claim, household/member/child/device/session data, or
secret is accepted or logged by this slice.

## Evidence pointers

- `infra/cloudflare/src/storage/account-identity-store.ts`
- `infra/cloudflare/tests/unit/account-identity-store.test.ts`
- `infra/cloudflare/src/env.ts`
- `infra/cloudflare/tests/unit/env-bindings.test.ts`
- `infra/cloudflare/wrangler.toml`
- `infra/cloudflare/wrangler.production.toml`

## Focused validation

```text
command: npm --prefix infra/cloudflare run test:unit
result: pass; 53 tests, 8 suites, 0 failures; account adapter 4/4
command: npm --prefix infra/cloudflare run lint
result: pass; TypeScript noEmit
command: npm run lint:architecture -- --files infra/cloudflare/src/storage/account-identity-store.ts,infra/cloudflare/src/env.ts,infra/cloudflare/tests/unit/account-identity-store.test.ts,infra/cloudflare/tests/unit/env-bindings.test.ts,infra/cloudflare/scripts/test-runner.ts
result: pass; Ocentra Enforcer architecture-policy
```

## No-claim boundary

This proof does not establish external provider token verification, login or
session route wiring, household/membership/role/device authority, invite or
recovery runtime, D1 migrations or deployment, Cloudflare worker production
readiness, or full WP01/plan closure. `ACCOUNT_IDENTITY_D1` remains optional
and manual-required until separately owned runtime-route and deployment proof
exists.
