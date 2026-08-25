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
# Module build/type/test scope
npm --prefix infra/cloudflare run build
npm --prefix infra/cloudflare run type-check
npm --prefix infra/cloudflare run test
npm --prefix infra/cloudflare run test:unit
npm --prefix infra/cloudflare run test:integration
npm --prefix infra/cloudflare run test:e2e
npm --prefix infra/cloudflare run test:contract
npm --prefix infra/cloudflare run test:security
npm --prefix infra/cloudflare run test:property
npm --prefix infra/cloudflare run test:fuzz

# Deployment scope only when the selected workpack touches deployment/promotion
npm --prefix infra/cloudflare run deploy:dev
npm --prefix infra/cloudflare run deploy

# Architecture scope: start with touched files; expand only when the workpack requires it
npm run lint:architecture -- --files infra/cloudflare docs/plans/cloudflare-control-plane-plan
```

Run through `npm run agent:run --` when collecting proof if the logging/evidence wrapper is available.

## Command ownership notes

- `infra/cloudflare` owns the Worker module, env guards, route manifest, auth adapter boundary, storage/queue binding model, local dev/seeding, test runner, and deployment proof.
- Domain packages own route request/response semantics. Cloudflare may consume public domain contract exports; private source imports are migration-sensitive compatibility debt unless explicitly public.
- `billing-domain` and `payment-subscription-plan` own billing product math, provider semantics, invoice/grace/referral qualification, and payment runtime readiness.
- `account-identity-family-plan` and `device-trust-bootstrap-plan` own account/session/admin/support and trusted-device authority.
- `portal-ux-household-surfaces-plan` proves consumer UI only.
- `data-custody-storage-plan` owns retention/export/deletion policy; this plan only proves support-safe Cloudflare storage boundaries when selected.

## Cloudflare E2E meaning

Do not use one proof family to claim the whole Cloudflare path. For this plan, E2E has separate meanings:

```text
module scaffold E2E: infra/cloudflare package/module tree -> source surface matrix -> scaffold/no-claim labels.
env/binding E2E: wrangler dev/prod config -> binding/resource refs -> environment separation proof.
worker guard E2E: request -> env/origin/size/kill-switch guard -> safe error/redacted log or dispatch.
route manifest E2E: route path/method/auth state -> domain-owned request/response model -> audit/proof id family.
auth boundary E2E: route auth state -> account/device/provider/internal adapter result -> allow/deny with negative cases.
storage/queue E2E: request or scheduled job -> DO/D1/KV/R2/Queue ownership path -> idempotency/retry/dead-letter/read-model proof.
local dev/seed E2E: local start -> seed fixtures -> teardown/reset proof.
test pyramid E2E: assertion matrix -> focused test family -> command output or exact blocker.
portal smoke E2E: portal consumer request -> Worker route -> redaction-safe response/status proof.
security/property/fuzz E2E: negative input/property/fuzz case -> safe rejection/redacted diagnostics -> no private-data leak.
deployment/promotion E2E: dev/prod env refs -> deploy/promotion/rollback proof -> no-claim boundaries.
payment handoff E2E: accepted Cloudflare proof roots + carried blockers -> WP12 handoff artifact -> downstream payment-plan acknowledgment.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Structured harness logging expectations

Every Cloudflare proof slice must preserve product-safe logging and local harness logging.

Product/runtime-safe logging:

