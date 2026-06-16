<!-- agent-capsule -->

> Agent Capsule
> Plan: `cloudflare-control-plane-plan`
> Doc: `Cloudflare Control Plane Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: runtime completion without matching artifacts.

<!-- /agent-capsule -->

# Cloudflare Control Plane Test Proof Expectations

## General rule

Use focused commands first. Broader validation is allowed only after focused commands pass or a precise blocker is recorded.

If a required Cloudflare module/test path does not exist yet, write a blocker artifact and leave the checklist row open.

## Common command set

Use the subset relevant to the selected workpack:

```bash
npm --prefix infra/cloudflare run build
npm --prefix infra/cloudflare run type-check
npm --prefix infra/cloudflare run test
npm --prefix infra/cloudflare run test:unit
npm --prefix infra/cloudflare run test:integration
npm --prefix infra/cloudflare run test:security
npm --prefix infra/cloudflare run test:property
npm run lint:architecture -- --files infra/cloudflare docs/plans/cloudflare-control-plane-plan
```

## Expected coverage by workpack

| Workpack | Expected proof focus |
| --- | --- |
| WP00 | games keep/adapt/strip map, game-only concern rejection, parent-safe module boundary |
| WP01 | module tree, package scripts, scaffold-only/no-claim labels, no consumer semantics |
| WP02 | wrangler envs, D1/DO/KV/R2/Queue binding names, secret custody, dev/prod separation |
| WP03 | worker entrypoint, env validation, request-size guard, origin/CORS behavior, kill-switch, scheduled hook shape |
| WP04 | route manifest, route groups, domain contract ownership, no ad hoc route strings |
| WP05 | auth/admin/support/webhook states, adapter boundary, signature/provider blockers |
| WP06 | DO/D1/KV/R2/Queue ownership, idempotency/cache/ledger/queue separation |
| WP07 | local dev, seed, fixture, teardown, emulator/miniflare/wrangler blockers |
| WP08 | test runner, exact assertion matrix, unit/integration/security/property/e2e family mapping |
| WP09 | portal-to-worker smoke, redacted request/response proof, no child private payloads |
| WP10 | security/property/fuzz/observability baseline with parent-only scope |
| WP11 | deploy/promotion/rollback/env separation/secret custody proof |
| WP12 | payment handoff assumptions, blockers, no-claim boundaries |

## Required negative states

```text
game-only code not copied
placeholder route not runtime proof
missing binding fails clearly
private/admin/support route lacks owner proof
provider/webhook assumption blocked until provider proof
D1/KV/R2/Queue claim has clear owner and purpose
payment remains blocked until WP12 handoff proof exists
production deployment claim requires WP11 proof
```

## Proof storage

Proof artifacts live under:

```text
output/cloudflare-control-plane-plan-proof/<workpack-id>/
```

Do not write new proof artifacts under `docs/proof/cloudflare-control-plane-plan/` unless preserving old references; new work should use `output/` proof roots.
