# Next Actions

## WP04 source integration checkpoint — 2026-08-25

WP04 route/model source is integrated through the current canonical tree. The
next bounded action is to repair the stale route-manifest tests against the
current model, restore the missing Cloudflare dependency tree, and then run the
complete unit/property/contract/integration set together. No proof, deployment,
runtime-dispatch, READY, or DONE claim is permitted from the source packet.

## Current slice

- Current slice: `Complete Account WP09 producer adapters, then Cloudflare WP06 issuer consumer/runtime`. Independent review accepts the Firebase verifier, D1 read/current-authority path, authoritative Account writer, bounded provider caller, duplicate-key-rejecting wire decoder, and private one-shot WP08 inner-wire verifier. Account WP09's durable issuer/key-registry/outbox core is integrated at `4f6245e51`, but live caller tracing found no protected signer, binding authenticator, delivery owner, production caller, or authenticated current-key handoff. Cloudflare has no issuer transport, current-key registry consumer, or runtime mount. No create/CAS/revoke method accepts caller-supplied authority values. Finish those six production roots before writing the complete nine-test migration/currentness/restart/replay packet. Migration execution, test runs, proof, deployment, and runtime-ready claims remain deferred. Normal WP06 remains blocked and is not `DONE`. #604 stays closed without merge.
- Current owner: `cloudflare-control-plane-plan`
- Current status: `in_progress`
- Final-tree scoped validation (2026-08-04): PR #608 merged to `main` as `5af4a1a92` after fresh full CI passed its product, security, and platform jobs. The merged local commands `npm --prefix infra/cloudflare run test:local-dev-workflow` (12 focused tests), `npm --prefix infra/cloudflare run lint`, `npm --prefix infra/cloudflare run proof:local-dev`, and `npm run lint:architecture -- --files infra/cloudflare/scripts/local-dev-proof.ts infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts` pass. The proof command retains its result only through the canonical redacted NDJSON artifact under `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/runs/<generated-run-id>`; it does not retain a raw stdout summary. This is local WP07 validation evidence only, not workpack closure. The successor is proof-only from current source after the real Wrangler/Workers-types dependency-resolution gap is cleared; it must not reuse or rebase PR #604.

## Ordered queue

