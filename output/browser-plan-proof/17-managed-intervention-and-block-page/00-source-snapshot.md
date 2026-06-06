# WP17 Source Snapshot

Workpack: `docs/plans/browser-plan/workpacks/17-managed-intervention-and-block-page.md`

Branch: `codex/browser-plan-implementation`

Base commit during implementation: `3aba15e716e1af59c41aadddbc9251ed99ca15bb`

Status: dirty worker lane with earlier browser-plan workpacks plus WP17 edits. `docs/product-capability-checklist.md` is intentionally clean and not part of this proof pack so A/primary can reconcile PR236 checklist rows.

Inspected source paths:

- `packages/activity-domain/src/browser-intervention-schemas.ts`
- `packages/activity-domain/src/browser-intervention.ts`
- `packages/activity-domain/src/browser.ts`
- `crates/agent-protocol/src/browser_intervention*.rs`
- `crates/agent-core/src/browser_intervention_event.rs`
- `crates/agent-core/src/activity_store_browser_intervention*.rs`
- `crates/agent-service/src/activity_api/browser_intervention_*.rs`
- `apps/portal/src/browser-intervention-read-model.ts`
- `apps/portal/src/browser-intervention-panel.ts`
- `scripts/test/managed-browser-intervention-proof.mjs`

Before-state gap:

- Managed browser intervention proof existed as a real browser harness for basic block pages, but intervention rows did not carry typed action ids, audit ids, evidence refs, or child delivery state through the TypeScript contract, Rust journal/read-model path, service payload, and portal parser.
- Social signup, social short-video, browser-game, game-purchase, and cloud-gaming proof cases were not represented in the managed intervention proof matrix.
