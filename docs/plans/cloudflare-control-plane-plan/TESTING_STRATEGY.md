# Testing Strategy

Purpose: strip the games module test surface down to what Parent actually needs
first.

Exact file-level assertions now live in
[`REQUIRED_TEST_ASSERTION_MATRIX.md`](REQUIRED_TEST_ASSERTION_MATRIX.md). This
document owns the family structure and the non-negotiable test-file inventory;
the assertion matrix owns the exact cases future execution must implement.

## Required first-class test families

| Family | Required now? | Why |
| --- | --- | --- |
| Unit | yes | env parsing, route manifest, auth states, request limits, kill switch, helper redaction |
| Integration | yes | health, pricing public route, auth rejection, webhook rejection, admin rejection |
| E2E | yes | parent portal to worker smoke for billing status |
| Contract | yes | billing route request/response contract expectations |
| Security | yes | no-provider-secrets-in-client, CORS rejection, request smuggling, response redaction |
| Property | yes | route/auth invariants and billing idempotency expectations |
| Fuzz | yes | provider webhook payloads and malformed request envelopes |
| Load / k6 | recommended later | rollout gate, not first scaffold gate |
| Mutation | recommended later | prove test strength before production promotion |
| Static analysis wrapper | recommended later | production hardening gate |

## Exact required test inventory

| Suite | Required test file | Required behavior |
| --- | --- | --- |
| Unit | `tests/unit/auth-boundary.test.ts` | Verify every auth state maps to the correct parent/admin/support/webhook/internal rule or blocker. Assertion IDs: `UT-AUTH-*`. |
| Unit | `tests/unit/route-manifest.test.ts` | Verify the route manifest includes every required billing, webhook, admin, and health path with explicit auth state and handler key. Assertion IDs: `UT-ROUTE-*`. |
| Unit | `tests/unit/env-bindings.test.ts` | Verify env parsing and required binding names, including the server-only secret names and placeholder blockers. Assertion IDs: `UT-ENV-*`. |
| Unit | `tests/unit/redaction.test.ts` | Verify helper redaction removes provider secrets, webhook secrets, child-data-sensitive fields, recovery markers, and device-secret markers from logs or error payloads. Assertion IDs: `UT-REDACT-*`. |
| Unit | `tests/unit/request-limits.test.ts` | Verify request-size limits fail fast with the expected `413` behavior and redacted failure path. Assertion IDs: `UT-LIMIT-*`. |
| Unit | `tests/unit/kill-switch.test.ts` | Verify state-changing routes and webhook processing are rejected when the kill switch is enabled. Assertion IDs: `UT-KILL-*`. |
| Integration | `tests/integration/worker-health.test.ts` | Verify `GET /health` returns the expected healthy response through the worker boundary without secret leakage. Assertion IDs: `IT-HEALTH-*`. |
| Integration | `tests/integration/pricing-public.test.ts` | Verify the public pricing route is reachable without private billing auth and remains client-safe. Assertion IDs: `IT-PRICE-*`. |
| Integration | `tests/integration/billing-status-auth.test.ts` | Verify billing status enforces parent-session auth and trusted-device follow-up where applicable. Assertion IDs: `IT-STATUS-*`. |
| Integration | `tests/integration/webhook-signature-rejection.test.ts` | Verify unsigned, invalid, malformed, or wrong-provider webhooks are rejected before processing. Assertion IDs: `IT-WEBHOOK-*`. |
| Integration | `tests/integration/admin-auth-rejection.test.ts` | Verify admin/support routes reject callers without the required role and remain audit-safe. Assertion IDs: `IT-ADMIN-*`. |
| E2E | `tests/e2e/portal-to-worker-billing-status.test.ts` | Verify the first consumer smoke reaches only billing status through the worker handoff path and returns a controlled parent-visible state. Assertion IDs: `E2E-PORTAL-*`. |
| Contract | `tests/contract/billing-api-contract.test.ts` | Verify billing request/response shapes stay aligned with the route manifest and do not leak provider or child-data boundaries. Assertion IDs: `CT-CONTRACT-*`. |
| Security | `tests/security/no-provider-secrets-in-client.test.ts` | Verify no provider secret leaks into any client-visible payload or bundle surface. Assertion IDs: `SEC-SECRETS-*`. |
| Security | `tests/security/cors-origin-rejection.test.ts` | Verify disallowed origins are rejected by the worker boundary and wildcard misconfigurations fail closed. Assertion IDs: `SEC-CORS-*`. |
| Security | `tests/security/request-smuggling.test.ts` | Verify malformed request envelopes are rejected before dispatch. Assertion IDs: `SEC-SMUGGLE-*`. |
| Security | `tests/security/redaction.test.ts` | Verify security-facing payloads and logs remain redacted at the boundary level. Assertion IDs: `SEC-REDACT-*`. |
| Property | `tests/property/route-auth-state.property.test.ts` | Verify route/auth mapping invariants across the manifest. Assertion IDs: `PROP-ROUTE-*`. |
| Property | `tests/property/billing-idempotency.property.test.ts` | Verify duplicate webhook, retry, and reconciliation inputs do not break ledger or DO idempotency. Assertion IDs: `PROP-IDEMP-*`. |
| Fuzz | `tests/fuzz/provider-webhook-payload.fuzz.test.ts` | Verify malformed provider webhook payloads and boundary inputs fail safely. Assertion IDs: `FUZZ-WEBHOOK-*`. |

## Runner contract

- `test:unit`
- `test:integration`
- `test:e2e`
- `test:contract`
- `test:security`
- `test:property`
- `test:fuzz`
- `test:all-cloudflare`
- `infra/cloudflare/scripts/test-runner.ts` is the WP08 source of truth for family-to-file routing: it executes only the required files for the selected family, emits proof IDs plus assertion IDs with `--list`, and reports extra same-directory tests as `unexpectedFilesByFamily` instead of silently widening WP08.

## Execution rules

- The next execution agent must implement every test file listed above and every
  assertion ID listed in `REQUIRED_TEST_ASSERTION_MATRIX.md`.
- Proof artifacts for WP08 and WP10 must list the assertion IDs executed, the
  exact command used, and any blocked IDs that remain manual-required.
- Extra Cloudflare tests in the same family directory stay outside the WP08
  required runner contract until this file and
  `REQUIRED_TEST_ASSERTION_MATRIX.md` add them explicitly.
- Observability proof is carried by the redaction, auth-rejection, billing
  status, and portal smoke cases; it is not a free-floating claim.

## Honesty rules

- A placeholder file or README is not proof.
- Missing runtime dependencies must be recorded as blockers in the source matrix and proof inventory.
- Payment may not treat these tests as passing until real command output exists.
- A complete assertion matrix does not unblock payment or imply runnable
  coverage; it only means the engineering spec is execution-ready.