| Order | Slice | Status | First-touch surface | Next action | Exit gate |
| --- | --- | --- | --- | --- | --- |
| 01 | WP01 module scaffold | validation / bounded scaffold source accepted | `infra/cloudflare/package.json`, Wrangler configs, Worker entrypoint, env and route surfaces | Preserve the reviewed scaffold boundary. Do not reopen source unless WP06 exposes a concrete missing seam. Tests/proof remain a later phase. | Implementation-phase input only; no runtime, deployment, provider, or DONE claim. |
| 02 | WP02 Wrangler env bindings | code-and-test-source complete / execution and proof deferred | `infra/cloudflare/src/env.ts`, Wrangler configs, `.dev.vars.example`, `tests/unit/env-bindings.test.ts` | Preserve accepted non-throwing production-origin validation and the complete real malformed-origin/wildcard/environment matrix at `4ddb47353`; run it only in the later execution phase. | Test results, retained proof, real binding provisioning, deployment/rollback, and `EntitlementSnapshotDO` composition remain open. |
| 03 | WP03 Worker entrypoint runtime guards | code-and-test-source complete / execution and proof deferred | `infra/cloudflare/src/index.ts`, `src/env.ts`, `src/security/redaction.ts`, and seven mapped real test roots | Preserve the reviewed fail-fast environment, CORS-before-OPTIONS, framing/size, route, kill-switch, auth-boundary, safe-error, scheduled reconciliation, and redaction source at `61c98efa8`. Run its complete mapped test packet only in the later execution phase. | Test results, scheduled-runtime behavior, retained proof, deployment, consumer runtime, CI, READY, and DONE remain open. |
| 05 | WP05 Firebase/provider webhook boundary | implementation-phase review accepted / proof deferred | `infra/cloudflare/src/providers/firebase-auth.ts`, `src/env.ts`, `src/auth/verifier.ts`, `src/auth/provider-webhook.ts`, `src/index.ts`, Wrangler config and local env example | Keep Firebase subject verification and provider webhook verification separate. Stripe may use the real timestamped HMAC path; non-Stripe providers remain explicit manual-required/unavailable. Return provider identity or webhook verification only; never create caller-selected family/device/admin/support authority. | Source-only packet; tests, proof, deployment secrets, runtime reachability, binding context, and DONE remain open. |
| 06 | WP06 authoritative Account D1 producer and provider caller | fail-closed Account core and private inner-wire verifier accepted; six production roots and nine tests open | Account WP09 protected signer/delivery/runtime; `src/auth/account-identity-authority-issuer-{transport,key-registry,runtime}.ts`; existing decoder/caller/runtime and D1 store/writer | Complete the three Account and three Cloudflare producer/consumer runtime roots as one authenticated handoff. Then write all nine expected migration, CAS, revoke, restart, currentness, key-rotation, and reachability tests together. | Verified current-authority resolution is reachable and mutation readiness remains manual-required. No caller-scalar mutation API or runtime mutation claim is allowed before the complete Account-to-Cloudflare binding. Migration execution, tests, proof, deployment, and DONE remain later gates. |
| 08 | WP08 account-identity runner/integration proof | blocked on Cloudflare WP06 and the WP01 module dependency environment | `infra/cloudflare/scripts/test-runner.ts`, `src/generated/billing-contracts.ts`, and selected integration surface | After WP06 proof and a non-empty `npm --prefix infra/cloudflare ls wrangler @cloudflare/workers-types` result, use the module script `npm --prefix infra/cloudflare run test:integration` to retain the storage-facing runner/proof result or a new exact blocker. Do not revive obsolete `packages/billing-domain/src/*` imports. | Cloudflare WP08 maps the WP06 storage assertions and command result to retained proof for Account WP06; no account-contract ownership or runtime-ready claim. |
| 07 | WP07 local dev/seed proof-only successor | proof phase deferred | current `infra/cloudflare` WP07 source and proof root | Run only after the current source-completion chain reaches its test/proof phase. Do not revive PR #604 or the stale private billing-domain import blocker. | The selected workpack has reproducible focused results, retained positive/negative/teardown evidence, and an explicit no-claim boundary. |
| 00-12 except WP01/WP02/WP06/WP07/WP08 | Selected Cloudflare workpack | source-present / retained-proof-absent | selected workpack's first-touch surface | Reconcile its actual production source first, then write the complete expected-test source before focused execution and proof. | The selected workpack has real bounded source, complete expected tests, reproducible focused results, and an explicit no-claim boundary. |

## Working rules

- Move exactly one row to `in_progress` when implementation starts.
- Do not start payment runtime slices while row 12 still lacks proof.
- Keep scaffold placeholders honest: `exists` is not the same as `validated`.
- Do not invent, shrink, or merge away test scope outside
  `REQUIRED_TEST_ASSERTION_MATRIX.md`.
- Keep `PLAN_EXECUTION_SCORECARD.md` aligned with real module and proof state.

## 2026-08-18 source-map refresh

The graph now maps the provider-webhook verifier separately from Firebase
account authentication. Stripe verification is source-present; other
providers and admin/support authority remain manual-required. No test, proof,
deployment, runtime-readiness, or DONE claim changed.

## 2026-08-19 WP06 producer-consumer routing

Account WP08's Rust producer transport is now source-present, but its issuance
is crate-private and typed unavailable without Account signer/key custody and
an authenticated producer adapter. WP06 remains a safe verified-provider
read/manual boundary. The next legal Cloudflare packet is the bounded private
service-binding adapter/verifier at the existing Account caller/runtime seam,
using an Account-owned durable public-key registry, followed by a same-
transaction D1 currentness/revocation/CAS recheck before any writer mutation
is mounted. Missing/untrusted key distribution remains manual-required; do not
invent a public route, Firebase authority, env/request/fixture key, request-
selected scalar, or arbitrary module. Expected subject, signature/time,
currentness, migration, reachability, restart, key rotation, replay, and
concurrency tests remain open.

The private decoder/inner-wire verifier source is accepted at canonical
`da84e6ee3`. Do not rewrite it into a public route or caller-provided key seam.
Account WP09's durable core is now integrated, but its protected signer,
authenticated delivery/binding adapter, and production caller remain absent;
Cloudflare's issuer transport, current-key consumer, and runtime mount are also
absent. Complete those production roots first, then write all nine expected
WP06 tests together; do not run piecemeal CI during the source wave.
