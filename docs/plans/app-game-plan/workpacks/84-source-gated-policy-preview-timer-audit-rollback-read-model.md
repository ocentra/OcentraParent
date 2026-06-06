# WP84 Source-Gated Policy Preview Timer Audit Rollback Read Model

## AI Worker Checklist

- [x] Confirm source docs read: `docs/feature-list.md`,
      `docs/features/app-game-control.md`,
      `docs/expectations/app-game-evidence.md`,
      `docs/expectations/policy.md`, and `packages/parent-domain/README.md`.
- [x] Confirm browser-game scope remains in browser-plan.
- [x] Confirm apps and games share low-level evidence but keep separate product
      meaning.
- [x] Hub lock covers this workpack and exact implementation/docs paths.
- [x] Existing source layout inspected; no parallel app-control or game-control
      truth created.
- [x] Before-state source snapshot recorded in
      `output/app-game-plan-proof/84-source-gated-policy-preview-timer-audit-rollback-read-model/00-source-snapshot.md`.
- [x] Contracts updated first where behavior changes.
- [x] Rust/service/portal parity not updated because this is a parent-domain
      audit/rollback read-model proof only.
- [x] Tests/proof listed in this workpack are implemented.
- [x] Security/no-claim negative proof captured: audit/rollback read model does
      not emit a service event, render portal UI, run the evaluator, start or
      schedule a timer, persist scheduler runtime/state-store rows, write audit
      runtime rows, write durable audit logs, execute rollback, dispatch an
      adapter, deliver child UX, enforce platform controls, or expose raw
      private source rows.
- [x] Feature/expectation/product-checklist/README update decision recorded.
- [x] Known gaps, deferred items, and no-claim boundaries recorded.

## Scope

Create a parent-domain proof that consumes WP83 source-gated policy preview
timer audit/rollback handoff rows and projects the same native app/native game
audit trail, rollback plan, and audit/rollback read-model proof requirements
into a parent-visible read-model contract. Rows blocked by source freshness or
compiler decisions remain blocked before audit/rollback visibility.

## Evidence

- `packages/parent-domain/src/app-game-source-gated-policy-preview-timer-audit-rollback-read-model.ts`
- `packages/parent-domain/src/app-game-source-gated-policy-preview-timer-audit-rollback-read-model-rules.ts`
- `packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-audit-rollback-read-model.test.ts`
- `scripts/test/app-game-source-gated-policy-preview-timer-audit-rollback-read-model-proof.mjs`
- `test-results/app-game-source-gated-policy-preview-timer-audit-rollback-read-model-proof/proof.json`
- `output/app-game-plan-proof/84-source-gated-policy-preview-timer-audit-rollback-read-model/proof.json`

## No-Claim Boundaries

- No package manifest export.
- No service runtime event.
- No portal UI.
- No policy evaluator runtime.
- No timer runtime.
- No timer scheduling.
- No scheduler persistence runtime.
- No durable scheduler state-store rows.
- No audit runtime.
- No durable audit log.
- No rollback runtime.
- No rollback execution.
- No adapter dispatch.
- No child delivery.
- No platform enforcement.
- No raw private source rows.

Product capability checklist remains unchanged because this proof adds
audit/rollback read-model classification evidence without moving feature
status.