```text
redact sensitive auth material, provider verification material, private billing fields, support-private notes, environment-only values, raw provider bodies unless explicitly allowed, and child/private telemetry
log route key, auth state, environment, binding family, queue/dead-letter state, idempotency state, redaction state, request id, proof id family, deployment state, and consumer handoff state when safe
separate local-dev, production, route-manifest, auth, storage, queue, portal-smoke, deployment, and payment-handoff states
never treat logs, source presence, route presence, or placeholder config as proof without command output or exact blocker
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, owner module, environment, route/auth state when relevant, exit code, result, artifact pointer, diagnostics summary, rollback/teardown note, dependency blocker note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## Expected coverage by workpack

| Workpack | Expected proof focus |
| --- | --- |
| WP00 | games keep/adapt/strip map, game-only concern rejection, parent-safe module boundary |
| WP01 | module tree, package scripts, scaffold-only/no-claim labels, no consumer semantics, and retained clean `wrangler`/`@cloudflare/workers-types` resolver graph before WP07 can run |
| WP02 | wrangler envs, D1/DO/KV/R2/Queue binding names, environment custody, dev/prod separation |
| WP03 | worker entrypoint, env validation, request-size guard, origin/CORS behavior, kill-switch, scheduled hook shape |
| WP04 | route manifest, route groups, domain contract ownership, no ad hoc route strings |
| WP05 | auth/admin/support/webhook states, adapter boundary, provider blockers |
| WP06 | DO/D1/KV/R2/Queue ownership plus Account D1 authoritative create/update/revoke/currentness/CAS, verified-provider-to-sealed-authority caller, idempotency, restart, and stale-generation rejection |
| WP07 | local dev, seed, fixture, teardown, emulator/miniflare/wrangler blockers |
| WP08 | test runner, exact assertion matrix, unit/integration/security/property/e2e family mapping |
| WP09 | portal-to-worker smoke, redacted request/response proof, no child private payloads |
| WP10 | security/property/fuzz/observability baseline with parent-only scope |
| WP11 | deploy/promotion/rollback/env separation proof |
| WP12 | payment handoff assumptions, blockers, no-claim boundaries, downstream acknowledgment |

## Account authority storage handoff

The Account WP08 producer transport is mapped at canonical source `c5ed3ce5c`,
and Account WP09's durable issuer/key-registry/outbox core is integrated at
`4f6245e51`. Cloudflare's bounded duplicate-key-rejecting decoder and private
one-shot WP08 inner-wire verifier are accepted at `da84e6ee3`. Live caller
tracing nevertheless finds no Account protected signer, binding/delivery
adapter, production caller, authenticated current-key handoff, or Cloudflare
issuer consumer/runtime. Expected Cloudflare WP06 test roots remain unwritten:

```text
infra/cloudflare/tests/unit/account-identity-authority-producer-transport.test.ts
infra/cloudflare/tests/unit/account-identity-authority-issuer-transport.test.ts
infra/cloudflare/tests/unit/account-identity-authority-issuer-key-registry.test.ts
infra/cloudflare/tests/integration/account-identity-authority-issuer-runtime.test.ts
infra/cloudflare/tests/unit/account-identity-authority-caller.test.ts
infra/cloudflare/tests/unit/account-identity-authority-runtime.test.ts
infra/cloudflare/tests/integration/account-identity-authority-currentness.test.ts
infra/cloudflare/tests/integration/account-identity-authority-restart-cas.test.ts
infra/cloudflare/tests/integration/account-identity-d1-migration.test.ts
```

The private transport verifier and JSON decoder are production-source roots,
not test substitutes. The nine expected test roots must cover the Account
service-binding/delivery adapter, durable public-key registry,
key-id hash checking and rotation/revocation, bounded canonical wire parsing,
signature and timestamp checks, verified-provider subject binding without
Firebase authority substitution, D1 currentness/revocation/CAS and
stale-generation/restart/concurrency negatives, and the absence of any public
caller-scalar mutation route. The registry must remain Account-owned and
durable; a missing/untrusted registry or binding is manual-required, never a
fallback to an env key, Firebase key, request key, fixture, or caller row. No
test or proof claim is made by this source acceptance.

WP06 consumes, but does not define, Account WP08's Rust-owned contract or
Account WP02's target-aware action authority. Its proof must name
`infra/cloudflare/wrangler.toml`, `src/env.ts`, the selected account-identity
D1 declarations, binding-specific account migration directory, read store,
authoritative writer/currentness/revocation/CAS owner, provider caller, and
`tests/integration/account-identity-d1-migration.test.ts`; retain the migration
result from `cd infra/cloudflare && npm exec -c "wrangler d1 migrations apply <account-identity-d1-database> --local"`, the module integration result from
`npm --prefix infra/cloudflare run test:integration`, and the focused
architecture result from `npm run lint:architecture -- --files infra/cloudflare/src/env.ts infra/cloudflare/src/storage/account-identity-store.ts infra/cloudflare/tests/integration/account-identity-d1-migration.test.ts`.
Current source declares the isolated Account binding and ordered migrations,
implements the Account-owned writer/currentness/revocation/CAS boundary, and
mounts verified-provider current-authority resolution through the verifier.
The mutation producer is not mounted in the Worker entrypoint, the migration
has not been applied, and the full expected-test source is absent. The command
remains deferred and must never be run against `BILLING_D1` as a substitute.
WP06 also retains the direct focused command
`cd infra/cloudflare && npm exec -c "node --import tsx --test tests/integration/account-identity-d1-migration.test.ts"`
in `03-account-identity-d1-migration-test.md`; the aggregate integration script
cannot substitute for or silently omit that required migration/adapter result.

Cloudflare WP08 follows WP06. It maps the selected account-identity integration
assertion and its module-runner result to the Cloudflare proof root and the
Account WP06 aggregation handoff. Missing runtime composition, expected-test
source, command output, or proof is recorded as an exact blocker and keeps
Cloudflare WP06/WP08 and Account WP06 blocked; neither packet may substitute a
test double or claim account authority.
The runner consumes the module-local generated billing-contract route, not
`packages/billing-domain/src/*`. Its current preflight
`npm --prefix infra/cloudflare ls wrangler @cloudflare/workers-types` is empty,
so WP01's dependency-resolution result and WP06's retained proof must exist
before WP08 runs or reports the selected integration family.

## Required negative states

```text
game-only code not copied
placeholder route not runtime proof
source presence is not runtime proof
missing binding fails clearly
private/admin/support route lacks owner proof
provider/webhook assumption blocked until provider proof
account/session and trusted-device authority remain dependency-gated until owning-plan proof exists
verified provider subject cannot substitute for target-aware sealed Account authority
read-adapter success cannot substitute for authoritative write/update/revoke/CAS currentness
stale generation, revoked mapping, provider/account mismatch, restart, and concurrent CAS fail closed
D1/KV/R2/Queue claim has clear owner and purpose
local dev proof is not production deployment proof
payment remains blocked until WP12 handoff proof exists and is consumed
production deployment claim requires WP11 proof
```

## Proof storage

Proof artifacts live under:

```text
output/cloudflare-control-plane-plan-proof/<workpack-id>/
```

Do not write new proof artifacts under `docs/proof/cloudflare-control-plane-plan/` unless preserving old references; new work should use `output/` proof roots.
