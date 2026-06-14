# Deployment Model

Purpose: define how the shared Cloudflare module promotes from local to production without leaking secrets or overclaiming readiness.

## Environments

| Environment | File | Notes |
| --- | --- | --- |
| Development | `wrangler.toml` | Local-friendly bindings, placeholder IDs, no real secrets in repo |
| Production | `wrangler.production.toml` | Production names, production origins, placeholder IDs only in repo |

## Required commands

- `npm --prefix infra/cloudflare run deploy:dev`
- `npm --prefix infra/cloudflare run deploy`

## Promotion rules

- Dev and production config must not share literal secret values in repo.
- Production origins must be explicit; no wildcard origin in production.
- Payment route cannot infer production readiness from scaffold deployment commands alone.

## Rollback rules

- Keep production rollback as `wrangler.production.toml` plus previous deploy artifact/version.
- Reconciliation, dead-letter, and secret rotation docs must survive rollback.
