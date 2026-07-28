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
- `npm --prefix infra/cloudflare run test:integration` passed at source base `2aab6310c`; the retained WP10 receipt records its local observability boundary.
- Security, property, and fuzz families were freshly rerun at source base `2aab6310c` and passed. Their passing local results do not prove production security, deployment, or payment readiness.
- The tracked WP10 receipt maps current command results, while acceptance and the required operational proof remain open.

## Exact blocker

- The raw WP10 proof root is absent and the new tracked receipt is not an accepted completion root.
- Current local command results do not substitute for production security, real provider traffic, or deployed observability evidence.
- `OBS-03` remains proof-required for accepted observability evidence, not because the current local integration command failed to boot.

## Validation truth

- `npm --prefix infra/cloudflare run test:security`
  - passed 16/16 at source base `2aab6310c`; local security behavior only
- `npm --prefix infra/cloudflare run test:property`
  - passed 9/9 at source base `2aab6310c`; local property behavior only
- `npm --prefix infra/cloudflare run test:fuzz`
  - passed 5/5 at source base `2aab6310c`; local fuzz behavior only
- `npm --prefix infra/cloudflare run test:integration`
  - passed at source base `2aab6310c`; local Worker behavior only
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
