# Games Infra Parity Map

Purpose: record the reusable games Cloudflare control-plane deep dive and reduce it to the exact Parent Cloudflare control-plane prerequisites.

## Sources inspected

- `package.json`
- `wrangler.toml`
- `wrangler.production.toml`
- `.env.example`
- `src/index.ts`
- `src/constants/env.ts`
- `src/utils/routes.ts`
- `src/utils/auth-middleware.ts`
- `src/handlers/payments.ts`
- `src/flows/payment-checkout-flow.ts`
- `src/flows/stripe-webhook-flow.ts`
- `src/durable-objects/PaymentDO.ts`
- `scripts/test-runner.ts`
- `docs/ARCHITECTURE.md`
- `docs/TEST-README.md`
- `docs/DOC-INDEX.md`
- `tests/`

## Keep 1:1 in concept

| Games pattern | Parent decision | Why |
| --- | --- | --- |
| Separate `infra/cloudflare/` package | Keep | Shared backend module is larger than payment. |
| Dedicated `wrangler.toml` and `wrangler.production.toml` | Keep | Parent needs explicit dev/prod bindings and no real secrets in repo. |
| Local env example file | Keep as `.dev.vars.example` | Parent needs reproducible local worker shape. |
| Worker entrypoint with env validation, CORS, request-size guard, kill switch, safe errors, scheduled hooks | Keep | These are shared control-plane requirements. |
| Manifest-based routing and handler dispatch | Keep | Parent should not let handlers invent raw route strings. |
| Auth middleware before private routes | Keep, but swap Firebase-specific assumption for adapter interface | Parent account authority is still unresolved. |
| Durable Object state coordination | Keep | Per-account, per-household, and idempotency writes need serialization. |
| PaymentDO-style event/state store | Adapt into Parent billing/referral/entitlement DO set | Parent keeps the control-plane pattern, not the game payment semantics. |
| Queue-backed reconciliation and delayed work | Keep | Payment, support/admin, and later portal flows need async retry. |
| Test runner with local and remote modes | Keep, reduced | Parent needs the runner pattern, not every games mode on day one. |
| Heavy docs/test/proof boundary | Keep | This repo expects execution-grade routes, not ad hoc infra notes. |

## Adapt for Parent

| Games surface | Parent reduction |
| --- | --- |
| `PAYMENT_DO` only | Split ownership into `BILLING_DO`, `REFERRAL_DO`, and `ENTITLEMENT_SNAPSHOT_DO` where needed. |
| Firebase-specific auth | Replace with adapter interface and `account-auth-adapter-manual-required` when unresolved. |
| Game R2 buckets (`MATCHES_BUCKET`, `ASSETS_BUCKET`, `AVATAR_BUCKET`) | Reduce to optional `BILLING_AUDIT_R2` for support-safe audit/export artifacts only. |
| Analytics/logging stack | Keep redacted observability and audit events; skip game-specific analytics noise. |
| Scheduled leaderboard and archive jobs | Keep only reconciliation and support-safe retention hooks. |
| Games `endpoint-domain` route manifest | Parent route manifest must be payment/portal/support focused and owned by Parent domain packages or route docs. |
| Pool/threads/production three-mode test runner | Parent needs local and remote smoke first; `unstable`/threads stays optional until a concrete debugging need appears. |
| Full external-tool matrix (k6, Schemathesis, mutation, CodeQL, Trivy, Semgrep) | Parent keeps unit/integration/e2e/contract/security/property/fuzz as required, with load/mutation/static-analysis as recommended rollout gates. |

## Strip completely

- Match coordination
- Lobby, party, matchmaking, presence, friends, messages
- Leaderboard, progression, rewards, tournament payout
- Marketplace and in-game inventory
- Game credits / GP / AC economy
- Solana payment and on-chain assumptions
- AI proxy, AI key escrow, AI catalog
- Asset serving, replay storage, avatar buckets, signed game URLs
- Anti-cheat and gameplay fraud detection
- Game notifications unless a future Parent notification route explicitly depends on Cloudflare

## Parent-required module tree

```text
infra/cloudflare/
  package.json
  wrangler.toml
  wrangler.production.toml
  .dev.vars.example
  src/
    index.ts
    env.ts
    routes.ts
    auth/
    handlers/
    flows/
    durable-objects/
    queues/
    storage/
    providers/
    security/
    observability/
  scripts/
  tests/
    unit/
    integration/
    e2e/
    contract/
    security/
    property/
    fuzz/
  docs/
    ARCHITECTURE.md
    LOCAL_DEV.md
    SECRETS_AND_ENV.md
    TESTING.md
    DEPLOYMENT.md
    ROUTES.md
    AUTH_BOUNDARY.md
    STORAGE_BINDINGS.md
```

## Required bindings stripped for Parent

| Binding group | Parent minimum |
| --- | --- |
| Durable Objects | `BILLING_DO`, `REFERRAL_DO`, `ENTITLEMENT_SNAPSHOT_DO` |
| D1 | `BILLING_D1` |
| Queues | `BILLING_RECONCILIATION_QUEUE`, `BILLING_DEAD_LETTER_QUEUE` |
| KV | `BILLING_RATE_LIMIT_KV`, `BILLING_CONFIG_KV` |
| Optional R2 | `BILLING_AUDIT_R2` |
| Optional analytics | `ANALYTICS` |
| Secrets | `STRIPE_SECRET_KEY`, `STRIPE_WEBHOOK_SECRET`, `RAZORPAY_KEY_ID`, `RAZORPAY_KEY_SECRET`, `PAYPAL_CLIENT_ID`, `PAYPAL_CLIENT_SECRET`, `APPLE_STORE_KEY_REF`, `GOOGLE_PLAY_SERVICE_ACCOUNT_REF`, `ENTITLEMENT_SIGNING_KEY_REF` |

Every secret remains server-only; never browser, portal bundle, desktop, or mobile.

## Test parity reduction

| Games test surface | Parent requirement |
| --- | --- |
| Large Vitest pool/threads/prod matrix | Keep local runner shape; keep remote smoke as a later gate. |
| Unit + integration + e2e + contract | Keep all four. |
| Security tests | Keep and focus on auth, CORS, request limits, redaction, secret leakage, webhook signature rejection, and admin/support rejections. |
| Property tests | Keep for route/auth state and idempotency invariants. |
| Fuzz tests | Keep for provider webhook payloads and request boundary inputs. |
| k6 / load tests | Recommended rollout gate, not slice-01 prerequisite. |
| Mutation tests | Recommended rollout gate, not slice-01 prerequisite. |
| Static analysis wrappers | Recommended before production promotion. |

## Carried non-goals and no-claim boundary

- This parity map does not import or re-own billing product math, referral qualification semantics, invoice/grace policy, or payment runtime readiness.
- This parity map does not choose the concrete account provider, parent session authority, trusted-parent-device authority, or setup/bootstrap semantics.
- This parity map does not authorize child telemetry, raw child data, or generic archive/storage expansion in Cloudflare D1, KV, Queue, or R2 surfaces.
- This parity map does not carry over game-only economy, Solana, matchmaking, social, AI proxy, asset delivery, leaderboard, or tournament concerns.
- This parity map does not claim deployment, queue operations, or portal consumer readiness from docs alone.

## Parent-first conclusion

Parent does need a separate Cloudflare control-plane plan and module scaffold. Payment cannot honestly start from checkout or webhook docs alone because the worker boundary, bindings, auth model, local dev loop, and test runner are shared prerequisites.
