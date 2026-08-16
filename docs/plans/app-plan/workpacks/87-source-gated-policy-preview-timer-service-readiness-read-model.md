# WP87 Source-Gated Policy Preview Timer Service Readiness Read Model

This app-plan row cross-records the shared app/game WP87 proof for the native
app meaning of source-gated policy preview timer service-readiness read-model
projection.

## Scope

The parent-domain proof consumes WP86 timer service-readiness handoff rows and
records whether a native app preview row still requires service timer runtime,
scheduler persistence, durable scheduler state-store, audit trail, rollback
plan, audit/rollback read-model proof, parent-surface proof, service-readiness
proof, and service read-API proof before any future service read model runtime,
protocol command, or portal visibility can be claimed. Rows blocked by source
freshness or compiler decisions remain blocked before service-readiness
visibility.

## Evidence

- `packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-read-model.ts`
- `packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-service-readiness-read-model.test.ts`
- `scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-read-model-proof.mjs`
- `output/app-plan-proof/87-source-gated-policy-preview-timer-service-readiness-read-model/proof.json`

## No-Claim Boundaries

This proof does not add a package export, agent protocol command/event, service
runtime event, service read API implementation, portal UI, policy evaluator
runtime, timer runtime, timer scheduling, scheduler persistence runtime, durable
scheduler state-store rows, audit runtime, durable audit logs, rollback runtime,
rollback execution, child delivery, adapter dispatch, broad app blocking,
platform enforcement, or raw private source rows.

Product capability checklist remains unchanged because no product feature
status moved.
