# Deployment Model

Purpose: define how the shared Cloudflare module promotes from local to production without leaking secrets or overclaiming readiness.

## Environments

| Environment | File | Notes |
| --- | --- | --- |
| Development | `wrangler.toml` | Local-friendly bindings, placeholder IDs, no real secrets in repo, current deploy dry-run blocked before publish |
| Production | `wrangler.production.toml` | Production names, production origins, placeholder IDs only in repo, current deploy dry-run blocked before publish |

## Required commands

- `npm --prefix infra/cloudflare run deploy:dev`
- `npm --prefix infra/cloudflare run deploy`

Current scoped proof rerun uses `--dry-run` against both commands because no real publishable target is proven in this worktree.

## Promotion rules

- Dev and production config must not share literal secret values in repo.
- Production origins must be explicit; no wildcard origin in production.
- Payment route cannot infer production readiness from scaffold deployment commands alone.
- A deploy script is not deployment proof when Wrangler stops at local bundling or placeholder-backed config.
- Post-deploy `/health`, `/public/pricing`, and `/auth/billing/status` smoke only apply after a real promoted environment exists.

## Current blocked state

- `npm --prefix infra/cloudflare run deploy:dev -- --dry-run` is currently blocked before publish because WP01's `npm --prefix infra/cloudflare ls wrangler @cloudflare/workers-types` preflight reports an empty dependency tree. `src/index.ts` consumes the checked-in module-local generated billing-contract artifact; it has no private billing-domain import gate.
- `npm --prefix infra/cloudflare run deploy -- --dry-run` is currently blocked by that same unresolved WP01 dependency graph. After it is restored, retain the actual current deploy diagnostics rather than carrying forward a removed billing-domain import failure.
- The 2026-08-24 WP02 repair removed `--env` selectors: development uses the top-level `wrangler.toml`, while production explicitly selects top-level `wrangler.production.toml`. Focused direct Wrangler 4.25.1 `--dry-run --no-bundle` checks resolve all 11 environment-specific bindings from each config with no inheritance warning. This is configuration-resolution evidence only; the package's local dependency/contracts-build preflight and deployment proof remain open.
- Both configs still expose placeholder-backed D1 and KV identifiers and manual-required auth/key references, so no promotion or rollback readiness may be inferred.

## Rollback rules

- Keep production rollback as `wrangler.production.toml` plus previous deploy artifact/version.
- Reconciliation, dead-letter, and secret rotation docs must survive rollback.
- Until a real deploy artifact/version exists, rollback remains manual-required rather than proved.
