# WP07 Local Dev Seeding And Fixtures Proof

## Proof identity

- Date: `2026-07-20`
- Branch: `codex/cloudflare-wp12-handoff`
- Validated base head: `4573ab4364f9c85357cc4e4f7eb8255b91fff92b`
- Persisted-proof run ID: `cloudflare-wp07-20260720-full-integration-b`
- Scope: Cloudflare control-plane WP07 only

## Proven execution truth

- Every checked-in `seed:*:local` command starts a bounded Wrangler local runtime against the selected `--persist-to` store.
- `/health` completes only after the billing read model seeds and directly reads back positive D1 status/account/referral rows, KV pricing rows, and R2 audit rows.
- Seed receipts carry the injected execution run ID, requested fixture family, explicit/default persistence selection, and direct binding counts.
- The focused integration proof seeds store A, restarts store A without duplicate D1 rows, seeds store B, mutates A, and proves B remains unchanged.
- Webhook and queue-replay counts come from shared inventories that the real Worker runtime and property suites execute.
- Proof milestones identify their actual owner as the integration-test wrapper, use a per-execution run ID, force and restore logger enable/store/test policy, and remain redacted.
- The any-cwd regression changes cwd before dynamically importing the workflow module.
- The standalone start probe remains honest: `preflightStatus = ready`, `importCheckStatus = passed`, and `runtimeBootStatus = unproven`. Separate seed/runtime receipts do not overwrite that semantic boundary.

## Green validation receipt

- `node --import tsx infra/cloudflare/scripts/local-dev-workflow.ts` -> passed; seed status `runnable`; counts `3, 4, 4, 2, 5, 2`.
- `node --import tsx --test infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts` -> passed, `4/4`.
- `node --import tsx --test infra/cloudflare/tests/integration/worker-runtime-real.test.ts` -> passed, `10/10`.
- `npm --prefix infra/cloudflare run test:unit` -> passed, `49/49`.
- `npm --prefix infra/cloudflare run test:property` -> passed, `9/9`.
- `npm --prefix infra/cloudflare run test:integration` -> passed, `62/62` after serializing local Wrangler ownership through the shared runtime lease.
- `npm --prefix infra/cloudflare run lint` -> passed.
- Focused `lint:architecture` over all ten touched TypeScript files -> passed with non-empty scope.
- Focused Enforcer `source-shape`, `required-tests`, `no-test-doubles`, and `validation-bypass` checks over all ten touched TypeScript files -> passed.
- `git diff --check` -> passed.

## Failure-to-green note

The first full integration-family run exposed concurrent Wrangler startup contention between the WP07 seed proof and the existing real-runtime suite. A shared, stale-safe local runtime lease was added; the complete integration family then passed `62/62`. The failed run is not counted as proof.

## No-claim boundary

- This receipt does not prove production deployment, production data, or payment-plan acceptance.
- It does not upgrade the standalone import probe to a Worker boot claim.
- Account authentication remains `account-auth-adapter-manual-required`.
- WP07 remains open until its owning checklist/index accepts this source and retained proof.
