# Workpack 10: Security Fuzz Property Observability

> **2026-07-28 correction:** The later missing-private-billing-import blocker text is historical. `infra/cloudflare` now imports module-local generated billing contracts. This workpack remains open because it has no tracked proof bundle; rerun after installing dependencies and record the actual result.

## Goal

Reduce the games security and external-tool surface to the Parent-required baseline.

## First-touch surface

- `TESTING_STRATEGY.md`
- `SECURITY_PRIVACY_OBSERVABILITY.md`

## Read inputs

- [TESTING_STRATEGY.md](../TESTING_STRATEGY.md)
- [REQUIRED_TEST_ASSERTION_MATRIX.md](../REQUIRED_TEST_ASSERTION_MATRIX.md)
- [SECURITY_PRIVACY_OBSERVABILITY.md](../SECURITY_PRIVACY_OBSERVABILITY.md)

## Output files

- `infra/cloudflare/tests/security/`
- `infra/cloudflare/tests/property/`
- `infra/cloudflare/tests/fuzz/`
- [REQUIRED_TEST_ASSERTION_MATRIX.md](../REQUIRED_TEST_ASSERTION_MATRIX.md)
- [SECURITY_PRIVACY_OBSERVABILITY.md](../SECURITY_PRIVACY_OBSERVABILITY.md)
- `output/cloudflare-control-plane-plan-proof/10-security-fuzz-property-observability/`

## Acceptance

- Security, property, and fuzz families are explicit.
- Every required security, property, fuzz, and carried observability assertion
  ID is explicit.
- Observability remains redacted and support-safe.
- Load, mutation, and static-analysis gates are clearly marked as later rollout gates when not first-slice prerequisites.

## Proof IDs

- `cloudflare-control.no-secrets-in-repo`
- `cloudflare-control.property-fuzz-boundary`

## Current execution truth

- Status: `blocked / proof-present`
- Proof root: `output/cloudflare-control-plane-plan-proof/10-security-fuzz-property-observability/`
- The current WP10 reruns use the narrowed WP08 family contract:
  - security: `tests/security/no-provider-secrets-in-client.test.ts`, `cors-origin-rejection.test.ts`, `request-smuggling.test.ts`, `redaction.test.ts`
  - property: `tests/property/route-auth-state.property.test.ts`, `billing-idempotency.property.test.ts`
  - fuzz: `tests/fuzz/provider-webhook-payload.fuzz.test.ts`
  - carried observability integration: `tests/integration/worker-health.test.ts`, `pricing-public.test.ts`, `billing-status-auth.test.ts`, `webhook-signature-rejection.test.ts`, `admin-auth-rejection.test.ts`
- Historical note: `output/cloudflare-control-plane-plan-proof/10-security-fuzz-property-observability/cfcp-c1-proof.md` remains useful prior evidence for the June 17 focused hardening slice, but it is not current rerun truth for WP10.

## Families proved vs blocked

- Proved at file/assertion inventory surface: security, property, fuzz, and carried observability integration families are explicit and mapped to exact assertion IDs.
- Runtime-blocked today: security, property, fuzz, and carried observability integration.
- Partial file-level passes inside blocked family runs:
  - security: `redaction`
  - property: `route-auth-state`
- Blocked file-level execution remains explicit:
  - security: `no-provider-secrets-in-client`, `cors-origin-rejection`, `request-smuggling`
  - property: `billing-idempotency`
  - fuzz: `provider-webhook-payload`
  - carried observability integration: `worker-health`, `pricing-public`, `billing-status-auth`, `webhook-signature-rejection`, `admin-auth-rejection`

## Exact blocker

- Current prerequisite for every WP10 module command: WP01 must restore the empty dependency tree shown by `npm --prefix infra/cloudflare ls wrangler @cloudflare/workers-types`.
- `infra/cloudflare/src/index.ts` imports module-local generated billing contracts; after the resolver graph is clean, record the exact security/property/fuzz failure if one remains.

## Validation truth

- `npm --prefix infra/cloudflare run test:security`
  - blocked
- `npm --prefix infra/cloudflare run test:property`
  - blocked
- `npm --prefix infra/cloudflare run test:fuzz`
  - blocked
- `npm --prefix infra/cloudflare run test:integration`
  - blocked
- No architecture gate was applicable in this packet because no WP10 TypeScript source or test file changed; this closure pass records current family truth through proof/docs only.

## Validation

- `npm --prefix infra/cloudflare run test:security`
- `npm --prefix infra/cloudflare run test:property`
- `npm --prefix infra/cloudflare run test:fuzz`
- `npm --prefix infra/cloudflare run test:integration`

## Negative cases

- Reject leaking provider secrets or child data.
- Reject security claims without explicit test family ownership.
- Reject observability claims that are not tied to a concrete redaction,
  auth-rejection, billing-status, or portal-smoke case.
- Reject treating the older `cfcp-c1-proof.md` green slice as current family readiness after the billing-domain boundary drift.

## Failure conditions

- Do not inherit game-only security tooling without a Parent reason.
- Do not mark WP10 runtime-complete from spec coverage alone.

## No-claim boundary

- This workpack does not prove current Cloudflare runtime readiness.
- This workpack does not prove payment handoff readiness, account authority, trusted-device authority, or portal completion.
- `OBS-03` stays blocked under current reruns because the required integration family does not boot.
- WP10 stays open/blocked until the missing billing-domain boundary import is restored and the required family commands rerun green.
