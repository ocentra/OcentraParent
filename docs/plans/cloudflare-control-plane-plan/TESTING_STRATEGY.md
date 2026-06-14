# Testing Strategy

Purpose: strip the games module test surface down to what Parent actually needs first.

## Required first-class test families

| Family | Required now? | Why |
| --- | --- | --- |
| Unit | yes | env parsing, route manifest, auth states, request limits, kill switch, redaction |
| Integration | yes | health, pricing public route, auth rejection, webhook rejection, admin rejection |
| E2E | yes | parent portal to worker smoke for billing status |
| Contract | yes | billing route request/response contract expectations |
| Security | yes | no-provider-secrets-in-client, CORS rejection, request smuggling, redaction |
| Property | yes | route/auth invariants and idempotency expectations |
| Fuzz | yes | provider webhook payloads and malformed request envelopes |
| Load / k6 | recommended later | rollout gate, not first scaffold gate |
| Mutation | recommended later | prove test strength before production promotion |
| Static analysis wrapper | recommended later | production hardening gate |

## Required placeholder test files

```text
tests/unit/auth-boundary.test.ts
tests/unit/route-manifest.test.ts
tests/unit/env-bindings.test.ts
tests/unit/redaction.test.ts
tests/unit/request-limits.test.ts
tests/unit/kill-switch.test.ts
tests/integration/worker-health.test.ts
tests/integration/pricing-public.test.ts
tests/integration/billing-status-auth.test.ts
tests/integration/webhook-signature-rejection.test.ts
tests/integration/admin-auth-rejection.test.ts
tests/e2e/portal-to-worker-billing-status.test.ts
tests/contract/billing-api-contract.test.ts
tests/security/no-provider-secrets-in-client.test.ts
tests/security/cors-origin-rejection.test.ts
tests/security/request-smuggling.test.ts
tests/property/route-auth-state.property.test.ts
tests/fuzz/provider-webhook-payload.fuzz.test.ts
```

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
