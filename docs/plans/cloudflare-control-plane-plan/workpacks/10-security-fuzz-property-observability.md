# Workpack 10: Security Fuzz Property Observability

## Goal

Reduce the games security and external-tool surface to the Parent-required baseline.

## First-touch surface

- `TESTING_STRATEGY.md`
- `SECURITY_PRIVACY_OBSERVABILITY.md`

## Read inputs

- [TESTING_STRATEGY.md](../TESTING_STRATEGY.md)
- [SECURITY_PRIVACY_OBSERVABILITY.md](../SECURITY_PRIVACY_OBSERVABILITY.md)

## Output files

- `infra/cloudflare/tests/security/`
- `infra/cloudflare/tests/property/`
- `infra/cloudflare/tests/fuzz/`
- [SECURITY_PRIVACY_OBSERVABILITY.md](../SECURITY_PRIVACY_OBSERVABILITY.md)
- `docs/proof/cloudflare-control-plane-plan/wp10-security-fuzz-property-observability/`

## Acceptance

- Security, property, and fuzz families are explicit.
- Observability remains redacted and support-safe.
- Load, mutation, and static-analysis gates are clearly marked as later rollout gates when not first-slice prerequisites.

## Proof IDs

- `cloudflare-control.no-secrets-in-repo`
- `cloudflare-control.property-fuzz-boundary`

## Validation

- Docs or scaffold validation: `npm run format:check`

## Negative cases

- Reject leaking provider secrets or child data.
- Reject security claims without explicit test family ownership.

## Failure conditions

- Do not inherit game-only security tooling without a Parent reason.
