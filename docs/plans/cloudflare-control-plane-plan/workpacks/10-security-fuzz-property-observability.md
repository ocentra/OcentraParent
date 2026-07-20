# Workpack 10: Security Fuzz Property Observability

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

- Status: `blocked / proof-required`
- Proof root: expected output path `output/cloudflare-control-plane-plan-proof/10-security-fuzz-property-observability/`
- The current WP10 reruns use the narrowed WP08 family contract:
  - security: `tests/security/no-provider-secrets-in-client.test.ts`, `cors-origin-rejection.test.ts`, `request-smuggling.test.ts`, `redaction.test.ts`
  - property: `tests/property/route-auth-state.property.test.ts`, `billing-idempotency.property.test.ts`
  - fuzz: `tests/fuzz/provider-webhook-payload.fuzz.test.ts`
  - carried observability integration: `tests/integration/worker-health.test.ts`, `pricing-public.test.ts`, `billing-status-auth.test.ts`, `webhook-signature-rejection.test.ts`, `admin-auth-rejection.test.ts`
- Historical note: `output/cloudflare-control-plane-plan-proof/10-security-fuzz-property-observability/cfcp-c1-proof.md` remains useful prior evidence for the June 17 focused hardening slice, but it is not current rerun truth for WP10.

## Families proved vs blocked

- Proved at file/assertion inventory surface: security, property, fuzz, and carried observability integration families are explicit and mapped to exact assertion IDs.
- `npm --prefix infra/cloudflare run test:integration` passed at validated source base `cbb842187`, but the retained WP12 receipt does not map that broader result to every WP10 observability assertion.
- Security, property, and fuzz families were not freshly rerun for the retained receipt. Their current execution state is unverified, not green and not test-failed.
- No file-level pass/fail claim is carried forward without a current assertion-mapped WP10 receipt.

## Exact blocker

- The WP10 proof root and cleanup-safe retained receipt are absent.
- Security, property, and fuzz commands lack current assertion-mapped results; the broader integration pass does not substitute for those families.
- `npm --prefix infra/cloudflare run lint` passed at validated source base `cbb842187`; module lint is not the WP10 blocker.
- `OBS-03` remains receipt-required because no current WP10 artifact maps the carried integration assertions, not because the currently retained evidence proves that integration cannot boot.

## Validation truth

- `npm --prefix infra/cloudflare run test:security`
  - not freshly rerun for the retained receipt; unverified
- `npm --prefix infra/cloudflare run test:property`
  - not freshly rerun for the retained receipt; unverified
- `npm --prefix infra/cloudflare run test:fuzz`
  - not freshly rerun for the retained receipt; unverified
- `npm --prefix infra/cloudflare run test:integration`
  - passed at validated source base `cbb842187`; no current assertion-mapped WP10 receipt exists
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
- Reject treating the older `cfcp-c1-proof.md` green slice as current family readiness after the proof artifact remains absent.

## Failure conditions

- Do not inherit game-only security tooling without a Parent reason.
- Do not mark WP10 runtime-complete from spec coverage alone.

## No-claim boundary

- This workpack does not prove current Cloudflare runtime readiness.
- This workpack does not prove payment handoff readiness, account authority, trusted-device authority, or portal completion.
- `OBS-03` stays proof-required until the carried integration assertions have current retained, assertion-mapped evidence.
- WP10 stays proof-required until security, property, fuzz, and integration results are current and the required proof artifact is recorded.
