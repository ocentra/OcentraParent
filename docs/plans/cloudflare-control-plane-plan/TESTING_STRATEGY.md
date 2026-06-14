# Testing Strategy

Purpose: strip the games module test surface down to what Parent actually needs first.

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
| Unit | `tests/unit/auth-boundary.test.ts` | Verify every auth state maps to the correct parent/admin/support/webhook/internal rule or blocker. |
| Unit | `tests/unit/route-manifest.test.ts` | Verify the route manifest includes every required billing, webhook, admin, and health path with explicit auth state and handler key. |
| Unit | `tests/unit/env-bindings.test.ts` | Verify env parsing and required binding names, including the server-only secret names and placeholder blockers. |
| Unit | `tests/unit/redaction.test.ts` | Verify helper redaction removes provider secrets, webhook secrets, and child-data-sensitive fields from logs or error payloads. |
| Unit | `tests/unit/request-limits.test.ts` | Verify request-size limits fail fast with the expected 413 behavior. |
| Unit | `tests/unit/kill-switch.test.ts` | Verify state-changing routes are rejected when the kill switch is enabled. |
| Integration | `tests/integration/worker-health.test.ts` | Verify `GET /health` returns the expected healthy response through the worker boundary. |
| Integration | `tests/integration/pricing-public.test.ts` | Verify the public pricing route is reachable without private billing auth. |
| Integration | `tests/integration/billing-status-auth.test.ts` | Verify billing status enforces parent-session auth and trusted-device follow-up where applicable. |
| Integration | `tests/integration/webhook-signature-rejection.test.ts` | Verify unsigned provider webhooks are rejected before processing. |
| Integration | `tests/integration/admin-auth-rejection.test.ts` | Verify admin/support routes reject callers without the required role. |
| E2E | `tests/e2e/portal-to-worker-billing-status.test.ts` | Verify the parent portal can reach billing status through the worker handoff path. |
| Contract | `tests/contract/billing-api-contract.test.ts` | Verify billing request/response shapes stay aligned with the contract docs and manifest. |
| Security | `tests/security/no-provider-secrets-in-client.test.ts` | Verify no provider secret leaks into any client-visible payload or bundle surface. |
| Security | `tests/security/cors-origin-rejection.test.ts` | Verify disallowed origins are rejected by the worker boundary. |
| Security | `tests/security/request-smuggling.test.ts` | Verify malformed request envelopes are rejected before dispatch. |
| Security | `tests/security/redaction.test.ts` | Verify security-facing payloads and logs remain redacted at the boundary level. |
| Property | `tests/property/route-auth-state.property.test.ts` | Verify route/auth mapping invariants across the manifest. |
| Property | `tests/property/billing-idempotency.property.test.ts` | Verify duplicate webhook, retry, and reconciliation inputs do not break ledger or DO idempotency. |
| Fuzz | `tests/fuzz/provider-webhook-payload.fuzz.test.ts` | Verify malformed provider webhook payloads and boundary inputs fail safely. |

## Runner contract

- `test:unit`
- `test:integration`
- `test:e2e`
- `test:contract`
- `test:security`
- `test:property`
- `test:fuzz`
- `test:all-cloudflare`

## Honesty rules

- A placeholder file or README is not proof.
- Missing runtime dependencies must be recorded as blockers in the source matrix and proof inventory.
- Payment may not treat these tests as passing until real command output exists.
