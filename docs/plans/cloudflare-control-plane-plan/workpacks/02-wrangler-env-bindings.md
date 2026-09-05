# Workpack 02: Wrangler Env Bindings

## Status

`code-and-test-source complete / execution-and-proof open`

The accepted production source is integrated at `7eabc9ff5`, and canonical
`4ddb47353` completes the expected env-binding test source plus the documented
Stripe webhook tolerance example. This is implementation-phase evidence only;
none of the tests has been executed and the workpack is not closed.

## Goal

Define development and production Wrangler config, binding names, and secret custody.

## First-touch surface

- `infra/cloudflare/wrangler.toml`
- `infra/cloudflare/wrangler.production.toml`
- `infra/cloudflare/.dev.vars.example`

## Read inputs

- [STORAGE_BINDING_MODEL.md](../STORAGE_BINDING_MODEL.md)
- [DEPLOYMENT_MODEL.md](../DEPLOYMENT_MODEL.md)

## Output files

- `infra/cloudflare/wrangler.toml`
- `infra/cloudflare/wrangler.production.toml`
- `infra/cloudflare/.dev.vars.example`
- `output/cloudflare-control-plane-plan-proof/02-wrangler-env-bindings/`

## Acceptance

- Development and production bindings are explicit.
- Secret names appear only as placeholders.
- No wildcard production origin claim exists.

## Current source truth

- Development and production Wrangler files declare explicit origins and
  binding names; checked-in identifiers and secrets remain placeholders.
- `infra/cloudflare/src/env.ts` now treats `APP_ORIGIN` and
  `CORS_ALLOWED_ORIGINS` as untrusted runtime values. Missing, non-string, and
  malformed values return controlled validation errors instead of throwing.
- Production rejects wildcard app and CORS origins after comma-list trimming.
  Local, test, and development environments retain their local behavior.
- Worker request handling validates the environment before parsing or using an
  origin. `EntitlementSnapshotDO` remains explicitly `not-wired`.
- Independent source review accepted this bounded behavior. No formal WP02 test
  run, retained proof, deployment, or runtime-readiness claim followed.

## Proof IDs

- `cloudflare-control.wrangler-dev-config`
- `cloudflare-control.wrangler-prod-config`
- `cloudflare-control.dev-vars-example`

## Validation

- Scoped validation: `npm --prefix infra/cloudflare run test:unit`
- Scoped validation: `npm --prefix infra/cloudflare run lint`
- Architecture validation: `npm run lint:architecture -- --files infra/cloudflare`

## Negative cases

- Reject real secret values in repo.
- Reject production wildcard origins.

## Failure conditions

- Do not claim deployability without real binding setup proof.

## Completion

- Status: code-and-test source complete; execution, retained proof, deployment,
  and runtime composition remain open.
- Expected proof root:
  `output/cloudflare-control-plane-plan-proof/02-wrangler-env-bindings/` (not
  tracked at the current canonical head).
- Owned config/source surface: `infra/cloudflare/wrangler.toml`,
  `infra/cloudflare/wrangler.production.toml`,
  `infra/cloudflare/.dev.vars.example`, `infra/cloudflare/src/env.ts`.
- Owned expected-test surface:
  `infra/cloudflare/tests/unit/env-bindings.test.ts`.

## What source and review establish

- Development and production Wrangler configs both declare the expected D1, KV, queue, optional R2, analytics, and Durable Object binding names explicitly.
- Production origin and CORS origin remain explicit as `https://parent.ocentra.com`; no wildcard production origin claim exists.
- Checked-in secret-bearing values in `.dev.vars.example` remain placeholders only; no real secret values are kept in repo.
- Auth and entitlement refs in checked-in Wrangler config remain explicit `manual-required` placeholders rather than accidental readiness claims.

## Open execution and delivery truth

- `infra/cloudflare/tests/unit/env-bindings.test.ts` now contains the complete
  undefined/non-string/malformed origin, comma-list wildcard, preserved local
  behavior, binding ownership, optional binding, unknown-key, and placeholder
  secret matrix. It has not been executed.
- Run the focused WP02 unit/lint commands in the later execution phase; checked
  in assertions are not test proof until they run.
- Real binding provisioning, deployment credentials, deployment/rollback
  validation, and the `EntitlementSnapshotDO` runtime remain outside this
  accepted packet and are still open.
- The historical private `packages/billing-domain/src/*` import blocker is
  resolved and must not be revived.

## Expected proof artifacts (not retained)

- `00-scope-summary.md`
- `01-negative-case-proof.md`
- `02-rollback-or-teardown-proof.md`
- `16-validation-commands.log`

## Focused validations

- The 2026-08-28 test-source packet used static review and coordination only;
  it ran no tests, builds, validation scans, proof, pre-commit, CI, or PR.
- Deferred test phase:
  `node --import tsx --test infra/cloudflare/tests/unit/env-bindings.test.ts`.
- Deferred broader module phase: `npm --prefix infra/cloudflare run test:unit`
  and `npm --prefix infra/cloudflare run lint`.
- Architecture remains subject to the separately recorded Protected Custody
  missing-test-scaffold baseline; it is not evidence against the accepted
  one-file WP02 source behavior.

## No-claim boundary

- No claim is made that the checked-in Wrangler files are deployable as-is.
- No claim is made that placeholder D1/KV identifiers or manual-required refs prove environment readiness.
- No claim is made that the Cloudflare worker boots successfully in this worktree.
