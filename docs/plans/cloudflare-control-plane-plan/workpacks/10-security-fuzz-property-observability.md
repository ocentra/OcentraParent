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

## Failure conditions

- Do not inherit game-only security tooling without a Parent reason.
- Do not mark WP10 runtime-complete from spec coverage alone.
