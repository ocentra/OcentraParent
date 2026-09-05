# Workpack 03: Worker Entrypoint Runtime Guards

> **2026-07-28 correction:** The later missing-private-billing-import blocker text is historical. `infra/cloudflare` now imports module-local generated billing contracts. This workpack remains open because it has no tracked proof bundle; rerun after installing dependencies and record the actual result.

## Goal

Freeze the shared guard chain for env validation, CORS, request-size limits, kill switch, safe errors, and scheduled hooks.

## First-touch surface

- `infra/cloudflare/src/index.ts`

## Read inputs

- [SECURITY_PRIVACY_OBSERVABILITY.md](../SECURITY_PRIVACY_OBSERVABILITY.md)
- [DEPLOYMENT_MODEL.md](../DEPLOYMENT_MODEL.md)

## Output files

- `infra/cloudflare/src/index.ts`
- `output/cloudflare-control-plane-plan-proof/03-worker-entrypoint-runtime-guards/`

## Acceptance

- The fetch path fails safe before dispatch.
- Scheduled reconciliation hook shape is explicit.
- Logging is redacted by default.

## Status

- `code-and-test-source complete / execution-and-proof open`
- Reviewed canonical source/test commit: `61c98efa8`
- Expected proof root remains absent:
  `output/cloudflare-control-plane-plan-proof/03-worker-entrypoint-runtime-guards/`

## Execution truth

- The shared worker entrypoint owns env validation, origin rejection before a
  successful OPTIONS response, request framing and size limits, kill-switch
  behavior, route/auth-boundary rejection, safe handler errors, and scheduled
  reconciliation failure logging.
- The structured guard logs are redacted, and the redaction owner covers the
  planned provider credential identifiers and key references.
- The graph maps three implementation roots and seven real test roots. The
  current packet adds the missing-environment, disallowed/allowlisted
  preflight, and provider-identifier redaction cases without test doubles.
- No WP03 test was executed in this code-first phase. The scheduled hook shape
  is explicit in source but is not runtime-proven.

## Exact blocker set

- Current next gate: run the complete mapped WP03 unit/security/integration
  packet during the repository execution phase, then retain the exact positive,
  negative, scheduled-runtime, and teardown result. Deployment and consumer
  runtime remain separate later gates.

## Deferred validations

- `npm --prefix infra/cloudflare run test:unit`
- `npm --prefix infra/cloudflare run test:integration`
- `npm run lint:architecture -- --files infra/cloudflare`

These commands describe the later execution gate; they were not run for
canonical `61c98efa8`.

## No-claim boundary

- This workpack does not prove a green worker boot.
- This workpack does not prove live runtime guard success through dispatch.
- This workpack does not prove deployment, portal smoke, or payment readiness.

## Failure conditions

- Reject dispatch before guard checks.
- Reject error bodies that imply raw secret or stack leakage.
- Do not treat a fail-safe stub as a proven runtime implementation.
