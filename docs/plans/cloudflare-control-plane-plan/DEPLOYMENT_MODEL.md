# Deployment Model

Purpose: define how the shared Cloudflare module promotes from local to production without leaking secrets or overclaiming readiness.

## Environments

| Environment | File | Notes |
| --- | --- | --- |
| Development | `wrangler.toml` | Local-friendly bindings, placeholder IDs, no real secrets in repo; the current `--dry-run` exits 0 and intentionally performs no publish |
| Production | `wrangler.production.toml` | Production names, production origins, placeholder IDs only in repo; the current `--dry-run` exits 0 and intentionally performs no publish |

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

## Current dry-run and blocked promotion state

- Current module lint truth: `npm --prefix infra/cloudflare run lint` passed at source base `2aab6310c`; this file no longer treats the Cloudflare module lint as active debt.
- `npm --prefix infra/cloudflare run deploy:dev -- --dry-run` exited 0 against source base `2aab6310c` on 2026-07-23, completed local bundling, and stopped at the explicit dry-run boundary without publishing.
- `npm --prefix infra/cloudflare run deploy -- --dry-run` exited 0 against the same source base, completed local bundling, and stopped at the explicit dry-run boundary without publishing.
- Both commands also emit a Wrangler warning because the scripts pass `--env development` or `--env production` without matching `[env.*]` sections in the chosen config file.
- No promoted endpoint exists from those commands, so post-deploy `/health`, `/public/pricing`, and `/auth/billing/status` smoke remains unrun and blocked.
- Both configs still expose placeholder-backed D1 and KV identifiers and manual-required auth/key references, so no promotion or rollback readiness may be inferred.

## Rollback rules

- Keep production rollback as `wrangler.production.toml` plus previous deploy artifact/version.
- Reconciliation, dead-letter, and secret rotation docs must survive rollback.
- Until a real deploy artifact/version exists, rollback remains manual-required rather than proved.
